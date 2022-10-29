#![feature(once_cell)]

use std::ffi::CStr;
use skyline::hooks::InlineCtx;


mod fuse;
mod criware;
mod utils;

#[macro_export]
macro_rules! reg_x {
    ($ctx:ident, $no:expr) => {
        unsafe { *$ctx.registers[$no].x.as_ref() }
    };
}

#[skyline::hook(offset = 0x1025e3c, inline)]
pub fn sixty_fps_hook(ctx: &mut InlineCtx) {
    unsafe {
        *ctx.registers[1].x.as_mut() = 0u64;
    }
}

#[skyline::hook(offset = 0x8a9134, inline)]
pub fn mount_directories(_: &InlineCtx) {
    let binder_hn = unsafe { criware::bind::get_binder_handle() };

    // fuse::mods::install_mods_vfs();

    // criware::bind::bind_directory(binder_hn, "mods:/").unwrap();
    criware::bind::bind_directory(binder_hn, "sd:/p5r").unwrap();
    criware::bind::bind_directory(binder_hn, "app0:/CPK/BIND").unwrap();
}

#[skyline::hook(offset = 0x130a930)]
pub fn load_file_hook(unk1: *const u8, binder: *const u8, filepath: *const u8, offset: u64, filesize: u64) -> i32 {
    let filename = unsafe { CStr::from_ptr(filepath as _) };
    
    println!("Filepath: {}", filename.to_str().unwrap());

    call_original!(unk1, binder, filepath, offset, filesize)
}

#[skyline::hook(offset = 0x12f0910, inline)]
pub fn print_criware_error(ctx: &InlineCtx) {
    let message = unsafe { CStr::from_ptr(reg_x!(ctx, 1) as *const u8 as _) };

    println!("Criware: {}", message.to_str().unwrap());
}

#[skyline::hook(offset = 0x7ef500)]
pub fn is_platform_pc() -> bool {
    true
}

#[skyline::main(name = "cpk")]
pub fn main() {
    // Install a panic handler to display a native error popup on Switch
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "Skyline plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(sixty_fps_hook, print_criware_error, load_file_hook, mount_directories);
}

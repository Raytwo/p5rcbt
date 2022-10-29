use std::{
    ffi::CStr,
    path::PathBuf
};
use camino::Utf8Path;
use skyline::hooks::InlineCtx;
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum DirBindingError {
    #[error("failed to bind directory with result: {0}")]
    CriwareErrorCode(i32),
    #[error("the path provided is not absolute: {0}")]
    PathNotAbsolute(PathBuf),
    #[error("the path provided is not a directory: {0}")]
    NotADirectory(PathBuf),
    #[error("the path provided does not exist on the mount point: {0}")]
    DirectoryDoesNotExist(PathBuf)
}

pub type CriFnBinderHandle = u64;

#[skyline::from_offset(0x8a92d0)]
pub fn get_binder_handle() -> CriFnBinderHandle;

#[skyline::from_offset(0x1302710)]
pub fn crifsbinder_bind_directory(binder: CriFnBinderHandle, src_binder: *const u8, path: *const u8, work: *const u8, work_size: i32, bind_id: &mut u32) -> i32;

pub fn bind_directory<P: AsRef<Utf8Path>>(binder_handle: CriFnBinderHandle, path: P) -> Result<u32, DirBindingError> {
    let path = path.as_ref();

    // nn::fs::CheckMountName?

    if !path.is_absolute() {
        Err(DirBindingError::PathNotAbsolute(path.to_path_buf().into_std_path_buf()))
    } else if !path.is_dir() {
        Err(DirBindingError::NotADirectory(path.to_path_buf().into_std_path_buf()))
    } else if !path.exists() {
        Err(DirBindingError::DirectoryDoesNotExist(path.to_path_buf().into_std_path_buf()))
    } else {
        let nullterm_path = format!("{}\0", path);

        let bind_path = skyline::c_str(&nullterm_path);
        let mut out_bind_id = 0;

        let result = unsafe { crifsbinder_bind_directory(binder_handle, 0 as _, bind_path, 0 as _, 0, &mut out_bind_id) };

        if result != 0 {
            Err(DirBindingError::CriwareErrorCode(result))
        } else {
            Ok(out_bind_id)
        }
    }
}

#[skyline::hook(offset = 0x8a9134, inline)]
pub fn mount_directories(_: &InlineCtx) {
    let binder_hn = unsafe { get_binder_handle() };

    bind_directory(binder_hn, "sd:/p5r").unwrap();
    bind_directory(binder_hn, "app0:/CPK/BIND").unwrap();
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

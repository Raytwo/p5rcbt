use std::{ffi::{CStr, CString}, fs::read_dir, str::FromStr, sync::LazyLock};
use camino::{Utf8Path, Utf8PathBuf};
use semver::Version;
use serde::{Deserialize, Serialize};
use skyline::hooks::InlineCtx;

use crate::script::Script;

mod criware;
mod script;
mod utils;

// Old offset: 0x1025e3c
#[skyline::hook(offset = 0x102603c, inline)]
pub fn sixty_fps_hook(ctx: &mut InlineCtx) {
    ctx.registers[1].set_x(0);
}

#[skyline::hook(offset = 0x8a9134, inline)]
pub fn cri_bind_cpk_hook(ctx: &InlineCtx) {
    let binder_hn = unsafe { criware::bind::get_binder_handle() };

    // Merge mods into one virtual filesystem that can then be mounted and accessed by Criware like normal.
    // fuse::mods::install_mods_vfs();
    // criware::bind::bind_directory(binder_hn, "mods:/").unwrap();

    let base_dir = Utf8PathBuf::from_str("sd:/p5r/bind").unwrap();

    // Bind CPKs first
    if let Ok(reader) = read_dir(&base_dir) {
        let mut entries: Vec<_> = reader
            .filter_map(Result::ok)
            .filter(|idk| idk.path().is_file() && Utf8Path::from_path(&idk.path()).unwrap().extension().map(|ext| ext.to_lowercase()) == Some(String::from("cpk")))
            .collect();

        entries.sort_by_key(|dir| dir.file_name());
        entries.reverse();

        // Only take valid entries
        for entry in entries {
            let cpk_path = CString::new(entry.path().to_str().unwrap()).unwrap();

            let result = unsafe { criware::bind::crifsbinder_bind_cpk(ctx.registers[0].x(), cpk_path.as_ptr() as _) };

            if result != 1 {
                panic!("Error while trying to bind a CPK for Criware: {}", result);
            }
        }
    }

    // Don't bother running the rest if the bind directory doesnt even exist.
    if let Ok(reader) = read_dir(&base_dir) {
        let mut entries: Vec<_> = reader
            .filter_map(Result::ok)
            .filter(|idk| idk.path().is_dir())
            .collect();

        entries.sort_by_key(|dir| dir.file_name());
        entries.reverse();

        // Only take valid entries
        for entry in entries {
            if let Err(error) = criware::bind::bind_directory(binder_hn, Utf8Path::from_path(&entry.path()).unwrap()) {
                panic!("Error while trying to bind a directory for Criware: {}", error);
            }
        }
    }
}

// Old offset: 0x130a930
#[skyline::hook(offset = 0x130ab30)]
pub fn load_file_hook(unk1: *const u8, binder: *const u8, filepath: *const u8, offset: u64, filesize: u64) -> i32 {
    let filename = unsafe { CStr::from_ptr(filepath as _) };
    
    println!("Filepath: {}", filename.to_str().unwrap());

    call_original!(unk1, binder, filepath, offset, filesize)
}

// Old offset: 0x12f0910
#[skyline::hook(offset = 0x12f0b10, inline)]
pub fn print_criware_error(ctx: &InlineCtx) {
    let message = unsafe { CStr::from_ptr(ctx.registers[1].x() as *const i8) };

    println!("Criware: {}", message.to_str().unwrap());
}

#[skyline::hook(offset = 0x7ef500)]
pub fn is_platform_pc() -> bool {
    true
}

#[skyline::hook(offset = 0x00e33520)]
pub fn PUT_fix() -> i32 
{
    let in_arg = Script::get_int_arg(0);
    println!("{}", in_arg);

    1
}

#[skyline::hook(offset = 0x00e33540)]
pub fn PUTS_fix() -> i32 
{
    let in_arg = Script::get_string_arg(0);
    let c_str = unsafe { CStr::from_ptr(in_arg as *const i8) };
    println!("{:?}", c_str);
    
    1
}

#[skyline::hook(offset = 0x00e33560)]
pub fn PUTF_fix() -> i32 
{
    let in_arg = Script::get_float_arg(0);
    println!("{}", in_arg);

    1
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub logging: bool,
    pub uncap_framerate: bool,
    pub pc_settings: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { logging: false, uncap_framerate: true, pc_settings: false }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    if let Ok(text) = std::fs::read("sd:/p5r/cfg/cbt.toml") {
        toml::from_slice(&text).unwrap_or_default()
    } else {
        Config::default()
    }
});

#[skyline::main(name = "cbt")]
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

        println!("P5R CBT v{}\nLocation: {}\n\n{}", utils::get_plugin_version(), location, msg);

        let err_msg = format!(
            "P5R CBT v{}\nLocation: {}\n\n{}\0",
            utils::get_plugin_version(),
            location,
            msg
        );

        skyline::error::show_error(69, "P5R CBT has panicked! Press 'Details' for more information.\n\0", err_msg.as_str());
    }));

    let current_ver = utils::get_game_version();

    if current_ver != Version::new(1, 0, 2) {
        skyline::error::show_error(69,
            &format!("P5RCBT is only compatible with Persona 5 Royal version 1.0.2, but you are running version {current_ver}."),
            &format!("P5RCBT is only compatible with Persona 5 Royal version 1.0.2, but you are running version {current_ver}.\nConsider updating your game or uninstalling P5R CBT.\n\nP5R CBT will not run for this play session.")
        );

        return;
    }

    if CONFIG.logging {
        skyline::install_hooks!(load_file_hook, print_criware_error);
    }

    if CONFIG.pc_settings {
        skyline::install_hook!(is_platform_pc);
    }

    if CONFIG.uncap_framerate {
        skyline::install_hook!(sixty_fps_hook);
    }

    skyline::install_hooks!(cri_bind_cpk_hook, PUT_fix, PUTS_fix, PUTF_fix);

    println!("P5R CBT is installed and running");
}

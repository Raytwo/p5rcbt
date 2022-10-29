use std::str::FromStr;
use skyline::nn;
use semver::Version;

pub mod env {
    use std::sync::LazyLock;

    #[non_exhaustive]
    pub enum RunEnvironment {
        Switch,
        Ryujinx,
        // Yuzu
    }

    static PLATFORM: LazyLock<RunEnvironment> = LazyLock::new(|| {
        if unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 } == 0x8004000 {
            RunEnvironment::Ryujinx
        } else {
            RunEnvironment::Switch
        }
    });

    pub fn get_running_env() -> &'static RunEnvironment {
        &PLATFORM
    }

    pub fn is_emulator() -> bool {
        matches!(get_running_env(), RunEnvironment::Switch)
    }

    pub fn is_ryujinx() -> bool {
        matches!(get_running_env(), RunEnvironment::Ryujinx)
    }
}

/// Wrapper function for getting the version string of the game from nnSdk
pub fn get_game_version() -> Version {
    unsafe {
        let mut version_string = nn::oe::DisplayVersion { name: [0x00; 16] };
        nn::oe::GetDisplayVersion(&mut version_string);
        Version::from_str(&skyline::from_c_str(version_string.name.as_ptr())).expect("The game's display version should parse as a proper semver.")
    }
}

pub fn get_plugin_version() -> Version {
    Version::from_str(env!("CARGO_PKG_VERSION")).expect("The plugin's version should follow proper semver.")
}

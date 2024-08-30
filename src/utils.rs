use std::str::FromStr;
use skyline::nn;
use semver::Version;

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

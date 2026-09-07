#![doc = include_str!("../README.md")]

use std::path::{Path, PathBuf};

const BIN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/prebuilt_binaries");

/// Target platform for ARM Neural Graphics libraries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    X64Windows,
    Aarch64Android,
}

impl Platform {
    /// Returns the platform matching the Cargo `CARGO_CFG_TARGET_OS` and `CARGO_CFG_TARGET_ARCH`
    /// environment variables.
    ///
    /// Cargo sets this for build scripts. Panics outside that context.  Returns [`None`] for
    /// unsupported OS/ARCH combinations.
    pub fn for_current_target() -> Option<Self> {
        let arch = std::env::var("CARGO_CFG_TARGET_ARCH")
            .expect("CARGO_CFG_TARGET_ARCH not set (are you in a build script?)");
        let os = std::env::var("CARGO_CFG_TARGET_OS")
            .expect("CARGO_CFG_TARGET_OS not set (are you in a build script?)");
        Some(match (arch.as_str(), os.as_str()) {
            ("x86_64", "windows") => Self::X64Windows, // TODO: Assert MSVC?
            // No Linux x64 or Android library is checked in for SDK v1.0.1: it publishes no
            // prebuilt Linux x64 library at all (that landed in v1.1.0), and its in-tree Android
            // library is a 110MB unstripped build that exceeds GitHub's file size limit.  Both
            // would have to be built from source to come back here, so report the target as
            // unsupported rather than handing out a path to a file that does not exist.
            _ => return None,
        })
    }
}

/// Returns the path to the link library for the given platform.
pub fn lib_path(platform: Platform) -> Result<PathBuf, String> {
    let bin_dir = Path::new(BIN_DIR);

    assert!(bin_dir.exists());

    let filename = match platform {
        Platform::X64Windows => "ngsdk_windows_x64.lib",
        x => return Err(format!("No link-library for {x:?}")),
    };

    Ok(bin_dir.join(filename))
}

/// Returns the path to a shared library for the given platform.
pub fn path(platform: Platform) -> PathBuf {
    let bin_dir = Path::new(BIN_DIR);

    assert!(bin_dir.exists());

    let filename = match platform {
        Platform::X64Windows => "ngsdk_windows_x64.dll",
        Platform::Aarch64Android => "libngsdk_android_arm.so",
    };

    bin_dir.join(filename)
}

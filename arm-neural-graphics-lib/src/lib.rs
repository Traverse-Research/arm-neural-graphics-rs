use std::path::{Path, PathBuf};

pub fn dll_path() -> PathBuf {
    Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("PrebuiltBinary/ngsdk_windows_x64.dll")
}

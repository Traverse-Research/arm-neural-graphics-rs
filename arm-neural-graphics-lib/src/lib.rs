use std::path::{Path, PathBuf};

pub fn dll_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("PrebuiltBinary/ngsdk_windows_x64.dll")
}

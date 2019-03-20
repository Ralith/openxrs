use std::sync::Arc;
use std::ptr;
use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::LinkedEntry::new();
    #[cfg(not(feature = "static"))]
    let entry = xr::LoadedEntry::load().unwrap();
    let instance = entry.create_instance(&xr::ApplicationInfo {
        application_name: "hello openxrs",
        application_version: 0,
        engine_name: "openxrs",
        engine_version: 0,
    });
}

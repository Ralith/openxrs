use std::sync::Arc;
use std::ptr;
use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::LinkedEntry::new();
    #[cfg(not(feature = "static"))]
    let entry = xr::LoadedEntry::load().unwrap();
    unsafe {
        let instance = xr::Instance::create(Arc::new(entry), &xr::sys::InstanceCreateInfo {
            ty: xr::sys::StructureType::INSTANCE_CREATE_INFO,
            next: ptr::null(),
            create_flags: Default::default(),
            application_info: xr::sys::ApplicationInfo {
                application_name: [0; xr::sys::MAX_APPLICATION_NAME_SIZE],
                application_version: 0,
                engine_name: [0; xr::sys::MAX_ENGINE_NAME_SIZE],
                engine_version: 0,
                api_version: xr::sys::CURRENT_API_VERSION,
            },
            enabled_api_layer_count: 0,
            enabled_api_layer_names: ptr::null(),
            enabled_extension_count: 0,
            enabled_extension_names: ptr::null(),
        }).unwrap();
    }
}

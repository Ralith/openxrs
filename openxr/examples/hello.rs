use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load().unwrap();

    let bindings = entry.enumerate_graphics_bindings().unwrap();
    println!("supported graphics bindings: {:?}", bindings);
    if !bindings.vulkan {
        panic!("vulkan unsupported");
    }
    let instance = entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "hello openxrs",
                application_version: 0,
                engine_name: "openxrs",
                engine_version: 0,
            },
            &xr::GraphicsBindings::VULKAN,
        )
        .unwrap();
    let instance_props = instance.properties().unwrap();
    println!(
        "loaded instance: {} v{}",
        instance_props.runtime_name, instance_props.runtime_version
    );

    let system = instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();
    let system_props = instance.system_properties(system).unwrap();
    println!(
        "selected system {}: {}",
        system_props.system_id.into_raw(),
        if system_props.system_name.is_empty() {
            "<unnamed>"
        } else {
            &system_props.system_name
        }
    );
}

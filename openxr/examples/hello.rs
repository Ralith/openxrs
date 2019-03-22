use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load().unwrap();

    let extensions = entry.enumerate_extensions().unwrap();
    println!("supported extensions: {:?}", extensions);
    if !extensions.khr_vulkan_enable {
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
            &xr::ExtensionSet {
                khr_vulkan_enable: true,
                ..Default::default()
            },
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

    if !instance
        .enumerate_view_configurations(system)
        .unwrap()
        .contains(&xr::ViewConfigurationType::PRIMARY_STEREO)
    {
        panic!("no primary stereo views");
    }

    let view_config_views = instance
        .enumerate_view_configuration_views(system, xr::ViewConfigurationType::PRIMARY_STEREO)
        .unwrap();
    println!("view configuration views: {:?}", view_config_views);
}

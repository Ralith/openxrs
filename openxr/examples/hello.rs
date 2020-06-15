use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load()
        .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

    let extensions = entry.enumerate_extensions().unwrap();
    println!("supported extensions: {:#?}", extensions);
    let layers = entry.enumerate_layers().unwrap();
    println!("supported layers: {:?}", layers);
    let instance = entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "hello openxrs",
                ..Default::default()
            },
            &xr::ExtensionSet::default(),
            &[],
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

    let view_config_views = instance
        .enumerate_view_configuration_views(system, xr::ViewConfigurationType::PRIMARY_STEREO)
        .unwrap();
    println!("view configuration views: {:#?}", view_config_views);
}

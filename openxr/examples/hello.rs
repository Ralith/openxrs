use openxr as xr;

#[cfg_attr(target_os = "android", ndk_glue::main)]
fn main() {
    #[cfg(feature = "linked")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "linked"))]
    let entry = unsafe {
        xr::Entry::load()
            .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature")
    };

    #[cfg(target_os = "android")]
    entry.initialize_android_loader();

    let extensions = entry.enumerate_extensions().unwrap();
    println!("supported extensions: {:#?}", extensions);
    let layers = entry.enumerate_layers().unwrap();
    println!("supported layers: {:?}", layers);
    println!("layer extensions:");
    for layer in layers {
        let extensions = entry.enumerate_layer_extensions(&layer.layer_name).unwrap();
        let extension_names = extensions.names();
        if !extension_names.is_empty() {
            println!("  - {}:", layer.layer_name);
            for ext in extension_names {
                println!("    - {}", String::from_utf8_lossy(ext));
            }
        }
    }
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

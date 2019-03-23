use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load().unwrap();

    let extensions = entry.enumerate_extensions().unwrap();
    println!("supported extensions: {:?}", extensions);
    if !extensions.khr_headless {
        panic!("headless unsupported");
    }
    let instance = entry
        .create_instance(
            xr::ApplicationInfo::new()
                .application_name("hello openxrs")
                .engine_name("hello openxrs"),
            &xr::ExtensionSet {
                khr_headless: true,
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

    let session = instance.create_session_headless(system).unwrap();
    session.begin(xr::ViewConfigurationType::PRIMARY_STEREO).unwrap();
    
    let space_tys = session.enumerate_reference_spaces().unwrap();
    println!("reference spaces: {:?}", space_tys);
    let has_stage = space_tys.contains(&xr::ReferenceSpaceType::STAGE);
    let has_view = space_tys.contains(&xr::ReferenceSpaceType::VIEW);

    let stage = if has_stage {
        Some(session.create_reference_space(
            xr::ReferenceSpaceType::STAGE,
            xr::Posef {
                position: xr::Vector3f { x: 0.0, y: 0.0, z: 0.0 },
                orientation: xr::Quaternionf { w: 1.0, x: 0.0, y: 0.0, z: 0.0 },
            }).unwrap())
    } else { None };

    let view = if has_view {
        Some(session.create_reference_space(
            xr::ReferenceSpaceType::VIEW,
            xr::Posef {
                position: xr::Vector3f { x: 0.0, y: 0.0, z: 0.0 },
                orientation: xr::Quaternionf { w: 1.0, x: 0.0, y: 0.0, z: 0.0 },
            }).unwrap())
    } else { None };

    let mut buffer = xr::EventDataBuffer::new();
    while let Some(e) = instance.poll_event(&mut buffer).unwrap() {
        use xr::Event::*;
        match e {
            SessionStateChanged(e) => {
                println!("session stage changed to {:?} at t={:?}", e.state(), e.time());
            }
            _ => {
                println!("unhand led event");
            }
        }
    }

    if let (&Some(ref stage), &Some(ref view)) = (&stage, &view) {
        let relation = view.locate(&stage, xr::Time::from_raw(0)).unwrap();
        let p = relation.pose.position;
        println!("view space is at: {} {} {}", p.x, p.y, p.z);
    }

    if let Some(ref stage) = stage {
        let (flags, views) = session.locate_views(xr::Time::from_raw(0), stage).unwrap();
        println!("view flags: {:?}", flags);
        for (i, view) in views.iter().enumerate() {
            println!("view {}:", i);
            let p = view.pose.position;
            println!("\tposition: {} {} {}", p.x, p.y, p.z);
            let f = view.fov;
            println!("\tfov: {} {} {} {}", f.angle_left, f.angle_right, f.angle_up, f.angle_down);
        }
    }

    session.end().unwrap();
}

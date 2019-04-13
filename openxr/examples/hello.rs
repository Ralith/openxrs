use openxr as xr;

fn main() {
    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load()
        .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

    let extensions = entry.enumerate_extensions().unwrap();
    println!("supported extensions: {:?}", extensions);
    if !extensions.khr_headless {
        panic!("headless unsupported");
    }
    if !extensions.khr_convert_timespec_time {
        panic!("timespec conversion unsupported");
    }
    let instance = entry
        .create_instance(
            xr::ApplicationInfo::new().application_name("hello openxrs"),
            &xr::ExtensionSet {
                khr_headless: true,
                khr_convert_timespec_time: true,
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
    session
        .begin(xr::ViewConfigurationType::PRIMARY_STEREO)
        .unwrap();

    let space_tys = session.enumerate_reference_spaces().unwrap();
    println!("reference spaces: {:?}", space_tys);
    let has_stage = space_tys.contains(&xr::ReferenceSpaceType::STAGE);
    let has_view = space_tys.contains(&xr::ReferenceSpaceType::VIEW);

    let stage = if has_stage {
        Some(
            session
                .create_reference_space(
                    xr::ReferenceSpaceType::STAGE,
                    xr::Posef {
                        position: xr::Vector3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: xr::Quaternionf {
                            w: 1.0,
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                )
                .unwrap(),
        )
    } else {
        None
    };

    let view = if has_view {
        Some(
            session
                .create_reference_space(
                    xr::ReferenceSpaceType::VIEW,
                    xr::Posef {
                        position: xr::Vector3f {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        orientation: xr::Quaternionf {
                            w: 1.0,
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                )
                .unwrap(),
        )
    } else {
        None
    };

    if let (&Some(ref stage), &Some(ref view)) = (&stage, &view) {
        let relation = view.locate(&stage, xr::Time::from_raw(0)).unwrap();
        let p = relation.pose.position;
        println!("view space is at: {} {} {}", p.x, p.y, p.z);
    }

    let action_set = session
        .create_action_set("actions", "Example Action Set", 0)
        .unwrap();
    let action = action_set
        .create_action::<bool>("ping", "Ping", &[])
        .unwrap();
    let binding_l = unsafe {
        instance
            .string_to_path("/user/hand/left/input/select/click")
            .unwrap()
    };
    let binding_r = unsafe {
        instance
            .string_to_path("/user/hand/right/input/select/click")
            .unwrap()
    };

    let simple_profile = unsafe {
        instance
            .string_to_path("/interaction_profiles/khr/simple_controller")
            .unwrap()
    };
    session
        .set_interaction_profile_suggested_bindings(
            simple_profile,
            &[
                xr::Binding::new(&action, binding_l),
                xr::Binding::new(&action, binding_r),
            ],
        )
        .unwrap();

    'main: loop {
        let mut buffer = xr::EventDataBuffer::new();
        while let Some(e) = instance.poll_event(&mut buffer).unwrap() {
            use xr::Event::*;
            match e {
                SessionStateChanged(e) => {
                    println!(
                        "session stage changed to {:?} at t={:?}",
                        e.state(),
                        e.time()
                    );
                    match e.state() {
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            break 'main;
                        }
                        _ => {}
                    }
                }
                _ => {
                    println!("unhandled event");
                }
            }
        }

        session
            .sync_action_data(&[xr::ActiveActionSet::new(&action_set)])
            .unwrap();
        if action.state(&[]).unwrap().current_state {
            println!("pong");
        }

        if let Some(ref stage) = stage {
            let now = instance.now().unwrap();
            let (flags, views) = session.locate_views(now, stage).unwrap();
            println!("view flags: {:?}", flags);
            for (i, view) in views.iter().enumerate() {
                let p = view.pose.position;
                println!("view {}: [{} {} {}]", i, p.x, p.y, p.z);
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    session.end().unwrap();
}

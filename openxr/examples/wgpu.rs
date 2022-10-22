use std::{
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use ash::vk::{self, Handle};
use openxr as xr;

use wgpu_hal::{api::Vulkan as HalVulkan, Api};

pub const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;
pub const VIEW_COUNT: u32 = 2;
const VIEW_TYPE: xr::ViewConfigurationType = xr::ViewConfigurationType::PRIMARY_STEREO;

/// Maximum number of frames in flight
const PIPELINE_DEPTH: u32 = 2;

struct Swapchain {
    handle: xr::Swapchain<xr::Vulkan>,
    width: u32,
    height: u32,
    textures: Vec<wgpu::Texture>,
}

fn main() {
    // Handle interrupts gracefully
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
    .expect("setting Ctrl-C handler");

    #[cfg(feature = "static")]
    let xr_entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let xr_entry = unsafe {
        xr::Entry::load()
            .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature")
    };

    #[cfg(target_os = "android")]
    xr_entry.initialize_android_loader().unwrap();

    // OpenXR will fail to initialize if we ask for an extension that OpenXR can't provide! So we
    // need to check all our extensions before initializing OpenXR with them. Note that even if the
    // extension is present, it's still possible you may not be able to use it. For example: the
    // hand tracking extension may be present, but the hand sensor might not be plugged in or turned
    // on. There are often additional checks that should be made before using certain features!
    let available_extensions = xr_entry.enumerate_extensions().unwrap();

    // If a required extension isn't present, you want to ditch out here! It's possible something
    // like your rendering API might not be provided by the active runtime. APIs like OpenGL don't
    // have universal support.
    assert!(available_extensions.khr_vulkan_enable2);

    // Initialize OpenXR with the extensions we've found!
    let mut enabled_extensions = xr::ExtensionSet::default();
    enabled_extensions.khr_vulkan_enable2 = true;
    #[cfg(target_os = "android")]
    {
        enabled_extensions.khr_android_create_instance = true;
    }
    let xr_instance = xr_entry
        .create_instance(
            &xr::ApplicationInfo {
                application_name: "openxrs example",
                application_version: 0,
                engine_name: "openxrs example",
                engine_version: 0,
            },
            &enabled_extensions,
            &[],
        )
        .unwrap();
    let instance_props = xr_instance.properties().unwrap();
    println!(
        "loaded OpenXR runtime: {} {}",
        instance_props.runtime_name, instance_props.runtime_version
    );

    // Request a form factor from the device (HMD, Handheld, etc.)
    let system = xr_instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();

    // Check what blend mode is valid for this device (opaque vs transparent displays). We'll just
    // take the first one available!
    let environment_blend_mode = xr_instance
        .enumerate_environment_blend_modes(system, VIEW_TYPE)
        .unwrap()[0];

    // OpenXR wants to ensure apps are using the correct graphics card and Vulkan features and
    // extensions, so the instance and device MUST be set up before Instance::create_session.

    let vk_target_version = vk::make_api_version(0, 1, 1, 0); // Vulkan 1.1 guarantees multiview support
    let vk_target_version_xr = xr::Version::new(1, 1, 0);

    let reqs = xr_instance
        .graphics_requirements::<xr::Vulkan>(system)
        .unwrap();

    if vk_target_version_xr < reqs.min_api_version_supported
        || vk_target_version_xr.major() > reqs.max_api_version_supported.major()
    {
        panic!(
            "OpenXR runtime requires Vulkan version > {}, < {}.0.0",
            reqs.min_api_version_supported,
            reqs.max_api_version_supported.major() + 1
        );
    }

    let (_wgpu_instance, _adapter, device, queue, session, mut frame_wait, mut frame_stream) = unsafe {
        let vk_entry = ash::Entry::load().unwrap();

        let vk_app_info = vk::ApplicationInfo::builder()
            .application_version(0)
            .engine_version(0)
            .api_version(vk_target_version);

        let vk_instance = {
            let vk_instance = xr_instance
                .create_vulkan_instance(
                    system,
                    std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
                    &vk::InstanceCreateInfo::builder().application_info(&vk_app_info) as *const _
                        as *const _,
                )
                .expect("XR error creating Vulkan instance")
                .map_err(vk::Result::from_raw)
                .expect("Vulkan error creating Vulkan instance");
            ash::Instance::load(
                vk_entry.static_fn(),
                vk::Instance::from_raw(vk_instance as _),
            )
        };

        let vk_physical_device = vk::PhysicalDevice::from_raw(
            xr_instance
                .vulkan_graphics_device(system, vk_instance.handle().as_raw() as _)
                .unwrap() as _,
        );

        let vk_device_properties = vk_instance.get_physical_device_properties(vk_physical_device);
        if vk_device_properties.api_version < vk_target_version {
            vk_instance.destroy_instance(None);
            panic!("Vulkan phyiscal device doesn't support version 1.1");
        }

        let queue_family_index = vk_instance
            .get_physical_device_queue_family_properties(vk_physical_device)
            .into_iter()
            .enumerate()
            .find_map(|(queue_family_index, info)| {
                if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    Some(queue_family_index as u32)
                } else {
                    None
                }
            })
            .expect("Vulkan device has no graphics queue");

        let vk_device = {
            let vk_device = xr_instance
                .create_vulkan_device(
                    system,
                    std::mem::transmute(vk_entry.static_fn().get_instance_proc_addr),
                    vk_physical_device.as_raw() as _,
                    &vk::DeviceCreateInfo::builder()
                        .queue_create_infos(&[vk::DeviceQueueCreateInfo::builder()
                            .queue_family_index(queue_family_index)
                            .queue_priorities(&[1.0])
                            .build()])
                        .push_next(&mut vk::PhysicalDeviceMultiviewFeatures {
                            multiview: vk::TRUE,
                            ..Default::default()
                        }) as *const _ as *const _,
                )
                .expect("XR error creating Vulkan device")
                .map_err(vk::Result::from_raw)
                .expect("Vulkan error creating Vulkan device");

            ash::Device::load(vk_instance.fp_v1_0(), vk::Device::from_raw(vk_device as _))
        };

        // A session represents this application's desire to display things! This is where we hook
        // up our graphics API. This does not start the session; for that, you'll need a call to
        // Session::begin, which we do in 'main_loop below.
        let (session, frame_wait, frame_stream) = xr_instance
            .create_session::<xr::Vulkan>(
                system,
                &xr::vulkan::SessionCreateInfo {
                    instance: vk_instance.handle().as_raw() as _,
                    physical_device: vk_physical_device.as_raw() as _,
                    device: vk_device.handle().as_raw() as _,
                    queue_family_index,
                    queue_index: 0,
                },
            )
            .unwrap();

        // Now wrap all our Vulkan stuff into WGPU stuff

        type HalInstance = <HalVulkan as Api>::Instance;
        let flags = wgpu_hal::InstanceFlags::empty();
        let extensions = HalInstance::required_extensions(&vk_entry, flags).unwrap();
        let hal_instance = HalInstance::from_raw(
            vk_entry,
            vk_instance,
            vk_target_version,
            0,
            extensions,
            flags,
            false,
            Some(Box::new(())),
        )
        .unwrap();

        let hal_exposed_adapter = hal_instance.expose_adapter(vk_physical_device).unwrap();
        println!(
            "Using adapter {:?}; features: {:?}",
            hal_exposed_adapter.info, hal_exposed_adapter.features
        );

        let limits = hal_exposed_adapter.capabilities.limits.clone();
        let phd_limits = hal_exposed_adapter
            .adapter
            .physical_device_capabilities()
            .properties()
            .limits;
        let uab_types = wgpu_hal::UpdateAfterBindTypes::from_limits(&limits, &phd_limits);

        let enabled_extensions = hal_exposed_adapter
            .adapter
            .required_device_extensions(hal_exposed_adapter.features);
        let hal_device = hal_exposed_adapter
            .adapter
            .device_from_raw(
                vk_device,
                true,
                &enabled_extensions,
                hal_exposed_adapter.features,
                uab_types,
                queue_family_index,
                0,
            )
            .unwrap();

        let device_desc = wgpu::DeviceDescriptor {
            label: None,
            features: hal_exposed_adapter.features,
            limits,
        };

        let wgpu_instance = wgpu::Instance::from_hal::<HalVulkan>(hal_instance);
        let wgpu_adapter = wgpu_instance.create_adapter_from_hal(hal_exposed_adapter);
        let (wgpu_device, wgpu_queue) = wgpu_adapter
            .create_device_from_hal(hal_device, &device_desc, None)
            .expect("Failed to create device");

        (
            wgpu_instance,
            wgpu_adapter,
            wgpu_device,
            wgpu_queue,
            session,
            frame_wait,
            frame_stream,
        )
    };

    // Load the shaders from disk
    let vert_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::util::make_spirv(include_bytes!("fullscreen.vert.spv")),
    });

    let frag_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::util::make_spirv(include_bytes!("debug_pattern.frag.spv")),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_format = wgpu::TextureFormat::Rgba8UnormSrgb;

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vert_shader,
            entry_point: "main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag_shader,
            entry_point: "main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: NonZeroU32::new(VIEW_COUNT),
    });

    // Create an action set to encapsulate our actions
    let action_set = xr_instance
        .create_action_set("input", "input pose information", 0)
        .unwrap();

    let right_action = action_set
        .create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])
        .unwrap();
    let left_action = action_set
        .create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])
        .unwrap();

    // Bind our actions to input devices using the given profile
    // If you want to access inputs specific to a particular device you may specify a different
    // interaction profile
    xr_instance
        .suggest_interaction_profile_bindings(
            xr_instance
                .string_to_path("/interaction_profiles/khr/simple_controller")
                .unwrap(),
            &[
                xr::Binding::new(
                    &right_action,
                    xr_instance
                        .string_to_path("/user/hand/right/input/grip/pose")
                        .unwrap(),
                ),
                xr::Binding::new(
                    &left_action,
                    xr_instance
                        .string_to_path("/user/hand/left/input/grip/pose")
                        .unwrap(),
                ),
            ],
        )
        .unwrap();

    // Attach the action set to the session
    session.attach_action_sets(&[&action_set]).unwrap();

    // Create an action space for each device we want to locate
    let right_space = right_action
        .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
        .unwrap();
    let left_space = left_action
        .create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)
        .unwrap();

    // OpenXR uses a couple different types of reference frames for positioning content; we need
    // to choose one for displaying our content! STAGE would be relative to the center of your
    // guardian system's bounds, and LOCAL would be relative to your device's starting location.
    let stage = session
        .create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)
        .unwrap();

    let mut swapchain = {
        // Now we need to find all the viewpoints we need to take care of! This is a
        // property of the view configuration type; in this example we use PRIMARY_STEREO,
        // so we should have 2 viewpoints.
        //
        // Because we are using multiview in this example, we require that all view
        // dimensions are identical.
        let views = xr_instance
            .enumerate_view_configuration_views(system, VIEW_TYPE)
            .unwrap();
        assert_eq!(views.len(), VIEW_COUNT as usize);
        assert_eq!(views[0], views[1]);

        // Create a swapchain for the viewpoints! A swapchain is a set of texture buffers
        // used for displaying to screen, typically this is a backbuffer and a front buffer,
        // one for rendering data to, and one for displaying on-screen.

        let width = views[0].recommended_image_rect_width;
        let height = views[0].recommended_image_rect_height;

        let handle = session
            .create_swapchain(&xr::SwapchainCreateInfo {
                create_flags: xr::SwapchainCreateFlags::EMPTY,
                usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                    | xr::SwapchainUsageFlags::SAMPLED,
                format: vk::Format::R8G8B8A8_SRGB.as_raw() as _,
                // The Vulkan graphics pipeline we create is not set up for multisampling,
                // so we hardcode this to 1. If we used a proper multisampling setup, we
                // could set this to `views[0].recommended_swapchain_sample_count`.
                sample_count: 1,
                width,
                height,
                face_count: 1,
                array_size: VIEW_COUNT,
                mip_count: 1,
            })
            .unwrap();

        // We'll want to track our own information about the swapchain, so we can draw stuff
        // onto it! We'll also create a buffer for each generated texture here as well.
        let images = handle.enumerate_images().unwrap();
        let wgpu_texture_desc = wgpu::TextureDescriptor {
            label: Some("Swapchain Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: VIEW_COUNT,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: COLOR_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let hal_texture_desc = wgpu_hal::TextureDescriptor {
            label: wgpu_texture_desc.label,
            size: wgpu_texture_desc.size,
            mip_level_count: wgpu_texture_desc.mip_level_count,
            sample_count: wgpu_texture_desc.sample_count,
            dimension: wgpu_texture_desc.dimension,
            format: wgpu_texture_desc.format,
            usage: wgpu_hal::TextureUses::COLOR_TARGET | wgpu_hal::TextureUses::RESOURCE,
            memory_flags: wgpu_hal::MemoryFlags::empty(),
        };
        Swapchain {
            handle,
            width,
            height,
            textures: images
                .into_iter()
                .map(|color_image| unsafe {
                    let hal_texture = <HalVulkan as Api>::Device::texture_from_raw(
                        vk::Image::from_raw(color_image),
                        &hal_texture_desc,
                        None,
                    );
                    device.create_texture_from_hal::<HalVulkan>(hal_texture, &wgpu_texture_desc)
                })
                .collect(),
        }
    };

    // Main loop
    let mut event_storage = xr::EventDataBuffer::new();
    let mut session_running = false;
    // Index of the current frame, wrapped by PIPELINE_DEPTH. Not to be confused with the
    // swapchain image index.
    let mut frame = 0;
    'main_loop: loop {
        if !running.load(Ordering::Relaxed) {
            println!("requesting exit");
            // The OpenXR runtime may want to perform a smooth transition between scenes, so we
            // can't necessarily exit instantly. Instead, we must notify the runtime of our
            // intent and wait for it to tell us when we're actually done.
            match session.request_exit() {
                Ok(()) => {}
                Err(xr::sys::Result::ERROR_SESSION_NOT_RUNNING) => break,
                Err(e) => panic!("{}", e),
            }
        }

        while let Some(event) = xr_instance.poll_event(&mut event_storage).unwrap() {
            use xr::Event::*;
            match event {
                SessionStateChanged(e) => {
                    // Session state change is where we can begin and end sessions, as well as
                    // find quit messages!
                    println!("entered state {:?}", e.state());
                    match e.state() {
                        xr::SessionState::READY => {
                            session.begin(VIEW_TYPE).unwrap();
                            session_running = true;
                        }
                        xr::SessionState::STOPPING => {
                            session.end().unwrap();
                            session_running = false;
                        }
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            break 'main_loop;
                        }
                        _ => {}
                    }
                }
                InstanceLossPending(_) => {
                    break 'main_loop;
                }
                EventsLost(e) => {
                    println!("lost {} events", e.lost_event_count());
                }
                _ => {}
            }
        }

        if !session_running {
            // Don't grind up the CPU
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        // Block until the previous frame is finished displaying, and is ready for another one.
        // Also returns a prediction of when the next frame will be displayed, for use with
        // predicting locations of controllers, viewpoints, etc.
        let xr_frame_state = frame_wait.wait().unwrap();
        // Must be called before any rendering is done!
        frame_stream.begin().unwrap();

        if !xr_frame_state.should_render {
            frame_stream
                .end(
                    xr_frame_state.predicted_display_time,
                    environment_blend_mode,
                    &[],
                )
                .unwrap();
            continue;
        }

        // We need to ask which swapchain image to use for rendering! Which one will we get?
        // Who knows! It's up to the runtime to decide.
        let image_index = swapchain.handle.acquire_image().unwrap();

        // Wait until the image is available to render to. The compositor could still be
        // reading from it.
        swapchain.handle.wait_image(xr::Duration::INFINITE).unwrap();

        let view_desc = wgpu::TextureViewDescriptor {
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(VIEW_COUNT),
            ..Default::default()
        };
        let view = swapchain.textures[image_index as usize].create_view(&view_desc);
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        session.sync_actions(&[(&action_set).into()]).unwrap();

        // Find where our controllers are located in the Stage space
        let right_location = right_space
            .locate(&stage, xr_frame_state.predicted_display_time)
            .unwrap();

        let left_location = left_space
            .locate(&stage, xr_frame_state.predicted_display_time)
            .unwrap();

        let mut printed = false;
        if left_action.is_active(&session, xr::Path::NULL).unwrap() {
            print!(
                "Left Hand: ({:0<12},{:0<12},{:0<12}), ",
                left_location.pose.position.x,
                left_location.pose.position.y,
                left_location.pose.position.z
            );
            printed = true;
        }

        if right_action.is_active(&session, xr::Path::NULL).unwrap() {
            print!(
                "Right Hand: ({:0<12},{:0<12},{:0<12})",
                right_location.pose.position.x,
                right_location.pose.position.y,
                right_location.pose.position.z
            );
            printed = true;
        }
        if printed {
            println!();
        }

        // Fetch the view transforms. To minimize latency, we intentionally do this *after*
        // recording commands to render the scene, i.e. at the last possible moment before
        // rendering begins in earnest on the GPU. Uniforms dependent on this data can be sent
        // to the GPU just-in-time by writing them to per-frame host-visible memory which the
        // GPU will only read once the command buffer is submitted.
        let (_, views) = session
            .locate_views(VIEW_TYPE, xr_frame_state.predicted_display_time, &stage)
            .unwrap();

        // Submit commands to the GPU, then tell OpenXR we're done with our part.
        queue.submit(Some(encoder.finish()));
        swapchain.handle.release_image().unwrap();

        // Tell OpenXR what to present for this frame
        let rect = xr::Rect2Di {
            offset: xr::Offset2Di { x: 0, y: 0 },
            extent: xr::Extent2Di {
                width: swapchain.width as _,
                height: swapchain.height as _,
            },
        };
        frame_stream
            .end(
                xr_frame_state.predicted_display_time,
                environment_blend_mode,
                &[
                    &xr::CompositionLayerProjection::new().space(&stage).views(&[
                        xr::CompositionLayerProjectionView::new()
                            .pose(views[0].pose)
                            .fov(views[0].fov)
                            .sub_image(
                                xr::SwapchainSubImage::new()
                                    .swapchain(&swapchain.handle)
                                    .image_array_index(0)
                                    .image_rect(rect),
                            ),
                        xr::CompositionLayerProjectionView::new()
                            .pose(views[1].pose)
                            .fov(views[1].fov)
                            .sub_image(
                                xr::SwapchainSubImage::new()
                                    .swapchain(&swapchain.handle)
                                    .image_array_index(1)
                                    .image_rect(rect),
                            ),
                    ]),
                ],
            )
            .unwrap();
        frame = (frame + 1) % PIPELINE_DEPTH as usize;
    }
}

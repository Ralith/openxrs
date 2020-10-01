//! Illustrates rendering using Vulkan with multiview. Supports any Vulkan 1.1 capable environment.
//!
//! Renders a smooth gradient across the entire view, with different colors per eye.
//!
//! This example uses minimal abstraction for clarity. Real-world code should encapsulate and
//! largely decouple its Vulkan and OpenXR components and handle errors gracefully.

use std::{
    ffi::CString,
    io::Cursor,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use ash::{
    util::read_spv,
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    vk::{self, Handle},
};
use openxr as xr;

fn main() {
    // Handle interrupts gracefully
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::Relaxed);
    })
    .expect("setting Ctrl-C handler");

    #[cfg(feature = "static")]
    let entry = xr::Entry::linked();
    #[cfg(not(feature = "static"))]
    let entry = xr::Entry::load()
        .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

    let mut enabled_extensions = xr::ExtensionSet::default();
    enabled_extensions.khr_vulkan_enable = true;
    let xr_instance = entry
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

    let system = xr_instance
        .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
        .unwrap();
    let vk_instance_exts = xr_instance
        .vulkan_instance_extensions(system)
        .unwrap()
        .split(' ')
        .map(|x| CString::new(x).unwrap())
        .collect::<Vec<_>>();
    println!(
        "required Vulkan instance extensions: {:?}",
        vk_instance_exts
    );
    let vk_instance_ext_ptrs = vk_instance_exts
        .iter()
        .map(|x| x.as_ptr())
        .collect::<Vec<_>>();

    let vk_entry = ash::Entry::new().unwrap();

    let vk_app_info = vk::ApplicationInfo::builder()
        .application_version(0)
        .engine_version(0)
        .api_version(vk::make_version(1, 1, 0)); // Vulkan 1.1 guarantees multiview support

    let vk_version = vk_entry
        .try_enumerate_instance_version()
        .unwrap()
        .unwrap_or_else(|| vk::make_version(1, 0, 0));
    let vk_version = xr::Version::new(
        vk::version_major(vk_version) as u16,
        vk::version_major(vk_version) as u16,
        0,
    );

    let reqs = xr_instance
        .graphics_requirements::<xr::Vulkan>(system)
        .unwrap();
    if reqs.min_api_version_supported > vk_version {
        panic!(
            "OpenXR runtime requires Vulkan version > {}",
            reqs.min_api_version_supported
        );
    }

    let vk_device_exts = xr_instance
        .vulkan_device_extensions(system)
        .unwrap()
        .split(' ')
        .map(|x| CString::new(x).unwrap())
        .collect::<Vec<_>>();
    println!("required Vulkan device extensions: {:?}", vk_device_exts);
    let vk_device_ext_ptrs = vk_device_exts
        .iter()
        .map(|x| x.as_ptr())
        .collect::<Vec<_>>();

    unsafe {
        let vk_instance = vk_entry
            .create_instance(
                &vk::InstanceCreateInfo::builder()
                    .application_info(&vk_app_info)
                    .enabled_extension_names(&vk_instance_ext_ptrs),
                None,
            )
            .expect("failed to create Vulkan instance");

        let vk_physical_device = vk::PhysicalDevice::from_raw(
            xr_instance
                .vulkan_graphics_device(system, vk_instance.handle().as_raw() as _)
                .unwrap() as _,
        );

        let vk_device_properties = vk_instance.get_physical_device_properties(vk_physical_device);
        if vk_device_properties.api_version < vk::make_version(1, 1, 0) {
            vk_instance.destroy_instance(None);
            panic!("Vulkan phyiscal device doesn't support version 1.1");
        }

        let queue_family_index = vk_instance
            .get_physical_device_queue_family_properties(vk_physical_device)
            .into_iter()
            .enumerate()
            .filter_map(|(queue_family_index, info)| {
                if info.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    Some(queue_family_index as u32)
                } else {
                    None
                }
            })
            .next()
            .expect("Vulkan device has no graphics queue");

        let vk_device = vk_instance
            .create_device(
                vk_physical_device,
                &vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&[vk::DeviceQueueCreateInfo::builder()
                        .queue_family_index(queue_family_index)
                        .queue_priorities(&[1.0])
                        .build()])
                    .enabled_extension_names(&vk_device_ext_ptrs)
                    .push_next(&mut vk::PhysicalDeviceVulkan11Features {
                        multiview: vk::TRUE,
                        ..Default::default()
                    }),
                None,
            )
            .unwrap();
        let queue = vk_device.get_device_queue(queue_family_index, 0);

        let view_mask = !(!0 << VIEW_COUNT);
        let render_pass = vk_device
            .create_render_pass(
                &vk::RenderPassCreateInfo::builder()
                    .attachments(&[vk::AttachmentDescription {
                        format: COLOR_FORMAT,
                        samples: vk::SampleCountFlags::TYPE_1,
                        load_op: vk::AttachmentLoadOp::CLEAR,
                        store_op: vk::AttachmentStoreOp::STORE,
                        initial_layout: vk::ImageLayout::UNDEFINED,
                        final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                        ..Default::default()
                    }])
                    .subpasses(&[vk::SubpassDescription::builder()
                        .color_attachments(&[vk::AttachmentReference {
                            attachment: 0,
                            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                        }])
                        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                        .build()])
                    .dependencies(&[vk::SubpassDependency {
                        src_subpass: vk::SUBPASS_EXTERNAL,
                        dst_subpass: 0,
                        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                        ..Default::default()
                    }])
                    .push_next(
                        &mut vk::RenderPassMultiviewCreateInfo::builder()
                            .view_masks(&[view_mask])
                            .correlation_masks(&[view_mask]),
                    ),
                None,
            )
            .unwrap();

        let vert = read_spv(&mut Cursor::new(&include_bytes!("fullscreen.vert.spv")[..])).unwrap();
        let frag = read_spv(&mut Cursor::new(
            &include_bytes!("debug_pattern.frag.spv")[..],
        ))
        .unwrap();
        let vert = vk_device
            .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&vert), None)
            .unwrap();
        let frag = vk_device
            .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&frag), None)
            .unwrap();

        let pipeline_layout = vk_device
            .create_pipeline_layout(
                &vk::PipelineLayoutCreateInfo::builder().set_layouts(&[]),
                None,
            )
            .unwrap();
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };
        let pipeline = vk_device
            .create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[vk::GraphicsPipelineCreateInfo::builder()
                    .stages(&[
                        vk::PipelineShaderStageCreateInfo {
                            stage: vk::ShaderStageFlags::VERTEX,
                            module: vert,
                            p_name: b"main\0".as_ptr() as _,
                            ..Default::default()
                        },
                        vk::PipelineShaderStageCreateInfo {
                            stage: vk::ShaderStageFlags::FRAGMENT,
                            module: frag,
                            p_name: b"main\0".as_ptr() as _,
                            ..Default::default()
                        },
                    ])
                    .vertex_input_state(&vk::PipelineVertexInputStateCreateInfo::default())
                    .input_assembly_state(
                        &vk::PipelineInputAssemblyStateCreateInfo::builder()
                            .topology(vk::PrimitiveTopology::TRIANGLE_LIST),
                    )
                    .viewport_state(
                        &vk::PipelineViewportStateCreateInfo::builder()
                            .scissor_count(1)
                            .viewport_count(1),
                    )
                    .rasterization_state(
                        &vk::PipelineRasterizationStateCreateInfo::builder()
                            .cull_mode(vk::CullModeFlags::NONE)
                            .polygon_mode(vk::PolygonMode::FILL)
                            .line_width(1.0),
                    )
                    .multisample_state(
                        &vk::PipelineMultisampleStateCreateInfo::builder()
                            .rasterization_samples(vk::SampleCountFlags::TYPE_1),
                    )
                    .depth_stencil_state(
                        &vk::PipelineDepthStencilStateCreateInfo::builder()
                            .depth_test_enable(false)
                            .depth_write_enable(false)
                            .front(noop_stencil_state)
                            .back(noop_stencil_state),
                    )
                    .color_blend_state(
                        &vk::PipelineColorBlendStateCreateInfo::builder().attachments(&[
                            vk::PipelineColorBlendAttachmentState {
                                blend_enable: vk::TRUE,
                                src_color_blend_factor: vk::BlendFactor::ONE,
                                dst_color_blend_factor: vk::BlendFactor::ZERO,
                                color_blend_op: vk::BlendOp::ADD,
                                color_write_mask: vk::ColorComponentFlags::R
                                    | vk::ColorComponentFlags::G
                                    | vk::ColorComponentFlags::B,
                                ..Default::default()
                            },
                        ]),
                    )
                    .dynamic_state(
                        &vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&[
                            vk::DynamicState::VIEWPORT,
                            vk::DynamicState::SCISSOR,
                        ]),
                    )
                    .layout(pipeline_layout)
                    .render_pass(render_pass)
                    .subpass(0)
                    .build()],
                None,
            )
            .unwrap()[0];

        vk_device.destroy_shader_module(vert, None);
        vk_device.destroy_shader_module(frag, None);

        let (session, mut frame_wait, mut frame_stream) = xr_instance
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
        let stage = session
            .create_reference_space(xr::ReferenceSpaceType::STAGE, xr::Posef::IDENTITY)
            .unwrap();

        let cmd_pool = vk_device
            .create_command_pool(
                &vk::CommandPoolCreateInfo::builder()
                    .queue_family_index(queue_family_index)
                    .flags(
                        vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER
                            | vk::CommandPoolCreateFlags::TRANSIENT,
                    ),
                None,
            )
            .unwrap();
        let cmds = vk_device
            .allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::builder()
                    .command_pool(cmd_pool)
                    .command_buffer_count(PIPELINE_DEPTH),
            )
            .unwrap();
        let fences = (0..PIPELINE_DEPTH)
            .map(|_| {
                vk_device
                    .create_fence(
                        &vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED),
                        None,
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();

        // Main loop
        let mut swapchain = None;
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
                        println!("entered state {:?}", e.state());
                        match e.state() {
                            xr::SessionState::READY => {
                                session
                                    .begin(xr::ViewConfigurationType::PRIMARY_STEREO)
                                    .unwrap();
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

            let xr_frame_state = frame_wait.wait().unwrap();
            frame_stream.begin().unwrap();

            if !xr_frame_state.should_render {
                frame_stream
                    .end(
                        xr_frame_state.predicted_display_time,
                        xr::EnvironmentBlendMode::OPAQUE,
                        &[],
                    )
                    .unwrap();
                continue;
            }

            if swapchain.is_none() {
                let views = xr_instance
                    .enumerate_view_configuration_views(
                        system,
                        xr::ViewConfigurationType::PRIMARY_STEREO,
                    )
                    .unwrap();
                let resolution = vk::Extent2D {
                    width: views[0].recommended_image_rect_width,
                    height: views[0].recommended_image_rect_height,
                };
                let handle = session
                    .create_swapchain(&xr::SwapchainCreateInfo {
                        create_flags: xr::SwapchainCreateFlags::EMPTY,
                        usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                            | xr::SwapchainUsageFlags::SAMPLED,
                        format: COLOR_FORMAT.as_raw() as _,
                        sample_count: 1,
                        width: resolution.width,
                        height: resolution.height,
                        face_count: 1,
                        array_size: VIEW_COUNT,
                        mip_count: 1,
                    })
                    .unwrap();
                let images = handle.enumerate_images().unwrap();
                swapchain = Some(Swapchain {
                    handle,
                    resolution,
                    buffers: images
                        .into_iter()
                        .map(|color_image| {
                            let color_image = vk::Image::from_raw(color_image);
                            let color = vk_device
                                .create_image_view(
                                    &vk::ImageViewCreateInfo::builder()
                                        .image(color_image)
                                        .view_type(vk::ImageViewType::TYPE_2D_ARRAY)
                                        .format(COLOR_FORMAT)
                                        .subresource_range(vk::ImageSubresourceRange {
                                            aspect_mask: vk::ImageAspectFlags::COLOR,
                                            base_mip_level: 0,
                                            level_count: 1,
                                            base_array_layer: 0,
                                            layer_count: VIEW_COUNT,
                                        }),
                                    None,
                                )
                                .unwrap();
                            let framebuffer = vk_device
                                .create_framebuffer(
                                    &vk::FramebufferCreateInfo::builder()
                                        .render_pass(render_pass)
                                        .width(resolution.width)
                                        .height(resolution.height)
                                        .attachments(&[color])
                                        .layers(1), // Multiview handles addressing multiple layers
                                    None,
                                )
                                .unwrap();
                            Framebuffer { framebuffer, color }
                        })
                        .collect(),
                });
            }
            let swapchain = swapchain.as_mut().unwrap();
            let image_index = swapchain.handle.acquire_image().unwrap();
            swapchain.handle.wait_image(xr::Duration::INFINITE).unwrap();

            // Ensure the last use of this frame's resources is 100% done
            vk_device
                .wait_for_fences(&[fences[frame]], true, u64::MAX)
                .unwrap();
            vk_device.reset_fences(&[fences[frame]]).unwrap();

            let cmd = cmds[frame];
            vk_device
                .begin_command_buffer(
                    cmd,
                    &vk::CommandBufferBeginInfo::builder()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
                )
                .unwrap();
            vk_device.cmd_begin_render_pass(
                cmd,
                &vk::RenderPassBeginInfo::builder()
                    .render_pass(render_pass)
                    .framebuffer(swapchain.buffers[image_index as usize].framebuffer)
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D::default(),
                        extent: swapchain.resolution,
                    })
                    .clear_values(&[vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: [0.0, 0.0, 0.0, 1.0],
                        },
                    }]),
                vk::SubpassContents::INLINE,
            );

            let viewports = [vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: swapchain.resolution.width as f32,
                height: swapchain.resolution.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }];
            let scissors = [vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: swapchain.resolution,
            }];
            vk_device.cmd_set_viewport(cmd, 0, &viewports);
            vk_device.cmd_set_scissor(cmd, 0, &scissors);

            // Draw the scene. Multiview means we only need to do this once, and the GPU will
            // automatically broadcast operations to all views. Shaders can use `gl_ViewIndex` to
            // e.g. select the correct view matrix.
            vk_device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, pipeline);
            vk_device.cmd_draw(cmd, 3, 1, 0, 0);

            vk_device.cmd_end_render_pass(cmd);
            vk_device.end_command_buffer(cmd).unwrap();

            // Fetch the view transforms. To minimize latency, we intentionally do this *after*
            // recording commands to render the scene, i.e. at the last possible moment before
            // rendering begins in earnest on the GPU. Uniforms dependent on this data can be sent
            // to the GPU just-in-time by writing them to per-frame host-visible memory which the
            // GPU will only read once the command buffer is submitted.
            let (_, views) = session
                .locate_views(
                    xr::ViewConfigurationType::PRIMARY_STEREO,
                    xr_frame_state.predicted_display_time,
                    &stage,
                )
                .unwrap();

            // Submit commands to the GPU, then tell OpenXR we're done with our part.
            vk_device
                .queue_submit(
                    queue,
                    &[vk::SubmitInfo::builder().command_buffers(&[cmd]).build()],
                    fences[frame],
                )
                .unwrap();
            swapchain.handle.release_image().unwrap();

            // Tell OpenXR what to present for this frame
            let rect = xr::Rect2Di {
                offset: xr::Offset2Di { x: 0, y: 0 },
                extent: xr::Extent2Di {
                    width: swapchain.resolution.width as _,
                    height: swapchain.resolution.height as _,
                },
            };
            frame_stream
                .end(
                    xr_frame_state.predicted_display_time,
                    xr::EnvironmentBlendMode::OPAQUE,
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

        // OpenXR MUST be allowed to clean up before we destroy Vulkan resources it could touch, so
        // first we must drop all its handles.
        drop((session, frame_wait, frame_stream, stage));

        // Ensure all in-flight frames are finished before destroying resources they might use
        vk_device.wait_for_fences(&fences, true, !0).unwrap();
        for fence in fences {
            vk_device.destroy_fence(fence, None);
        }

        if let Some(swapchain) = swapchain {
            for buffer in swapchain.buffers {
                vk_device.destroy_framebuffer(buffer.framebuffer, None);
                vk_device.destroy_image_view(buffer.color, None);
            }
        }

        vk_device.destroy_pipeline(pipeline, None);
        vk_device.destroy_pipeline_layout(pipeline_layout, None);
        vk_device.destroy_command_pool(cmd_pool, None);
        vk_device.destroy_render_pass(render_pass, None);
        vk_device.destroy_device(None);
        vk_instance.destroy_instance(None);
    }

    println!("exiting cleanly");
}

pub const COLOR_FORMAT: vk::Format = vk::Format::B8G8R8A8_SRGB;
pub const VIEW_COUNT: u32 = 2;

struct Swapchain {
    handle: xr::Swapchain<xr::Vulkan>,
    buffers: Vec<Framebuffer>,
    resolution: vk::Extent2D,
}

struct Framebuffer {
    framebuffer: vk::Framebuffer,
    color: vk::ImageView,
}

/// Maximum number of frames in flight
const PIPELINE_DEPTH: u32 = 2;

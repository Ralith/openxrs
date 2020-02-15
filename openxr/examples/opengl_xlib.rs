#[cfg(target_os = "linux")]
mod inner {
    use std::ffi::{c_void, CString};
    use std::mem;
    use std::os::raw::c_int;
    use std::ptr;
    use std::ptr::null_mut;

    use glium::texture::{DepthFormat, DepthTexture2dArray, MipmapsOption, SrgbTexture2dArray};
    use openxr as xr;
    use x11::{glx, glx::arb, xlib};

    const GL_SRGB8_ALPHA8: u32 = 0x8C43;

    type GlXcreateContextAttribsArb = unsafe extern "C" fn(
        dpy: *mut xlib::Display,
        fbc: glx::GLXFBConfig,
        share_context: glx::GLXContext,
        direct: xlib::Bool,
        attribs: *const c_int,
    ) -> glx::GLXContext;

    pub struct Backend {
        context: glx::GLXContext,
        display: *mut xlib::Display,
        visual: *mut xlib::XVisualInfo,
        fb_config: *mut glx::GLXFBConfig,
        drawable: x11::xlib::Drawable,
    }

    impl Backend {
        pub fn new() -> Self {
            let mut fbcount = 0;

            let attr = [
                glx::GLX_RGBA,
                glx::GLX_DEPTH_SIZE,
                24,
                glx::GLX_DOUBLEBUFFER,
                0,
            ];

            let visual_attribs = [0];

            let context_attribs = [
                arb::GLX_CONTEXT_MAJOR_VERSION_ARB,
                4,
                arb::GLX_CONTEXT_MINOR_VERSION_ARB,
                5,
                arb::GLX_CONTEXT_PROFILE_MASK_ARB,
                arb::GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
                0,
            ];

            unsafe {
                let c_proc_name = CString::new("glXCreateContextAttribsARB").unwrap();
                let proc_addr = glx::glXGetProcAddress(c_proc_name.as_ptr() as *const u8);
                let glx_create_context_attribs =
                    mem::transmute::<_, GlXcreateContextAttribsArb>(proc_addr);

                let display = xlib::XOpenDisplay(ptr::null());
                let root = xlib::XDefaultRootWindow(display);
                let visual = glx::glXChooseVisual(display, 0, attr.as_ptr() as *mut _);
                let fb_config = glx::glXChooseFBConfig(
                    display,
                    xlib::XDefaultScreen(display),
                    visual_attribs.as_ptr(),
                    &mut fbcount,
                );

                let context = glx_create_context_attribs(
                    display,
                    *fb_config,
                    null_mut(),
                    xlib::True,
                    &context_attribs[0] as *const c_int,
                );
                if context.is_null() {
                    panic!("glXCreateContextAttribsARB failed")
                }
                glx::glXMakeCurrent(display, root, context);

                Self {
                    context,
                    display,
                    visual,
                    fb_config,
                    drawable: root,
                }
            }
        }
        pub fn xr_create_session(
            &self,
            xr: &xr::Instance,
            system: xr::SystemId,
        ) -> xr::Result<(
            xr::Session<xr::OpenGL>,
            xr::FrameWaiter,
            xr::FrameStream<xr::OpenGL>,
        )> {
            unsafe {
                let visualid = { *self.visual }.visualid as u32;
                let info = xr::opengl::SessionCreateInfo::Xlib {
                    x_display: self.display as *mut _,
                    glx_fb_config: *self.fb_config as *mut _,
                    glx_drawable: self.drawable,
                    visualid,
                    glx_context: self.context as *mut _,
                };
                xr.create_session(system, &info)
            }
        }
    }

    impl Drop for Backend {
        fn drop(&mut self) {
            unsafe {
                x11::xlib::XFree(self.fb_config as *mut _);
                x11::xlib::XFree(self.visual as *mut _);
                x11::xlib::XCloseDisplay(self.display);
            }
        }
    }

    unsafe impl glium::backend::Backend for Backend {
        fn swap_buffers(&self) -> Result<(), glium::SwapBuffersError> {
            unsafe {
                x11::glx::glXSwapBuffers(self.display, self.drawable);
            }
            Ok(())
        }

        unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
            let addr = CString::new(symbol.as_bytes()).unwrap();
            let addr = addr.as_ptr();
            let proc_addr = glx::glXGetProcAddressARB(addr as *const _);
            match proc_addr {
                Some(proc_addr) => proc_addr as *const _,
                _ => ptr::null(),
            }
        }

        fn get_framebuffer_dimensions(&self) -> (u32, u32) {
            (0, 0)
        }

        fn is_current(&self) -> bool {
            true
        }

        unsafe fn make_current(&self) {
            glx::glXMakeCurrent(self.display, self.drawable, self.context);
        }
    }

    pub struct OpenXR {
        instance: xr::Instance,
        system: xr::SystemId,
        session: xr::Session<xr::OpenGL>,
        resolution: (u32, u32),
        predicted_display_time: Option<xr::Time>,
        frame_waiter: xr::FrameWaiter,
        frame_stream: xr::FrameStream<xr::OpenGL>,
        swapchain: Option<xr::Swapchain<xr::OpenGL>>,
    }

    impl OpenXR {
        fn new(backend: &mut Backend) -> Self {
            #[cfg(feature = "static")]
            let entry = xr::Entry::linked();
            #[cfg(not(feature = "static"))]
            let entry = xr::Entry::load()
                .expect("couldn't find the OpenXR loader; try enabling the \"static\" feature");

            let extensions = entry
                .enumerate_extensions()
                .expect("Cannot enumerate extensions");
            let app_info = xr::ApplicationInfo {
                application_name: "openxrs example",
                ..Default::default()
            };
            if !extensions.khr_opengl_enable {
                panic!("XR: OpenGL extension unsupported");
            }
            let extension_set = xr::ExtensionSet {
                khr_opengl_enable: true,
                ..Default::default()
            };
            let instance = entry.create_instance(&app_info, &extension_set).unwrap();
            let instance_props = instance.properties().expect("Cannot load instance props");
            println!(
                "loaded instance: {} v{}",
                instance_props.runtime_name, instance_props.runtime_version
            );

            let system = instance
                .system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)
                .unwrap();
            let (session, frame_waiter, frame_stream) =
                backend.xr_create_session(&instance, system).unwrap();

            Self {
                instance,
                system,
                session,
                frame_waiter,
                frame_stream,
                resolution: (0, 0),
                predicted_display_time: None,
                swapchain: None,
            }
        }
        pub fn handle_events(&mut self) -> bool {
            // Check for change in resolution
            if self.swapchain.is_some() {
                let view_configuration_views = self
                    .instance
                    .enumerate_view_configuration_views(
                        self.system,
                        xr::ViewConfigurationType::PRIMARY_STEREO,
                    )
                    .unwrap();
                let resolution = (
                    view_configuration_views[0].recommended_image_rect_width,
                    view_configuration_views[0].recommended_image_rect_height,
                );
                if resolution != self.resolution {
                    self.resolution = resolution;
                    self.update_swapchain();
                }
            }

            // Handle events
            let mut buffer = xr::EventDataBuffer::new();
            while let Some(event) = self.instance.poll_event(&mut buffer).unwrap() {
                use xr::Event::*;
                match event {
                    SessionStateChanged(session_change) => match session_change.state() {
                        xr::SessionState::READY => {
                            self.session
                                .begin(xr::ViewConfigurationType::PRIMARY_STEREO)
                                .unwrap();
                            self.init_swapchain();
                        }
                        xr::SessionState::STOPPING => {
                            self.swapchain = None;
                            self.session.end().unwrap();
                        }
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            return false;
                        }
                        _ => {}
                    },
                    InstanceLossPending(_) => {
                        return false;
                    }
                    EventsLost(e) => {
                        println!("lost {} events", e.lost_event_count());
                    }
                    _ => {
                        println!("unhandled event");
                    }
                }
            }
            true
        }
        pub fn init_swapchain(&mut self) {
            let swapchain_formats = self.session.enumerate_swapchain_formats().unwrap();
            if !swapchain_formats.contains(&GL_SRGB8_ALPHA8) {
                panic!("XR: Cannot use OpenGL GL_SRGB8_ALPHA8 swapchain format");
            }
            let view_configuration_views = self
                .instance
                .enumerate_view_configuration_views(
                    self.system,
                    xr::ViewConfigurationType::PRIMARY_STEREO,
                )
                .unwrap();
            self.resolution = (
                view_configuration_views[0].recommended_image_rect_width,
                view_configuration_views[0].recommended_image_rect_height,
            );
            self.update_swapchain();
        }
        pub fn update_swapchain(&mut self) {
            let swapchain_create_info: xr::SwapchainCreateInfo<xr::OpenGL> =
                xr::SwapchainCreateInfo {
                    create_flags: xr::SwapchainCreateFlags::EMPTY,
                    usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                        | xr::SwapchainUsageFlags::SAMPLED,
                    format: GL_SRGB8_ALPHA8,
                    sample_count: 1,
                    width: self.resolution.0,
                    height: self.resolution.1,
                    face_count: 1,
                    // One image per eye
                    array_size: 2,
                    mip_count: 1,
                };
            let swapchain = self
                .session
                .create_swapchain(&swapchain_create_info)
                .unwrap();

            self.swapchain = Some(swapchain);
        }
        pub fn frame_begin(&mut self) {
            let state = self.frame_waiter.wait().unwrap();
            self.predicted_display_time = Some(state.predicted_display_time);
            self.frame_stream.begin().unwrap();
        }
        pub fn get_swapchain_image(&mut self) -> Option<u32> {
            let swapchain = self.swapchain.as_mut()?;
            let images = swapchain.enumerate_images().unwrap();
            let image_id = swapchain.acquire_image().unwrap();
            swapchain.wait_image(xr::Duration::INFINITE).unwrap();
            let image = images[image_id as usize];
            Some(image)
        }
        pub fn frame_end(&mut self) {
            let swapchain = self.swapchain.as_ref().unwrap();
            let eye_rect = xr::Rect2Di {
                offset: xr::Offset2Di { x: 0, y: 0 },
                extent: xr::Extent2Di {
                    width: self.resolution.0 as i32,
                    height: self.resolution.1 as i32,
                },
            };

            let time = self.predicted_display_time.take().unwrap();
            let left_subimage: xr::SwapchainSubImage<xr::OpenGL> = openxr::SwapchainSubImage::new()
                .swapchain(swapchain)
                .image_array_index(0)
                .image_rect(eye_rect);
            let right_subimage: xr::SwapchainSubImage<xr::OpenGL> =
                openxr::SwapchainSubImage::new()
                    .swapchain(swapchain)
                    .image_array_index(1)
                    .image_rect(eye_rect);

            let projection_view_left =
                xr::CompositionLayerProjectionView::new().sub_image(left_subimage);
            let projection_view_right =
                xr::CompositionLayerProjectionView::new().sub_image(right_subimage);
            let proj_views = [projection_view_left, projection_view_right];
            let projection = xr::CompositionLayerProjection::new().views(&proj_views);
            self.frame_stream
                .end(time, xr::EnvironmentBlendMode::OPAQUE, &[&projection])
                .unwrap();
        }
        pub fn release_swapchain_image(&mut self) {
            let swapchain = self.swapchain.as_mut().unwrap();
            swapchain.release_image().unwrap();
        }
    }

    pub fn main() {
        use glium::Surface;

        let mut backend = Backend::new();
        let mut open_xr = OpenXR::new(&mut backend);
        let context =
            unsafe { glium::backend::Context::new(backend, false, Default::default()) }.unwrap();
        let mut depth_buffer = DepthTexture2dArray::empty_with_format(
            &context,
            DepthFormat::F32,
            MipmapsOption::EmptyMipmaps,
            open_xr.resolution.0,
            open_xr.resolution.1,
            2,
        )
        .unwrap();
        while open_xr.handle_events() {
            let swapchain_image = open_xr.get_swapchain_image();
            if depth_buffer.dimensions() != open_xr.resolution {
                depth_buffer = DepthTexture2dArray::empty_with_format(
                    &context,
                    DepthFormat::F32,
                    MipmapsOption::EmptyMipmaps,
                    open_xr.resolution.0,
                    open_xr.resolution.1,
                    2,
                )
                .unwrap()
            };
            if let Some(swapchain_image) = swapchain_image {
                open_xr.frame_begin();
                let texture_array = unsafe {
                    SrgbTexture2dArray::from_id(
                        &context,
                        glium::texture::SrgbFormat::U8U8U8U8,
                        swapchain_image,
                        false,
                        glium::texture::MipmapsOption::NoMipmap,
                        glium::texture::Dimensions::Texture2dArray {
                            width: open_xr.resolution.0,
                            height: open_xr.resolution.1,
                            array_size: 2,
                        },
                    )
                };

                let texture_left = texture_array.layer(0).unwrap().mipmap(0).unwrap();
                let depth_left = depth_buffer.layer(0).unwrap().mipmap(0).unwrap();
                let texture_right = texture_array.layer(1).unwrap().mipmap(0).unwrap();
                let depth_right = depth_buffer.layer(1).unwrap().mipmap(0).unwrap();

                let mut target_left = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                    &context,
                    texture_left,
                    depth_left,
                )
                .unwrap();

                let mut target_right = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                    &context,
                    texture_right,
                    depth_right,
                )
                .unwrap();

                target_left.clear_color_and_depth((0.6, 0.0, 0.0, 1.0), 1.0);
                target_right.clear_color_and_depth((0.0, 0.0, 0.6, 1.0), 1.0);
                open_xr.release_swapchain_image();
                open_xr.frame_end();
            }
        }
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    {
        inner::main()
    }
}

use std::{mem, ptr};

use sys::platform::*;

use crate::*;

/// The OpenGL graphics API
///
/// See [`XR_KHR_opengl_enable`] for safety details.
///
/// [`XR_KHR_opengl_enable`]: https://www.khronos.org/registry/OpenXR/specs/0.90/html/xrspec.html#XR_KHR_opengl_enable
pub enum OpenGL {}

impl Graphics for OpenGL {
    type Requirements = Requirements;
    type SessionCreateInfo = SessionCreateInfo;
    type Format = u32;
    type SwapchainImage = u32;

    fn raise_format(x: i64) -> u32 {
        x as _
    }
    fn lower_format(x: u32) -> i64 {
        x.into()
    }

    fn requirements(inst: &Instance, system: SystemId) -> Result<Requirements> {
        let mut out;
        unsafe {
            out = sys::GraphicsRequirementsOpenGLKHR {
                ty: sys::GraphicsRequirementsOpenGLKHR::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((inst.opengl().get_open_gl_graphics_requirements)(
                inst.as_raw(),
                system,
                &mut out,
            ))?;
        }
        Ok(Requirements {
            min_api_version_supported: Version::from_raw(out.min_api_version_supported),
            max_api_version_supported: Version::from_raw(out.max_api_version_supported),
        })
    }

    unsafe fn create_session(
        instance: Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<Session<Self>> {
        match *info {
            SessionCreateInfo::Xlib {
                x_display,
                visualid,
                glx_fb_config,
                glx_drawable,
                glx_context,
            } => {
                let binding = sys::GraphicsBindingOpenGLXlibKHR {
                    ty: sys::GraphicsBindingOpenGLXlibKHR::TYPE,
                    next: ptr::null(),
                    x_display,
                    visualid,
                    glx_fb_config,
                    glx_drawable,
                    glx_context,
                };
                let info = sys::SessionCreateInfo {
                    ty: sys::SessionCreateInfo::TYPE,
                    next: &binding as *const _ as *const _,
                    create_flags: Default::default(),
                    system_id: system,
                };
                let mut out = sys::Session::NULL;
                cvt((instance.fp().create_session)(
                    instance.as_raw(),
                    &info,
                    &mut out,
                ))?;
                Ok(Session::from_raw(instance.clone(), out))
            }
        }
    }

    fn enumerate_swapchain_images(swapchain: &Swapchain<Self>) -> Result<Vec<Self::SwapchainImage>> {
        let images = get_arr_init(
            sys::SwapchainImageOpenGLKHR {
                ty: sys::SwapchainImageOpenGLKHR::TYPE,
                next: ptr::null_mut(),
                image: 0,
            },
            |capacity, count, buf| unsafe {
                (swapchain.instance().fp().enumerate_swapchain_images)(
                    swapchain.as_raw(),
                    capacity,
                    count,
                    buf as *mut _,
                )
            },
        )?;
        Ok(images.into_iter().map(|x| x.image).collect())
    }
}

#[derive(Copy, Clone)]
pub struct Requirements {
    pub min_api_version_supported: Version,
    pub max_api_version_supported: Version,
}

pub enum SessionCreateInfo {
    Xlib {
        x_display: *mut Display,
        visualid: u32,
        glx_fb_config: GLXFBConfig,
        glx_drawable: GLXDrawable,
        glx_context: GLXContext,
    },
}

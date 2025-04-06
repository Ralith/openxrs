use std::ptr;

use sys::platform::*;

use crate::sys::Handle as _;
use crate::*;

/// The OpenGL graphics API
///
/// See [`XR_KHR_opengl_enable`] for safety details.
///
/// [`XR_KHR_opengl_enable`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable
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
        let out = unsafe {
            let mut x = sys::GraphicsRequirementsOpenGLKHR::out(ptr::null_mut());
            cvt((inst.opengl().get_open_gl_graphics_requirements)(
                inst.as_raw(),
                system,
                x.as_mut_ptr(),
            ))?;
            x.assume_init()
        };
        Ok(Requirements {
            min_api_version_supported: out.min_api_version_supported,
            max_api_version_supported: out.max_api_version_supported,
        })
    }

    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        match *info {
            #[cfg(windows)]
            SessionCreateInfo::Windows { h_dc, h_glrc } => {
                let binding = sys::GraphicsBindingOpenGLWin32KHR {
                    ty: sys::GraphicsBindingOpenGLWin32KHR::TYPE,
                    next: ptr::null(),
                    h_dc,
                    h_glrc,
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
                Ok(out)
            }
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
                Ok(out)
            }
            SessionCreateInfo::Wayland { display } => {
                let binding = sys::GraphicsBindingOpenGLWaylandKHR {
                    ty: sys::GraphicsBindingOpenGLWaylandKHR::TYPE,
                    next: ptr::null(),
                    display,
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
                Ok(out)
            }
        }
    }

    fn enumerate_swapchain_images(
        swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
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

    fn enumerate_depth_environment_swapchain_images(
        swapchain: &EnvironmentDepthSwapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        let images = get_arr_init(
            sys::SwapchainImageOpenGLKHR {
                ty: sys::SwapchainImageOpenGLKHR::TYPE,
                next: ptr::null_mut(),
                image: 0,
            },
            |capacity, count, buf| unsafe {
                (swapchain.fp().enumerate_environment_depth_swapchain_images)(
                    swapchain.handle,
                    capacity,
                    count,
                    buf as *mut _,
                )
            },
        )?;
        Ok(images.into_iter().map(|x| x.image as _).collect())
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
    Wayland {
        display: *mut wl_display,
    },
    #[cfg(windows)]
    Windows {
        h_dc: HDC,
        h_glrc: HGLRC,
    },
}

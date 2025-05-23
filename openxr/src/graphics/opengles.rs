use std::ptr;

#[cfg(target_os = "android")]
use sys::{Handle as _, platform::*};

use crate::*;

/// The OpenGL ES graphics API
///
/// See [`XR_KHR_opengl_es_enable`] for safety details.
///
/// [`XR_KHR_opengl_es_enable`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_es_enable
pub enum OpenGlEs {}

impl Graphics for OpenGlEs {
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
            let mut x = sys::GraphicsRequirementsOpenGLESKHR::out(ptr::null_mut());
            cvt((inst.opengles().get_open_gles_graphics_requirements)(
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
    #[allow(unused_variables)]
    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        match *info {
            #[cfg(target_os = "android")]
            SessionCreateInfo::Android {
                display,
                config,
                context,
            } => {
                let binding = sys::GraphicsBindingOpenGLESAndroidKHR {
                    ty: sys::GraphicsBindingOpenGLESAndroidKHR::TYPE,
                    next: ptr::null(),
                    display,
                    config,
                    context,
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
            sys::SwapchainImageOpenGLESKHR {
                ty: sys::SwapchainImageOpenGLESKHR::TYPE,
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
    #[cfg(target_os = "android")]
    Android {
        display: EGLDisplay,
        config: EGLConfig,
        context: EGLContext,
    },
}

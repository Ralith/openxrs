use std::{ffi::c_void, ptr};

use crate::sys::Handle as _;
use crate::*;

/// The Metal graphics API
///
/// See [`XR_KHR_metal_enable`] for safety details.
///
/// [`XR_KHR_metal_enable`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_metal_enable
pub enum Metal {}

impl Graphics for Metal {
    type Requirements = Requirements;
    type SessionCreateInfo = SessionCreateInfo;
    type Format = u64;
    type SwapchainImage = *mut c_void;

    fn raise_format(x: i64) -> Self::Format {
        x as _
    }

    fn lower_format(x: Self::Format) -> i64 {
        x as _
    }

    fn requirements(instance: &Instance, system: SystemId) -> Result<Requirements> {
        let out = unsafe {
            let mut x = sys::GraphicsRequirementsMetalKHR::out(ptr::null_mut());
            cvt((instance.metal().get_metal_graphics_requirements)(
                instance.as_raw(),
                system,
                x.as_mut_ptr(),
            ))?;
            x.assume_init()
        };
        Ok(Requirements {
            metal_device: out.metal_device,
        })
    }

    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        let binding = sys::GraphicsBindingMetalKHR {
            ty: sys::GraphicsBindingMetalKHR::TYPE,
            next: ptr::null(),
            command_queue: info.command_queue,
        };
        let info = sys::SessionCreateInfo {
            ty: sys::SessionCreateInfo::TYPE,
            next: &binding as *const _ as *const _,
            create_flags: Default::default(),
            system_id: system,
        };
        let mut out = sys::Session::NULL;
        unsafe {
            cvt((instance.fp().create_session)(
                instance.as_raw(),
                &info,
                &mut out,
            ))?;
        }
        Ok(out)
    }

    fn enumerate_swapchain_images(
        swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        let images = get_arr_init(
            sys::SwapchainImageMetalKHR {
                ty: sys::SwapchainImageMetalKHR::TYPE,
                next: ptr::null_mut(),
                texture: ptr::null_mut(),
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
        Ok(images.into_iter().map(|image| image.texture).collect())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Requirements {
    pub metal_device: *mut c_void,
}

#[derive(Debug, Copy, Clone)]
pub struct SessionCreateInfo {
    pub command_queue: *mut c_void,
}

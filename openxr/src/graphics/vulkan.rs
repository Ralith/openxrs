use std::{mem, ptr};

use sys::platform::*;

use crate::*;

/// The Vulkan graphics API
///
/// See [`XR_KHR_vulkan_enable`] for safety details.
///
/// [`XR_KHR_vulkan_enable`]: https://www.khronos.org/registry/OpenXR/specs/0.90/html/xrspec.html#XR_KHR_vulkan_enable
pub enum Vulkan {}

impl Graphics for Vulkan {
    type Requirements = Requirements;
    type Format = VkFormat;
    type SessionCreateInfo = SessionCreateInfo;
    type SwapchainImage = VkImage;

    fn raise_format(x: i64) -> Self::Format {
        x as _
    }
    fn lower_format(x: Self::Format) -> i64 {
        x as _
    }

    fn requirements(instance: &Instance, system: SystemId) -> Result<Requirements> {
        let mut out;
        unsafe {
            out = sys::GraphicsRequirementsVulkanKHR {
                ty: sys::GraphicsRequirementsVulkanKHR::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((instance.vulkan().get_vulkan_graphics_requirements)(
                instance.as_raw(),
                system,
                &mut out,
            ))?;
        }
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
        let binding = sys::GraphicsBindingVulkanKHR {
            ty: sys::GraphicsBindingVulkanKHR::TYPE,
            next: ptr::null(),
            instance: info.instance,
            physical_device: info.physical_device,
            device: info.device,
            queue_family_index: info.queue_family_index,
            queue_index: info.queue_index,
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

    fn enumerate_swapchain_images(
        swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        let images = get_arr_init(
            sys::SwapchainImageVulkanKHR {
                ty: sys::SwapchainImageVulkanKHR::TYPE,
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
        Ok(images.into_iter().map(|x| x.image as _).collect())
    }
}

#[derive(Copy, Clone)]
pub struct Requirements {
    pub min_api_version_supported: Version,
    pub max_api_version_supported: Version,
}

#[derive(Copy, Clone)]
pub struct SessionCreateInfo {
    pub instance: VkInstance,
    pub physical_device: VkPhysicalDevice,
    pub device: VkDevice,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

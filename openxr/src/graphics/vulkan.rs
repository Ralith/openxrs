use std::ptr;

use sys::platform::*;

use crate::*;

/// The Vulkan graphics API
///
/// See [`XR_KHR_vulkan_enable2`] for safety details.
///
/// [`XR_KHR_vulkan_enable2`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2
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
        let out = unsafe {
            let mut x = sys::GraphicsRequirementsVulkanKHR::out(ptr::null_mut());
            let fp = if instance.exts().khr_vulkan_enable2.is_some() {
                instance.vulkan().get_vulkan_graphics_requirements2
            } else {
                instance.vulkan_legacy().get_vulkan_graphics_requirements
            };
            cvt(fp(instance.as_raw(), system, x.as_mut_ptr()))?;
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

#[derive(Debug, Copy, Clone)]
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

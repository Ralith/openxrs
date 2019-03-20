use std::{mem, ptr, ffi::CString};

#[cfg(feature = "ash")]
use ash::vk;

use crate::*;

impl<E: Entry> Instance<E> {
    #[inline]
    pub fn properties(&self) -> Result<InstanceProperties> {
        unsafe {
            let mut p = sys::InstanceProperties {
                ty: sys::InstanceProperties::TYPE,
                ..mem::zeroed()
            };
            cvt((self.raw().get_instance_properties)(self.as_raw(), &mut p))?;
            Ok(InstanceProperties {
                runtime_version: p.runtime_version,
                runtime_name: fixed_str(&p.runtime_name).into(),
            })
        }
    }

    #[inline]
    pub fn result_to_string(&self, result: sys::Result) -> Result<String> {
        unsafe {
            let mut s = [0; sys::MAX_RESULT_STRING_SIZE];
            cvt((self.raw().result_to_string)(self.as_raw(), result, s.as_mut_ptr()))?;
            Ok(fixed_str(&s).into())
        }
    }

    #[inline]
    pub fn structure_type_to_string(&self, ty: StructureType) -> Result<String> {
        unsafe {
            let mut s = [0; sys::MAX_STRUCTURE_NAME_SIZE];
            cvt((self.raw().structure_type_to_string)(self.as_raw(), ty, s.as_mut_ptr()))?;
            Ok(fixed_str(&s).into())
        }
    }

    #[inline]
    pub fn system(&self, form_factor: FormFactor) -> Result<SystemId> {
        let info = sys::SystemGetInfo {
            ty: sys::SystemGetInfo::TYPE,
            next: ptr::null_mut(),
            form_factor,
        };
        let mut out = SystemId::NULL;
        unsafe {
            cvt((self.raw().get_system)(self.as_raw(), &info, &mut out))?;
        }
        Ok(out)
    }

    #[inline]
    pub fn system_properties(&self, system: SystemId) -> Result<SystemProperties> {
        unsafe {
            let mut p = sys::SystemProperties {
                ty: sys::SystemProperties::TYPE,
                ..mem::zeroed()
            };
            cvt((self.raw().get_system_properties)(self.as_raw(), system, &mut p))?;
            Ok(SystemProperties {
                system_id: p.system_id,
                vendor_id: p.vendor_id,
                system_name: fixed_str(&p.system_name).into(),
                graphics_properties: p.graphics_properties,
                tracking_properties: SystemTrackingProperties {
                    orientation_tracking: p.tracking_properties.orientation_tracking.into(),
                    position_tracking: p.tracking_properties.position_tracking.into(),
                }
            })
        }
    }

    #[inline]
    pub fn string_to_path(&self, string: &str) -> Result<Path> {
        let string = CString::new(string).map_err(|_| sys::Result::ERROR_PATH_FORMAT_INVALID)?;
        let mut out = Path::NULL;
        unsafe {
            cvt((self.raw().string_to_path)(self.as_raw(), string.as_ptr(), &mut out))?;
        }
        Ok(out)
    }

    #[inline]
    pub fn path_to_string(&self, path: Path) -> Result<String> {
        let mut count = 0;
        unsafe {
            cvt((self.raw().path_to_string)(self.as_raw(), path, 0, &mut count, ptr::null_mut()))?;
            let capacity = count;
            let mut out = Vec::with_capacity(capacity as usize);
            cvt((self.raw().path_to_string)(self.as_raw(), path, capacity, &mut count, out.as_mut_ptr() as _))?;
            out.set_len(count as usize);
            Ok(String::from_utf8_unchecked(out))
        }
    }

    #[cfg(feature = "ash")]
    pub fn create_session_vulkan(&self, info: &VulkanSessionCreateInfo) -> Result<Session<E>> {
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
            next: &binding as _,
            create_flags: Default::default(),
            system_id: info.system_id,
        };
        let mut out = sys::Session::NULL;
        unsafe {
            cvt((self.raw().create_session)(self.as_raw(), &info, &mut out))?;
        }
        Ok(Session::new(self.clone(), out))
    }
}

#[cfg(feature = "ash")]
#[derive(Copy, Clone)]
pub struct VulkanSessionCreateInfo {
    pub system_id: SystemId,
    pub instance: vk::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub queue_family_index: u32,
    pub queue_index: u32,
}

#[derive(Clone)]
pub struct InstanceProperties {
    pub runtime_version: u32,
    pub runtime_name: String,
}

#[derive(Clone)]
pub struct SystemProperties {
    pub system_id: SystemId,
    pub vendor_id: u32,
    pub system_name: String,
    pub graphics_properties: SystemGraphicsProperties,
    pub tracking_properties: SystemTrackingProperties,
}

#[derive(Copy, Clone)]
pub struct SystemTrackingProperties {
    pub orientation_tracking: bool,
    pub position_tracking: bool,
}

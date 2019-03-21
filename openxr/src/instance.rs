use std::{ffi::CString, mem, ptr};

#[cfg(feature = "vulkan")]
use ash::vk;

use crate::*;

impl Instance {
    #[inline]
    pub fn properties(&self) -> Result<InstanceProperties> {
        unsafe {
            let mut p = sys::InstanceProperties {
                ty: sys::InstanceProperties::TYPE,
                ..mem::zeroed()
            };
            cvt((self.fp().get_instance_properties)(self.as_raw(), &mut p))?;
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
            cvt((self.fp().result_to_string)(
                self.as_raw(),
                result,
                s.as_mut_ptr(),
            ))?;
            Ok(fixed_str(&s).into())
        }
    }

    #[inline]
    pub fn structure_type_to_string(&self, ty: StructureType) -> Result<String> {
        unsafe {
            let mut s = [0; sys::MAX_STRUCTURE_NAME_SIZE];
            cvt((self.fp().structure_type_to_string)(
                self.as_raw(),
                ty,
                s.as_mut_ptr(),
            ))?;
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
            cvt((self.fp().get_system)(self.as_raw(), &info, &mut out))?;
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
            cvt((self.fp().get_system_properties)(
                self.as_raw(),
                system,
                &mut p,
            ))?;
            Ok(SystemProperties {
                system_id: p.system_id,
                vendor_id: p.vendor_id,
                system_name: fixed_str(&p.system_name).into(),
                graphics_properties: p.graphics_properties,
                tracking_properties: SystemTrackingProperties {
                    orientation_tracking: p.tracking_properties.orientation_tracking.into(),
                    position_tracking: p.tracking_properties.position_tracking.into(),
                },
            })
        }
    }

    #[inline]
    pub fn string_to_path(&self, string: &str) -> Result<Path> {
        let string = CString::new(string).map_err(|_| sys::Result::ERROR_PATH_FORMAT_INVALID)?;
        let mut out = Path::NULL;
        unsafe {
            cvt((self.fp().string_to_path)(
                self.as_raw(),
                string.as_ptr(),
                &mut out,
            ))?;
        }
        Ok(out)
    }

    #[inline]
    pub fn path_to_string(&self, path: Path) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.fp().path_to_string)(self.as_raw(), path, input, output, buf)
        })
    }

    /// Identify the Vulkan instance extensions required by a system
    ///
    /// Returns a space-delimited list of Vulkan instance extension names
    #[cfg(feature = "vulkan")]
    pub fn vulkan_instance_extensions(&self, system: SystemId) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.vulkan().get_vulkan_instance_extensions)(
                self.as_raw(),
                system,
                input,
                output,
                buf,
            )
        })
    }

    /// Identify the Vulkan device extensions required by a system
    ///
    /// Returns a space-delimited list of Vulkan device extension names
    #[cfg(feature = "vulkan")]
    pub fn vulkan_device_extensions(&self, system: SystemId) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.vulkan().get_vulkan_device_extensions)(self.as_raw(), system, input, output, buf)
        })
    }

    /// Identify the Vulkan graphics device to use for a system
    #[cfg(feature = "vulkan")]
    pub fn vulkan_graphics_device(
        &self,
        system: SystemId,
        vk_instance: vk::Instance,
    ) -> Result<vk::PhysicalDevice> {
        let mut out = vk::PhysicalDevice::null();
        unsafe {
            cvt((self.vulkan().get_vulkan_graphics_device)(
                self.as_raw(),
                system,
                vk_instance,
                &mut out,
            ))?;
        }
        Ok(out)
    }

    /// Query graphics API version requirements
    pub fn graphics_requirements<G: Graphics>(&self, system: SystemId) -> Result<G::Requirements> {
        G::requirements(self, system)
    }

    /// Create a session for Vulkan graphics
    ///
    /// # Safety
    ///
    /// The requirements documented by the graphics API extension must be respected. Among other
    /// requirements, `info` must contain valid handles, and certain operations must be externally
    /// synchronized.
    #[cfg(feature = "vulkan")]
    pub unsafe fn create_session<G: Graphics>(
        &self,
        system: SystemId,
        info: &G::SessionCreateInfo,
    ) -> Result<Session<G>> {
        G::create_session(self.clone(), system, info)
    }

    /// Get the next event, if available
    ///
    /// Returns immediately regardless of whether an event was available.
    pub fn poll_event(&self) -> Result<Option<Event>> {
        unsafe {
            let mut out = sys::EventDataBuffer {
                ty: sys::EventDataBuffer::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            loop {
                let status = cvt((self.fp().poll_event)(self.as_raw(), &mut out))?;
                if status == sys::Result::EVENT_UNAVAILABLE {
                    return Ok(None);
                }
                debug_assert_eq!(status, sys::Result::SUCCESS);
                if let x @ Some(_) = Event::from_raw(&out) {
                    return Ok(x);
                }
            }
        }
    }

    //
    // Internal helpers
    //

    #[cfg(feature = "vulkan")]
    pub(crate) fn vulkan(&self) -> &raw::VulkanEnableKHR {
        self.exts().khr_vulkan_enable.as_ref().unwrap()
    }
    #[cfg(feature = "opengl")]
    pub(crate) fn opengl(&self) -> &raw::OpenglEnableKHR {
        self.exts().khr_opengl_enable.as_ref().unwrap()
    }
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

use std::{ffi::CString, mem, ptr};

use sys::platform::*;

use crate::*;

impl Instance {
    /// Set the debug name of this `Instance`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        if let Some(fp) = self.exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::INSTANCE,
                object_handle: self.as_raw().into_raw(),
                object_name: name.as_ptr(),
            };
            unsafe {
                cvt((fp.set_debug_utils_object_name)(self.as_raw(), &info))?;
            }
        }
        Ok(())
    }

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

    /// Construct a `Path` from a string
    ///
    /// # Safety
    ///
    /// A returned `Path` must not used with any other instance.
    #[inline]
    pub unsafe fn string_to_path(&self, string: &str) -> Result<Path> {
        let string = CString::new(string).map_err(|_| sys::Result::ERROR_PATH_FORMAT_INVALID)?;
        let mut out = Path::NULL;
        cvt((self.fp().string_to_path)(
            self.as_raw(),
            string.as_ptr(),
            &mut out,
        ))?;
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
    /// Returns a space-delimited list of Vulkan instance extension names.
    #[inline]
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
    /// Returns a space-delimited list of Vulkan device extension names.
    #[inline]
    pub fn vulkan_device_extensions(&self, system: SystemId) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.vulkan().get_vulkan_device_extensions)(self.as_raw(), system, input, output, buf)
        })
    }

    /// Identify the Vulkan graphics device to use for a system
    #[inline]
    pub fn vulkan_graphics_device(
        &self,
        system: SystemId,
        vk_instance: VkInstance,
    ) -> Result<VkPhysicalDevice> {
        let mut out = ptr::null();
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

    /// Create a session for a particular graphics API
    ///
    /// # Safety
    ///
    /// The requirements documented by the graphics API extension must be respected. Among other
    /// requirements, `info` must contain valid handles, and certain operations must be externally
    /// synchronized.
    #[inline]
    pub unsafe fn create_session<G: Graphics>(
        &self,
        system: SystemId,
        info: &G::SessionCreateInfo,
    ) -> Result<(Session<G>, FrameStream<G>)> {
        let handle = G::create_session(self, system, info)?;
        Ok(Session::from_raw(self.clone(), handle))
    }

    /// Create a session without graphics support
    #[inline]
    pub fn create_session_headless(&self, system: SystemId) -> Result<Session<Headless>> {
        unsafe { Ok(self.create_session(system, &())?.0) }
    }

    /// Get the next event, if available
    ///
    /// Returns immediately regardless of whether an event was available.
    #[inline]
    pub fn poll_event<'a>(&self, storage: &'a mut EventDataBuffer) -> Result<Option<Event<'a>>> {
        unsafe {
            // Work around a shortcoming in NLL as of 2019-03-22
            let storage: *mut EventDataBuffer = storage;
            loop {
                let status = cvt((self.fp().poll_event)(self.as_raw(), &mut (*storage).inner))?;
                if status == sys::Result::EVENT_UNAVAILABLE {
                    return Ok(None);
                }
                debug_assert_eq!(status, sys::Result::SUCCESS);
                if let x @ Some(_) = Event::from_raw(&(*storage).inner) {
                    return Ok(x);
                }
            }
        }
    }

    /// Enumerates the supported view configuration types
    #[inline]
    pub fn enumerate_view_configurations(
        &self,
        system: SystemId,
    ) -> Result<Vec<ViewConfigurationType>> {
        get_arr(|cap, count, buf| unsafe {
            (self.fp().enumerate_view_configurations)(self.as_raw(), system, cap, count, buf)
        })
    }

    /// Query properties of an individual view configuration
    #[inline]
    pub fn view_configuration_properties(
        &self,
        system: SystemId,
        ty: ViewConfigurationType,
    ) -> Result<ViewConfigurationProperties> {
        let mut out;
        unsafe {
            out = sys::ViewConfigurationProperties {
                ty: sys::ViewConfigurationProperties::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((self.fp().get_view_configuration_properties)(
                self.as_raw(),
                system,
                ty,
                &mut out,
            ))?;
        }
        Ok(ViewConfigurationProperties {
            view_configuration_type: out.view_configuration_type,
            fov_mutable: out.fov_mutable != sys::FALSE,
        })
    }

    #[inline]
    pub fn enumerate_view_configuration_views(
        &self,
        system: SystemId,
        ty: ViewConfigurationType,
    ) -> Result<Vec<ViewConfigurationView>> {
        let views = get_arr_init(
            unsafe {
                sys::ViewConfigurationView {
                    ty: sys::ViewConfigurationView::TYPE,
                    next: ptr::null_mut(),
                    ..mem::uninitialized()
                }
            },
            |capacity, count, buf| unsafe {
                (self.fp().enumerate_view_configuration_views)(
                    self.as_raw(),
                    system,
                    ty,
                    capacity,
                    count,
                    buf as *mut _,
                )
            },
        )?;
        Ok(views
            .into_iter()
            .map(|x| ViewConfigurationView {
                recommended_image_rect_width: x.recommended_image_rect_width,
                max_image_rect_width: x.max_image_rect_width,
                recommended_image_rect_height: x.recommended_image_rect_height,
                max_image_rect_height: x.max_image_rect_height,
                recommended_swapchain_sample_count: x.recommended_swapchain_sample_count,
                max_swapchain_sample_count: x.max_swapchain_sample_count,
            })
            .collect())
    }

    #[inline]
    pub fn enumerate_environment_blend_modes(
        &self,
        system: SystemId,
    ) -> Result<Vec<EnvironmentBlendMode>> {
        get_arr(|cap, count, buf| unsafe {
            (self.fp().enumerate_environment_blend_modes)(self.as_raw(), system, cap, count, buf)
        })
    }

    /// Obtain the current `Time`
    ///
    /// Requires KHR_convert_timespec_time. Most applications should use times from
    /// `FrameStream::wait` and `Action::state` instead.
    #[inline]
    #[cfg(not(windows))]
    pub fn now(&self) -> Result<Time> {
        unsafe {
            let mut now = mem::uninitialized();
            libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now);
            let mut out = mem::uninitialized();
            cvt((self
                .exts()
                .khr_convert_timespec_time
                .as_ref()
                .expect("KHR_convert_timespec_time not loaded")
                .convert_timespec_time_to_time)(
                self.as_raw(),
                &now,
                &mut out,
            ))?;
            Ok(out)
        }
    }

    /// Obtain the current `Time`
    ///
    /// Requires KHR_win32_convert_performance_counter_time. Most applications should use
    /// times from `FrameStream::wait` and `Action::state` instead.
    #[inline]
    #[cfg(windows)]
    pub fn now(&self) -> Result<Time> {
        unsafe {
            let mut now = mem::uninitialized();
            winapi::um::profileapi::QueryPerformanceCounter(&mut now);
            let mut out = mem::uninitialized();
            cvt((self
                .exts()
                .khr_win32_convert_performance_counter_time
                .as_ref()
                .expect("KHR_win32_convert_performance_counter_time not loaded")
                .convert_win32_performance_counter_to_time)(
                self.as_raw(),
                &now,
                &mut out,
            ))?;
            Ok(out)
        }
    }

    //
    // Internal helpers
    //

    pub(crate) fn vulkan(&self) -> &raw::VulkanEnableKHR {
        self.exts()
            .khr_vulkan_enable
            .as_ref()
            .expect("KHR_vulkan_enable not loaded")
    }
    pub(crate) fn opengl(&self) -> &raw::OpenglEnableKHR {
        self.exts()
            .khr_opengl_enable
            .as_ref()
            .expect("KHR_opengl_enable not loaded")
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct SystemTrackingProperties {
    pub orientation_tracking: bool,
    pub position_tracking: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct ViewConfigurationProperties {
    pub view_configuration_type: ViewConfigurationType,
    pub fov_mutable: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct ViewConfigurationView {
    pub recommended_image_rect_width: u32,
    pub max_image_rect_width: u32,
    pub recommended_image_rect_height: u32,
    pub max_image_rect_height: u32,
    pub recommended_swapchain_sample_count: u32,
    pub max_swapchain_sample_count: u32,
}

pub struct EventDataBuffer {
    inner: sys::EventDataBuffer,
}

impl EventDataBuffer {
    pub fn new() -> Self {
        Self {
            inner: sys::EventDataBuffer {
                ty: sys::EventDataBuffer::TYPE,
                next: ptr::null_mut(),
                ..unsafe { mem::uninitialized() }
            },
        }
    }
}

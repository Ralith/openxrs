use std::{
    ffi::CString,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
    sync::{Arc, Mutex},
};

use sys::platform::*;

use crate::*;

/// Root object mediating an application's interaction with OpenXR
///
/// Constructed from an `Entry`.
#[derive(Clone)]
pub struct Instance {
    inner: Arc<InstanceInner>,
}

impl Instance {
    /// Take ownership of an existing instance handle
    ///
    /// # Safety
    ///
    /// `handle` must be the instance handle that was used to load `exts`.
    pub unsafe fn from_raw(
        entry: Entry,
        handle: sys::Instance,
        exts: InstanceExtensions,
    ) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(InstanceInner {
                raw: raw::Instance::load(&entry, handle)?,
                exts,
                handle,
                entry,
                set_name_lock: Mutex::new(()),
            }),
        })
    }

    #[inline]
    pub fn as_raw(&self) -> sys::Instance {
        self.inner.handle
    }

    /// Access the entry points used to create self
    #[inline]
    pub fn entry(&self) -> &Entry {
        &self.inner.entry
    }

    /// Access the core function pointers
    #[inline]
    pub fn fp(&self) -> &raw::Instance {
        &self.inner.raw
    }

    /// Access the internal extension function pointers
    #[inline]
    pub fn exts(&self) -> &InstanceExtensions {
        &self.inner.exts
    }

    /// Set the debug name of this `Instance`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        self.set_name_raw(self.as_raw().into_raw(), name)
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

    #[inline]
    pub fn supports_hand_tracking(&self, system: SystemId) -> Result<bool> {
        unsafe {
            let mut hand = sys::SystemHandTrackingPropertiesEXT::out(ptr::null_mut());
            let mut p = sys::SystemProperties::out(&mut hand as *mut _ as _);
            cvt((self.fp().get_system_properties)(
                self.as_raw(),
                system,
                p.as_mut_ptr(),
            ))?;
            Ok(hand.assume_init().supports_hand_tracking.into())
        }
    }

    /// Construct a `Path` from a string
    ///
    /// A `Path` should only be used with the instance that produced it.
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

    /// Create a Vulkan instance suitable for use with a particular `system`
    ///
    /// `Instance::graphics_requirements::<Vulkan>()` must be called first.
    ///
    /// # Safety
    ///
    /// See [`XR_KHR_vulkan_enable2`].
    ///
    /// [`XR_KHR_vulkan_enable2`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#_vulkan_instance_creation
    #[inline]
    pub unsafe fn create_vulkan_instance(
        &self,
        system: SystemId,
        get_instance_proc_addr: VkGetInstanceProcAddr,
        create_info: *const VkInstanceCreateInfo,
    ) -> Result<Result<VkInstance, VkResult>> {
        let mut instance = ptr::null();
        let mut result = 0;
        cvt((self.vulkan().create_vulkan_instance)(
            self.as_raw(),
            &sys::VulkanInstanceCreateInfoKHR {
                ty: sys::VulkanInstanceCreateInfoKHR::TYPE,
                next: ptr::null(),
                system_id: system,
                create_flags: sys::VulkanInstanceCreateFlagsKHR::EMPTY,
                pfn_get_instance_proc_addr: Some(get_instance_proc_addr),
                vulkan_create_info: create_info,
                vulkan_allocator: ptr::null(),
            },
            &mut instance,
            &mut result,
        ))?;
        if result < 0 {
            return Ok(Err(result));
        }
        Ok(Ok(instance))
    }

    /// Identify the Vulkan instance extensions required by a system
    ///
    /// Returns a space-delimited list of Vulkan instance extension names.
    ///
    /// **Note:** This method requires the `khr_vulkan_enable` extension. Applications should use
    /// `khr_vulkan_enable2` instead, if possible.
    #[inline]
    pub fn vulkan_legacy_instance_extensions(&self, system: SystemId) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.vulkan_legacy().get_vulkan_instance_extensions)(
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
    ///
    /// **Note:** This method requires the `khr_vulkan_enable` extension. Applications should use
    /// `khr_vulkan_enable2` instead, if possible.
    #[inline]
    pub fn vulkan_legacy_device_extensions(&self, system: SystemId) -> Result<String> {
        get_str(|input, output, buf| unsafe {
            (self.vulkan_legacy().get_vulkan_device_extensions)(
                self.as_raw(),
                system,
                input,
                output,
                buf,
            )
        })
    }

    /// Get a suitable [`VkPhysicalDevice`] for use with a particular `system`
    ///
    /// When using 'khr_vulkan_enable2`, call [`Instance::create_vulkan_instance()`] first to get a
    /// suitable `VkInstance`.
    #[inline]
    pub fn vulkan_graphics_device(
        &self,
        system: SystemId,
        vulkan_instance: VkInstance,
    ) -> Result<VkPhysicalDevice> {
        let mut out = ptr::null();
        unsafe {
            if self.exts().khr_vulkan_enable2.is_some() {
                cvt((self.vulkan().get_vulkan_graphics_device2)(
                    self.as_raw(),
                    &sys::VulkanGraphicsDeviceGetInfoKHR {
                        ty: sys::VulkanGraphicsDeviceGetInfoKHR::TYPE,
                        next: ptr::null(),
                        system_id: system,
                        vulkan_instance,
                    },
                    &mut out,
                ))?;
            } else {
                cvt((self.vulkan_legacy().get_vulkan_graphics_device)(
                    self.as_raw(),
                    system,
                    vulkan_instance,
                    &mut out,
                ))?;
            }
        }
        Ok(out)
    }

    /// Get a suitable [`VkDevice`] for use with a particular `system`
    ///
    /// Call [`Instance::vulkan_graphics_device()`] first to get a suitable `VkPhysicalDevice`.
    ///
    /// # Safety
    ///
    /// See [`XR_KHR_vulkan_enable2`].
    ///
    /// [`XR_KHR_vulkan_enable2`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#_vulkan_device_creation
    #[inline]
    pub unsafe fn create_vulkan_device(
        &self,
        system: SystemId,
        get_instance_proc_addr: VkGetInstanceProcAddr,
        physical_device: VkPhysicalDevice,
        create_info: *const VkDeviceCreateInfo,
    ) -> Result<Result<VkDevice, VkResult>> {
        let mut device = ptr::null();
        let mut result = 0;
        cvt((self.vulkan().create_vulkan_device)(
            self.as_raw(),
            &sys::VulkanDeviceCreateInfoKHR {
                ty: sys::VulkanDeviceCreateInfoKHR::TYPE,
                next: ptr::null(),
                system_id: system,
                create_flags: sys::VulkanDeviceCreateFlagsKHR::EMPTY,
                pfn_get_instance_proc_addr: Some(get_instance_proc_addr),
                vulkan_physical_device: physical_device,
                vulkan_create_info: create_info,
                vulkan_allocator: ptr::null(),
            },
            &mut device,
            &mut result,
        ))?;
        if result < 0 {
            return Ok(Err(result));
        }
        Ok(Ok(device))
    }

    /// Query graphics API version requirements
    pub fn graphics_requirements<G: Graphics>(&self, system: SystemId) -> Result<G::Requirements> {
        G::requirements(self, system)
    }

    /// Create a session for a particular graphics API
    ///
    /// Returns three separate objects:
    /// - `Session` exposes most session-related operations and is cheap to clone
    /// - `FrameWaiter` allows blocking until it is time to begin graphics device work, and cannot
    ///   be cloned
    /// - `FrameStream` allows callers to mark the beginning of graphics device work and submit
    ///   completed frames for presentation
    ///
    /// These objects are separate to ensure multithreaded pipelined renderers can safely wait for
    /// the cue to begin a new frame while a prior frame is still being rendered without additional
    /// synchronization.
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
    ) -> Result<(Session<G>, FrameWaiter, FrameStream<G>)> {
        let handle = G::create_session(self, system, info)?;
        Ok(Session::from_raw(self.clone(), handle, Box::new(())))
    }

    /// Refer to [`Instance::create_session()`]. The extra `drop_guard` argument is dropped after
    /// the session is destroyed and this can be used to ensure safety.
    ///
    /// # Safety
    ///
    /// The requirements documented by the graphics API extension must be respected. Among other
    /// requirements, `info` must contain valid handles, and certain operations must be externally
    /// synchronized.
    #[inline]
    pub unsafe fn create_session_with_guard<G: Graphics>(
        &self,
        system: SystemId,
        info: &G::SessionCreateInfo,
        drop_guard: DropGuard,
    ) -> Result<(Session<G>, FrameWaiter, FrameStream<G>)> {
        let handle = G::create_session(self, system, info)?;
        Ok(Session::from_raw(self.clone(), handle, drop_guard))
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
                ((*storage).inner.as_mut_ptr() as *mut sys::BaseInStructure).write(
                    sys::BaseInStructure {
                        ty: sys::EventDataBuffer::TYPE,
                        next: ptr::null(),
                    },
                );
                let status = cvt((self.fp().poll_event)(
                    self.as_raw(),
                    (*storage).inner.as_mut_ptr(),
                ))?;
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
        let out = unsafe {
            let mut x = sys::ViewConfigurationProperties::out(ptr::null_mut());
            cvt((self.fp().get_view_configuration_properties)(
                self.as_raw(),
                system,
                ty,
                x.as_mut_ptr(),
            ))?;
            x.assume_init()
        };
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
            sys::ViewConfigurationView::out(ptr::null_mut()),
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
            .map(|x| {
                let x = unsafe { x.assume_init() };
                ViewConfigurationView {
                    recommended_image_rect_width: x.recommended_image_rect_width,
                    max_image_rect_width: x.max_image_rect_width,
                    recommended_image_rect_height: x.recommended_image_rect_height,
                    max_image_rect_height: x.max_image_rect_height,
                    recommended_swapchain_sample_count: x.recommended_swapchain_sample_count,
                    max_swapchain_sample_count: x.max_swapchain_sample_count,
                }
            })
            .collect())
    }

    #[inline]
    pub fn enumerate_environment_blend_modes(
        &self,
        system: SystemId,
        view_configuration_type: ViewConfigurationType,
    ) -> Result<Vec<EnvironmentBlendMode>> {
        get_arr(|cap, count, buf| unsafe {
            (self.fp().enumerate_environment_blend_modes)(
                self.as_raw(),
                system,
                view_configuration_type,
                cap,
                count,
                buf,
            )
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
            let mut now = MaybeUninit::uninit();
            libc::clock_gettime(libc::CLOCK_MONOTONIC, now.as_mut_ptr());
            let now = now.assume_init();
            let mut out = MaybeUninit::uninit();
            cvt((self
                .exts()
                .khr_convert_timespec_time
                .as_ref()
                .expect("KHR_convert_timespec_time not loaded")
                .convert_timespec_time_to_time)(
                self.as_raw(),
                &now,
                out.as_mut_ptr(),
            ))?;
            Ok(out.assume_init())
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
            let mut now = MaybeUninit::uninit();
            winapi::um::profileapi::QueryPerformanceCounter(now.as_mut_ptr());
            let now = now.assume_init();
            let mut out = MaybeUninit::uninit();
            cvt((self
                .exts()
                .khr_win32_convert_performance_counter_time
                .as_ref()
                .expect("KHR_win32_convert_performance_counter_time not loaded")
                .convert_win32_performance_counter_to_time)(
                self.as_raw(),
                &now,
                out.as_mut_ptr(),
            ))?;
            Ok(out.assume_init())
        }
    }

    /// Specify default bindings for a well-known input archetype
    #[inline]
    pub fn suggest_interaction_profile_bindings(
        &self,
        interaction_profile: Path,
        bindings: &[Binding],
    ) -> Result<()> {
        let info = sys::InteractionProfileSuggestedBinding {
            ty: sys::InteractionProfileSuggestedBinding::TYPE,
            next: ptr::null(),
            interaction_profile,
            count_suggested_bindings: bindings.len() as u32,
            suggested_bindings: bindings.as_ptr() as *const _ as _,
        };
        unsafe {
            cvt((self.fp().suggest_interaction_profile_bindings)(
                self.as_raw(),
                &info,
            ))?;
        }
        Ok(())
    }

    /// Allocate a new [`ActionSet`]
    ///
    /// [`ActionSet`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#input-action-creation
    #[inline]
    pub fn create_action_set(
        &self,
        name: &str,
        localized_name: &str,
        priority: u32,
    ) -> Result<ActionSet> {
        let info = builder::ActionSetCreateInfo::new()
            .action_set_name(name)
            .localized_action_set_name(localized_name)
            .priority(priority);
        unsafe {
            let mut out = sys::ActionSet::NULL;
            cvt((self.fp().create_action_set)(
                self.as_raw(),
                info.as_raw(),
                &mut out,
            ))?;
            Ok(ActionSet::from_raw(self.clone(), out))
        }
    }

    //
    // Internal helpers
    //

    pub(crate) fn set_name_raw(&self, object: u64, name: &str) -> Result<()> {
        if let Some(fp) = self.exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::INSTANCE,
                object_handle: object,
                object_name: name.as_ptr(),
            };
            // The following function call must be synchronized for each object and shouldn't be
            // performance-relevant, so we use a conservative instance-global lock for simplicity.
            let guard = self.inner.set_name_lock.lock().unwrap();
            unsafe {
                cvt((fp.set_debug_utils_object_name)(self.as_raw(), &info))?;
            }
            drop(guard);
        }
        Ok(())
    }
    pub(crate) fn vulkan(&self) -> &raw::VulkanEnable2KHR {
        self.exts()
            .khr_vulkan_enable2
            .as_ref()
            .expect("KHR_vulkan_enable2 not loaded")
    }
    pub(crate) fn vulkan_legacy(&self) -> &raw::VulkanEnableKHR {
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
    #[cfg(windows)]
    pub(crate) fn d3d11(&self) -> &raw::D3d11EnableKHR {
        self.exts()
            .khr_d3d11_enable
            .as_ref()
            .expect("KHR_d3d11_enable not loaded")
    }
    pub(crate) fn visibility_mask(&self) -> &raw::VisibilityMaskKHR {
        self.exts()
            .khr_visibility_mask
            .as_ref()
            .expect("KHR_visibility_mask not loaded")
    }
}

struct InstanceInner {
    entry: Entry,
    handle: sys::Instance,
    raw: raw::Instance,
    exts: InstanceExtensions,
    set_name_lock: Mutex<()>,
}

impl Drop for InstanceInner {
    fn drop(&mut self) {
        unsafe {
            (self.raw.destroy_instance)(self.handle);
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstanceProperties {
    pub runtime_version: Version,
    pub runtime_name: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ViewConfigurationView {
    pub recommended_image_rect_width: u32,
    pub max_image_rect_width: u32,
    pub recommended_image_rect_height: u32,
    pub max_image_rect_height: u32,
    pub recommended_swapchain_sample_count: u32,
    pub max_swapchain_sample_count: u32,
}

pub struct EventDataBuffer {
    inner: MaybeUninit<sys::EventDataBuffer>,
}

impl EventDataBuffer {
    pub fn new() -> Self {
        Self {
            inner: MaybeUninit::uninit(),
        }
    }
}

impl Default for EventDataBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Binding<'a> {
    _inner: sys::ActionSuggestedBinding,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Binding<'a> {
    #[inline]
    pub fn new<T: ActionTy>(action: &'a Action<T>, binding: Path) -> Self {
        Self {
            _inner: sys::ActionSuggestedBinding {
                action: action.as_raw(),
                binding,
            },
            _marker: PhantomData,
        }
    }
}

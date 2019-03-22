use crate::{Entry, Result, Time};
use std::{os::raw::c_char, sync::Arc};
pub use sys::{
    ActionType, AndroidThreadTypeKHR, Color4f, CompositionLayerFlags,
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, EnvironmentBlendMode,
    Extent2Df, Extent2Di, EyeVisibility, FormFactor, Fovf, InputSourceLocalizedNameFlags,
    InstanceCreateFlags, ObjectType, Offset2Df, Offset2Di, PerfSettingsDomainEXT,
    PerfSettingsLevelEXT, PerfSettingsNotificationLevelEXT, PerfSettingsSubDomainEXT, Posef,
    Quaternionf, Rect2Df, Rect2Di, ReferenceSpaceType, SessionCreateFlags, SessionState,
    SpaceRelationFlags, StructureType, SwapchainCreateFlags, SwapchainUsageFlags,
    SystemGraphicsProperties, Vector2f, Vector3f, Vector4f, ViewConfigurationType, ViewStateFlags,
    VisibilityMaskTypeKHR,
};
struct InstanceInner {
    entry: Entry,
    handle: sys::Instance,
    raw: raw::Instance,
    exts: InstanceExtensions,
}
impl Drop for InstanceInner {
    fn drop(&mut self) {
        unsafe {
            (self.raw.destroy_instance)(self.handle);
        }
    }
}
#[derive(Clone)]
pub struct Instance {
    inner: Arc<InstanceInner>,
}
impl Instance {
    #[doc = r" Take ownership of an existing instance handle"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" `handle` must be the instance handle that was used to load `exts`."]
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
            }),
        })
    }
    #[inline]
    pub fn as_raw(&self) -> sys::Instance {
        self.inner.handle
    }
    #[doc = r" Access the entry points used to create self"]
    #[inline]
    pub fn entry(&self) -> &Entry {
        &self.inner.entry
    }
    #[doc = r" Access the core function pointers"]
    #[inline]
    pub fn fp(&self) -> &raw::Instance {
        &self.inner.raw
    }
    #[doc = r" Access the internal extension function pointers"]
    #[inline]
    pub fn exts(&self) -> &InstanceExtensions {
        &self.inner.exts
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct ExtensionSet {
    pub ext_performance_settings: bool,
    pub ext_thermal_query: bool,
    pub ext_debug_utils: bool,
    #[cfg(target_os = "android")]
    pub khr_android_thread_settings: bool,
    #[cfg(target_os = "android")]
    pub khr_android_surface_swapchain: bool,
    pub khr_composition_layer_cube: bool,
    #[cfg(target_os = "android")]
    pub khr_android_create_instance: bool,
    pub khr_composition_layer_depth: bool,
    pub khr_headless: bool,
    #[cfg(feature = "vulkan")]
    pub khr_vulkan_swapchain_format_list: bool,
    pub khr_composition_layer_cylinder: bool,
    pub khr_composition_layer_equirect: bool,
    #[cfg(feature = "opengl")]
    pub khr_opengl_enable: bool,
    #[cfg(feature = "opengles")]
    pub khr_opengl_es_enable: bool,
    #[cfg(feature = "vulkan")]
    pub khr_vulkan_enable: bool,
    #[cfg(feature = "d3d")]
    pub khr_d3d10_enable: bool,
    #[cfg(feature = "d3d")]
    pub khr_d3d11_enable: bool,
    #[cfg(feature = "d3d")]
    pub khr_d3d12_enable: bool,
    pub khr_visibility_mask: bool,
    #[cfg(target_os = "windows")]
    pub khr_win32_convert_performance_counter_time: bool,
    pub khr_convert_timespec_time: bool,
}
impl ExtensionSet {
    pub(crate) fn from_properties(properties: &[sys::ExtensionProperties]) -> Self {
        let mut out = Self::default();
        for ext in properties {
            match crate::fixed_str_bytes(&ext.extension_name) {
                raw::PerformanceSettingsEXT::NAME => {
                    out.ext_performance_settings = true;
                }
                raw::ThermalQueryEXT::NAME => {
                    out.ext_thermal_query = true;
                }
                raw::DebugUtilsEXT::NAME => {
                    out.ext_debug_utils = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidThreadSettingsKHR::NAME => {
                    out.khr_android_thread_settings = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSurfaceSwapchainKHR::NAME => {
                    out.khr_android_surface_swapchain = true;
                }
                raw::CompositionLayerCubeKHR::NAME => {
                    out.khr_composition_layer_cube = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidCreateInstanceKHR::NAME => {
                    out.khr_android_create_instance = true;
                }
                raw::CompositionLayerDepthKHR::NAME => {
                    out.khr_composition_layer_depth = true;
                }
                raw::HeadlessKHR::NAME => {
                    out.khr_headless = true;
                }
                #[cfg(feature = "vulkan")]
                raw::VulkanSwapchainFormatListKHR::NAME => {
                    out.khr_vulkan_swapchain_format_list = true;
                }
                raw::CompositionLayerCylinderKHR::NAME => {
                    out.khr_composition_layer_cylinder = true;
                }
                raw::CompositionLayerEquirectKHR::NAME => {
                    out.khr_composition_layer_equirect = true;
                }
                #[cfg(feature = "opengl")]
                raw::OpenglEnableKHR::NAME => {
                    out.khr_opengl_enable = true;
                }
                #[cfg(feature = "opengles")]
                raw::OpenglEsEnableKHR::NAME => {
                    out.khr_opengl_es_enable = true;
                }
                #[cfg(feature = "vulkan")]
                raw::VulkanEnableKHR::NAME => {
                    out.khr_vulkan_enable = true;
                }
                #[cfg(feature = "d3d")]
                raw::D3d10EnableKHR::NAME => {
                    out.khr_d3d10_enable = true;
                }
                #[cfg(feature = "d3d")]
                raw::D3d11EnableKHR::NAME => {
                    out.khr_d3d11_enable = true;
                }
                #[cfg(feature = "d3d")]
                raw::D3d12EnableKHR::NAME => {
                    out.khr_d3d12_enable = true;
                }
                raw::VisibilityMaskKHR::NAME => {
                    out.khr_visibility_mask = true;
                }
                #[cfg(target_os = "windows")]
                raw::Win32ConvertPerformanceCounterTimeKHR::NAME => {
                    out.khr_win32_convert_performance_counter_time = true;
                }
                raw::ConvertTimespecTimeKHR::NAME => {
                    out.khr_convert_timespec_time = true;
                }
                _ => {}
            }
        }
        out
    }
    pub(crate) fn names(&self) -> Vec<*const c_char> {
        let mut out = Vec::new();
        {
            if self.ext_performance_settings {
                out.push(raw::PerformanceSettingsEXT::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.ext_thermal_query {
                out.push(raw::ThermalQueryEXT::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.ext_debug_utils {
                out.push(raw::DebugUtilsEXT::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_thread_settings {
                out.push(raw::AndroidThreadSettingsKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_surface_swapchain {
                out.push(raw::AndroidSurfaceSwapchainKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_composition_layer_cube {
                out.push(raw::CompositionLayerCubeKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_create_instance {
                out.push(raw::AndroidCreateInstanceKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_composition_layer_depth {
                out.push(raw::CompositionLayerDepthKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_headless {
                out.push(raw::HeadlessKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "vulkan")]
        {
            if self.khr_vulkan_swapchain_format_list {
                out.push(raw::VulkanSwapchainFormatListKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_composition_layer_cylinder {
                out.push(raw::CompositionLayerCylinderKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_composition_layer_equirect {
                out.push(raw::CompositionLayerEquirectKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "opengl")]
        {
            if self.khr_opengl_enable {
                out.push(raw::OpenglEnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "opengles")]
        {
            if self.khr_opengl_es_enable {
                out.push(raw::OpenglEsEnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "vulkan")]
        {
            if self.khr_vulkan_enable {
                out.push(raw::VulkanEnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "d3d")]
        {
            if self.khr_d3d10_enable {
                out.push(raw::D3d10EnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "d3d")]
        {
            if self.khr_d3d11_enable {
                out.push(raw::D3d11EnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(feature = "d3d")]
        {
            if self.khr_d3d12_enable {
                out.push(raw::D3d12EnableKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        {
            if self.khr_visibility_mask {
                out.push(raw::VisibilityMaskKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        #[cfg(target_os = "windows")]
        {
            if self.khr_win32_convert_performance_counter_time {
                out.push(
                    raw::Win32ConvertPerformanceCounterTimeKHR::NAME.as_ptr() as *const _ as _,
                );
            }
        }
        {
            if self.khr_convert_timespec_time {
                out.push(raw::ConvertTimespecTimeKHR::NAME.as_ptr() as *const _ as _);
            }
        }
        out
    }
}
#[doc = r" Extensions used internally by the bindings"]
#[derive(Default, Copy, Clone)]
pub struct InstanceExtensions {
    pub ext_performance_settings: Option<raw::PerformanceSettingsEXT>,
    pub ext_thermal_query: Option<raw::ThermalQueryEXT>,
    pub ext_debug_utils: Option<raw::DebugUtilsEXT>,
    #[cfg(target_os = "android")]
    pub khr_android_thread_settings: Option<raw::AndroidThreadSettingsKHR>,
    #[cfg(target_os = "android")]
    pub khr_android_surface_swapchain: Option<raw::AndroidSurfaceSwapchainKHR>,
    pub khr_composition_layer_cube: Option<raw::CompositionLayerCubeKHR>,
    #[cfg(target_os = "android")]
    pub khr_android_create_instance: Option<raw::AndroidCreateInstanceKHR>,
    pub khr_composition_layer_depth: Option<raw::CompositionLayerDepthKHR>,
    pub khr_headless: Option<raw::HeadlessKHR>,
    #[cfg(feature = "vulkan")]
    pub khr_vulkan_swapchain_format_list: Option<raw::VulkanSwapchainFormatListKHR>,
    pub khr_composition_layer_cylinder: Option<raw::CompositionLayerCylinderKHR>,
    pub khr_composition_layer_equirect: Option<raw::CompositionLayerEquirectKHR>,
    #[cfg(feature = "opengl")]
    pub khr_opengl_enable: Option<raw::OpenglEnableKHR>,
    #[cfg(feature = "opengles")]
    pub khr_opengl_es_enable: Option<raw::OpenglEsEnableKHR>,
    #[cfg(feature = "vulkan")]
    pub khr_vulkan_enable: Option<raw::VulkanEnableKHR>,
    #[cfg(feature = "d3d")]
    pub khr_d3d10_enable: Option<raw::D3d10EnableKHR>,
    #[cfg(feature = "d3d")]
    pub khr_d3d11_enable: Option<raw::D3d11EnableKHR>,
    #[cfg(feature = "d3d")]
    pub khr_d3d12_enable: Option<raw::D3d12EnableKHR>,
    pub khr_visibility_mask: Option<raw::VisibilityMaskKHR>,
    #[cfg(target_os = "windows")]
    pub khr_win32_convert_performance_counter_time:
        Option<raw::Win32ConvertPerformanceCounterTimeKHR>,
    pub khr_convert_timespec_time: Option<raw::ConvertTimespecTimeKHR>,
}
impl InstanceExtensions {
    #[doc = r" Load extension function pointer tables"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" `instance` must be a valid instance handle."]
    pub unsafe fn load(
        entry: &Entry,
        instance: sys::Instance,
        required: &ExtensionSet,
    ) -> Result<Self> {
        Ok(Self {
            ext_performance_settings: if required.ext_performance_settings {
                Some(raw::PerformanceSettingsEXT::load(entry, instance)?)
            } else {
                None
            },
            ext_thermal_query: if required.ext_thermal_query {
                Some(raw::ThermalQueryEXT::load(entry, instance)?)
            } else {
                None
            },
            ext_debug_utils: if required.ext_debug_utils {
                Some(raw::DebugUtilsEXT::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(target_os = "android")]
            khr_android_thread_settings: if required.khr_android_thread_settings {
                Some(raw::AndroidThreadSettingsKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(target_os = "android")]
            khr_android_surface_swapchain: if required.khr_android_surface_swapchain {
                Some(raw::AndroidSurfaceSwapchainKHR::load(entry, instance)?)
            } else {
                None
            },
            khr_composition_layer_cube: if required.khr_composition_layer_cube {
                Some(raw::CompositionLayerCubeKHR {})
            } else {
                None
            },
            #[cfg(target_os = "android")]
            khr_android_create_instance: if required.khr_android_create_instance {
                Some(raw::AndroidCreateInstanceKHR {})
            } else {
                None
            },
            khr_composition_layer_depth: if required.khr_composition_layer_depth {
                Some(raw::CompositionLayerDepthKHR {})
            } else {
                None
            },
            khr_headless: if required.khr_headless {
                Some(raw::HeadlessKHR {})
            } else {
                None
            },
            #[cfg(feature = "vulkan")]
            khr_vulkan_swapchain_format_list: if required.khr_vulkan_swapchain_format_list {
                Some(raw::VulkanSwapchainFormatListKHR {})
            } else {
                None
            },
            khr_composition_layer_cylinder: if required.khr_composition_layer_cylinder {
                Some(raw::CompositionLayerCylinderKHR {})
            } else {
                None
            },
            khr_composition_layer_equirect: if required.khr_composition_layer_equirect {
                Some(raw::CompositionLayerEquirectKHR {})
            } else {
                None
            },
            #[cfg(feature = "opengl")]
            khr_opengl_enable: if required.khr_opengl_enable {
                Some(raw::OpenglEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(feature = "opengles")]
            khr_opengl_es_enable: if required.khr_opengl_es_enable {
                Some(raw::OpenglEsEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(feature = "vulkan")]
            khr_vulkan_enable: if required.khr_vulkan_enable {
                Some(raw::VulkanEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(feature = "d3d")]
            khr_d3d10_enable: if required.khr_d3d10_enable {
                Some(raw::D3d10EnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(feature = "d3d")]
            khr_d3d11_enable: if required.khr_d3d11_enable {
                Some(raw::D3d11EnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(feature = "d3d")]
            khr_d3d12_enable: if required.khr_d3d12_enable {
                Some(raw::D3d12EnableKHR::load(entry, instance)?)
            } else {
                None
            },
            khr_visibility_mask: if required.khr_visibility_mask {
                Some(raw::VisibilityMaskKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(target_os = "windows")]
            khr_win32_convert_performance_counter_time: if required
                .khr_win32_convert_performance_counter_time
            {
                Some(raw::Win32ConvertPerformanceCounterTimeKHR::load(
                    entry, instance,
                )?)
            } else {
                None
            },
            khr_convert_timespec_time: if required.khr_convert_timespec_time {
                Some(raw::ConvertTimespecTimeKHR::load(entry, instance)?)
            } else {
                None
            },
        })
    }
}
#[derive(Copy, Clone)]
pub enum Event {
    EventsLost {
        lost_event_count: u32,
    },
    InstanceLossPending {
        loss_time: Time,
    },
    InteractionProfileChanged,
    PerfSettingsEXT {
        domain: PerfSettingsDomainEXT,
        sub_domain: PerfSettingsSubDomainEXT,
        from_level: PerfSettingsNotificationLevelEXT,
        to_level: PerfSettingsNotificationLevelEXT,
    },
    ReferenceSpaceChangePending {
        reference_space_type: ReferenceSpaceType,
        change_time: Time,
        pose_valid: bool,
        pose_in_previous_space: Posef,
    },
    SessionStateChanged {
        session: sys::Session,
        state: SessionState,
        time: Time,
    },
    VisibilityMaskChangedKHR {
        view_configuration_type: ViewConfigurationType,
        view_index: u32,
    },
}
impl Event {
    #[doc = r" Decode an event"]
    #[doc = r""]
    #[doc = r" Returns `None` if the event type is not recognized, e.g. if it's from an unknown extension"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" `raw` must refer to an `EventDataBuffer` populated by a successful call to `xrPollEvent`."]
    pub unsafe fn from_raw(raw: &sys::EventDataBuffer) -> Option<Self> {
        Some(match raw.ty {
            sys::StructureType::EVENT_DATA_EVENTS_LOST => {
                let sys::EventDataEventsLost {
                    lost_event_count, ..
                } = *(raw as *const _ as *const sys::EventDataEventsLost);
                Event::EventsLost { lost_event_count }
            }
            sys::StructureType::EVENT_DATA_INSTANCE_LOSS_PENDING => {
                let sys::EventDataInstanceLossPending { loss_time, .. } =
                    *(raw as *const _ as *const sys::EventDataInstanceLossPending);
                Event::InstanceLossPending { loss_time }
            }
            sys::StructureType::EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
                Event::InteractionProfileChanged
            }
            sys::StructureType::EVENT_DATA_PERF_SETTINGS_EXT => {
                let sys::EventDataPerfSettingsEXT {
                    domain,
                    sub_domain,
                    from_level,
                    to_level,
                    ..
                } = *(raw as *const _ as *const sys::EventDataPerfSettingsEXT);
                Event::PerfSettingsEXT {
                    domain,
                    sub_domain,
                    from_level,
                    to_level,
                }
            }
            sys::StructureType::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                let sys::EventDataReferenceSpaceChangePending {
                    reference_space_type,
                    change_time,
                    pose_valid,
                    pose_in_previous_space,
                    ..
                } = *(raw as *const _ as *const sys::EventDataReferenceSpaceChangePending);
                Event::ReferenceSpaceChangePending {
                    reference_space_type,
                    change_time,
                    pose_valid: pose_valid != sys::FALSE,
                    pose_in_previous_space,
                }
            }
            sys::StructureType::EVENT_DATA_SESSION_STATE_CHANGED => {
                let sys::EventDataSessionStateChanged {
                    session,
                    state,
                    time,
                    ..
                } = *(raw as *const _ as *const sys::EventDataSessionStateChanged);
                Event::SessionStateChanged {
                    session,
                    state,
                    time,
                }
            }
            sys::StructureType::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
                let sys::EventDataVisibilityMaskChangedKHR {
                    view_configuration_type,
                    view_index,
                    ..
                } = *(raw as *const _ as *const sys::EventDataVisibilityMaskChangedKHR);
                Event::VisibilityMaskChangedKHR {
                    view_configuration_type,
                    view_index,
                }
            }
            _ => {
                return None;
            }
        })
    }
}
pub mod raw {
    use crate::{Entry, Result};
    use std::{ffi::CStr, mem};
    use sys::pfn;
    #[derive(Copy, Clone)]
    pub struct Instance {
        pub acquire_swapchain_image: pfn::AcquireSwapchainImage,
        pub apply_haptic_feedback: pfn::ApplyHapticFeedback,
        pub begin_frame: pfn::BeginFrame,
        pub begin_session: pfn::BeginSession,
        pub create_action: pfn::CreateAction,
        pub create_action_set: pfn::CreateActionSet,
        pub create_action_space: pfn::CreateActionSpace,
        pub create_instance: pfn::CreateInstance,
        pub create_reference_space: pfn::CreateReferenceSpace,
        pub create_session: pfn::CreateSession,
        pub create_swapchain: pfn::CreateSwapchain,
        pub destroy_action: pfn::DestroyAction,
        pub destroy_action_set: pfn::DestroyActionSet,
        pub destroy_instance: pfn::DestroyInstance,
        pub destroy_session: pfn::DestroySession,
        pub destroy_space: pfn::DestroySpace,
        pub destroy_swapchain: pfn::DestroySwapchain,
        pub end_frame: pfn::EndFrame,
        pub end_session: pfn::EndSession,
        pub enumerate_api_layer_properties: pfn::EnumerateApiLayerProperties,
        pub enumerate_environment_blend_modes: pfn::EnumerateEnvironmentBlendModes,
        pub enumerate_instance_extension_properties: pfn::EnumerateInstanceExtensionProperties,
        pub enumerate_reference_spaces: pfn::EnumerateReferenceSpaces,
        pub enumerate_swapchain_formats: pfn::EnumerateSwapchainFormats,
        pub enumerate_swapchain_images: pfn::EnumerateSwapchainImages,
        pub enumerate_view_configuration_views: pfn::EnumerateViewConfigurationViews,
        pub enumerate_view_configurations: pfn::EnumerateViewConfigurations,
        pub get_action_state_boolean: pfn::GetActionStateBoolean,
        pub get_action_state_pose: pfn::GetActionStatePose,
        pub get_action_state_vector1f: pfn::GetActionStateVector1f,
        pub get_action_state_vector2f: pfn::GetActionStateVector2f,
        pub get_bound_sources_for_action: pfn::GetBoundSourcesForAction,
        pub get_current_interaction_profile: pfn::GetCurrentInteractionProfile,
        pub get_input_source_localized_name: pfn::GetInputSourceLocalizedName,
        pub get_instance_proc_addr: pfn::GetInstanceProcAddr,
        pub get_instance_properties: pfn::GetInstanceProperties,
        pub get_reference_space_bounds_rect: pfn::GetReferenceSpaceBoundsRect,
        pub get_system: pfn::GetSystem,
        pub get_system_properties: pfn::GetSystemProperties,
        pub get_view_configuration_properties: pfn::GetViewConfigurationProperties,
        pub locate_space: pfn::LocateSpace,
        pub locate_views: pfn::LocateViews,
        pub path_to_string: pfn::PathToString,
        pub poll_event: pfn::PollEvent,
        pub release_swapchain_image: pfn::ReleaseSwapchainImage,
        pub result_to_string: pfn::ResultToString,
        pub set_interaction_profile_suggested_bindings: pfn::SetInteractionProfileSuggestedBindings,
        pub stop_haptic_feedback: pfn::StopHapticFeedback,
        pub string_to_path: pfn::StringToPath,
        pub structure_type_to_string: pfn::StructureTypeToString,
        pub sync_action_data: pfn::SyncActionData,
        pub wait_frame: pfn::WaitFrame,
        pub wait_swapchain_image: pfn::WaitSwapchainImage,
    }
    impl Instance {
        #[doc = r" Load the core function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                acquire_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrAcquireSwapchainImage\0"),
                )?),
                apply_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrApplyHapticFeedback\0"),
                )?),
                begin_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginFrame\0"),
                )?),
                begin_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginSession\0"),
                )?),
                create_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateAction\0"),
                )?),
                create_action_set: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSet\0"),
                )?),
                create_action_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSpace\0"),
                )?),
                create_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateInstance\0"),
                )?),
                create_reference_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateReferenceSpace\0"),
                )?),
                create_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSession\0"),
                )?),
                create_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSwapchain\0"),
                )?),
                destroy_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyAction\0"),
                )?),
                destroy_action_set: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyActionSet\0"),
                )?),
                destroy_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyInstance\0"),
                )?),
                destroy_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySession\0"),
                )?),
                destroy_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpace\0"),
                )?),
                destroy_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySwapchain\0"),
                )?),
                end_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndFrame\0"),
                )?),
                end_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndSession\0"),
                )?),
                enumerate_api_layer_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateApiLayerProperties\0"),
                )?),
                enumerate_environment_blend_modes: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateEnvironmentBlendModes\0"),
                )?),
                enumerate_instance_extension_properties: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumerateInstanceExtensionProperties\0",
                        ),
                    )?,
                ),
                enumerate_reference_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateReferenceSpaces\0"),
                )?),
                enumerate_swapchain_formats: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainFormats\0"),
                )?),
                enumerate_swapchain_images: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainImages\0"),
                )?),
                enumerate_view_configuration_views: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurationViews\0"),
                )?),
                enumerate_view_configurations: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurations\0"),
                )?),
                get_action_state_boolean: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateBoolean\0"),
                )?),
                get_action_state_pose: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStatePose\0"),
                )?),
                get_action_state_vector1f: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector1f\0"),
                )?),
                get_action_state_vector2f: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector2f\0"),
                )?),
                get_bound_sources_for_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetBoundSourcesForAction\0"),
                )?),
                get_current_interaction_profile: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetCurrentInteractionProfile\0"),
                )?),
                get_input_source_localized_name: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInputSourceLocalizedName\0"),
                )?),
                get_instance_proc_addr: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProcAddr\0"),
                )?),
                get_instance_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProperties\0"),
                )?),
                get_reference_space_bounds_rect: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetReferenceSpaceBoundsRect\0"),
                )?),
                get_system: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystem\0"),
                )?),
                get_system_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystemProperties\0"),
                )?),
                get_view_configuration_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetViewConfigurationProperties\0"),
                )?),
                locate_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateSpace\0"),
                )?),
                locate_views: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateViews\0"),
                )?),
                path_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPathToString\0"),
                )?),
                poll_event: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPollEvent\0"),
                )?),
                release_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrReleaseSwapchainImage\0"),
                )?),
                result_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrResultToString\0"),
                )?),
                set_interaction_profile_suggested_bindings: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSetInteractionProfileSuggestedBindings\0",
                        ),
                    )?,
                ),
                stop_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStopHapticFeedback\0"),
                )?),
                string_to_path: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStringToPath\0"),
                )?),
                structure_type_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStructureTypeToString\0"),
                )?),
                sync_action_data: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSyncActionData\0"),
                )?),
                wait_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitFrame\0"),
                )?),
                wait_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitSwapchainImage\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct PerformanceSettingsEXT {
        pub perf_settings_set_performance_level: pfn::PerfSettingsSetPerformanceLevelEXT,
    }
    impl PerformanceSettingsEXT {
        pub const VERSION: u32 = sys::EXT_performance_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                perf_settings_set_performance_level: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPerfSettingsSetPerformanceLevelEXT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ThermalQueryEXT {
        pub thermal_get_temperature_trend: pfn::ThermalGetTemperatureTrendEXT,
    }
    impl ThermalQueryEXT {
        pub const VERSION: u32 = sys::EXT_thermal_query_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_THERMAL_QUERY_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                thermal_get_temperature_trend: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrThermalGetTemperatureTrendEXT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct DebugUtilsEXT {
        pub set_debug_utils_object_name: pfn::SetDebugUtilsObjectNameEXT,
        pub create_debug_utils_messenger: pfn::CreateDebugUtilsMessengerEXT,
        pub destroy_debug_utils_messenger: pfn::DestroyDebugUtilsMessengerEXT,
        pub submit_debug_utils_message: pfn::SubmitDebugUtilsMessageEXT,
        pub session_begin_debug_utils_label_region: pfn::SessionBeginDebugUtilsLabelRegionEXT,
        pub session_end_debug_utils_label_region: pfn::SessionEndDebugUtilsLabelRegionEXT,
        pub session_insert_debug_utils_label: pfn::SessionInsertDebugUtilsLabelEXT,
    }
    impl DebugUtilsEXT {
        pub const VERSION: u32 = sys::EXT_debug_utils_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_DEBUG_UTILS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_debug_utils_object_name: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetDebugUtilsObjectNameEXT\0"),
                )?),
                create_debug_utils_messenger: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateDebugUtilsMessengerEXT\0"),
                )?),
                destroy_debug_utils_messenger: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyDebugUtilsMessengerEXT\0"),
                )?),
                submit_debug_utils_message: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSubmitDebugUtilsMessageEXT\0"),
                )?),
                session_begin_debug_utils_label_region: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSessionBeginDebugUtilsLabelRegionEXT\0",
                        ),
                    )?,
                ),
                session_end_debug_utils_label_region: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSessionEndDebugUtilsLabelRegionEXT\0",
                        ),
                    )?,
                ),
                session_insert_debug_utils_label: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSessionInsertDebugUtilsLabelEXT\0"),
                )?),
            })
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidThreadSettingsKHR {
        pub set_android_application_thread: pfn::SetAndroidApplicationThreadKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidThreadSettingsKHR {
        pub const VERSION: u32 = sys::KHR_android_thread_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_THREAD_SETTINGS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_android_application_thread: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetAndroidApplicationThreadKHR\0"),
                )?),
            })
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidSurfaceSwapchainKHR {
        pub create_swapchain_android_surface: pfn::CreateSwapchainAndroidSurfaceKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidSurfaceSwapchainKHR {
        pub const VERSION: u32 = sys::KHR_android_surface_swapchain_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_SURFACE_SWAPCHAIN_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_swapchain_android_surface: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSwapchainAndroidSurfaceKHR\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerCubeKHR {}
    impl CompositionLayerCubeKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_cube_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_CUBE_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidCreateInstanceKHR {}
    #[cfg(target_os = "android")]
    impl AndroidCreateInstanceKHR {
        pub const VERSION: u32 = sys::KHR_android_create_instance_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthKHR {}
    impl CompositionLayerDepthKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_depth_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HeadlessKHR {}
    impl HeadlessKHR {
        pub const VERSION: u32 = sys::KHR_headless_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_HEADLESS_EXTENSION_NAME;
    }
    #[cfg(feature = "vulkan")]
    #[derive(Copy, Clone)]
    pub struct VulkanSwapchainFormatListKHR {}
    #[cfg(feature = "vulkan")]
    impl VulkanSwapchainFormatListKHR {
        pub const VERSION: u32 = sys::KHR_vulkan_swapchain_format_list_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerCylinderKHR {}
    impl CompositionLayerCylinderKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_cylinder_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerEquirectKHR {}
    impl CompositionLayerEquirectKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_equirect_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME;
    }
    #[cfg(feature = "opengl")]
    #[derive(Copy, Clone)]
    pub struct OpenglEnableKHR {
        pub get_open_gl_graphics_requirements: pfn::GetOpenGLGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengl")]
    impl OpenglEnableKHR {
        pub const VERSION: u32 = sys::KHR_opengl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_OPENGL_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_open_gl_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetOpenGLGraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[cfg(feature = "opengles")]
    #[derive(Copy, Clone)]
    pub struct OpenglEsEnableKHR {
        pub get_open_gles_graphics_requirements: pfn::GetOpenGLESGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengles")]
    impl OpenglEsEnableKHR {
        pub const VERSION: u32 = sys::KHR_opengl_es_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_OPENGL_ES_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_open_gles_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetOpenGLESGraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[cfg(feature = "vulkan")]
    #[derive(Copy, Clone)]
    pub struct VulkanEnableKHR {
        pub get_vulkan_instance_extensions: pfn::GetVulkanInstanceExtensionsKHR,
        pub get_vulkan_device_extensions: pfn::GetVulkanDeviceExtensionsKHR,
        pub get_vulkan_graphics_device: pfn::GetVulkanGraphicsDeviceKHR,
        pub get_vulkan_graphics_requirements: pfn::GetVulkanGraphicsRequirementsKHR,
    }
    #[cfg(feature = "vulkan")]
    impl VulkanEnableKHR {
        pub const VERSION: u32 = sys::KHR_vulkan_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_vulkan_instance_extensions: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanInstanceExtensionsKHR\0"),
                )?),
                get_vulkan_device_extensions: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanDeviceExtensionsKHR\0"),
                )?),
                get_vulkan_graphics_device: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanGraphicsDeviceKHR\0"),
                )?),
                get_vulkan_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanGraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[cfg(feature = "d3d")]
    #[derive(Copy, Clone)]
    pub struct D3d10EnableKHR {
        pub get_d3d10_graphics_requirements: pfn::GetD3D10GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d10EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D10_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D10_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_d3d10_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetD3D10GraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[cfg(feature = "d3d")]
    #[derive(Copy, Clone)]
    pub struct D3d11EnableKHR {
        pub get_d3d11_graphics_requirements: pfn::GetD3D11GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d11EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D11_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D11_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_d3d11_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetD3D11GraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[cfg(feature = "d3d")]
    #[derive(Copy, Clone)]
    pub struct D3d12EnableKHR {
        pub get_d3d12_graphics_requirements: pfn::GetD3D12GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d12EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D12_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D12_ENABLE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_d3d12_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetD3D12GraphicsRequirementsKHR\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct VisibilityMaskKHR {
        pub get_visibility_mask: pfn::GetVisibilityMaskKHR,
    }
    impl VisibilityMaskKHR {
        pub const VERSION: u32 = sys::KHR_visibility_mask_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VISIBILITY_MASK_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_visibility_mask: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVisibilityMaskKHR\0"),
                )?),
            })
        }
    }
    #[cfg(target_os = "windows")]
    #[derive(Copy, Clone)]
    pub struct Win32ConvertPerformanceCounterTimeKHR {
        pub convert_win32_performance_counter_to_time: pfn::ConvertWin32PerformanceCounterToTimeKHR,
        pub convert_time_to_win32_performance_counter: pfn::ConvertTimeToWin32PerformanceCounterKHR,
    }
    #[cfg(target_os = "windows")]
    impl Win32ConvertPerformanceCounterTimeKHR {
        pub const VERSION: u32 = sys::KHR_win32_convert_performance_counter_time_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                convert_win32_performance_counter_to_time: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrConvertWin32PerformanceCounterToTimeKHR\0",
                        ),
                    )?,
                ),
                convert_time_to_win32_performance_counter: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrConvertTimeToWin32PerformanceCounterKHR\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ConvertTimespecTimeKHR {
        pub convert_timespec_time_to_time: pfn::ConvertTimespecTimeToTimeKHR,
        pub convert_time_to_timespec_time: pfn::ConvertTimeToTimespecTimeKHR,
    }
    impl ConvertTimespecTimeKHR {
        pub const VERSION: u32 = sys::KHR_convert_timespec_time_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                convert_timespec_time_to_time: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrConvertTimespecTimeToTimeKHR\0"),
                )?),
                convert_time_to_timespec_time: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrConvertTimeToTimespecTimeKHR\0"),
                )?),
            })
        }
    }
}

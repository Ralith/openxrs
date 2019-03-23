#![doc = r" Automatically generated code; do not edit!"]
use crate::*;
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
pub enum Event<'a> {
    EventsLost(EventsLost<'a>),
    InstanceLossPending(InstanceLossPending<'a>),
    SessionStateChanged(SessionStateChanged<'a>),
    ReferenceSpaceChangePending(ReferenceSpaceChangePending<'a>),
    PerfSettingsEXT(PerfSettingsEXT<'a>),
    VisibilityMaskChangedKHR(VisibilityMaskChangedKHR<'a>),
    InteractionProfileChanged,
}
impl<'a> Event<'a> {
    #[doc = r" Decode an event"]
    #[doc = r""]
    #[doc = r" Returns `None` if the event type is not recognized, e.g. if it's from an unknown extension"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" `raw` must refer to an `EventDataBuffer` populated by a successful call to"]
    #[doc = r" `xrPollEvent`, which has not been moved since."]
    pub unsafe fn from_raw(raw: &'a sys::EventDataBuffer) -> Option<Self> {
        Some(match raw.ty {
            sys::StructureType::EVENT_DATA_EVENTS_LOST => {
                let typed = &*(raw as *const _ as *const sys::EventDataEventsLost);
                Event::EventsLost(EventsLost::new(typed))
            }
            sys::StructureType::EVENT_DATA_INSTANCE_LOSS_PENDING => {
                let typed = &*(raw as *const _ as *const sys::EventDataInstanceLossPending);
                Event::InstanceLossPending(InstanceLossPending::new(typed))
            }
            sys::StructureType::EVENT_DATA_SESSION_STATE_CHANGED => {
                let typed = &*(raw as *const _ as *const sys::EventDataSessionStateChanged);
                Event::SessionStateChanged(SessionStateChanged::new(typed))
            }
            sys::StructureType::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                let typed = &*(raw as *const _ as *const sys::EventDataReferenceSpaceChangePending);
                Event::ReferenceSpaceChangePending(ReferenceSpaceChangePending::new(typed))
            }
            sys::StructureType::EVENT_DATA_PERF_SETTINGS_EXT => {
                let typed = &*(raw as *const _ as *const sys::EventDataPerfSettingsEXT);
                Event::PerfSettingsEXT(PerfSettingsEXT::new(typed))
            }
            sys::StructureType::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
                let typed = &*(raw as *const _ as *const sys::EventDataVisibilityMaskChangedKHR);
                Event::VisibilityMaskChangedKHR(VisibilityMaskChangedKHR::new(typed))
            }
            sys::StructureType::EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
                Event::InteractionProfileChanged
            }
            _ => {
                return None;
            }
        })
    }
}
#[derive(Copy, Clone)]
pub struct EventsLost<'a>(&'a sys::EventDataEventsLost);
impl<'a> EventsLost<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataEventsLost) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn lost_event_count(&self) -> u32 {
        (self.0).lost_event_count
    }
}
#[derive(Copy, Clone)]
pub struct InstanceLossPending<'a>(&'a sys::EventDataInstanceLossPending);
impl<'a> InstanceLossPending<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataInstanceLossPending) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn loss_time(&self) -> Time {
        (self.0).loss_time
    }
}
#[derive(Copy, Clone)]
pub struct SessionStateChanged<'a>(&'a sys::EventDataSessionStateChanged);
impl<'a> SessionStateChanged<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataSessionStateChanged) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn session(&self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn state(&self) -> SessionState {
        (self.0).state
    }
    #[inline]
    pub fn time(&self) -> Time {
        (self.0).time
    }
}
#[derive(Copy, Clone)]
pub struct ReferenceSpaceChangePending<'a>(&'a sys::EventDataReferenceSpaceChangePending);
impl<'a> ReferenceSpaceChangePending<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataReferenceSpaceChangePending) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        (self.0).reference_space_type
    }
    #[inline]
    pub fn change_time(&self) -> Time {
        (self.0).change_time
    }
    #[inline]
    pub fn pose_valid(&self) -> bool {
        (self.0).pose_valid.into()
    }
    #[inline]
    pub fn pose_in_previous_space(&self) -> Posef {
        (self.0).pose_in_previous_space
    }
}
#[derive(Copy, Clone)]
pub struct PerfSettingsEXT<'a>(&'a sys::EventDataPerfSettingsEXT);
impl<'a> PerfSettingsEXT<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataPerfSettingsEXT) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn domain(&self) -> PerfSettingsDomainEXT {
        (self.0).domain
    }
    #[inline]
    pub fn sub_domain(&self) -> PerfSettingsSubDomainEXT {
        (self.0).sub_domain
    }
    #[inline]
    pub fn from_level(&self) -> PerfSettingsNotificationLevelEXT {
        (self.0).from_level
    }
    #[inline]
    pub fn to_level(&self) -> PerfSettingsNotificationLevelEXT {
        (self.0).to_level
    }
}
#[derive(Copy, Clone)]
pub struct VisibilityMaskChangedKHR<'a>(&'a sys::EventDataVisibilityMaskChangedKHR);
impl<'a> VisibilityMaskChangedKHR<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataVisibilityMaskChangedKHR) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn view_configuration_type(&self) -> ViewConfigurationType {
        (self.0).view_configuration_type
    }
    #[inline]
    pub fn view_index(&self) -> u32 {
        (self.0).view_index
    }
}
#[derive(Copy, Clone)]
pub struct InteractionProfileChanged<'a>(&'a sys::EventDataInteractionProfileChanged);
impl<'a> InteractionProfileChanged<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataInteractionProfileChanged) -> Self {
        Self(inner)
    }
}
pub mod raw {
    use crate::{Entry, Result};
    use std::{ffi::CStr, mem};
    use sys::pfn;
    #[derive(Copy, Clone)]
    pub struct Instance {
        pub get_instance_proc_addr: pfn::GetInstanceProcAddr,
        pub enumerate_api_layer_properties: pfn::EnumerateApiLayerProperties,
        pub enumerate_instance_extension_properties: pfn::EnumerateInstanceExtensionProperties,
        pub create_instance: pfn::CreateInstance,
        pub destroy_instance: pfn::DestroyInstance,
        pub result_to_string: pfn::ResultToString,
        pub structure_type_to_string: pfn::StructureTypeToString,
        pub get_instance_properties: pfn::GetInstanceProperties,
        pub get_system: pfn::GetSystem,
        pub get_system_properties: pfn::GetSystemProperties,
        pub create_session: pfn::CreateSession,
        pub destroy_session: pfn::DestroySession,
        pub destroy_space: pfn::DestroySpace,
        pub enumerate_swapchain_formats: pfn::EnumerateSwapchainFormats,
        pub create_swapchain: pfn::CreateSwapchain,
        pub destroy_swapchain: pfn::DestroySwapchain,
        pub enumerate_swapchain_images: pfn::EnumerateSwapchainImages,
        pub acquire_swapchain_image: pfn::AcquireSwapchainImage,
        pub wait_swapchain_image: pfn::WaitSwapchainImage,
        pub release_swapchain_image: pfn::ReleaseSwapchainImage,
        pub begin_session: pfn::BeginSession,
        pub end_session: pfn::EndSession,
        pub enumerate_reference_spaces: pfn::EnumerateReferenceSpaces,
        pub create_reference_space: pfn::CreateReferenceSpace,
        pub create_action_space: pfn::CreateActionSpace,
        pub locate_space: pfn::LocateSpace,
        pub enumerate_view_configurations: pfn::EnumerateViewConfigurations,
        pub enumerate_environment_blend_modes: pfn::EnumerateEnvironmentBlendModes,
        pub get_view_configuration_properties: pfn::GetViewConfigurationProperties,
        pub enumerate_view_configuration_views: pfn::EnumerateViewConfigurationViews,
        pub begin_frame: pfn::BeginFrame,
        pub locate_views: pfn::LocateViews,
        pub end_frame: pfn::EndFrame,
        pub wait_frame: pfn::WaitFrame,
        pub apply_haptic_feedback: pfn::ApplyHapticFeedback,
        pub stop_haptic_feedback: pfn::StopHapticFeedback,
        pub poll_event: pfn::PollEvent,
        pub string_to_path: pfn::StringToPath,
        pub path_to_string: pfn::PathToString,
        pub get_reference_space_bounds_rect: pfn::GetReferenceSpaceBoundsRect,
        pub get_action_state_boolean: pfn::GetActionStateBoolean,
        pub get_action_state_vector1f: pfn::GetActionStateVector1f,
        pub get_action_state_vector2f: pfn::GetActionStateVector2f,
        pub get_action_state_pose: pfn::GetActionStatePose,
        pub create_action_set: pfn::CreateActionSet,
        pub destroy_action_set: pfn::DestroyActionSet,
        pub create_action: pfn::CreateAction,
        pub destroy_action: pfn::DestroyAction,
        pub set_interaction_profile_suggested_bindings: pfn::SetInteractionProfileSuggestedBindings,
        pub get_current_interaction_profile: pfn::GetCurrentInteractionProfile,
        pub sync_action_data: pfn::SyncActionData,
        pub get_bound_sources_for_action: pfn::GetBoundSourcesForAction,
        pub get_input_source_localized_name: pfn::GetInputSourceLocalizedName,
    }
    impl Instance {
        #[doc = r" Load the core function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_instance_proc_addr: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProcAddr\0"),
                )?),
                enumerate_api_layer_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateApiLayerProperties\0"),
                )?),
                enumerate_instance_extension_properties: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumerateInstanceExtensionProperties\0",
                        ),
                    )?,
                ),
                create_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateInstance\0"),
                )?),
                destroy_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyInstance\0"),
                )?),
                result_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrResultToString\0"),
                )?),
                structure_type_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStructureTypeToString\0"),
                )?),
                get_instance_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProperties\0"),
                )?),
                get_system: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystem\0"),
                )?),
                get_system_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystemProperties\0"),
                )?),
                create_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSession\0"),
                )?),
                destroy_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySession\0"),
                )?),
                destroy_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpace\0"),
                )?),
                enumerate_swapchain_formats: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainFormats\0"),
                )?),
                create_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSwapchain\0"),
                )?),
                destroy_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySwapchain\0"),
                )?),
                enumerate_swapchain_images: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainImages\0"),
                )?),
                acquire_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrAcquireSwapchainImage\0"),
                )?),
                wait_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitSwapchainImage\0"),
                )?),
                release_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrReleaseSwapchainImage\0"),
                )?),
                begin_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginSession\0"),
                )?),
                end_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndSession\0"),
                )?),
                enumerate_reference_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateReferenceSpaces\0"),
                )?),
                create_reference_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateReferenceSpace\0"),
                )?),
                create_action_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSpace\0"),
                )?),
                locate_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateSpace\0"),
                )?),
                enumerate_view_configurations: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurations\0"),
                )?),
                enumerate_environment_blend_modes: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateEnvironmentBlendModes\0"),
                )?),
                get_view_configuration_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetViewConfigurationProperties\0"),
                )?),
                enumerate_view_configuration_views: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurationViews\0"),
                )?),
                begin_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginFrame\0"),
                )?),
                locate_views: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateViews\0"),
                )?),
                end_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndFrame\0"),
                )?),
                wait_frame: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitFrame\0"),
                )?),
                apply_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrApplyHapticFeedback\0"),
                )?),
                stop_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStopHapticFeedback\0"),
                )?),
                poll_event: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPollEvent\0"),
                )?),
                string_to_path: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStringToPath\0"),
                )?),
                path_to_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPathToString\0"),
                )?),
                get_reference_space_bounds_rect: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetReferenceSpaceBoundsRect\0"),
                )?),
                get_action_state_boolean: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateBoolean\0"),
                )?),
                get_action_state_vector1f: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector1f\0"),
                )?),
                get_action_state_vector2f: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector2f\0"),
                )?),
                get_action_state_pose: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStatePose\0"),
                )?),
                create_action_set: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSet\0"),
                )?),
                destroy_action_set: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyActionSet\0"),
                )?),
                create_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateAction\0"),
                )?),
                destroy_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyAction\0"),
                )?),
                set_interaction_profile_suggested_bindings: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSetInteractionProfileSuggestedBindings\0",
                        ),
                    )?,
                ),
                get_current_interaction_profile: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetCurrentInteractionProfile\0"),
                )?),
                sync_action_data: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSyncActionData\0"),
                )?),
                get_bound_sources_for_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetBoundSourcesForAction\0"),
                )?),
                get_input_source_localized_name: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInputSourceLocalizedName\0"),
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
#[allow(unused)]
pub(crate) mod builder {
    use crate::*;
    #[cfg(feature = "vulkan")]
    use ash::vk;
    use std::{marker::PhantomData, mem, ops::Deref};
    #[cfg(all(feature = "opengl", feature = "xlib"))]
    use sys::{Display, GLXContext, GLXDrawable, GLXFBConfig};
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ApiLayerProperties<'a> {
        inner: sys::ApiLayerProperties,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ApiLayerProperties<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ApiLayerProperties {
                    ty: sys::StructureType::API_LAYER_PROPERTIES,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ApiLayerProperties) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ApiLayerProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ApiLayerProperties {
            &self.inner
        }
        #[inline]
        pub fn layer_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.layer_name, value);
            self
        }
        #[inline]
        pub fn spec_version(mut self, value: u32) -> Self {
            self.inner.spec_version = value;
            self
        }
        #[inline]
        pub fn implementation_version(mut self, value: u32) -> Self {
            self.inner.implementation_version = value;
            self
        }
        #[inline]
        pub fn description(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.description, value);
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ExtensionProperties<'a> {
        inner: sys::ExtensionProperties,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ExtensionProperties<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ExtensionProperties {
                    ty: sys::StructureType::EXTENSION_PROPERTIES,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ExtensionProperties) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ExtensionProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ExtensionProperties {
            &self.inner
        }
        #[inline]
        pub fn extension_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.extension_name, value);
            self
        }
        #[inline]
        pub fn spec_version(mut self, value: u32) -> Self {
            self.inner.spec_version = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ApplicationInfo {
        inner: sys::ApplicationInfo,
    }
    impl ApplicationInfo {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ApplicationInfo {
                    ..unsafe { mem::zeroed() }
                },
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ApplicationInfo) -> Self {
            Self { inner }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ApplicationInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ApplicationInfo {
            &self.inner
        }
        #[inline]
        pub fn application_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.application_name, value);
            self
        }
        #[inline]
        pub fn application_version(mut self, value: u32) -> Self {
            self.inner.application_version = value;
            self
        }
        #[inline]
        pub fn engine_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.engine_name, value);
            self
        }
        #[inline]
        pub fn engine_version(mut self, value: u32) -> Self {
            self.inner.engine_version = value;
            self
        }
        #[inline]
        pub fn api_version(mut self, value: u32) -> Self {
            self.inner.api_version = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct InstanceProperties<'a> {
        inner: sys::InstanceProperties,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> InstanceProperties<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::InstanceProperties {
                    ty: sys::StructureType::INSTANCE_PROPERTIES,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::InstanceProperties) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::InstanceProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::InstanceProperties {
            &self.inner
        }
        #[inline]
        pub fn runtime_version(mut self, value: u32) -> Self {
            self.inner.runtime_version = value;
            self
        }
        #[inline]
        pub fn runtime_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.runtime_name, value);
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SystemGetInfo<'a> {
        inner: sys::SystemGetInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SystemGetInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SystemGetInfo {
                    ty: sys::StructureType::SYSTEM_GET_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SystemGetInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SystemGetInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SystemGetInfo {
            &self.inner
        }
        #[inline]
        pub fn form_factor(mut self, value: FormFactor) -> Self {
            self.inner.form_factor = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SystemProperties<'a> {
        inner: sys::SystemProperties,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SystemProperties<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SystemProperties {
                    ty: sys::StructureType::SYSTEM_PROPERTIES,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SystemProperties) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SystemProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SystemProperties {
            &self.inner
        }
        #[inline]
        pub fn system_id(mut self, value: SystemId) -> Self {
            self.inner.system_id = value;
            self
        }
        #[inline]
        pub fn vendor_id(mut self, value: u32) -> Self {
            self.inner.vendor_id = value;
            self
        }
        #[inline]
        pub fn system_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.system_name, value);
            self
        }
        #[inline]
        pub fn graphics_properties(mut self, value: SystemGraphicsProperties) -> Self {
            self.inner.graphics_properties = value;
            self
        }
        #[inline]
        pub fn tracking_properties(mut self, value: SystemTrackingProperties) -> Self {
            self.inner.tracking_properties = value.inner;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SystemTrackingProperties {
        inner: sys::SystemTrackingProperties,
    }
    impl SystemTrackingProperties {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SystemTrackingProperties {
                    ..unsafe { mem::zeroed() }
                },
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SystemTrackingProperties) -> Self {
            Self { inner }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SystemTrackingProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SystemTrackingProperties {
            &self.inner
        }
        #[inline]
        pub fn orientation_tracking(mut self, value: bool) -> Self {
            self.inner.orientation_tracking = value.into();
            self
        }
        #[inline]
        pub fn position_tracking(mut self, value: bool) -> Self {
            self.inner.position_tracking = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(all(target_os = "windows", feature = "opengl"))]
    pub struct GraphicsBindingOpenGLWin32KHR<'a> {
        inner: sys::GraphicsBindingOpenGLWin32KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(all(target_os = "windows", feature = "opengl"))]
    impl<'a> GraphicsBindingOpenGLWin32KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingOpenGLWin32KHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_OPENGL_WIN32_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingOpenGLWin32KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingOpenGLWin32KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingOpenGLWin32KHR {
            &self.inner
        }
        #[inline]
        pub fn h_dc(mut self, value: HDC) -> Self {
            self.inner.h_dc = value;
            self
        }
        #[inline]
        pub fn h_glrc(mut self, value: HGLRC) -> Self {
            self.inner.h_glrc = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(all(feature = "opengl", feature = "xlib"))]
    pub struct GraphicsBindingOpenGLXlibKHR<'a> {
        inner: sys::GraphicsBindingOpenGLXlibKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(all(feature = "opengl", feature = "xlib"))]
    impl<'a> GraphicsBindingOpenGLXlibKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingOpenGLXlibKHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_OPENGL_XLIB_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingOpenGLXlibKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingOpenGLXlibKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingOpenGLXlibKHR {
            &self.inner
        }
        #[inline]
        pub fn x_display(mut self, value: &'a Display) -> Self {
            self.inner.x_display = value as *const _ as _;
            self
        }
        #[inline]
        pub fn visualid(mut self, value: u32) -> Self {
            self.inner.visualid = value;
            self
        }
        #[inline]
        pub fn glx_fb_config(mut self, value: GLXFBConfig) -> Self {
            self.inner.glx_fb_config = value;
            self
        }
        #[inline]
        pub fn glx_drawable(mut self, value: GLXDrawable) -> Self {
            self.inner.glx_drawable = value;
            self
        }
        #[inline]
        pub fn glx_context(mut self, value: GLXContext) -> Self {
            self.inner.glx_context = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(all(feature = "opengl", feature = "xcb"))]
    pub struct GraphicsBindingOpenGLXcbKHR<'a> {
        inner: sys::GraphicsBindingOpenGLXcbKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(all(feature = "opengl", feature = "xcb"))]
    impl<'a> GraphicsBindingOpenGLXcbKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingOpenGLXcbKHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_OPENGL_XCB_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingOpenGLXcbKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingOpenGLXcbKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingOpenGLXcbKHR {
            &self.inner
        }
        #[inline]
        pub fn connection(mut self, value: &'a xcb_connection_t) -> Self {
            self.inner.connection = value as *const _ as _;
            self
        }
        #[inline]
        pub fn screen_number(mut self, value: u32) -> Self {
            self.inner.screen_number = value;
            self
        }
        #[inline]
        pub fn fbconfigid(mut self, value: xcb_glx_fbconfig_t) -> Self {
            self.inner.fbconfigid = value;
            self
        }
        #[inline]
        pub fn visualid(mut self, value: xcb_visualid_t) -> Self {
            self.inner.visualid = value;
            self
        }
        #[inline]
        pub fn glx_drawable(mut self, value: xcb_glx_drawable_t) -> Self {
            self.inner.glx_drawable = value;
            self
        }
        #[inline]
        pub fn glx_context(mut self, value: xcb_glx_context_t) -> Self {
            self.inner.glx_context = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(all(feature = "opengl", feature = "wayland"))]
    pub struct GraphicsBindingOpenGLWaylandKHR<'a> {
        inner: sys::GraphicsBindingOpenGLWaylandKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(all(feature = "opengl", feature = "wayland"))]
    impl<'a> GraphicsBindingOpenGLWaylandKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingOpenGLWaylandKHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_OPENGL_WAYLAND_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingOpenGLWaylandKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingOpenGLWaylandKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingOpenGLWaylandKHR {
            &self.inner
        }
        #[inline]
        pub fn display(mut self, value: &'a wl_display) -> Self {
            self.inner.display = value as *const _ as _;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsBindingD3D10KHR<'a> {
        inner: sys::GraphicsBindingD3D10KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsBindingD3D10KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingD3D10KHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_D3D10_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingD3D10KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingD3D10KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingD3D10KHR {
            &self.inner
        }
        #[inline]
        pub fn device(mut self, value: &'a ID3D10Device) -> Self {
            self.inner.device = value as *const _ as _;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsBindingD3D11KHR<'a> {
        inner: sys::GraphicsBindingD3D11KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsBindingD3D11KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingD3D11KHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_D3D11_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingD3D11KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingD3D11KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingD3D11KHR {
            &self.inner
        }
        #[inline]
        pub fn device(mut self, value: &'a ID3D11Device) -> Self {
            self.inner.device = value as *const _ as _;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsBindingD3D12KHR<'a> {
        inner: sys::GraphicsBindingD3D12KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsBindingD3D12KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingD3D12KHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_D3D12_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingD3D12KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingD3D12KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingD3D12KHR {
            &self.inner
        }
        #[inline]
        pub fn device(mut self, value: &'a ID3D12Device) -> Self {
            self.inner.device = value as *const _ as _;
            self
        }
        #[inline]
        pub fn queue(mut self, value: &'a ID3D12CommandQueue) -> Self {
            self.inner.queue = value as *const _ as _;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(all(target_os = "android", feature = "opengles"))]
    pub struct GraphicsBindingOpenGLESAndroidKHR<'a> {
        inner: sys::GraphicsBindingOpenGLESAndroidKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(all(target_os = "android", feature = "opengles"))]
    impl<'a> GraphicsBindingOpenGLESAndroidKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingOpenGLESAndroidKHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingOpenGLESAndroidKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingOpenGLESAndroidKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingOpenGLESAndroidKHR {
            &self.inner
        }
        #[inline]
        pub fn display(mut self, value: EGLDisplay) -> Self {
            self.inner.display = value;
            self
        }
        #[inline]
        pub fn config(mut self, value: EGLConfig) -> Self {
            self.inner.config = value;
            self
        }
        #[inline]
        pub fn context(mut self, value: EGLContext) -> Self {
            self.inner.context = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "vulkan")]
    pub struct GraphicsBindingVulkanKHR<'a> {
        inner: sys::GraphicsBindingVulkanKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "vulkan")]
    impl<'a> GraphicsBindingVulkanKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsBindingVulkanKHR {
                    ty: sys::StructureType::GRAPHICS_BINDING_VULKAN_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsBindingVulkanKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsBindingVulkanKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsBindingVulkanKHR {
            &self.inner
        }
        #[inline]
        pub fn instance(mut self, value: vk::Instance) -> Self {
            self.inner.instance = value;
            self
        }
        #[inline]
        pub fn physical_device(mut self, value: vk::PhysicalDevice) -> Self {
            self.inner.physical_device = value;
            self
        }
        #[inline]
        pub fn device(mut self, value: vk::Device) -> Self {
            self.inner.device = value;
            self
        }
        #[inline]
        pub fn queue_family_index(mut self, value: u32) -> Self {
            self.inner.queue_family_index = value;
            self
        }
        #[inline]
        pub fn queue_index(mut self, value: u32) -> Self {
            self.inner.queue_index = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SessionCreateInfo<'a> {
        inner: sys::SessionCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SessionCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SessionCreateInfo {
                    ty: sys::StructureType::SESSION_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SessionCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SessionCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SessionCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn create_flags(mut self, value: SessionCreateFlags) -> Self {
            self.inner.create_flags = value;
            self
        }
        #[inline]
        pub fn system_id(mut self, value: SystemId) -> Self {
            self.inner.system_id = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SessionBeginInfo<'a> {
        inner: sys::SessionBeginInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SessionBeginInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SessionBeginInfo {
                    ty: sys::StructureType::SESSION_BEGIN_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SessionBeginInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SessionBeginInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SessionBeginInfo {
            &self.inner
        }
        #[inline]
        pub fn primary_view_configuration_type(mut self, value: ViewConfigurationType) -> Self {
            self.inner.primary_view_configuration_type = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainCreateInfo<'a> {
        inner: sys::SwapchainCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainCreateInfo {
                    ty: sys::StructureType::SWAPCHAIN_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SwapchainCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn create_flags(mut self, value: SwapchainCreateFlags) -> Self {
            self.inner.create_flags = value;
            self
        }
        #[inline]
        pub fn usage_flags(mut self, value: SwapchainUsageFlags) -> Self {
            self.inner.usage_flags = value;
            self
        }
        #[inline]
        pub fn format(mut self, value: i64) -> Self {
            self.inner.format = value;
            self
        }
        #[inline]
        pub fn sample_count(mut self, value: u32) -> Self {
            self.inner.sample_count = value;
            self
        }
        #[inline]
        pub fn width(mut self, value: u32) -> Self {
            self.inner.width = value;
            self
        }
        #[inline]
        pub fn height(mut self, value: u32) -> Self {
            self.inner.height = value;
            self
        }
        #[inline]
        pub fn face_count(mut self, value: u32) -> Self {
            self.inner.face_count = value;
            self
        }
        #[inline]
        pub fn array_size(mut self, value: u32) -> Self {
            self.inner.array_size = value;
            self
        }
        #[inline]
        pub fn mip_count(mut self, value: u32) -> Self {
            self.inner.mip_count = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainImageAcquireInfo<'a> {
        inner: sys::SwapchainImageAcquireInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainImageAcquireInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainImageAcquireInfo {
                    ty: sys::StructureType::SWAPCHAIN_IMAGE_ACQUIRE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SwapchainImageAcquireInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainImageAcquireInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainImageAcquireInfo {
            &self.inner
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainImageWaitInfo<'a> {
        inner: sys::SwapchainImageWaitInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainImageWaitInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainImageWaitInfo {
                    ty: sys::StructureType::SWAPCHAIN_IMAGE_WAIT_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SwapchainImageWaitInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainImageWaitInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainImageWaitInfo {
            &self.inner
        }
        #[inline]
        pub fn timeout(mut self, value: Duration) -> Self {
            self.inner.timeout = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainImageReleaseInfo<'a> {
        inner: sys::SwapchainImageReleaseInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainImageReleaseInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainImageReleaseInfo {
                    ty: sys::StructureType::SWAPCHAIN_IMAGE_RELEASE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SwapchainImageReleaseInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainImageReleaseInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainImageReleaseInfo {
            &self.inner
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ReferenceSpaceCreateInfo<'a> {
        inner: sys::ReferenceSpaceCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ReferenceSpaceCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ReferenceSpaceCreateInfo {
                    ty: sys::StructureType::REFERENCE_SPACE_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ReferenceSpaceCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ReferenceSpaceCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ReferenceSpaceCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn reference_space_type(mut self, value: ReferenceSpaceType) -> Self {
            self.inner.reference_space_type = value;
            self
        }
        #[inline]
        pub fn pose_in_reference_space(mut self, value: Posef) -> Self {
            self.inner.pose_in_reference_space = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionSpaceCreateInfo<'a> {
        inner: sys::ActionSpaceCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionSpaceCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionSpaceCreateInfo {
                    ty: sys::StructureType::ACTION_SPACE_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionSpaceCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionSpaceCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionSpaceCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn subaction_path(mut self, value: Path) -> Self {
            self.inner.subaction_path = value;
            self
        }
        #[inline]
        pub fn pose_in_action_space(mut self, value: Posef) -> Self {
            self.inner.pose_in_action_space = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceRelation<'a> {
        inner: sys::SpaceRelation,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SpaceRelation<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceRelation {
                    ty: sys::StructureType::SPACE_RELATION,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SpaceRelation) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceRelation {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceRelation {
            &self.inner
        }
        #[inline]
        pub fn relation_flags(mut self, value: SpaceRelationFlags) -> Self {
            self.inner.relation_flags = value;
            self
        }
        #[inline]
        pub fn time(mut self, value: Time) -> Self {
            self.inner.time = value;
            self
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn linear_velocity(mut self, value: Vector3f) -> Self {
            self.inner.linear_velocity = value;
            self
        }
        #[inline]
        pub fn angular_velocity(mut self, value: Vector3f) -> Self {
            self.inner.angular_velocity = value;
            self
        }
        #[inline]
        pub fn linear_acceleration(mut self, value: Vector3f) -> Self {
            self.inner.linear_acceleration = value;
            self
        }
        #[inline]
        pub fn angular_acceleration(mut self, value: Vector3f) -> Self {
            self.inner.angular_acceleration = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct View<'a> {
        inner: sys::View,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> View<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::View {
                    ty: sys::StructureType::VIEW,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::View) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::View {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::View {
            &self.inner
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn fov(mut self, value: Fovf) -> Self {
            self.inner.fov = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ViewLocateInfo<'a> {
        inner: sys::ViewLocateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ViewLocateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ViewLocateInfo {
                    ty: sys::StructureType::VIEW_LOCATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ViewLocateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ViewLocateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ViewLocateInfo {
            &self.inner
        }
        #[inline]
        pub fn display_time(mut self, value: Time) -> Self {
            self.inner.display_time = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ViewState<'a> {
        inner: sys::ViewState,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ViewState<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ViewState {
                    ty: sys::StructureType::VIEW_STATE,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ViewState) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ViewState {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ViewState {
            &self.inner
        }
        #[inline]
        pub fn view_state_flags(mut self, value: ViewStateFlags) -> Self {
            self.inner.view_state_flags = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ViewConfigurationView<'a> {
        inner: sys::ViewConfigurationView,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ViewConfigurationView<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ViewConfigurationView {
                    ty: sys::StructureType::VIEW_CONFIGURATION_VIEW,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ViewConfigurationView) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ViewConfigurationView {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ViewConfigurationView {
            &self.inner
        }
        #[inline]
        pub fn recommended_image_rect_width(mut self, value: u32) -> Self {
            self.inner.recommended_image_rect_width = value;
            self
        }
        #[inline]
        pub fn max_image_rect_width(mut self, value: u32) -> Self {
            self.inner.max_image_rect_width = value;
            self
        }
        #[inline]
        pub fn recommended_image_rect_height(mut self, value: u32) -> Self {
            self.inner.recommended_image_rect_height = value;
            self
        }
        #[inline]
        pub fn max_image_rect_height(mut self, value: u32) -> Self {
            self.inner.max_image_rect_height = value;
            self
        }
        #[inline]
        pub fn recommended_swapchain_sample_count(mut self, value: u32) -> Self {
            self.inner.recommended_swapchain_sample_count = value;
            self
        }
        #[inline]
        pub fn max_swapchain_sample_count(mut self, value: u32) -> Self {
            self.inner.max_swapchain_sample_count = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainSubImage<'a, G: Graphics> {
        inner: sys::SwapchainSubImage,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> SwapchainSubImage<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainSubImage {
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::SwapchainSubImage) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainSubImage {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainSubImage {
            &self.inner
        }
        #[inline]
        pub fn swapchain(mut self, value: &'a Swapchain<G>) -> Self {
            self.inner.swapchain = value.as_raw();
            self
        }
        #[inline]
        pub fn image_rect(mut self, value: Rect2Di) -> Self {
            self.inner.image_rect = value;
            self
        }
        #[inline]
        pub fn image_array_index(mut self, value: u32) -> Self {
            self.inner.image_array_index = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerProjectionView<'a, G: Graphics> {
        inner: sys::CompositionLayerProjectionView,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerProjectionView<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerProjectionView {
                    ty: sys::StructureType::COMPOSITION_LAYER_PROJECTION_VIEW,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerProjectionView) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerProjectionView {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerProjectionView {
            &self.inner
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn fov(mut self, value: Fovf) -> Self {
            self.inner.fov = value;
            self
        }
        #[inline]
        pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
            self.inner.sub_image = value.inner;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerDepthInfoKHR<'a, G: Graphics> {
        inner: sys::CompositionLayerDepthInfoKHR,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerDepthInfoKHR<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerDepthInfoKHR {
                    ty: sys::StructureType::COMPOSITION_LAYER_DEPTH_INFO_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerDepthInfoKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerDepthInfoKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerDepthInfoKHR {
            &self.inner
        }
        #[inline]
        pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
            self.inner.sub_image = value.inner;
            self
        }
        #[inline]
        pub fn min_depth(mut self, value: f32) -> Self {
            self.inner.min_depth = value;
            self
        }
        #[inline]
        pub fn max_depth(mut self, value: f32) -> Self {
            self.inner.max_depth = value;
            self
        }
        #[inline]
        pub fn near_z(mut self, value: f32) -> Self {
            self.inner.near_z = value;
            self
        }
        #[inline]
        pub fn far_z(mut self, value: f32) -> Self {
            self.inner.far_z = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct FrameBeginInfo<'a> {
        inner: sys::FrameBeginInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> FrameBeginInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::FrameBeginInfo {
                    ty: sys::StructureType::FRAME_BEGIN_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::FrameBeginInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::FrameBeginInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::FrameBeginInfo {
            &self.inner
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct FrameWaitInfo<'a> {
        inner: sys::FrameWaitInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> FrameWaitInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::FrameWaitInfo {
                    ty: sys::StructureType::FRAME_WAIT_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::FrameWaitInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::FrameWaitInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::FrameWaitInfo {
            &self.inner
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct FrameState<'a> {
        inner: sys::FrameState,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> FrameState<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::FrameState {
                    ty: sys::StructureType::FRAME_STATE,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::FrameState) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::FrameState {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::FrameState {
            &self.inner
        }
        #[inline]
        pub fn predicted_display_time(mut self, value: Time) -> Self {
            self.inner.predicted_display_time = value;
            self
        }
        #[inline]
        pub fn predicted_display_period(mut self, value: Duration) -> Self {
            self.inner.predicted_display_period = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct EventDataBuffer<'a> {
        inner: sys::EventDataBuffer,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> EventDataBuffer<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::EventDataBuffer {
                    ty: sys::StructureType::EVENT_DATA_BUFFER,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::EventDataBuffer) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::EventDataBuffer {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::EventDataBuffer {
            &self.inner
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ViewConfigurationProperties<'a> {
        inner: sys::ViewConfigurationProperties,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ViewConfigurationProperties<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ViewConfigurationProperties {
                    ty: sys::StructureType::VIEW_CONFIGURATION_PROPERTIES,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ViewConfigurationProperties) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ViewConfigurationProperties {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ViewConfigurationProperties {
            &self.inner
        }
        #[inline]
        pub fn view_configuration_type(mut self, value: ViewConfigurationType) -> Self {
            self.inner.view_configuration_type = value;
            self
        }
        #[inline]
        pub fn fov_mutable(mut self, value: bool) -> Self {
            self.inner.fov_mutable = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionStateBoolean<'a> {
        inner: sys::ActionStateBoolean,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionStateBoolean<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionStateBoolean {
                    ty: sys::StructureType::ACTION_STATE_BOOLEAN,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionStateBoolean) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionStateBoolean {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionStateBoolean {
            &self.inner
        }
        #[inline]
        pub fn current_state(mut self, value: bool) -> Self {
            self.inner.current_state = value.into();
            self
        }
        #[inline]
        pub fn changed_since_last_sync(mut self, value: bool) -> Self {
            self.inner.changed_since_last_sync = value.into();
            self
        }
        #[inline]
        pub fn last_change_time(mut self, value: Time) -> Self {
            self.inner.last_change_time = value;
            self
        }
        #[inline]
        pub fn is_active(mut self, value: bool) -> Self {
            self.inner.is_active = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionStateVector1f<'a> {
        inner: sys::ActionStateVector1f,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionStateVector1f<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionStateVector1f {
                    ty: sys::StructureType::ACTION_STATE_VECTOR1F,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionStateVector1f) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionStateVector1f {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionStateVector1f {
            &self.inner
        }
        #[inline]
        pub fn current_state(mut self, value: f32) -> Self {
            self.inner.current_state = value;
            self
        }
        #[inline]
        pub fn changed_since_last_sync(mut self, value: bool) -> Self {
            self.inner.changed_since_last_sync = value.into();
            self
        }
        #[inline]
        pub fn last_change_time(mut self, value: Time) -> Self {
            self.inner.last_change_time = value;
            self
        }
        #[inline]
        pub fn is_active(mut self, value: bool) -> Self {
            self.inner.is_active = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionStateVector2f<'a> {
        inner: sys::ActionStateVector2f,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionStateVector2f<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionStateVector2f {
                    ty: sys::StructureType::ACTION_STATE_VECTOR2F,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionStateVector2f) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionStateVector2f {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionStateVector2f {
            &self.inner
        }
        #[inline]
        pub fn current_state(mut self, value: Vector2f) -> Self {
            self.inner.current_state = value;
            self
        }
        #[inline]
        pub fn changed_since_last_sync(mut self, value: bool) -> Self {
            self.inner.changed_since_last_sync = value.into();
            self
        }
        #[inline]
        pub fn last_change_time(mut self, value: Time) -> Self {
            self.inner.last_change_time = value;
            self
        }
        #[inline]
        pub fn is_active(mut self, value: bool) -> Self {
            self.inner.is_active = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionStatePose<'a> {
        inner: sys::ActionStatePose,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionStatePose<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionStatePose {
                    ty: sys::StructureType::ACTION_STATE_POSE,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionStatePose) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionStatePose {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionStatePose {
            &self.inner
        }
        #[inline]
        pub fn is_active(mut self, value: bool) -> Self {
            self.inner.is_active = value.into();
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionSetCreateInfo<'a> {
        inner: sys::ActionSetCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionSetCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionSetCreateInfo {
                    ty: sys::StructureType::ACTION_SET_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionSetCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionSetCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionSetCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn action_set_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.action_set_name, value);
            self
        }
        #[inline]
        pub fn localized_action_set_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.localized_action_set_name, value);
            self
        }
        #[inline]
        pub fn priority(mut self, value: u32) -> Self {
            self.inner.priority = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActiveActionSet<'a> {
        inner: sys::ActiveActionSet,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActiveActionSet<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActiveActionSet {
                    ty: sys::StructureType::ACTIVE_ACTION_SET,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActiveActionSet) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActiveActionSet {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActiveActionSet {
            &self.inner
        }
        #[inline]
        pub fn action_set(mut self, value: &'a ActionSet) -> Self {
            self.inner.action_set = value.as_raw();
            self
        }
        #[inline]
        pub fn subaction_path(mut self, value: Path) -> Self {
            self.inner.subaction_path = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct InteractionProfileInfo<'a> {
        inner: sys::InteractionProfileInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> InteractionProfileInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::InteractionProfileInfo {
                    ty: sys::StructureType::INTERACTION_PROFILE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::InteractionProfileInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::InteractionProfileInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::InteractionProfileInfo {
            &self.inner
        }
        #[inline]
        pub fn interaction_profile(mut self, value: Path) -> Self {
            self.inner.interaction_profile = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ActionCreateInfo<'a> {
        inner: sys::ActionCreateInfo,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> ActionCreateInfo<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ActionCreateInfo {
                    ty: sys::StructureType::ACTION_CREATE_INFO,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::ActionCreateInfo) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ActionCreateInfo {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ActionCreateInfo {
            &self.inner
        }
        #[inline]
        pub fn action_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.action_name, value);
            self
        }
        #[inline]
        pub fn action_type(mut self, value: ActionType) -> Self {
            self.inner.action_type = value;
            self
        }
        #[inline]
        pub fn subaction_paths(mut self, value: &'a [Path]) -> Self {
            self.inner.subaction_paths = value.as_ptr() as *const _ as _;
            self.inner.count_subaction_paths = value.len() as u32;
            self
        }
        #[inline]
        pub fn localized_action_name(mut self, value: &str) -> Self {
            place_cstr(&mut self.inner.localized_action_name, value);
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "vulkan")]
    pub struct VulkanSwapchainFormatListCreateInfoKHR<'a> {
        inner: sys::VulkanSwapchainFormatListCreateInfoKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "vulkan")]
    impl<'a> VulkanSwapchainFormatListCreateInfoKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::VulkanSwapchainFormatListCreateInfoKHR {
                    ty: sys::StructureType::VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::VulkanSwapchainFormatListCreateInfoKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::VulkanSwapchainFormatListCreateInfoKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::VulkanSwapchainFormatListCreateInfoKHR {
            &self.inner
        }
        #[inline]
        pub fn view_formats(mut self, value: &'a [vk::Format]) -> Self {
            self.inner.view_formats = value.as_ptr() as *const _ as _;
            self.inner.view_format_count = value.len() as u32;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct VisibilityMaskKHR<'a> {
        inner: sys::VisibilityMaskKHR,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> VisibilityMaskKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::VisibilityMaskKHR {
                    ty: sys::StructureType::VISIBILITY_MASK_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::VisibilityMaskKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::VisibilityMaskKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::VisibilityMaskKHR {
            &self.inner
        }
        #[inline]
        pub fn vertices(mut self, value: &'a [Vector2f]) -> Self {
            self.inner.vertices = value.as_ptr() as *const _ as _;
            self.inner.vertex_count = value.len() as u32;
            self
        }
        #[inline]
        pub fn indices(mut self, value: &'a [u32]) -> Self {
            self.inner.indices = value.as_ptr() as *const _ as _;
            self.inner.index_count = value.len() as u32;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "opengl")]
    pub struct GraphicsRequirementsOpenGLKHR<'a> {
        inner: sys::GraphicsRequirementsOpenGLKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "opengl")]
    impl<'a> GraphicsRequirementsOpenGLKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsOpenGLKHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_OPENGL_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsOpenGLKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsOpenGLKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsOpenGLKHR {
            &self.inner
        }
        #[inline]
        pub fn min_api_version_supported(mut self, value: u32) -> Self {
            self.inner.min_api_version_supported = value;
            self
        }
        #[inline]
        pub fn max_api_version_supported(mut self, value: u32) -> Self {
            self.inner.max_api_version_supported = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "opengles")]
    pub struct GraphicsRequirementsOpenGLESKHR<'a> {
        inner: sys::GraphicsRequirementsOpenGLESKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "opengles")]
    impl<'a> GraphicsRequirementsOpenGLESKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsOpenGLESKHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsOpenGLESKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsOpenGLESKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsOpenGLESKHR {
            &self.inner
        }
        #[inline]
        pub fn min_api_version_supported(mut self, value: u32) -> Self {
            self.inner.min_api_version_supported = value;
            self
        }
        #[inline]
        pub fn max_api_version_supported(mut self, value: u32) -> Self {
            self.inner.max_api_version_supported = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "vulkan")]
    pub struct GraphicsRequirementsVulkanKHR<'a> {
        inner: sys::GraphicsRequirementsVulkanKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "vulkan")]
    impl<'a> GraphicsRequirementsVulkanKHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsVulkanKHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_VULKAN_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsVulkanKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsVulkanKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsVulkanKHR {
            &self.inner
        }
        #[inline]
        pub fn min_api_version_supported(mut self, value: u32) -> Self {
            self.inner.min_api_version_supported = value;
            self
        }
        #[inline]
        pub fn max_api_version_supported(mut self, value: u32) -> Self {
            self.inner.max_api_version_supported = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsRequirementsD3D10KHR<'a> {
        inner: sys::GraphicsRequirementsD3D10KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsRequirementsD3D10KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsD3D10KHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_D3D10_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsD3D10KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsD3D10KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsD3D10KHR {
            &self.inner
        }
        #[inline]
        pub fn adapter_luid(mut self, value: LUID) -> Self {
            self.inner.adapter_luid = value;
            self
        }
        #[inline]
        pub fn min_feature_level(mut self, value: D3D10_FEATURE_LEVEL1) -> Self {
            self.inner.min_feature_level = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsRequirementsD3D11KHR<'a> {
        inner: sys::GraphicsRequirementsD3D11KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsRequirementsD3D11KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsD3D11KHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_D3D11_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsD3D11KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsD3D11KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsD3D11KHR {
            &self.inner
        }
        #[inline]
        pub fn adapter_luid(mut self, value: LUID) -> Self {
            self.inner.adapter_luid = value;
            self
        }
        #[inline]
        pub fn min_feature_level(mut self, value: D3D_FEATURE_LEVEL) -> Self {
            self.inner.min_feature_level = value;
            self
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    #[cfg(feature = "d3d")]
    pub struct GraphicsRequirementsD3D12KHR<'a> {
        inner: sys::GraphicsRequirementsD3D12KHR,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(feature = "d3d")]
    impl<'a> GraphicsRequirementsD3D12KHR<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::GraphicsRequirementsD3D12KHR {
                    ty: sys::StructureType::GRAPHICS_REQUIREMENTS_D3D12_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::GraphicsRequirementsD3D12KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::GraphicsRequirementsD3D12KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::GraphicsRequirementsD3D12KHR {
            &self.inner
        }
        #[inline]
        pub fn adapter_luid(mut self, value: LUID) -> Self {
            self.inner.adapter_luid = value;
            self
        }
        #[inline]
        pub fn min_feature_level(mut self, value: D3D_FEATURE_LEVEL) -> Self {
            self.inner.min_feature_level = value;
            self
        }
    }
    #[repr(transparent)]
    pub struct CompositionLayerBase<'a, G: Graphics> {
        _inner: sys::CompositionLayerBaseHeader,
        _marker: PhantomData<&'a G>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerProjection<'a, G: Graphics> {
        inner: sys::CompositionLayerProjection,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerProjection<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerProjection {
                    ty: sys::StructureType::COMPOSITION_LAYER_PROJECTION,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerProjection) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerProjection {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerProjection {
            &self.inner
        }
        #[inline]
        pub fn layer_flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.layer_flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn views(mut self, value: &'a [CompositionLayerProjectionView<'a, G>]) -> Self {
            self.inner.views = value.as_ptr() as *const _ as _;
            self.inner.view_count = value.len() as u32;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerProjection<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerQuad<'a, G: Graphics> {
        inner: sys::CompositionLayerQuad,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerQuad<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerQuad {
                    ty: sys::StructureType::COMPOSITION_LAYER_QUAD,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerQuad) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerQuad {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerQuad {
            &self.inner
        }
        #[inline]
        pub fn layer_flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.layer_flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn eye_visibility(mut self, value: EyeVisibility) -> Self {
            self.inner.eye_visibility = value;
            self
        }
        #[inline]
        pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
            self.inner.sub_image = value.inner;
            self
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn size(mut self, value: Vector2f) -> Self {
            self.inner.size = value;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerQuad<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerCylinderKHR<'a, G: Graphics> {
        inner: sys::CompositionLayerCylinderKHR,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerCylinderKHR<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerCylinderKHR {
                    ty: sys::StructureType::COMPOSITION_LAYER_CYLINDER_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerCylinderKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerCylinderKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerCylinderKHR {
            &self.inner
        }
        #[inline]
        pub fn layer_flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.layer_flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn eye_visibility(mut self, value: EyeVisibility) -> Self {
            self.inner.eye_visibility = value;
            self
        }
        #[inline]
        pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
            self.inner.sub_image = value.inner;
            self
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn radius(mut self, value: f32) -> Self {
            self.inner.radius = value;
            self
        }
        #[inline]
        pub fn central_angle(mut self, value: f32) -> Self {
            self.inner.central_angle = value;
            self
        }
        #[inline]
        pub fn aspect_ratio(mut self, value: f32) -> Self {
            self.inner.aspect_ratio = value;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerCylinderKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerCubeKHR<'a, G: Graphics> {
        inner: sys::CompositionLayerCubeKHR,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerCubeKHR<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerCubeKHR {
                    ty: sys::StructureType::COMPOSITION_LAYER_CUBE_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerCubeKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerCubeKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerCubeKHR {
            &self.inner
        }
        #[inline]
        pub fn layer_flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.layer_flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn eye_visibility(mut self, value: EyeVisibility) -> Self {
            self.inner.eye_visibility = value;
            self
        }
        #[inline]
        pub fn swapchain(mut self, value: &'a Swapchain<G>) -> Self {
            self.inner.swapchain = value.as_raw();
            self
        }
        #[inline]
        pub fn image_array_index(mut self, value: u32) -> Self {
            self.inner.image_array_index = value;
            self
        }
        #[inline]
        pub fn orientation(mut self, value: Quaternionf) -> Self {
            self.inner.orientation = value;
            self
        }
        #[inline]
        pub fn offset(mut self, value: Vector3f) -> Self {
            self.inner.offset = value;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerCubeKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerEquirectKHR<'a, G: Graphics> {
        inner: sys::CompositionLayerEquirectKHR,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerEquirectKHR<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerEquirectKHR {
                    ty: sys::StructureType::COMPOSITION_LAYER_EQUIRECT_KHR,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::CompositionLayerEquirectKHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerEquirectKHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerEquirectKHR {
            &self.inner
        }
        #[inline]
        pub fn layer_flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.layer_flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn eye_visibility(mut self, value: EyeVisibility) -> Self {
            self.inner.eye_visibility = value;
            self
        }
        #[inline]
        pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
            self.inner.sub_image = value.inner;
            self
        }
        #[inline]
        pub fn pose(mut self, value: Posef) -> Self {
            self.inner.pose = value;
            self
        }
        #[inline]
        pub fn offset(mut self, value: Vector3f) -> Self {
            self.inner.offset = value;
            self
        }
        #[inline]
        pub fn scale(mut self, value: Vector2f) -> Self {
            self.inner.scale = value;
            self
        }
        #[inline]
        pub fn bias(mut self, value: Vector2f) -> Self {
            self.inner.bias = value;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerEquirectKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[repr(transparent)]
    pub struct HapticBase<'a> {
        _inner: sys::HapticBaseHeader,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct HapticVibration<'a> {
        inner: sys::HapticVibration,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> HapticVibration<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::HapticVibration {
                    ty: sys::StructureType::HAPTIC_VIBRATION,
                    ..unsafe { mem::zeroed() }
                },
                _marker: PhantomData,
            }
        }
        #[doc = r" Initialize with the supplied raw values"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" The guarantees normally enforced by this builder (e.g. lifetimes) must be"]
        #[doc = r" preserved."]
        #[inline]
        pub unsafe fn from_raw(inner: sys::HapticVibration) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::HapticVibration {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::HapticVibration {
            &self.inner
        }
        #[inline]
        pub fn duration(mut self, value: Duration) -> Self {
            self.inner.duration = value;
            self
        }
        #[inline]
        pub fn frequency(mut self, value: f32) -> Self {
            self.inner.frequency = value;
            self
        }
        #[inline]
        pub fn amplitude(mut self, value: f32) -> Self {
            self.inner.amplitude = value;
            self
        }
    }
    impl<'a> Deref for HapticVibration<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
}

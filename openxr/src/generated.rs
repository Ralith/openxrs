use crate::{Entry, Result};
use std::{ffi::CStr, mem};
pub struct Instance<E: Entry> {
    entry: E,
    handle: sys::Instance,
    raw: raw::Instance,
}
impl<E: Entry> Instance<E> {
    pub unsafe fn from_raw(entry: E, handle: sys::Instance) -> Result<Self> {
        Ok(Self {
            raw: raw::Instance {
                acquire_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrAcquireSwapchainImage\0"),
                )?),
                apply_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrApplyHapticFeedback\0"),
                )?),
                begin_frame: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginFrame\0"),
                )?),
                begin_session: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginSession\0"),
                )?),
                create_action: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateAction\0"),
                )?),
                create_action_set: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSet\0"),
                )?),
                create_action_space: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateActionSpace\0"),
                )?),
                create_instance: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateInstance\0"),
                )?),
                create_reference_space: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateReferenceSpace\0"),
                )?),
                create_session: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSession\0"),
                )?),
                create_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSwapchain\0"),
                )?),
                destroy_action: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyAction\0"),
                )?),
                destroy_action_set: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyActionSet\0"),
                )?),
                destroy_instance: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyInstance\0"),
                )?),
                destroy_session: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySession\0"),
                )?),
                destroy_space: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpace\0"),
                )?),
                destroy_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySwapchain\0"),
                )?),
                end_frame: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndFrame\0"),
                )?),
                end_session: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEndSession\0"),
                )?),
                enumerate_api_layer_properties: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateApiLayerProperties\0"),
                )?),
                enumerate_environment_blend_modes: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateEnvironmentBlendModes\0"),
                )?),
                enumerate_instance_extension_properties: mem::transmute(
                    entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumerateInstanceExtensionProperties\0",
                        ),
                    )?,
                ),
                enumerate_reference_spaces: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateReferenceSpaces\0"),
                )?),
                enumerate_swapchain_formats: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainFormats\0"),
                )?),
                enumerate_swapchain_images: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateSwapchainImages\0"),
                )?),
                enumerate_view_configuration_views: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurationViews\0"),
                )?),
                enumerate_view_configurations: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViewConfigurations\0"),
                )?),
                get_action_state_boolean: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateBoolean\0"),
                )?),
                get_action_state_pose: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStatePose\0"),
                )?),
                get_action_state_vector1f: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector1f\0"),
                )?),
                get_action_state_vector2f: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateVector2f\0"),
                )?),
                get_bound_sources_for_action: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetBoundSourcesForAction\0"),
                )?),
                get_current_interaction_profile: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetCurrentInteractionProfile\0"),
                )?),
                get_input_source_localized_name: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInputSourceLocalizedName\0"),
                )?),
                get_instance_proc_addr: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProcAddr\0"),
                )?),
                get_instance_properties: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInstanceProperties\0"),
                )?),
                get_reference_space_bounds_rect: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetReferenceSpaceBoundsRect\0"),
                )?),
                get_system: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystem\0"),
                )?),
                get_system_properties: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSystemProperties\0"),
                )?),
                get_view_configuration_properties: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetViewConfigurationProperties\0"),
                )?),
                locate_space: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateSpace\0"),
                )?),
                locate_views: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateViews\0"),
                )?),
                path_to_string: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrPathToString\0"),
                )?),
                poll_event: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrPollEvent\0"),
                )?),
                release_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrReleaseSwapchainImage\0"),
                )?),
                result_to_string: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrResultToString\0"),
                )?),
                set_interaction_profile_suggested_bindings: mem::transmute(
                    entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSetInteractionProfileSuggestedBindings\0",
                        ),
                    )?,
                ),
                stop_haptic_feedback: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrStopHapticFeedback\0"),
                )?),
                string_to_path: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrStringToPath\0"),
                )?),
                structure_type_to_string: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrStructureTypeToString\0"),
                )?),
                sync_action_data: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrSyncActionData\0"),
                )?),
                wait_frame: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitFrame\0"),
                )?),
                wait_swapchain_image: mem::transmute(entry.get_instance_proc_addr(
                    handle,
                    CStr::from_bytes_with_nul_unchecked(b"xrWaitSwapchainImage\0"),
                )?),
            },
            handle,
            entry,
        })
    }
    pub fn as_raw(&self) -> sys::Instance {
        self.handle
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::Instance {
        &self.raw
    }
}
pub struct PerformanceSettingsEXT<E> {
    _entry_guard: E,
    raw: raw::PerformanceSettingsEXT,
}
impl<E: Entry> PerformanceSettingsEXT<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::PerformanceSettingsEXT {
                    perf_settings_set_performance_level: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrPerfSettingsSetPerformanceLevelEXT\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::PerformanceSettingsEXT {
        &self.raw
    }
}
pub struct ThermalQueryEXT<E> {
    _entry_guard: E,
    raw: raw::ThermalQueryEXT,
}
impl<E: Entry> ThermalQueryEXT<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::ThermalQueryEXT {
                    thermal_get_temperature_trend: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrThermalGetTemperatureTrendEXT\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::ThermalQueryEXT {
        &self.raw
    }
}
pub struct DebugUtilsEXT<E> {
    _entry_guard: E,
    raw: raw::DebugUtilsEXT,
}
impl<E: Entry> DebugUtilsEXT<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::DebugUtilsEXT {
                    set_debug_utils_object_name: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrSetDebugUtilsObjectNameEXT\0"),
                    )?),
                    create_debug_utils_messenger: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrCreateDebugUtilsMessengerEXT\0"),
                    )?),
                    destroy_debug_utils_messenger: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrDestroyDebugUtilsMessengerEXT\0"),
                    )?),
                    submit_debug_utils_message: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrSubmitDebugUtilsMessageEXT\0"),
                    )?),
                    session_begin_debug_utils_label_region: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrSessionBeginDebugUtilsLabelRegionEXT\0",
                            ),
                        )?,
                    ),
                    session_end_debug_utils_label_region: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrSessionEndDebugUtilsLabelRegionEXT\0",
                            ),
                        )?,
                    ),
                    session_insert_debug_utils_label: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrSessionInsertDebugUtilsLabelEXT\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::DebugUtilsEXT {
        &self.raw
    }
}
#[cfg(target_os = "android")]
pub struct AndroidThreadSettingsKHR<E> {
    _entry_guard: E,
    raw: raw::AndroidThreadSettingsKHR,
}
#[cfg(target_os = "android")]
impl<E: Entry> AndroidThreadSettingsKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::AndroidThreadSettingsKHR {
                    set_android_application_thread: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrSetAndroidApplicationThreadKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::AndroidThreadSettingsKHR {
        &self.raw
    }
}
#[cfg(target_os = "android")]
pub struct AndroidSurfaceSwapchainKHR<E> {
    _entry_guard: E,
    raw: raw::AndroidSurfaceSwapchainKHR,
}
#[cfg(target_os = "android")]
impl<E: Entry> AndroidSurfaceSwapchainKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::AndroidSurfaceSwapchainKHR {
                    create_swapchain_android_surface: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrCreateSwapchainAndroidSurfaceKHR\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::AndroidSurfaceSwapchainKHR {
        &self.raw
    }
}
#[cfg(feature = "opengl")]
pub struct OpenglEnableKHR<E> {
    _entry_guard: E,
    raw: raw::OpenglEnableKHR,
}
#[cfg(feature = "opengl")]
impl<E: Entry> OpenglEnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::OpenglEnableKHR {
                    get_open_gl_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrGetOpenGLGraphicsRequirementsKHR\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::OpenglEnableKHR {
        &self.raw
    }
}
#[cfg(feature = "opengles")]
pub struct OpenglEsEnableKHR<E> {
    _entry_guard: E,
    raw: raw::OpenglEsEnableKHR,
}
#[cfg(feature = "opengles")]
impl<E: Entry> OpenglEsEnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::OpenglEsEnableKHR {
                    get_open_gles_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrGetOpenGLESGraphicsRequirementsKHR\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::OpenglEsEnableKHR {
        &self.raw
    }
}
#[cfg(feature = "ash")]
pub struct VulkanEnableKHR<E> {
    _entry_guard: E,
    raw: raw::VulkanEnableKHR,
}
#[cfg(feature = "ash")]
impl<E: Entry> VulkanEnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::VulkanEnableKHR {
                    get_vulkan_instance_extensions: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanInstanceExtensionsKHR\0"),
                    )?),
                    get_vulkan_device_extensions: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanDeviceExtensionsKHR\0"),
                    )?),
                    get_vulkan_graphics_device: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanGraphicsDeviceKHR\0"),
                    )?),
                    get_vulkan_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrGetVulkanGraphicsRequirementsKHR\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::VulkanEnableKHR {
        &self.raw
    }
}
#[cfg(feature = "d3d")]
pub struct D3d10EnableKHR<E> {
    _entry_guard: E,
    raw: raw::D3d10EnableKHR,
}
#[cfg(feature = "d3d")]
impl<E: Entry> D3d10EnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::D3d10EnableKHR {
                    get_d3d10_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetD3D10GraphicsRequirementsKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::D3d10EnableKHR {
        &self.raw
    }
}
#[cfg(feature = "d3d")]
pub struct D3d11EnableKHR<E> {
    _entry_guard: E,
    raw: raw::D3d11EnableKHR,
}
#[cfg(feature = "d3d")]
impl<E: Entry> D3d11EnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::D3d11EnableKHR {
                    get_d3d11_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetD3D11GraphicsRequirementsKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::D3d11EnableKHR {
        &self.raw
    }
}
#[cfg(feature = "d3d")]
pub struct D3d12EnableKHR<E> {
    _entry_guard: E,
    raw: raw::D3d12EnableKHR,
}
#[cfg(feature = "d3d")]
impl<E: Entry> D3d12EnableKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::D3d12EnableKHR {
                    get_d3d12_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetD3D12GraphicsRequirementsKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::D3d12EnableKHR {
        &self.raw
    }
}
pub struct VisibilityMaskKHR<E> {
    _entry_guard: E,
    raw: raw::VisibilityMaskKHR,
}
impl<E: Entry> VisibilityMaskKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::VisibilityMaskKHR {
                    get_visibility_mask: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrGetVisibilityMaskKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::VisibilityMaskKHR {
        &self.raw
    }
}
#[cfg(target_os = "windows")]
pub struct Win32ConvertPerformanceCounterTimeKHR<E> {
    _entry_guard: E,
    raw: raw::Win32ConvertPerformanceCounterTimeKHR,
}
#[cfg(target_os = "windows")]
impl<E: Entry> Win32ConvertPerformanceCounterTimeKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::Win32ConvertPerformanceCounterTimeKHR {
                    convert_win32_performance_counter_to_time: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrConvertWin32PerformanceCounterToTimeKHR\0",
                            ),
                        )?,
                    ),
                    convert_time_to_win32_performance_counter: mem::transmute(
                        entry.get_instance_proc_addr(
                            handle,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrConvertTimeToWin32PerformanceCounterKHR\0",
                            ),
                        )?,
                    ),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::Win32ConvertPerformanceCounterTimeKHR {
        &self.raw
    }
}
#[cfg(feature = "libc")]
pub struct ConvertTimespecTimeKHR<E> {
    _entry_guard: E,
    raw: raw::ConvertTimespecTimeKHR,
}
#[cfg(feature = "libc")]
impl<E: Entry> ConvertTimespecTimeKHR<E> {
    pub fn load(instance: &Instance<E>) -> Result<Self>
    where
        E: Clone,
    {
        let entry = instance.entry.clone();
        let handle = instance.handle;
        unsafe {
            Ok(Self {
                raw: raw::ConvertTimespecTimeKHR {
                    convert_timespec_time_to_time: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrConvertTimespecTimeToTimeKHR\0"),
                    )?),
                    convert_time_to_timespec_time: mem::transmute(entry.get_instance_proc_addr(
                        handle,
                        CStr::from_bytes_with_nul_unchecked(b"xrConvertTimeToTimespecTimeKHR\0"),
                    )?),
                },
                _entry_guard: entry,
            })
        }
    }
    #[doc = r" Access the raw function pointers"]
    pub fn raw(&self) -> &raw::ConvertTimespecTimeKHR {
        &self.raw
    }
}
pub mod raw {
    use sys::pfn;
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
    pub struct PerformanceSettingsEXT {
        pub perf_settings_set_performance_level: pfn::PerfSettingsSetPerformanceLevelEXT,
    }
    impl PerformanceSettingsEXT {
        pub const VERSION: u32 = sys::EXT_performance_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME;
    }
    pub struct ThermalQueryEXT {
        pub thermal_get_temperature_trend: pfn::ThermalGetTemperatureTrendEXT,
    }
    impl ThermalQueryEXT {
        pub const VERSION: u32 = sys::EXT_thermal_query_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_THERMAL_QUERY_EXTENSION_NAME;
    }
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
    }
    #[cfg(target_os = "android")]
    pub struct AndroidThreadSettingsKHR {
        pub set_android_application_thread: pfn::SetAndroidApplicationThreadKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidThreadSettingsKHR {
        pub const VERSION: u32 = sys::KHR_android_thread_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_THREAD_SETTINGS_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    pub struct AndroidSurfaceSwapchainKHR {
        pub create_swapchain_android_surface: pfn::CreateSwapchainAndroidSurfaceKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidSurfaceSwapchainKHR {
        pub const VERSION: u32 = sys::KHR_android_surface_swapchain_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_SURFACE_SWAPCHAIN_EXTENSION_NAME;
    }
    pub struct CompositionLayerCubeKHR {}
    impl CompositionLayerCubeKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_cube_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_CUBE_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    pub struct AndroidCreateInstanceKHR {}
    #[cfg(target_os = "android")]
    impl AndroidCreateInstanceKHR {
        pub const VERSION: u32 = sys::KHR_android_create_instance_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME;
    }
    pub struct CompositionLayerDepthKHR {}
    impl CompositionLayerDepthKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_depth_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME;
    }
    pub struct HeadlessKHR {}
    impl HeadlessKHR {
        pub const VERSION: u32 = sys::KHR_headless_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_HEADLESS_EXTENSION_NAME;
    }
    #[cfg(feature = "ash")]
    pub struct VulkanSwapchainFormatListKHR {}
    #[cfg(feature = "ash")]
    impl VulkanSwapchainFormatListKHR {
        pub const VERSION: u32 = sys::KHR_vulkan_swapchain_format_list_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME;
    }
    pub struct CompositionLayerCylinderKHR {}
    impl CompositionLayerCylinderKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_cylinder_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME;
    }
    pub struct CompositionLayerEquirectKHR {}
    impl CompositionLayerEquirectKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_equirect_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME;
    }
    #[cfg(feature = "opengl")]
    pub struct OpenglEnableKHR {
        pub get_open_gl_graphics_requirements: pfn::GetOpenGLGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengl")]
    impl OpenglEnableKHR {
        pub const VERSION: u32 = sys::KHR_opengl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_OPENGL_ENABLE_EXTENSION_NAME;
    }
    #[cfg(feature = "opengles")]
    pub struct OpenglEsEnableKHR {
        pub get_open_gles_graphics_requirements: pfn::GetOpenGLESGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengles")]
    impl OpenglEsEnableKHR {
        pub const VERSION: u32 = sys::KHR_opengl_es_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_OPENGL_ES_ENABLE_EXTENSION_NAME;
    }
    #[cfg(feature = "ash")]
    pub struct VulkanEnableKHR {
        pub get_vulkan_instance_extensions: pfn::GetVulkanInstanceExtensionsKHR,
        pub get_vulkan_device_extensions: pfn::GetVulkanDeviceExtensionsKHR,
        pub get_vulkan_graphics_device: pfn::GetVulkanGraphicsDeviceKHR,
        pub get_vulkan_graphics_requirements: pfn::GetVulkanGraphicsRequirementsKHR,
    }
    #[cfg(feature = "ash")]
    impl VulkanEnableKHR {
        pub const VERSION: u32 = sys::KHR_vulkan_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_ENABLE_EXTENSION_NAME;
    }
    #[cfg(feature = "d3d")]
    pub struct D3d10EnableKHR {
        pub get_d3d10_graphics_requirements: pfn::GetD3D10GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d10EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D10_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D10_ENABLE_EXTENSION_NAME;
    }
    #[cfg(feature = "d3d")]
    pub struct D3d11EnableKHR {
        pub get_d3d11_graphics_requirements: pfn::GetD3D11GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d11EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D11_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D11_ENABLE_EXTENSION_NAME;
    }
    #[cfg(feature = "d3d")]
    pub struct D3d12EnableKHR {
        pub get_d3d12_graphics_requirements: pfn::GetD3D12GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d12EnableKHR {
        pub const VERSION: u32 = sys::KHR_D3D12_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_D3D12_ENABLE_EXTENSION_NAME;
    }
    pub struct VisibilityMaskKHR {
        pub get_visibility_mask: pfn::GetVisibilityMaskKHR,
    }
    impl VisibilityMaskKHR {
        pub const VERSION: u32 = sys::KHR_visibility_mask_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VISIBILITY_MASK_EXTENSION_NAME;
    }
    #[cfg(target_os = "windows")]
    pub struct Win32ConvertPerformanceCounterTimeKHR {
        pub convert_win32_performance_counter_to_time: pfn::ConvertWin32PerformanceCounterToTimeKHR,
        pub convert_time_to_win32_performance_counter: pfn::ConvertTimeToWin32PerformanceCounterKHR,
    }
    #[cfg(target_os = "windows")]
    impl Win32ConvertPerformanceCounterTimeKHR {
        pub const VERSION: u32 = sys::KHR_win32_convert_performance_counter_time_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME;
    }
    #[cfg(feature = "libc")]
    pub struct ConvertTimespecTimeKHR {
        pub convert_timespec_time_to_time: pfn::ConvertTimespecTimeToTimeKHR,
        pub convert_time_to_timespec_time: pfn::ConvertTimeToTimespecTimeKHR,
    }
    #[cfg(feature = "libc")]
    impl ConvertTimespecTimeKHR {
        pub const VERSION: u32 = sys::KHR_convert_timespec_time_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME;
    }
}

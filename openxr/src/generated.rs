#![doc = r" Automatically generated code; do not edit!"]
#![allow(clippy::wrong_self_convention, clippy::transmute_ptr_to_ptr)]
use crate::*;
use std::borrow::Cow;
use std::mem::MaybeUninit;
pub use sys::{
    ActionType, AndroidThreadTypeKHR, Color4f, CompositionLayerFlags,
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT, EnvironmentBlendMode,
    Extent2Df, Extent2Di, EyeVisibility, FormFactor, Fovf, HandEXT, HandJointEXT,
    HandJointLocationEXT, HandJointSetEXT, HandJointVelocityEXT, HandMeshVertexMSFT,
    HandPoseTypeMSFT, InputSourceLocalizedNameFlags, InstanceCreateFlags, ObjectType, Offset2Df,
    Offset2Di, OverlayMainSessionFlagsEXTX, OverlaySessionCreateFlagsEXTX, PerfSettingsDomainEXT,
    PerfSettingsLevelEXT, PerfSettingsNotificationLevelEXT, PerfSettingsSubDomainEXT, Posef,
    Quaternionf, Rect2Df, Rect2Di, ReferenceSpaceType, SessionCreateFlags, SessionState,
    SpaceLocationFlags, SpaceVelocityFlags, SpatialGraphNodeTypeMSFT, StructureType,
    SwapchainCreateFlags, SwapchainUsageFlags, SystemGraphicsProperties, Vector2f, Vector3f,
    Vector4f, ViewConfigurationType, ViewStateFlags, VisibilityMaskTypeKHR,
    VulkanDeviceCreateFlagsKHR, VulkanInstanceCreateFlagsKHR,
};
#[doc = r" A subset of known extensions"]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[non_exhaustive]
pub struct ExtensionSet {
    pub epic_view_configuration_fov: bool,
    pub ext_performance_settings: bool,
    pub ext_thermal_query: bool,
    pub ext_debug_utils: bool,
    pub ext_eye_gaze_interaction: bool,
    pub ext_view_configuration_depth_range: bool,
    pub ext_conformance_automation: bool,
    pub ext_hand_tracking: bool,
    #[cfg(windows)]
    pub ext_win32_appcontainer_compatible: bool,
    pub ext_samsung_odyssey_controller: bool,
    pub ext_hp_mixed_reality_controller: bool,
    pub extx_overlay: bool,
    pub huawei_controller_interaction: bool,
    #[cfg(target_os = "android")]
    pub khr_android_thread_settings: bool,
    #[cfg(target_os = "android")]
    pub khr_android_surface_swapchain: bool,
    pub khr_composition_layer_cube: bool,
    #[cfg(target_os = "android")]
    pub khr_android_create_instance: bool,
    pub khr_composition_layer_depth: bool,
    pub khr_vulkan_swapchain_format_list: bool,
    pub khr_composition_layer_cylinder: bool,
    pub khr_composition_layer_equirect: bool,
    pub khr_opengl_enable: bool,
    pub khr_opengl_es_enable: bool,
    pub khr_vulkan_enable: bool,
    #[cfg(windows)]
    pub khr_d3d11_enable: bool,
    #[cfg(windows)]
    pub khr_d3d12_enable: bool,
    pub khr_visibility_mask: bool,
    pub khr_composition_layer_color_scale_bias: bool,
    #[cfg(windows)]
    pub khr_win32_convert_performance_counter_time: bool,
    pub khr_convert_timespec_time: bool,
    pub khr_loader_init: bool,
    #[cfg(target_os = "android")]
    pub khr_loader_init_android: bool,
    pub khr_vulkan_enable2: bool,
    pub khr_composition_layer_equirect2: bool,
    pub mnd_headless: bool,
    pub mnd_swapchain_usage_input_attachment_bit: bool,
    pub mndx_egl_enable: bool,
    pub msft_unbounded_reference_space: bool,
    pub msft_spatial_anchor: bool,
    pub msft_spatial_graph_bridge: bool,
    pub msft_hand_interaction: bool,
    pub msft_hand_tracking_mesh: bool,
    pub msft_secondary_view_configuration: bool,
    pub msft_first_person_observer: bool,
    pub msft_controller_model: bool,
    #[cfg(windows)]
    pub msft_holographic_window_attachment: bool,
    #[cfg(target_os = "android")]
    pub oculus_android_session_state_enable: bool,
    pub valve_analog_threshold: bool,
    pub varjo_quad_views: bool,
    #[doc = r" Extensions unknown to the high-level bindings"]
    pub other: Vec<String>,
}
impl ExtensionSet {
    pub(crate) fn from_properties(properties: &[sys::ExtensionProperties]) -> Self {
        let mut out = Self::default();
        for ext in properties {
            match crate::fixed_str_bytes(&ext.extension_name) {
                raw::ViewConfigurationFovEPIC::NAME => {
                    out.epic_view_configuration_fov = true;
                }
                raw::PerformanceSettingsEXT::NAME => {
                    out.ext_performance_settings = true;
                }
                raw::ThermalQueryEXT::NAME => {
                    out.ext_thermal_query = true;
                }
                raw::DebugUtilsEXT::NAME => {
                    out.ext_debug_utils = true;
                }
                raw::EyeGazeInteractionEXT::NAME => {
                    out.ext_eye_gaze_interaction = true;
                }
                raw::ViewConfigurationDepthRangeEXT::NAME => {
                    out.ext_view_configuration_depth_range = true;
                }
                raw::ConformanceAutomationEXT::NAME => {
                    out.ext_conformance_automation = true;
                }
                raw::HandTrackingEXT::NAME => {
                    out.ext_hand_tracking = true;
                }
                #[cfg(windows)]
                raw::Win32AppcontainerCompatibleEXT::NAME => {
                    out.ext_win32_appcontainer_compatible = true;
                }
                raw::SamsungOdysseyControllerEXT::NAME => {
                    out.ext_samsung_odyssey_controller = true;
                }
                raw::HpMixedRealityControllerEXT::NAME => {
                    out.ext_hp_mixed_reality_controller = true;
                }
                raw::OverlayEXTX::NAME => {
                    out.extx_overlay = true;
                }
                raw::ControllerInteractionHUAWEI::NAME => {
                    out.huawei_controller_interaction = true;
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
                raw::VulkanSwapchainFormatListKHR::NAME => {
                    out.khr_vulkan_swapchain_format_list = true;
                }
                raw::CompositionLayerCylinderKHR::NAME => {
                    out.khr_composition_layer_cylinder = true;
                }
                raw::CompositionLayerEquirectKHR::NAME => {
                    out.khr_composition_layer_equirect = true;
                }
                raw::OpenglEnableKHR::NAME => {
                    out.khr_opengl_enable = true;
                }
                raw::OpenglEsEnableKHR::NAME => {
                    out.khr_opengl_es_enable = true;
                }
                raw::VulkanEnableKHR::NAME => {
                    out.khr_vulkan_enable = true;
                }
                #[cfg(windows)]
                raw::D3d11EnableKHR::NAME => {
                    out.khr_d3d11_enable = true;
                }
                #[cfg(windows)]
                raw::D3d12EnableKHR::NAME => {
                    out.khr_d3d12_enable = true;
                }
                raw::VisibilityMaskKHR::NAME => {
                    out.khr_visibility_mask = true;
                }
                raw::CompositionLayerColorScaleBiasKHR::NAME => {
                    out.khr_composition_layer_color_scale_bias = true;
                }
                #[cfg(windows)]
                raw::Win32ConvertPerformanceCounterTimeKHR::NAME => {
                    out.khr_win32_convert_performance_counter_time = true;
                }
                raw::ConvertTimespecTimeKHR::NAME => {
                    out.khr_convert_timespec_time = true;
                }
                raw::LoaderInitKHR::NAME => {
                    out.khr_loader_init = true;
                }
                #[cfg(target_os = "android")]
                raw::LoaderInitAndroidKHR::NAME => {
                    out.khr_loader_init_android = true;
                }
                raw::VulkanEnable2KHR::NAME => {
                    out.khr_vulkan_enable2 = true;
                }
                raw::CompositionLayerEquirect2KHR::NAME => {
                    out.khr_composition_layer_equirect2 = true;
                }
                raw::HeadlessMND::NAME => {
                    out.mnd_headless = true;
                }
                raw::SwapchainUsageInputAttachmentBitMND::NAME => {
                    out.mnd_swapchain_usage_input_attachment_bit = true;
                }
                raw::EglEnableMNDX::NAME => {
                    out.mndx_egl_enable = true;
                }
                raw::UnboundedReferenceSpaceMSFT::NAME => {
                    out.msft_unbounded_reference_space = true;
                }
                raw::SpatialAnchorMSFT::NAME => {
                    out.msft_spatial_anchor = true;
                }
                raw::SpatialGraphBridgeMSFT::NAME => {
                    out.msft_spatial_graph_bridge = true;
                }
                raw::HandInteractionMSFT::NAME => {
                    out.msft_hand_interaction = true;
                }
                raw::HandTrackingMeshMSFT::NAME => {
                    out.msft_hand_tracking_mesh = true;
                }
                raw::SecondaryViewConfigurationMSFT::NAME => {
                    out.msft_secondary_view_configuration = true;
                }
                raw::FirstPersonObserverMSFT::NAME => {
                    out.msft_first_person_observer = true;
                }
                raw::ControllerModelMSFT::NAME => {
                    out.msft_controller_model = true;
                }
                #[cfg(windows)]
                raw::HolographicWindowAttachmentMSFT::NAME => {
                    out.msft_holographic_window_attachment = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSessionStateEnableOCULUS::NAME => {
                    out.oculus_android_session_state_enable = true;
                }
                raw::AnalogThresholdVALVE::NAME => {
                    out.valve_analog_threshold = true;
                }
                raw::QuadViewsVARJO::NAME => {
                    out.varjo_quad_views = true;
                }
                bytes => {
                    if let Ok(name) = std::str::from_utf8(bytes) {
                        out.other.push(name.into());
                    }
                }
            }
        }
        out
    }
    pub(crate) fn names(&self) -> Vec<Cow<'static, [u8]>> {
        let mut out = Vec::new();
        {
            if self.epic_view_configuration_fov {
                out.push(raw::ViewConfigurationFovEPIC::NAME.into());
            }
        }
        {
            if self.ext_performance_settings {
                out.push(raw::PerformanceSettingsEXT::NAME.into());
            }
        }
        {
            if self.ext_thermal_query {
                out.push(raw::ThermalQueryEXT::NAME.into());
            }
        }
        {
            if self.ext_debug_utils {
                out.push(raw::DebugUtilsEXT::NAME.into());
            }
        }
        {
            if self.ext_eye_gaze_interaction {
                out.push(raw::EyeGazeInteractionEXT::NAME.into());
            }
        }
        {
            if self.ext_view_configuration_depth_range {
                out.push(raw::ViewConfigurationDepthRangeEXT::NAME.into());
            }
        }
        {
            if self.ext_conformance_automation {
                out.push(raw::ConformanceAutomationEXT::NAME.into());
            }
        }
        {
            if self.ext_hand_tracking {
                out.push(raw::HandTrackingEXT::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.ext_win32_appcontainer_compatible {
                out.push(raw::Win32AppcontainerCompatibleEXT::NAME.into());
            }
        }
        {
            if self.ext_samsung_odyssey_controller {
                out.push(raw::SamsungOdysseyControllerEXT::NAME.into());
            }
        }
        {
            if self.ext_hp_mixed_reality_controller {
                out.push(raw::HpMixedRealityControllerEXT::NAME.into());
            }
        }
        {
            if self.extx_overlay {
                out.push(raw::OverlayEXTX::NAME.into());
            }
        }
        {
            if self.huawei_controller_interaction {
                out.push(raw::ControllerInteractionHUAWEI::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_thread_settings {
                out.push(raw::AndroidThreadSettingsKHR::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_surface_swapchain {
                out.push(raw::AndroidSurfaceSwapchainKHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_cube {
                out.push(raw::CompositionLayerCubeKHR::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_create_instance {
                out.push(raw::AndroidCreateInstanceKHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_depth {
                out.push(raw::CompositionLayerDepthKHR::NAME.into());
            }
        }
        {
            if self.khr_vulkan_swapchain_format_list {
                out.push(raw::VulkanSwapchainFormatListKHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_cylinder {
                out.push(raw::CompositionLayerCylinderKHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_equirect {
                out.push(raw::CompositionLayerEquirectKHR::NAME.into());
            }
        }
        {
            if self.khr_opengl_enable {
                out.push(raw::OpenglEnableKHR::NAME.into());
            }
        }
        {
            if self.khr_opengl_es_enable {
                out.push(raw::OpenglEsEnableKHR::NAME.into());
            }
        }
        {
            if self.khr_vulkan_enable {
                out.push(raw::VulkanEnableKHR::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.khr_d3d11_enable {
                out.push(raw::D3d11EnableKHR::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.khr_d3d12_enable {
                out.push(raw::D3d12EnableKHR::NAME.into());
            }
        }
        {
            if self.khr_visibility_mask {
                out.push(raw::VisibilityMaskKHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_color_scale_bias {
                out.push(raw::CompositionLayerColorScaleBiasKHR::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.khr_win32_convert_performance_counter_time {
                out.push(raw::Win32ConvertPerformanceCounterTimeKHR::NAME.into());
            }
        }
        {
            if self.khr_convert_timespec_time {
                out.push(raw::ConvertTimespecTimeKHR::NAME.into());
            }
        }
        {
            if self.khr_loader_init {
                out.push(raw::LoaderInitKHR::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_loader_init_android {
                out.push(raw::LoaderInitAndroidKHR::NAME.into());
            }
        }
        {
            if self.khr_vulkan_enable2 {
                out.push(raw::VulkanEnable2KHR::NAME.into());
            }
        }
        {
            if self.khr_composition_layer_equirect2 {
                out.push(raw::CompositionLayerEquirect2KHR::NAME.into());
            }
        }
        {
            if self.mnd_headless {
                out.push(raw::HeadlessMND::NAME.into());
            }
        }
        {
            if self.mnd_swapchain_usage_input_attachment_bit {
                out.push(raw::SwapchainUsageInputAttachmentBitMND::NAME.into());
            }
        }
        {
            if self.mndx_egl_enable {
                out.push(raw::EglEnableMNDX::NAME.into());
            }
        }
        {
            if self.msft_unbounded_reference_space {
                out.push(raw::UnboundedReferenceSpaceMSFT::NAME.into());
            }
        }
        {
            if self.msft_spatial_anchor {
                out.push(raw::SpatialAnchorMSFT::NAME.into());
            }
        }
        {
            if self.msft_spatial_graph_bridge {
                out.push(raw::SpatialGraphBridgeMSFT::NAME.into());
            }
        }
        {
            if self.msft_hand_interaction {
                out.push(raw::HandInteractionMSFT::NAME.into());
            }
        }
        {
            if self.msft_hand_tracking_mesh {
                out.push(raw::HandTrackingMeshMSFT::NAME.into());
            }
        }
        {
            if self.msft_secondary_view_configuration {
                out.push(raw::SecondaryViewConfigurationMSFT::NAME.into());
            }
        }
        {
            if self.msft_first_person_observer {
                out.push(raw::FirstPersonObserverMSFT::NAME.into());
            }
        }
        {
            if self.msft_controller_model {
                out.push(raw::ControllerModelMSFT::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.msft_holographic_window_attachment {
                out.push(raw::HolographicWindowAttachmentMSFT::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.oculus_android_session_state_enable {
                out.push(raw::AndroidSessionStateEnableOCULUS::NAME.into());
            }
        }
        {
            if self.valve_analog_threshold {
                out.push(raw::AnalogThresholdVALVE::NAME.into());
            }
        }
        {
            if self.varjo_quad_views {
                out.push(raw::QuadViewsVARJO::NAME.into());
            }
        }
        for name in &self.other {
            let mut bytes = Vec::with_capacity(name.len() + 1);
            bytes.extend_from_slice(name.as_bytes());
            bytes.push(0);
            out.push(bytes.into());
        }
        out
    }
}
#[doc = r" Extensions used internally by the bindings"]
#[derive(Default, Copy, Clone)]
pub struct InstanceExtensions {
    pub epic_view_configuration_fov: Option<raw::ViewConfigurationFovEPIC>,
    pub ext_performance_settings: Option<raw::PerformanceSettingsEXT>,
    pub ext_thermal_query: Option<raw::ThermalQueryEXT>,
    pub ext_debug_utils: Option<raw::DebugUtilsEXT>,
    pub ext_eye_gaze_interaction: Option<raw::EyeGazeInteractionEXT>,
    pub ext_view_configuration_depth_range: Option<raw::ViewConfigurationDepthRangeEXT>,
    pub ext_conformance_automation: Option<raw::ConformanceAutomationEXT>,
    pub ext_hand_tracking: Option<raw::HandTrackingEXT>,
    #[cfg(windows)]
    pub ext_win32_appcontainer_compatible: Option<raw::Win32AppcontainerCompatibleEXT>,
    pub ext_samsung_odyssey_controller: Option<raw::SamsungOdysseyControllerEXT>,
    pub ext_hp_mixed_reality_controller: Option<raw::HpMixedRealityControllerEXT>,
    pub extx_overlay: Option<raw::OverlayEXTX>,
    pub huawei_controller_interaction: Option<raw::ControllerInteractionHUAWEI>,
    #[cfg(target_os = "android")]
    pub khr_android_thread_settings: Option<raw::AndroidThreadSettingsKHR>,
    #[cfg(target_os = "android")]
    pub khr_android_surface_swapchain: Option<raw::AndroidSurfaceSwapchainKHR>,
    pub khr_composition_layer_cube: Option<raw::CompositionLayerCubeKHR>,
    #[cfg(target_os = "android")]
    pub khr_android_create_instance: Option<raw::AndroidCreateInstanceKHR>,
    pub khr_composition_layer_depth: Option<raw::CompositionLayerDepthKHR>,
    pub khr_vulkan_swapchain_format_list: Option<raw::VulkanSwapchainFormatListKHR>,
    pub khr_composition_layer_cylinder: Option<raw::CompositionLayerCylinderKHR>,
    pub khr_composition_layer_equirect: Option<raw::CompositionLayerEquirectKHR>,
    pub khr_opengl_enable: Option<raw::OpenglEnableKHR>,
    pub khr_opengl_es_enable: Option<raw::OpenglEsEnableKHR>,
    pub khr_vulkan_enable: Option<raw::VulkanEnableKHR>,
    #[cfg(windows)]
    pub khr_d3d11_enable: Option<raw::D3d11EnableKHR>,
    #[cfg(windows)]
    pub khr_d3d12_enable: Option<raw::D3d12EnableKHR>,
    pub khr_visibility_mask: Option<raw::VisibilityMaskKHR>,
    pub khr_composition_layer_color_scale_bias: Option<raw::CompositionLayerColorScaleBiasKHR>,
    #[cfg(windows)]
    pub khr_win32_convert_performance_counter_time:
        Option<raw::Win32ConvertPerformanceCounterTimeKHR>,
    pub khr_convert_timespec_time: Option<raw::ConvertTimespecTimeKHR>,
    pub khr_loader_init: Option<raw::LoaderInitKHR>,
    #[cfg(target_os = "android")]
    pub khr_loader_init_android: Option<raw::LoaderInitAndroidKHR>,
    pub khr_vulkan_enable2: Option<raw::VulkanEnable2KHR>,
    pub khr_composition_layer_equirect2: Option<raw::CompositionLayerEquirect2KHR>,
    pub mnd_headless: Option<raw::HeadlessMND>,
    pub mnd_swapchain_usage_input_attachment_bit: Option<raw::SwapchainUsageInputAttachmentBitMND>,
    pub mndx_egl_enable: Option<raw::EglEnableMNDX>,
    pub msft_unbounded_reference_space: Option<raw::UnboundedReferenceSpaceMSFT>,
    pub msft_spatial_anchor: Option<raw::SpatialAnchorMSFT>,
    pub msft_spatial_graph_bridge: Option<raw::SpatialGraphBridgeMSFT>,
    pub msft_hand_interaction: Option<raw::HandInteractionMSFT>,
    pub msft_hand_tracking_mesh: Option<raw::HandTrackingMeshMSFT>,
    pub msft_secondary_view_configuration: Option<raw::SecondaryViewConfigurationMSFT>,
    pub msft_first_person_observer: Option<raw::FirstPersonObserverMSFT>,
    pub msft_controller_model: Option<raw::ControllerModelMSFT>,
    #[cfg(windows)]
    pub msft_holographic_window_attachment: Option<raw::HolographicWindowAttachmentMSFT>,
    #[cfg(target_os = "android")]
    pub oculus_android_session_state_enable: Option<raw::AndroidSessionStateEnableOCULUS>,
    pub valve_analog_threshold: Option<raw::AnalogThresholdVALVE>,
    pub varjo_quad_views: Option<raw::QuadViewsVARJO>,
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
            epic_view_configuration_fov: if required.epic_view_configuration_fov {
                Some(raw::ViewConfigurationFovEPIC {})
            } else {
                None
            },
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
            ext_eye_gaze_interaction: if required.ext_eye_gaze_interaction {
                Some(raw::EyeGazeInteractionEXT {})
            } else {
                None
            },
            ext_view_configuration_depth_range: if required.ext_view_configuration_depth_range {
                Some(raw::ViewConfigurationDepthRangeEXT {})
            } else {
                None
            },
            ext_conformance_automation: if required.ext_conformance_automation {
                Some(raw::ConformanceAutomationEXT::load(entry, instance)?)
            } else {
                None
            },
            ext_hand_tracking: if required.ext_hand_tracking {
                Some(raw::HandTrackingEXT::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(windows)]
            ext_win32_appcontainer_compatible: if required.ext_win32_appcontainer_compatible {
                Some(raw::Win32AppcontainerCompatibleEXT {})
            } else {
                None
            },
            ext_samsung_odyssey_controller: if required.ext_samsung_odyssey_controller {
                Some(raw::SamsungOdysseyControllerEXT {})
            } else {
                None
            },
            ext_hp_mixed_reality_controller: if required.ext_hp_mixed_reality_controller {
                Some(raw::HpMixedRealityControllerEXT {})
            } else {
                None
            },
            extx_overlay: if required.extx_overlay {
                Some(raw::OverlayEXTX {})
            } else {
                None
            },
            huawei_controller_interaction: if required.huawei_controller_interaction {
                Some(raw::ControllerInteractionHUAWEI {})
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
            khr_opengl_enable: if required.khr_opengl_enable {
                Some(raw::OpenglEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            khr_opengl_es_enable: if required.khr_opengl_es_enable {
                Some(raw::OpenglEsEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            khr_vulkan_enable: if required.khr_vulkan_enable {
                Some(raw::VulkanEnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(windows)]
            khr_d3d11_enable: if required.khr_d3d11_enable {
                Some(raw::D3d11EnableKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(windows)]
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
            khr_composition_layer_color_scale_bias: if required
                .khr_composition_layer_color_scale_bias
            {
                Some(raw::CompositionLayerColorScaleBiasKHR {})
            } else {
                None
            },
            #[cfg(windows)]
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
            khr_loader_init: if required.khr_loader_init {
                Some(raw::LoaderInitKHR::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(target_os = "android")]
            khr_loader_init_android: if required.khr_loader_init_android {
                Some(raw::LoaderInitAndroidKHR {})
            } else {
                None
            },
            khr_vulkan_enable2: if required.khr_vulkan_enable2 {
                Some(raw::VulkanEnable2KHR::load(entry, instance)?)
            } else {
                None
            },
            khr_composition_layer_equirect2: if required.khr_composition_layer_equirect2 {
                Some(raw::CompositionLayerEquirect2KHR {})
            } else {
                None
            },
            mnd_headless: if required.mnd_headless {
                Some(raw::HeadlessMND {})
            } else {
                None
            },
            mnd_swapchain_usage_input_attachment_bit: if required
                .mnd_swapchain_usage_input_attachment_bit
            {
                Some(raw::SwapchainUsageInputAttachmentBitMND {})
            } else {
                None
            },
            mndx_egl_enable: if required.mndx_egl_enable {
                Some(raw::EglEnableMNDX {})
            } else {
                None
            },
            msft_unbounded_reference_space: if required.msft_unbounded_reference_space {
                Some(raw::UnboundedReferenceSpaceMSFT {})
            } else {
                None
            },
            msft_spatial_anchor: if required.msft_spatial_anchor {
                Some(raw::SpatialAnchorMSFT::load(entry, instance)?)
            } else {
                None
            },
            msft_spatial_graph_bridge: if required.msft_spatial_graph_bridge {
                Some(raw::SpatialGraphBridgeMSFT::load(entry, instance)?)
            } else {
                None
            },
            msft_hand_interaction: if required.msft_hand_interaction {
                Some(raw::HandInteractionMSFT {})
            } else {
                None
            },
            msft_hand_tracking_mesh: if required.msft_hand_tracking_mesh {
                Some(raw::HandTrackingMeshMSFT::load(entry, instance)?)
            } else {
                None
            },
            msft_secondary_view_configuration: if required.msft_secondary_view_configuration {
                Some(raw::SecondaryViewConfigurationMSFT {})
            } else {
                None
            },
            msft_first_person_observer: if required.msft_first_person_observer {
                Some(raw::FirstPersonObserverMSFT {})
            } else {
                None
            },
            msft_controller_model: if required.msft_controller_model {
                Some(raw::ControllerModelMSFT::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(windows)]
            msft_holographic_window_attachment: if required.msft_holographic_window_attachment {
                Some(raw::HolographicWindowAttachmentMSFT {})
            } else {
                None
            },
            #[cfg(target_os = "android")]
            oculus_android_session_state_enable: if required.oculus_android_session_state_enable {
                Some(raw::AndroidSessionStateEnableOCULUS {})
            } else {
                None
            },
            valve_analog_threshold: if required.valve_analog_threshold {
                Some(raw::AnalogThresholdVALVE {})
            } else {
                None
            },
            varjo_quad_views: if required.varjo_quad_views {
                Some(raw::QuadViewsVARJO {})
            } else {
                None
            },
        })
    }
}
#[derive(Copy, Clone)]
#[non_exhaustive]
pub enum Event<'a> {
    EventsLost(EventsLost<'a>),
    InstanceLossPending(InstanceLossPending<'a>),
    SessionStateChanged(SessionStateChanged<'a>),
    ReferenceSpaceChangePending(ReferenceSpaceChangePending<'a>),
    PerfSettingsEXT(PerfSettingsEXT<'a>),
    VisibilityMaskChangedKHR(VisibilityMaskChangedKHR<'a>),
    InteractionProfileChanged(InteractionProfileChanged<'a>),
    MainSessionVisibilityChangedEXTX(MainSessionVisibilityChangedEXTX<'a>),
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
    pub unsafe fn from_raw(raw: &'a MaybeUninit<sys::EventDataBuffer>) -> Option<Self> {
        let raw = raw.as_ptr();
        Some(match (raw as *const sys::BaseInStructure).read().ty {
            sys::StructureType::EVENT_DATA_EVENTS_LOST => {
                let typed = &*(raw as *const sys::EventDataEventsLost);
                Event::EventsLost(EventsLost::new(typed))
            }
            sys::StructureType::EVENT_DATA_INSTANCE_LOSS_PENDING => {
                let typed = &*(raw as *const sys::EventDataInstanceLossPending);
                Event::InstanceLossPending(InstanceLossPending::new(typed))
            }
            sys::StructureType::EVENT_DATA_SESSION_STATE_CHANGED => {
                let typed = &*(raw as *const sys::EventDataSessionStateChanged);
                Event::SessionStateChanged(SessionStateChanged::new(typed))
            }
            sys::StructureType::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                let typed = &*(raw as *const sys::EventDataReferenceSpaceChangePending);
                Event::ReferenceSpaceChangePending(ReferenceSpaceChangePending::new(typed))
            }
            sys::StructureType::EVENT_DATA_PERF_SETTINGS_EXT => {
                let typed = &*(raw as *const sys::EventDataPerfSettingsEXT);
                Event::PerfSettingsEXT(PerfSettingsEXT::new(typed))
            }
            sys::StructureType::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
                let typed = &*(raw as *const sys::EventDataVisibilityMaskChangedKHR);
                Event::VisibilityMaskChangedKHR(VisibilityMaskChangedKHR::new(typed))
            }
            sys::StructureType::EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
                let typed = &*(raw as *const sys::EventDataInteractionProfileChanged);
                Event::InteractionProfileChanged(InteractionProfileChanged::new(typed))
            }
            sys::StructureType::EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX => {
                let typed = &*(raw as *const sys::EventDataMainSessionVisibilityChangedEXTX);
                Event::MainSessionVisibilityChangedEXTX(MainSessionVisibilityChangedEXTX::new(
                    typed,
                ))
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
    pub fn lost_event_count(self) -> u32 {
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
    pub fn loss_time(self) -> Time {
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
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn state(self) -> SessionState {
        (self.0).state
    }
    #[inline]
    pub fn time(self) -> Time {
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
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn reference_space_type(self) -> ReferenceSpaceType {
        (self.0).reference_space_type
    }
    #[inline]
    pub fn change_time(self) -> Time {
        (self.0).change_time
    }
    #[inline]
    pub fn pose_valid(self) -> bool {
        (self.0).pose_valid.into()
    }
    #[inline]
    pub fn pose_in_previous_space(self) -> Posef {
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
    pub fn domain(self) -> PerfSettingsDomainEXT {
        (self.0).domain
    }
    #[inline]
    pub fn sub_domain(self) -> PerfSettingsSubDomainEXT {
        (self.0).sub_domain
    }
    #[inline]
    pub fn from_level(self) -> PerfSettingsNotificationLevelEXT {
        (self.0).from_level
    }
    #[inline]
    pub fn to_level(self) -> PerfSettingsNotificationLevelEXT {
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
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn view_configuration_type(self) -> ViewConfigurationType {
        (self.0).view_configuration_type
    }
    #[inline]
    pub fn view_index(self) -> u32 {
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
    #[inline]
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
}
#[derive(Copy, Clone)]
pub struct MainSessionVisibilityChangedEXTX<'a>(&'a sys::EventDataMainSessionVisibilityChangedEXTX);
impl<'a> MainSessionVisibilityChangedEXTX<'a> {
    #[inline]
    pub fn new(inner: &'a sys::EventDataMainSessionVisibilityChangedEXTX) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn visible(self) -> bool {
        (self.0).visible.into()
    }
    #[inline]
    pub fn flags(self) -> OverlayMainSessionFlagsEXTX {
        (self.0).flags
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
        pub request_exit_session: pfn::RequestExitSession,
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
        pub get_action_state_float: pfn::GetActionStateFloat,
        pub get_action_state_vector2f: pfn::GetActionStateVector2f,
        pub get_action_state_pose: pfn::GetActionStatePose,
        pub create_action_set: pfn::CreateActionSet,
        pub destroy_action_set: pfn::DestroyActionSet,
        pub create_action: pfn::CreateAction,
        pub destroy_action: pfn::DestroyAction,
        pub suggest_interaction_profile_bindings: pfn::SuggestInteractionProfileBindings,
        pub attach_session_action_sets: pfn::AttachSessionActionSets,
        pub get_current_interaction_profile: pfn::GetCurrentInteractionProfile,
        pub sync_actions: pfn::SyncActions,
        pub enumerate_bound_sources_for_action: pfn::EnumerateBoundSourcesForAction,
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
                request_exit_session: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrRequestExitSession\0"),
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
                get_action_state_float: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetActionStateFloat\0"),
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
                suggest_interaction_profile_bindings: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSuggestInteractionProfileBindings\0",
                        ),
                    )?,
                ),
                attach_session_action_sets: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrAttachSessionActionSets\0"),
                )?),
                get_current_interaction_profile: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetCurrentInteractionProfile\0"),
                )?),
                sync_actions: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSyncActions\0"),
                )?),
                enumerate_bound_sources_for_action: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateBoundSourcesForAction\0"),
                )?),
                get_input_source_localized_name: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetInputSourceLocalizedName\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViewConfigurationFovEPIC {}
    impl ViewConfigurationFovEPIC {
        pub const VERSION: u32 = sys::EPIC_view_configuration_fov_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME;
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
    #[derive(Copy, Clone)]
    pub struct EyeGazeInteractionEXT {}
    impl EyeGazeInteractionEXT {
        pub const VERSION: u32 = sys::EXT_eye_gaze_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_EYE_GAZE_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ViewConfigurationDepthRangeEXT {}
    impl ViewConfigurationDepthRangeEXT {
        pub const VERSION: u32 = sys::EXT_view_configuration_depth_range_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ConformanceAutomationEXT {
        pub set_input_device_active: pfn::SetInputDeviceActiveEXT,
        pub set_input_device_state_bool: pfn::SetInputDeviceStateBoolEXT,
        pub set_input_device_state_float: pfn::SetInputDeviceStateFloatEXT,
        pub set_input_device_state_vector2f: pfn::SetInputDeviceStateVector2fEXT,
        pub set_input_device_location: pfn::SetInputDeviceLocationEXT,
    }
    impl ConformanceAutomationEXT {
        pub const VERSION: u32 = sys::EXT_conformance_automation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_CONFORMANCE_AUTOMATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_input_device_active: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetInputDeviceActiveEXT\0"),
                )?),
                set_input_device_state_bool: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetInputDeviceStateBoolEXT\0"),
                )?),
                set_input_device_state_float: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetInputDeviceStateFloatEXT\0"),
                )?),
                set_input_device_state_vector2f: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetInputDeviceStateVector2fEXT\0"),
                )?),
                set_input_device_location: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetInputDeviceLocationEXT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingEXT {
        pub create_hand_tracker: pfn::CreateHandTrackerEXT,
        pub destroy_hand_tracker: pfn::DestroyHandTrackerEXT,
        pub locate_hand_joints: pfn::LocateHandJointsEXT,
    }
    impl HandTrackingEXT {
        pub const VERSION: u32 = sys::EXT_hand_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_TRACKING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_hand_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateHandTrackerEXT\0"),
                )?),
                destroy_hand_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyHandTrackerEXT\0"),
                )?),
                locate_hand_joints: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateHandJointsEXT\0"),
                )?),
            })
        }
    }
    #[cfg(windows)]
    #[derive(Copy, Clone)]
    pub struct Win32AppcontainerCompatibleEXT {}
    #[cfg(windows)]
    impl Win32AppcontainerCompatibleEXT {
        pub const VERSION: u32 = sys::EXT_win32_appcontainer_compatible_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_WIN32_APPCONTAINER_COMPATIBLE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SamsungOdysseyControllerEXT {}
    impl SamsungOdysseyControllerEXT {
        pub const VERSION: u32 = sys::EXT_samsung_odyssey_controller_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HpMixedRealityControllerEXT {}
    impl HpMixedRealityControllerEXT {
        pub const VERSION: u32 = sys::EXT_hp_mixed_reality_controller_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct OverlayEXTX {}
    impl OverlayEXTX {
        pub const VERSION: u32 = sys::EXTX_overlay_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXTX_OVERLAY_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionHUAWEI {}
    impl ControllerInteractionHUAWEI {
        pub const VERSION: u32 = sys::HUAWEI_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME;
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
    pub struct VulkanSwapchainFormatListKHR {}
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
    #[derive(Copy, Clone)]
    pub struct OpenglEnableKHR {
        pub get_open_gl_graphics_requirements: pfn::GetOpenGLGraphicsRequirementsKHR,
    }
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
    #[derive(Copy, Clone)]
    pub struct OpenglEsEnableKHR {
        pub get_open_gles_graphics_requirements: pfn::GetOpenGLESGraphicsRequirementsKHR,
    }
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
    #[derive(Copy, Clone)]
    pub struct VulkanEnableKHR {
        pub get_vulkan_instance_extensions: pfn::GetVulkanInstanceExtensionsKHR,
        pub get_vulkan_device_extensions: pfn::GetVulkanDeviceExtensionsKHR,
        pub get_vulkan_graphics_device: pfn::GetVulkanGraphicsDeviceKHR,
        pub get_vulkan_graphics_requirements: pfn::GetVulkanGraphicsRequirementsKHR,
    }
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
    #[cfg(windows)]
    #[derive(Copy, Clone)]
    pub struct D3d11EnableKHR {
        pub get_d3d11_graphics_requirements: pfn::GetD3D11GraphicsRequirementsKHR,
    }
    #[cfg(windows)]
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
    #[cfg(windows)]
    #[derive(Copy, Clone)]
    pub struct D3d12EnableKHR {
        pub get_d3d12_graphics_requirements: pfn::GetD3D12GraphicsRequirementsKHR,
    }
    #[cfg(windows)]
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
    #[derive(Copy, Clone)]
    pub struct CompositionLayerColorScaleBiasKHR {}
    impl CompositionLayerColorScaleBiasKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_color_scale_bias_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_COLOR_SCALE_BIAS_EXTENSION_NAME;
    }
    #[cfg(windows)]
    #[derive(Copy, Clone)]
    pub struct Win32ConvertPerformanceCounterTimeKHR {
        pub convert_win32_performance_counter_to_time: pfn::ConvertWin32PerformanceCounterToTimeKHR,
        pub convert_time_to_win32_performance_counter: pfn::ConvertTimeToWin32PerformanceCounterKHR,
    }
    #[cfg(windows)]
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
    #[derive(Copy, Clone)]
    pub struct LoaderInitKHR {
        pub initialize_loader: pfn::InitializeLoaderKHR,
    }
    impl LoaderInitKHR {
        pub const VERSION: u32 = sys::KHR_loader_init_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_LOADER_INIT_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                initialize_loader: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrInitializeLoaderKHR\0"),
                )?),
            })
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct LoaderInitAndroidKHR {}
    #[cfg(target_os = "android")]
    impl LoaderInitAndroidKHR {
        pub const VERSION: u32 = sys::KHR_loader_init_android_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_LOADER_INIT_ANDROID_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct VulkanEnable2KHR {
        pub create_vulkan_instance: pfn::CreateVulkanInstanceKHR,
        pub create_vulkan_device: pfn::CreateVulkanDeviceKHR,
        pub get_vulkan_graphics_device2: pfn::GetVulkanGraphicsDevice2KHR,
        pub get_vulkan_graphics_requirements2: pfn::GetVulkanGraphicsRequirements2KHR,
    }
    impl VulkanEnable2KHR {
        pub const VERSION: u32 = sys::KHR_vulkan_enable2_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_ENABLE2_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_vulkan_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateVulkanInstanceKHR\0"),
                )?),
                create_vulkan_device: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateVulkanDeviceKHR\0"),
                )?),
                get_vulkan_graphics_device2: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanGraphicsDevice2KHR\0"),
                )?),
                get_vulkan_graphics_requirements2: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVulkanGraphicsRequirements2KHR\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerEquirect2KHR {}
    impl CompositionLayerEquirect2KHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_equirect2_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_EQUIRECT2_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HeadlessMND {}
    impl HeadlessMND {
        pub const VERSION: u32 = sys::MND_headless_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MND_HEADLESS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUsageInputAttachmentBitMND {}
    impl SwapchainUsageInputAttachmentBitMND {
        pub const VERSION: u32 = sys::MND_swapchain_usage_input_attachment_bit_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct EglEnableMNDX {}
    impl EglEnableMNDX {
        pub const VERSION: u32 = sys::MNDX_egl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MNDX_EGL_ENABLE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct UnboundedReferenceSpaceMSFT {}
    impl UnboundedReferenceSpaceMSFT {
        pub const VERSION: u32 = sys::MSFT_unbounded_reference_space_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorMSFT {
        pub create_spatial_anchor: pfn::CreateSpatialAnchorMSFT,
        pub create_spatial_anchor_space: pfn::CreateSpatialAnchorSpaceMSFT,
        pub destroy_spatial_anchor: pfn::DestroySpatialAnchorMSFT,
    }
    impl SpatialAnchorMSFT {
        pub const VERSION: u32 = sys::MSFT_spatial_anchor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_SPATIAL_ANCHOR_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpatialAnchorMSFT\0"),
                )?),
                create_spatial_anchor_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpatialAnchorSpaceMSFT\0"),
                )?),
                destroy_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpatialAnchorMSFT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialGraphBridgeMSFT {
        pub create_spatial_graph_node_space: pfn::CreateSpatialGraphNodeSpaceMSFT,
    }
    impl SpatialGraphBridgeMSFT {
        pub const VERSION: u32 = sys::MSFT_spatial_graph_bridge_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_SPATIAL_GRAPH_BRIDGE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_graph_node_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpatialGraphNodeSpaceMSFT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionMSFT {}
    impl HandInteractionMSFT {
        pub const VERSION: u32 = sys::MSFT_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_HAND_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingMeshMSFT {
        pub create_hand_mesh_space: pfn::CreateHandMeshSpaceMSFT,
        pub update_hand_mesh: pfn::UpdateHandMeshMSFT,
    }
    impl HandTrackingMeshMSFT {
        pub const VERSION: u32 = sys::MSFT_hand_tracking_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_HAND_TRACKING_MESH_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_hand_mesh_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateHandMeshSpaceMSFT\0"),
                )?),
                update_hand_mesh: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrUpdateHandMeshMSFT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SecondaryViewConfigurationMSFT {}
    impl SecondaryViewConfigurationMSFT {
        pub const VERSION: u32 = sys::MSFT_secondary_view_configuration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_SECONDARY_VIEW_CONFIGURATION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct FirstPersonObserverMSFT {}
    impl FirstPersonObserverMSFT {
        pub const VERSION: u32 = sys::MSFT_first_person_observer_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_FIRST_PERSON_OBSERVER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ControllerModelMSFT {
        pub get_controller_model_key: pfn::GetControllerModelKeyMSFT,
        pub load_controller_model: pfn::LoadControllerModelMSFT,
        pub get_controller_model_properties: pfn::GetControllerModelPropertiesMSFT,
        pub get_controller_model_state: pfn::GetControllerModelStateMSFT,
    }
    impl ControllerModelMSFT {
        pub const VERSION: u32 = sys::MSFT_controller_model_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_CONTROLLER_MODEL_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_controller_model_key: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetControllerModelKeyMSFT\0"),
                )?),
                load_controller_model: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLoadControllerModelMSFT\0"),
                )?),
                get_controller_model_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetControllerModelPropertiesMSFT\0"),
                )?),
                get_controller_model_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetControllerModelStateMSFT\0"),
                )?),
            })
        }
    }
    #[cfg(windows)]
    #[derive(Copy, Clone)]
    pub struct HolographicWindowAttachmentMSFT {}
    #[cfg(windows)]
    impl HolographicWindowAttachmentMSFT {
        pub const VERSION: u32 = sys::MSFT_holographic_window_attachment_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_HOLOGRAPHIC_WINDOW_ATTACHMENT_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidSessionStateEnableOCULUS {}
    #[cfg(target_os = "android")]
    impl AndroidSessionStateEnableOCULUS {
        pub const VERSION: u32 = sys::OCULUS_android_session_state_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct AnalogThresholdVALVE {}
    impl AnalogThresholdVALVE {
        pub const VERSION: u32 = sys::VALVE_analog_threshold_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VALVE_ANALOG_THRESHOLD_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct QuadViewsVARJO {}
    impl QuadViewsVARJO {
        pub const VERSION: u32 = sys::VARJO_quad_views_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_QUAD_VIEWS_EXTENSION_NAME;
    }
}
#[allow(unused)]
pub(crate) mod builder {
    use crate::*;
    use std::{marker::PhantomData, mem, ops::Deref};
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
    impl<'a, G: Graphics> Default for SwapchainSubImage<'a, G> {
        fn default() -> Self {
            Self::new()
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
    impl<'a, G: Graphics> Default for CompositionLayerProjectionView<'a, G> {
        fn default() -> Self {
            Self::new()
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
    impl<'a> Default for ActionSetCreateInfo<'a> {
        fn default() -> Self {
            Self::new()
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
    impl<'a> Default for ActionCreateInfo<'a> {
        fn default() -> Self {
            Self::new()
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
    impl<'a, G: Graphics> Default for CompositionLayerProjection<'a, G> {
        fn default() -> Self {
            Self::new()
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
        pub fn size(mut self, value: Extent2Df) -> Self {
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
    impl<'a, G: Graphics> Default for CompositionLayerQuad<'a, G> {
        fn default() -> Self {
            Self::new()
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
    impl<'a, G: Graphics> Default for CompositionLayerCylinderKHR<'a, G> {
        fn default() -> Self {
            Self::new()
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
    }
    impl<'a, G: Graphics> Deref for CompositionLayerCubeKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a, G: Graphics> Default for CompositionLayerCubeKHR<'a, G> {
        fn default() -> Self {
            Self::new()
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
        pub fn radius(mut self, value: f32) -> Self {
            self.inner.radius = value;
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
    impl<'a, G: Graphics> Default for CompositionLayerEquirectKHR<'a, G> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerEquirect2KHR<'a, G: Graphics> {
        inner: sys::CompositionLayerEquirect2KHR,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerEquirect2KHR<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerEquirect2KHR {
                    ty: sys::StructureType::COMPOSITION_LAYER_EQUIRECT2_KHR,
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
        pub unsafe fn from_raw(inner: sys::CompositionLayerEquirect2KHR) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerEquirect2KHR {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerEquirect2KHR {
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
        pub fn central_horizontal_angle(mut self, value: f32) -> Self {
            self.inner.central_horizontal_angle = value;
            self
        }
        #[inline]
        pub fn upper_vertical_angle(mut self, value: f32) -> Self {
            self.inner.upper_vertical_angle = value;
            self
        }
        #[inline]
        pub fn lower_vertical_angle(mut self, value: f32) -> Self {
            self.inner.lower_vertical_angle = value;
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerEquirect2KHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a, G: Graphics> Default for CompositionLayerEquirect2KHR<'a, G> {
        fn default() -> Self {
            Self::new()
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
    impl<'a> Default for HapticVibration<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
}

#![doc = r" Automatically generated code; do not edit!"]
#![allow(
    clippy::wrong_self_convention,
    clippy::transmute_ptr_to_ptr,
    clippy::missing_transmute_annotations
)]
use crate::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::iter::FromIterator;
use std::mem::MaybeUninit;
pub use sys::platform::{
    EGLenum, VkComponentSwizzle, VkFilter, VkSamplerAddressMode, VkSamplerMipmapMode,
};
pub use sys::{
    ActionType, AndroidSurfaceSwapchainFlagsFB, AndroidThreadTypeKHR, BlendFactorFB, BodyJointFB,
    BodyJointLocationFB, BodyJointSetFB, BodySkeletonJointFB, Boxf, Color3f, Color4f, ColorSpaceFB,
    CompareOpFB, CompositionLayerFlags, CompositionLayerImageLayoutFlagsFB,
    CompositionLayerSecureContentFlagsFB, CompositionLayerSettingsFlagsFB,
    CompositionLayerSpaceWarpInfoFlagsFB, DebugUtilsMessageSeverityFlagsEXT,
    DebugUtilsMessageTypeFlagsEXT, DigitalLensControlFlagsALMALENCE, EnvironmentBlendMode,
    EnvironmentDepthProviderCreateFlagsMETA, EnvironmentDepthSwapchainCreateFlagsMETA, Extent2Df,
    Extent2Di, Extent3Df, ExternalCameraAttachedToDeviceOCULUS, ExternalCameraExtrinsicsOCULUS,
    ExternalCameraIntrinsicsOCULUS, ExternalCameraStatusFlagsOCULUS, EyeCalibrationStatusML,
    EyeExpressionHTC, EyePositionFB, EyeVisibility, FaceConfidence2FB, FaceConfidenceFB,
    FaceExpression2FB, FaceExpressionFB, FaceExpressionSet2FB, FaceExpressionSetFB,
    FaceTrackingDataSource2FB, FacialTrackingTypeHTC, ForceFeedbackCurlApplyLocationMNDX,
    ForceFeedbackCurlLocationMNDX, FormFactor, FoveationConfigurationHTC, FoveationDynamicFB,
    FoveationDynamicFlagsHTC, FoveationEyeTrackedProfileCreateFlagsMETA,
    FoveationEyeTrackedStateFlagsMETA, FoveationLevelFB, FoveationLevelHTC, FoveationModeHTC, Fovf,
    FrameEndInfoFlagsML, Frustumf, FutureStateEXT, GlobalDimmerFrameEndInfoFlagsML, HandEXT,
    HandForearmJointULTRALEAP, HandJointEXT, HandJointLocationEXT, HandJointSetEXT,
    HandJointVelocityEXT, HandJointsMotionRangeEXT, HandMeshVertexMSFT, HandPoseTypeMSFT,
    HandTrackingAimFlagsFB, HandTrackingDataSourceEXT, HeadsetFitStatusML,
    InputSourceLocalizedNameFlags, InstanceCreateFlags, KeyboardTrackingFlagsFB,
    KeyboardTrackingQueryFlagsFB, LipExpressionHTC, LoaderInterfaceStructs, LocalDimmingModeMETA,
    LocalizationMapConfidenceML, LocalizationMapErrorFlagsML, LocalizationMapStateML,
    LocalizationMapTypeML, MarkerAprilTagDictML, MarkerArucoDictML, MarkerDetectorCameraML,
    MarkerDetectorCornerRefineMethodML, MarkerDetectorFpsML, MarkerDetectorFullAnalysisIntervalML,
    MarkerDetectorProfileML, MarkerDetectorResolutionML, MarkerDetectorStatusML, MarkerTypeML,
    MeshComputeLodMSFT, NegotiateApiLayerRequest, NegotiateLoaderInfo, NegotiateRuntimeRequest,
    ObjectType, Offset2Df, Offset2Di, Offset3DfFB, OverlayMainSessionFlagsEXTX,
    OverlaySessionCreateFlagsEXTX, PassthroughCapabilityFlagsFB, PassthroughColorLutChannelsMETA,
    PassthroughFlagsFB, PassthroughFormHTC, PassthroughLayerPurposeFB,
    PassthroughPreferenceFlagsMETA, PassthroughStateChangedFlagsFB, PerfSettingsDomainEXT,
    PerfSettingsLevelEXT, PerfSettingsNotificationLevelEXT, PerfSettingsSubDomainEXT,
    PerformanceMetricsCounterFlagsMETA, PerformanceMetricsCounterUnitMETA,
    PlaneDetectionCapabilityFlagsEXT, PlaneDetectionStateEXT, PlaneDetectorFlagsEXT,
    PlaneDetectorOrientationEXT, PlaneDetectorSemanticTypeEXT, Posef, Quaternionf, Rect2Df,
    Rect2Di, Rect3DfFB, ReferenceSpaceType, RenderModelFlagsFB, ReprojectionModeMSFT,
    SceneComponentTypeMSFT, SceneComputeConsistencyMSFT, SceneComputeFeatureMSFT,
    SceneComputeStateMSFT, SceneMarkerQRCodeSymbolTypeMSFT, SceneMarkerTypeMSFT,
    SceneObjectTypeMSFT, ScenePlaneAlignmentTypeMSFT, SemanticLabelsSupportFlagsFB,
    SessionCreateFlags, SessionState, SpaceComponentTypeFB, SpaceLocationData, SpaceLocationFlags,
    SpacePersistenceModeFB, SpaceQueryActionFB, SpaceStorageLocationFB, SpaceVelocityData,
    SpaceVelocityFlags, SpatialGraphNodeTypeMSFT, Spheref, StructureType, SwapchainCreateFlags,
    SwapchainCreateFoveationFlagsFB, SwapchainStateFoveationFlagsFB, SwapchainUsageFlags,
    SystemGraphicsProperties, TrackingOptimizationSettingsDomainQCOM,
    TrackingOptimizationSettingsHintQCOM, TriangleMeshFlagsFB, Vector2f, Vector3f, Vector4f,
    Vector4sFB, ViewConfigurationType, ViewStateFlags, VirtualKeyboardInputSourceMETA,
    VirtualKeyboardInputStateFlagsMETA, VirtualKeyboardLocationTypeMETA, VisibilityMaskTypeKHR,
    VulkanDeviceCreateFlagsKHR, VulkanInstanceCreateFlagsKHR, WindingOrderFB,
};
#[doc = r" A subset of known extensions"]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[non_exhaustive]
pub struct ExtensionSet {
    pub almalence_digital_lens_control: bool,
    pub bd_controller_interaction: bool,
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
    pub ext_dpad_binding: bool,
    pub ext_hand_joints_motion_range: bool,
    pub ext_samsung_odyssey_controller: bool,
    pub ext_hp_mixed_reality_controller: bool,
    pub ext_palm_pose: bool,
    pub ext_uuid: bool,
    pub ext_hand_interaction: bool,
    pub ext_active_action_set_priority: bool,
    pub ext_local_floor: bool,
    pub ext_hand_tracking_data_source: bool,
    pub ext_plane_detection: bool,
    pub ext_future: bool,
    pub ext_user_presence: bool,
    pub ext_composition_layer_inverted_alpha: bool,
    pub fb_composition_layer_image_layout: bool,
    pub fb_composition_layer_alpha_blend: bool,
    #[cfg(target_os = "android")]
    pub fb_android_surface_swapchain_create: bool,
    pub fb_swapchain_update_state: bool,
    pub fb_composition_layer_secure_content: bool,
    pub fb_body_tracking: bool,
    pub fb_display_refresh_rate: bool,
    pub fb_color_space: bool,
    pub fb_hand_tracking_mesh: bool,
    pub fb_hand_tracking_aim: bool,
    pub fb_hand_tracking_capsules: bool,
    pub fb_spatial_entity: bool,
    pub fb_foveation: bool,
    pub fb_foveation_configuration: bool,
    pub fb_keyboard_tracking: bool,
    pub fb_triangle_mesh: bool,
    pub fb_passthrough: bool,
    pub fb_render_model: bool,
    pub fb_spatial_entity_query: bool,
    pub fb_spatial_entity_storage: bool,
    pub fb_foveation_vulkan: bool,
    #[cfg(target_os = "android")]
    pub fb_swapchain_update_state_android_surface: bool,
    pub fb_swapchain_update_state_opengl_es: bool,
    pub fb_swapchain_update_state_vulkan: bool,
    pub fb_touch_controller_pro: bool,
    pub fb_spatial_entity_sharing: bool,
    pub fb_space_warp: bool,
    pub fb_haptic_amplitude_envelope: bool,
    pub fb_scene: bool,
    pub fb_scene_capture: bool,
    pub fb_spatial_entity_container: bool,
    pub fb_face_tracking: bool,
    pub fb_eye_tracking_social: bool,
    pub fb_passthrough_keyboard_hands: bool,
    pub fb_composition_layer_settings: bool,
    pub fb_touch_controller_proximity: bool,
    pub fb_haptic_pcm: bool,
    pub fb_composition_layer_depth_test: bool,
    pub fb_spatial_entity_storage_batch: bool,
    pub fb_spatial_entity_user: bool,
    pub fb_face_tracking2: bool,
    pub htc_vive_cosmos_controller_interaction: bool,
    pub htc_facial_tracking: bool,
    pub htc_vive_focus3_controller_interaction: bool,
    pub htc_hand_interaction: bool,
    pub htc_vive_wrist_tracker_interaction: bool,
    pub htc_passthrough: bool,
    pub htc_foveation: bool,
    pub htc_anchor: bool,
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
    #[cfg(target_vendor = "apple")]
    pub khr_metal_enable: bool,
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
    pub khr_binding_modification: bool,
    pub khr_swapchain_usage_input_attachment_bit: bool,
    pub khr_locate_spaces: bool,
    pub khr_maintenance1: bool,
    pub meta_foveation_eye_tracked: bool,
    pub meta_local_dimming: bool,
    pub meta_passthrough_preferences: bool,
    pub meta_virtual_keyboard: bool,
    pub meta_vulkan_swapchain_create_info: bool,
    pub meta_performance_metrics: bool,
    pub meta_headset_id: bool,
    pub meta_recommended_layer_resolution: bool,
    pub meta_passthrough_color_lut: bool,
    pub meta_spatial_entity_mesh: bool,
    pub meta_automatic_layer_filter: bool,
    pub meta_touch_controller_plus: bool,
    pub meta_environment_depth: bool,
    pub ml_ml2_controller_interaction: bool,
    pub ml_frame_end_info: bool,
    pub ml_global_dimmer: bool,
    pub ml_compat: bool,
    pub ml_marker_understanding: bool,
    pub ml_localization_map: bool,
    pub ml_user_calibration: bool,
    pub mnd_headless: bool,
    pub mnd_swapchain_usage_input_attachment_bit: bool,
    pub msft_unbounded_reference_space: bool,
    pub msft_spatial_anchor: bool,
    pub msft_spatial_graph_bridge: bool,
    pub msft_hand_interaction: bool,
    pub msft_hand_tracking_mesh: bool,
    pub msft_secondary_view_configuration: bool,
    pub msft_first_person_observer: bool,
    pub msft_controller_model: bool,
    #[cfg(windows)]
    pub msft_perception_anchor_interop: bool,
    #[cfg(windows)]
    pub msft_holographic_window_attachment: bool,
    pub msft_composition_layer_reprojection: bool,
    pub msft_spatial_anchor_persistence: bool,
    #[cfg(target_os = "android")]
    pub oculus_android_session_state_enable: bool,
    pub oculus_audio_device_guid: bool,
    pub oculus_external_camera: bool,
    pub oppo_controller_interaction: bool,
    pub qcom_tracking_optimization_settings: bool,
    pub ultraleap_hand_tracking_forearm: bool,
    pub valve_analog_threshold: bool,
    pub varjo_quad_views: bool,
    pub varjo_foveated_rendering: bool,
    pub varjo_composition_layer_depth_test: bool,
    pub varjo_environment_depth_estimation: bool,
    pub varjo_marker_tracking: bool,
    pub varjo_view_offset: bool,
    pub varjo_xr4_controller_interaction: bool,
    pub yvr_controller_interaction: bool,
    pub extx_overlay: bool,
    pub mndx_egl_enable: bool,
    pub mndx_force_feedback_curl: bool,
    pub htcx_vive_tracker_interaction: bool,
    #[doc = r" Extensions unknown to the high-level bindings"]
    pub other: Vec<String>,
}
impl<'a> FromIterator<&'a str> for ExtensionSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut out = Self::default();
        for name in iter {
            match name {
                raw::DigitalLensControlALMALENCE::NAME_STR => {
                    out.almalence_digital_lens_control = true;
                }
                raw::ControllerInteractionBD::NAME_STR => {
                    out.bd_controller_interaction = true;
                }
                raw::ViewConfigurationFovEPIC::NAME_STR => {
                    out.epic_view_configuration_fov = true;
                }
                raw::PerformanceSettingsEXT::NAME_STR => {
                    out.ext_performance_settings = true;
                }
                raw::ThermalQueryEXT::NAME_STR => {
                    out.ext_thermal_query = true;
                }
                raw::DebugUtilsEXT::NAME_STR => {
                    out.ext_debug_utils = true;
                }
                raw::EyeGazeInteractionEXT::NAME_STR => {
                    out.ext_eye_gaze_interaction = true;
                }
                raw::ViewConfigurationDepthRangeEXT::NAME_STR => {
                    out.ext_view_configuration_depth_range = true;
                }
                raw::ConformanceAutomationEXT::NAME_STR => {
                    out.ext_conformance_automation = true;
                }
                raw::HandTrackingEXT::NAME_STR => {
                    out.ext_hand_tracking = true;
                }
                #[cfg(windows)]
                raw::Win32AppcontainerCompatibleEXT::NAME_STR => {
                    out.ext_win32_appcontainer_compatible = true;
                }
                raw::DpadBindingEXT::NAME_STR => {
                    out.ext_dpad_binding = true;
                }
                raw::HandJointsMotionRangeEXT::NAME_STR => {
                    out.ext_hand_joints_motion_range = true;
                }
                raw::SamsungOdysseyControllerEXT::NAME_STR => {
                    out.ext_samsung_odyssey_controller = true;
                }
                raw::HpMixedRealityControllerEXT::NAME_STR => {
                    out.ext_hp_mixed_reality_controller = true;
                }
                raw::PalmPoseEXT::NAME_STR => {
                    out.ext_palm_pose = true;
                }
                raw::UuidEXT::NAME_STR => {
                    out.ext_uuid = true;
                }
                raw::HandInteractionEXT::NAME_STR => {
                    out.ext_hand_interaction = true;
                }
                raw::ActiveActionSetPriorityEXT::NAME_STR => {
                    out.ext_active_action_set_priority = true;
                }
                raw::LocalFloorEXT::NAME_STR => {
                    out.ext_local_floor = true;
                }
                raw::HandTrackingDataSourceEXT::NAME_STR => {
                    out.ext_hand_tracking_data_source = true;
                }
                raw::PlaneDetectionEXT::NAME_STR => {
                    out.ext_plane_detection = true;
                }
                raw::FutureEXT::NAME_STR => {
                    out.ext_future = true;
                }
                raw::UserPresenceEXT::NAME_STR => {
                    out.ext_user_presence = true;
                }
                raw::CompositionLayerInvertedAlphaEXT::NAME_STR => {
                    out.ext_composition_layer_inverted_alpha = true;
                }
                raw::CompositionLayerImageLayoutFB::NAME_STR => {
                    out.fb_composition_layer_image_layout = true;
                }
                raw::CompositionLayerAlphaBlendFB::NAME_STR => {
                    out.fb_composition_layer_alpha_blend = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSurfaceSwapchainCreateFB::NAME_STR => {
                    out.fb_android_surface_swapchain_create = true;
                }
                raw::SwapchainUpdateStateFB::NAME_STR => {
                    out.fb_swapchain_update_state = true;
                }
                raw::CompositionLayerSecureContentFB::NAME_STR => {
                    out.fb_composition_layer_secure_content = true;
                }
                raw::BodyTrackingFB::NAME_STR => {
                    out.fb_body_tracking = true;
                }
                raw::DisplayRefreshRateFB::NAME_STR => {
                    out.fb_display_refresh_rate = true;
                }
                raw::ColorSpaceFB::NAME_STR => {
                    out.fb_color_space = true;
                }
                raw::HandTrackingMeshFB::NAME_STR => {
                    out.fb_hand_tracking_mesh = true;
                }
                raw::HandTrackingAimFB::NAME_STR => {
                    out.fb_hand_tracking_aim = true;
                }
                raw::HandTrackingCapsulesFB::NAME_STR => {
                    out.fb_hand_tracking_capsules = true;
                }
                raw::SpatialEntityFB::NAME_STR => {
                    out.fb_spatial_entity = true;
                }
                raw::FoveationFB::NAME_STR => {
                    out.fb_foveation = true;
                }
                raw::FoveationConfigurationFB::NAME_STR => {
                    out.fb_foveation_configuration = true;
                }
                raw::KeyboardTrackingFB::NAME_STR => {
                    out.fb_keyboard_tracking = true;
                }
                raw::TriangleMeshFB::NAME_STR => {
                    out.fb_triangle_mesh = true;
                }
                raw::PassthroughFB::NAME_STR => {
                    out.fb_passthrough = true;
                }
                raw::RenderModelFB::NAME_STR => {
                    out.fb_render_model = true;
                }
                raw::SpatialEntityQueryFB::NAME_STR => {
                    out.fb_spatial_entity_query = true;
                }
                raw::SpatialEntityStorageFB::NAME_STR => {
                    out.fb_spatial_entity_storage = true;
                }
                raw::FoveationVulkanFB::NAME_STR => {
                    out.fb_foveation_vulkan = true;
                }
                #[cfg(target_os = "android")]
                raw::SwapchainUpdateStateAndroidSurfaceFB::NAME_STR => {
                    out.fb_swapchain_update_state_android_surface = true;
                }
                raw::SwapchainUpdateStateOpenglEsFB::NAME_STR => {
                    out.fb_swapchain_update_state_opengl_es = true;
                }
                raw::SwapchainUpdateStateVulkanFB::NAME_STR => {
                    out.fb_swapchain_update_state_vulkan = true;
                }
                raw::TouchControllerProFB::NAME_STR => {
                    out.fb_touch_controller_pro = true;
                }
                raw::SpatialEntitySharingFB::NAME_STR => {
                    out.fb_spatial_entity_sharing = true;
                }
                raw::SpaceWarpFB::NAME_STR => {
                    out.fb_space_warp = true;
                }
                raw::HapticAmplitudeEnvelopeFB::NAME_STR => {
                    out.fb_haptic_amplitude_envelope = true;
                }
                raw::SceneFB::NAME_STR => {
                    out.fb_scene = true;
                }
                raw::SceneCaptureFB::NAME_STR => {
                    out.fb_scene_capture = true;
                }
                raw::SpatialEntityContainerFB::NAME_STR => {
                    out.fb_spatial_entity_container = true;
                }
                raw::FaceTrackingFB::NAME_STR => {
                    out.fb_face_tracking = true;
                }
                raw::EyeTrackingSocialFB::NAME_STR => {
                    out.fb_eye_tracking_social = true;
                }
                raw::PassthroughKeyboardHandsFB::NAME_STR => {
                    out.fb_passthrough_keyboard_hands = true;
                }
                raw::CompositionLayerSettingsFB::NAME_STR => {
                    out.fb_composition_layer_settings = true;
                }
                raw::TouchControllerProximityFB::NAME_STR => {
                    out.fb_touch_controller_proximity = true;
                }
                raw::HapticPcmFB::NAME_STR => {
                    out.fb_haptic_pcm = true;
                }
                raw::CompositionLayerDepthTestFB::NAME_STR => {
                    out.fb_composition_layer_depth_test = true;
                }
                raw::SpatialEntityStorageBatchFB::NAME_STR => {
                    out.fb_spatial_entity_storage_batch = true;
                }
                raw::SpatialEntityUserFB::NAME_STR => {
                    out.fb_spatial_entity_user = true;
                }
                raw::FaceTracking2FB::NAME_STR => {
                    out.fb_face_tracking2 = true;
                }
                raw::ViveCosmosControllerInteractionHTC::NAME_STR => {
                    out.htc_vive_cosmos_controller_interaction = true;
                }
                raw::FacialTrackingHTC::NAME_STR => {
                    out.htc_facial_tracking = true;
                }
                raw::ViveFocus3ControllerInteractionHTC::NAME_STR => {
                    out.htc_vive_focus3_controller_interaction = true;
                }
                raw::HandInteractionHTC::NAME_STR => {
                    out.htc_hand_interaction = true;
                }
                raw::ViveWristTrackerInteractionHTC::NAME_STR => {
                    out.htc_vive_wrist_tracker_interaction = true;
                }
                raw::PassthroughHTC::NAME_STR => {
                    out.htc_passthrough = true;
                }
                raw::FoveationHTC::NAME_STR => {
                    out.htc_foveation = true;
                }
                raw::AnchorHTC::NAME_STR => {
                    out.htc_anchor = true;
                }
                raw::ControllerInteractionHUAWEI::NAME_STR => {
                    out.huawei_controller_interaction = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidThreadSettingsKHR::NAME_STR => {
                    out.khr_android_thread_settings = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSurfaceSwapchainKHR::NAME_STR => {
                    out.khr_android_surface_swapchain = true;
                }
                raw::CompositionLayerCubeKHR::NAME_STR => {
                    out.khr_composition_layer_cube = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidCreateInstanceKHR::NAME_STR => {
                    out.khr_android_create_instance = true;
                }
                raw::CompositionLayerDepthKHR::NAME_STR => {
                    out.khr_composition_layer_depth = true;
                }
                raw::VulkanSwapchainFormatListKHR::NAME_STR => {
                    out.khr_vulkan_swapchain_format_list = true;
                }
                raw::CompositionLayerCylinderKHR::NAME_STR => {
                    out.khr_composition_layer_cylinder = true;
                }
                raw::CompositionLayerEquirectKHR::NAME_STR => {
                    out.khr_composition_layer_equirect = true;
                }
                raw::OpenglEnableKHR::NAME_STR => {
                    out.khr_opengl_enable = true;
                }
                raw::OpenglEsEnableKHR::NAME_STR => {
                    out.khr_opengl_es_enable = true;
                }
                raw::VulkanEnableKHR::NAME_STR => {
                    out.khr_vulkan_enable = true;
                }
                #[cfg(windows)]
                raw::D3d11EnableKHR::NAME_STR => {
                    out.khr_d3d11_enable = true;
                }
                #[cfg(windows)]
                raw::D3d12EnableKHR::NAME_STR => {
                    out.khr_d3d12_enable = true;
                }
                #[cfg(target_vendor = "apple")]
                raw::MetalEnableKHR::NAME_STR => {
                    out.khr_metal_enable = true;
                }
                raw::VisibilityMaskKHR::NAME_STR => {
                    out.khr_visibility_mask = true;
                }
                raw::CompositionLayerColorScaleBiasKHR::NAME_STR => {
                    out.khr_composition_layer_color_scale_bias = true;
                }
                #[cfg(windows)]
                raw::Win32ConvertPerformanceCounterTimeKHR::NAME_STR => {
                    out.khr_win32_convert_performance_counter_time = true;
                }
                raw::ConvertTimespecTimeKHR::NAME_STR => {
                    out.khr_convert_timespec_time = true;
                }
                raw::LoaderInitKHR::NAME_STR => {
                    out.khr_loader_init = true;
                }
                #[cfg(target_os = "android")]
                raw::LoaderInitAndroidKHR::NAME_STR => {
                    out.khr_loader_init_android = true;
                }
                raw::VulkanEnable2KHR::NAME_STR => {
                    out.khr_vulkan_enable2 = true;
                }
                raw::CompositionLayerEquirect2KHR::NAME_STR => {
                    out.khr_composition_layer_equirect2 = true;
                }
                raw::BindingModificationKHR::NAME_STR => {
                    out.khr_binding_modification = true;
                }
                raw::SwapchainUsageInputAttachmentBitKHR::NAME_STR => {
                    out.khr_swapchain_usage_input_attachment_bit = true;
                }
                raw::LocateSpacesKHR::NAME_STR => {
                    out.khr_locate_spaces = true;
                }
                raw::Maintenance1KHR::NAME_STR => {
                    out.khr_maintenance1 = true;
                }
                raw::FoveationEyeTrackedMETA::NAME_STR => {
                    out.meta_foveation_eye_tracked = true;
                }
                raw::LocalDimmingMETA::NAME_STR => {
                    out.meta_local_dimming = true;
                }
                raw::PassthroughPreferencesMETA::NAME_STR => {
                    out.meta_passthrough_preferences = true;
                }
                raw::VirtualKeyboardMETA::NAME_STR => {
                    out.meta_virtual_keyboard = true;
                }
                raw::VulkanSwapchainCreateInfoMETA::NAME_STR => {
                    out.meta_vulkan_swapchain_create_info = true;
                }
                raw::PerformanceMetricsMETA::NAME_STR => {
                    out.meta_performance_metrics = true;
                }
                raw::HeadsetIdMETA::NAME_STR => {
                    out.meta_headset_id = true;
                }
                raw::RecommendedLayerResolutionMETA::NAME_STR => {
                    out.meta_recommended_layer_resolution = true;
                }
                raw::PassthroughColorLutMETA::NAME_STR => {
                    out.meta_passthrough_color_lut = true;
                }
                raw::SpatialEntityMeshMETA::NAME_STR => {
                    out.meta_spatial_entity_mesh = true;
                }
                raw::AutomaticLayerFilterMETA::NAME_STR => {
                    out.meta_automatic_layer_filter = true;
                }
                raw::TouchControllerPlusMETA::NAME_STR => {
                    out.meta_touch_controller_plus = true;
                }
                raw::EnvironmentDepthMETA::NAME_STR => {
                    out.meta_environment_depth = true;
                }
                raw::Ml2ControllerInteractionML::NAME_STR => {
                    out.ml_ml2_controller_interaction = true;
                }
                raw::FrameEndInfoML::NAME_STR => {
                    out.ml_frame_end_info = true;
                }
                raw::GlobalDimmerML::NAME_STR => {
                    out.ml_global_dimmer = true;
                }
                raw::CompatML::NAME_STR => {
                    out.ml_compat = true;
                }
                raw::MarkerUnderstandingML::NAME_STR => {
                    out.ml_marker_understanding = true;
                }
                raw::LocalizationMapML::NAME_STR => {
                    out.ml_localization_map = true;
                }
                raw::UserCalibrationML::NAME_STR => {
                    out.ml_user_calibration = true;
                }
                raw::HeadlessMND::NAME_STR => {
                    out.mnd_headless = true;
                }
                raw::SwapchainUsageInputAttachmentBitMND::NAME_STR => {
                    out.mnd_swapchain_usage_input_attachment_bit = true;
                }
                raw::UnboundedReferenceSpaceMSFT::NAME_STR => {
                    out.msft_unbounded_reference_space = true;
                }
                raw::SpatialAnchorMSFT::NAME_STR => {
                    out.msft_spatial_anchor = true;
                }
                raw::SpatialGraphBridgeMSFT::NAME_STR => {
                    out.msft_spatial_graph_bridge = true;
                }
                raw::HandInteractionMSFT::NAME_STR => {
                    out.msft_hand_interaction = true;
                }
                raw::HandTrackingMeshMSFT::NAME_STR => {
                    out.msft_hand_tracking_mesh = true;
                }
                raw::SecondaryViewConfigurationMSFT::NAME_STR => {
                    out.msft_secondary_view_configuration = true;
                }
                raw::FirstPersonObserverMSFT::NAME_STR => {
                    out.msft_first_person_observer = true;
                }
                raw::ControllerModelMSFT::NAME_STR => {
                    out.msft_controller_model = true;
                }
                #[cfg(windows)]
                raw::PerceptionAnchorInteropMSFT::NAME_STR => {
                    out.msft_perception_anchor_interop = true;
                }
                #[cfg(windows)]
                raw::HolographicWindowAttachmentMSFT::NAME_STR => {
                    out.msft_holographic_window_attachment = true;
                }
                raw::CompositionLayerReprojectionMSFT::NAME_STR => {
                    out.msft_composition_layer_reprojection = true;
                }
                raw::SpatialAnchorPersistenceMSFT::NAME_STR => {
                    out.msft_spatial_anchor_persistence = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSessionStateEnableOCULUS::NAME_STR => {
                    out.oculus_android_session_state_enable = true;
                }
                raw::AudioDeviceGuidOCULUS::NAME_STR => {
                    out.oculus_audio_device_guid = true;
                }
                raw::ExternalCameraOCULUS::NAME_STR => {
                    out.oculus_external_camera = true;
                }
                raw::ControllerInteractionOPPO::NAME_STR => {
                    out.oppo_controller_interaction = true;
                }
                raw::TrackingOptimizationSettingsQCOM::NAME_STR => {
                    out.qcom_tracking_optimization_settings = true;
                }
                raw::HandTrackingForearmULTRALEAP::NAME_STR => {
                    out.ultraleap_hand_tracking_forearm = true;
                }
                raw::AnalogThresholdVALVE::NAME_STR => {
                    out.valve_analog_threshold = true;
                }
                raw::QuadViewsVARJO::NAME_STR => {
                    out.varjo_quad_views = true;
                }
                raw::FoveatedRenderingVARJO::NAME_STR => {
                    out.varjo_foveated_rendering = true;
                }
                raw::CompositionLayerDepthTestVARJO::NAME_STR => {
                    out.varjo_composition_layer_depth_test = true;
                }
                raw::EnvironmentDepthEstimationVARJO::NAME_STR => {
                    out.varjo_environment_depth_estimation = true;
                }
                raw::MarkerTrackingVARJO::NAME_STR => {
                    out.varjo_marker_tracking = true;
                }
                raw::ViewOffsetVARJO::NAME_STR => {
                    out.varjo_view_offset = true;
                }
                raw::Xr4ControllerInteractionVARJO::NAME_STR => {
                    out.varjo_xr4_controller_interaction = true;
                }
                raw::ControllerInteractionYVR::NAME_STR => {
                    out.yvr_controller_interaction = true;
                }
                raw::OverlayEXTX::NAME_STR => {
                    out.extx_overlay = true;
                }
                raw::EglEnableMNDX::NAME_STR => {
                    out.mndx_egl_enable = true;
                }
                raw::ForceFeedbackCurlMNDX::NAME_STR => {
                    out.mndx_force_feedback_curl = true;
                }
                raw::ViveTrackerInteractionHTCX::NAME_STR => {
                    out.htcx_vive_tracker_interaction = true;
                }
                _ => out.other.push(name.to_string()),
            }
        }
        out
    }
}
impl ExtensionSet {
    pub(crate) fn from_properties(properties: &[sys::ExtensionProperties]) -> Self {
        properties
            .iter()
            .map(|ext| {
                let name = unsafe {
                    &*(&ext.extension_name as *const _ as *const [u8; sys::MAX_EXTENSION_NAME_SIZE])
                };
                CStr::from_bytes_until_nul(name)
                    .expect("extension names should be null terminated strings")
                    .to_str()
                    .expect("extension names should be valid UTF-8")
            })
            .collect()
    }
    pub(crate) fn names(&self) -> Vec<Cow<'static, [u8]>> {
        let mut out = Vec::new();
        {
            if self.almalence_digital_lens_control {
                out.push(raw::DigitalLensControlALMALENCE::NAME.into());
            }
        }
        {
            if self.bd_controller_interaction {
                out.push(raw::ControllerInteractionBD::NAME.into());
            }
        }
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
            if self.ext_dpad_binding {
                out.push(raw::DpadBindingEXT::NAME.into());
            }
        }
        {
            if self.ext_hand_joints_motion_range {
                out.push(raw::HandJointsMotionRangeEXT::NAME.into());
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
            if self.ext_palm_pose {
                out.push(raw::PalmPoseEXT::NAME.into());
            }
        }
        {
            if self.ext_uuid {
                out.push(raw::UuidEXT::NAME.into());
            }
        }
        {
            if self.ext_hand_interaction {
                out.push(raw::HandInteractionEXT::NAME.into());
            }
        }
        {
            if self.ext_active_action_set_priority {
                out.push(raw::ActiveActionSetPriorityEXT::NAME.into());
            }
        }
        {
            if self.ext_local_floor {
                out.push(raw::LocalFloorEXT::NAME.into());
            }
        }
        {
            if self.ext_hand_tracking_data_source {
                out.push(raw::HandTrackingDataSourceEXT::NAME.into());
            }
        }
        {
            if self.ext_plane_detection {
                out.push(raw::PlaneDetectionEXT::NAME.into());
            }
        }
        {
            if self.ext_future {
                out.push(raw::FutureEXT::NAME.into());
            }
        }
        {
            if self.ext_user_presence {
                out.push(raw::UserPresenceEXT::NAME.into());
            }
        }
        {
            if self.ext_composition_layer_inverted_alpha {
                out.push(raw::CompositionLayerInvertedAlphaEXT::NAME.into());
            }
        }
        {
            if self.fb_composition_layer_image_layout {
                out.push(raw::CompositionLayerImageLayoutFB::NAME.into());
            }
        }
        {
            if self.fb_composition_layer_alpha_blend {
                out.push(raw::CompositionLayerAlphaBlendFB::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.fb_android_surface_swapchain_create {
                out.push(raw::AndroidSurfaceSwapchainCreateFB::NAME.into());
            }
        }
        {
            if self.fb_swapchain_update_state {
                out.push(raw::SwapchainUpdateStateFB::NAME.into());
            }
        }
        {
            if self.fb_composition_layer_secure_content {
                out.push(raw::CompositionLayerSecureContentFB::NAME.into());
            }
        }
        {
            if self.fb_body_tracking {
                out.push(raw::BodyTrackingFB::NAME.into());
            }
        }
        {
            if self.fb_display_refresh_rate {
                out.push(raw::DisplayRefreshRateFB::NAME.into());
            }
        }
        {
            if self.fb_color_space {
                out.push(raw::ColorSpaceFB::NAME.into());
            }
        }
        {
            if self.fb_hand_tracking_mesh {
                out.push(raw::HandTrackingMeshFB::NAME.into());
            }
        }
        {
            if self.fb_hand_tracking_aim {
                out.push(raw::HandTrackingAimFB::NAME.into());
            }
        }
        {
            if self.fb_hand_tracking_capsules {
                out.push(raw::HandTrackingCapsulesFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity {
                out.push(raw::SpatialEntityFB::NAME.into());
            }
        }
        {
            if self.fb_foveation {
                out.push(raw::FoveationFB::NAME.into());
            }
        }
        {
            if self.fb_foveation_configuration {
                out.push(raw::FoveationConfigurationFB::NAME.into());
            }
        }
        {
            if self.fb_keyboard_tracking {
                out.push(raw::KeyboardTrackingFB::NAME.into());
            }
        }
        {
            if self.fb_triangle_mesh {
                out.push(raw::TriangleMeshFB::NAME.into());
            }
        }
        {
            if self.fb_passthrough {
                out.push(raw::PassthroughFB::NAME.into());
            }
        }
        {
            if self.fb_render_model {
                out.push(raw::RenderModelFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_query {
                out.push(raw::SpatialEntityQueryFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_storage {
                out.push(raw::SpatialEntityStorageFB::NAME.into());
            }
        }
        {
            if self.fb_foveation_vulkan {
                out.push(raw::FoveationVulkanFB::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.fb_swapchain_update_state_android_surface {
                out.push(raw::SwapchainUpdateStateAndroidSurfaceFB::NAME.into());
            }
        }
        {
            if self.fb_swapchain_update_state_opengl_es {
                out.push(raw::SwapchainUpdateStateOpenglEsFB::NAME.into());
            }
        }
        {
            if self.fb_swapchain_update_state_vulkan {
                out.push(raw::SwapchainUpdateStateVulkanFB::NAME.into());
            }
        }
        {
            if self.fb_touch_controller_pro {
                out.push(raw::TouchControllerProFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_sharing {
                out.push(raw::SpatialEntitySharingFB::NAME.into());
            }
        }
        {
            if self.fb_space_warp {
                out.push(raw::SpaceWarpFB::NAME.into());
            }
        }
        {
            if self.fb_haptic_amplitude_envelope {
                out.push(raw::HapticAmplitudeEnvelopeFB::NAME.into());
            }
        }
        {
            if self.fb_scene {
                out.push(raw::SceneFB::NAME.into());
            }
        }
        {
            if self.fb_scene_capture {
                out.push(raw::SceneCaptureFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_container {
                out.push(raw::SpatialEntityContainerFB::NAME.into());
            }
        }
        {
            if self.fb_face_tracking {
                out.push(raw::FaceTrackingFB::NAME.into());
            }
        }
        {
            if self.fb_eye_tracking_social {
                out.push(raw::EyeTrackingSocialFB::NAME.into());
            }
        }
        {
            if self.fb_passthrough_keyboard_hands {
                out.push(raw::PassthroughKeyboardHandsFB::NAME.into());
            }
        }
        {
            if self.fb_composition_layer_settings {
                out.push(raw::CompositionLayerSettingsFB::NAME.into());
            }
        }
        {
            if self.fb_touch_controller_proximity {
                out.push(raw::TouchControllerProximityFB::NAME.into());
            }
        }
        {
            if self.fb_haptic_pcm {
                out.push(raw::HapticPcmFB::NAME.into());
            }
        }
        {
            if self.fb_composition_layer_depth_test {
                out.push(raw::CompositionLayerDepthTestFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_storage_batch {
                out.push(raw::SpatialEntityStorageBatchFB::NAME.into());
            }
        }
        {
            if self.fb_spatial_entity_user {
                out.push(raw::SpatialEntityUserFB::NAME.into());
            }
        }
        {
            if self.fb_face_tracking2 {
                out.push(raw::FaceTracking2FB::NAME.into());
            }
        }
        {
            if self.htc_vive_cosmos_controller_interaction {
                out.push(raw::ViveCosmosControllerInteractionHTC::NAME.into());
            }
        }
        {
            if self.htc_facial_tracking {
                out.push(raw::FacialTrackingHTC::NAME.into());
            }
        }
        {
            if self.htc_vive_focus3_controller_interaction {
                out.push(raw::ViveFocus3ControllerInteractionHTC::NAME.into());
            }
        }
        {
            if self.htc_hand_interaction {
                out.push(raw::HandInteractionHTC::NAME.into());
            }
        }
        {
            if self.htc_vive_wrist_tracker_interaction {
                out.push(raw::ViveWristTrackerInteractionHTC::NAME.into());
            }
        }
        {
            if self.htc_passthrough {
                out.push(raw::PassthroughHTC::NAME.into());
            }
        }
        {
            if self.htc_foveation {
                out.push(raw::FoveationHTC::NAME.into());
            }
        }
        {
            if self.htc_anchor {
                out.push(raw::AnchorHTC::NAME.into());
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
        #[cfg(target_vendor = "apple")]
        {
            if self.khr_metal_enable {
                out.push(raw::MetalEnableKHR::NAME.into());
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
            if self.khr_binding_modification {
                out.push(raw::BindingModificationKHR::NAME.into());
            }
        }
        {
            if self.khr_swapchain_usage_input_attachment_bit {
                out.push(raw::SwapchainUsageInputAttachmentBitKHR::NAME.into());
            }
        }
        {
            if self.khr_locate_spaces {
                out.push(raw::LocateSpacesKHR::NAME.into());
            }
        }
        {
            if self.khr_maintenance1 {
                out.push(raw::Maintenance1KHR::NAME.into());
            }
        }
        {
            if self.meta_foveation_eye_tracked {
                out.push(raw::FoveationEyeTrackedMETA::NAME.into());
            }
        }
        {
            if self.meta_local_dimming {
                out.push(raw::LocalDimmingMETA::NAME.into());
            }
        }
        {
            if self.meta_passthrough_preferences {
                out.push(raw::PassthroughPreferencesMETA::NAME.into());
            }
        }
        {
            if self.meta_virtual_keyboard {
                out.push(raw::VirtualKeyboardMETA::NAME.into());
            }
        }
        {
            if self.meta_vulkan_swapchain_create_info {
                out.push(raw::VulkanSwapchainCreateInfoMETA::NAME.into());
            }
        }
        {
            if self.meta_performance_metrics {
                out.push(raw::PerformanceMetricsMETA::NAME.into());
            }
        }
        {
            if self.meta_headset_id {
                out.push(raw::HeadsetIdMETA::NAME.into());
            }
        }
        {
            if self.meta_recommended_layer_resolution {
                out.push(raw::RecommendedLayerResolutionMETA::NAME.into());
            }
        }
        {
            if self.meta_passthrough_color_lut {
                out.push(raw::PassthroughColorLutMETA::NAME.into());
            }
        }
        {
            if self.meta_spatial_entity_mesh {
                out.push(raw::SpatialEntityMeshMETA::NAME.into());
            }
        }
        {
            if self.meta_automatic_layer_filter {
                out.push(raw::AutomaticLayerFilterMETA::NAME.into());
            }
        }
        {
            if self.meta_touch_controller_plus {
                out.push(raw::TouchControllerPlusMETA::NAME.into());
            }
        }
        {
            if self.meta_environment_depth {
                out.push(raw::EnvironmentDepthMETA::NAME.into());
            }
        }
        {
            if self.ml_ml2_controller_interaction {
                out.push(raw::Ml2ControllerInteractionML::NAME.into());
            }
        }
        {
            if self.ml_frame_end_info {
                out.push(raw::FrameEndInfoML::NAME.into());
            }
        }
        {
            if self.ml_global_dimmer {
                out.push(raw::GlobalDimmerML::NAME.into());
            }
        }
        {
            if self.ml_compat {
                out.push(raw::CompatML::NAME.into());
            }
        }
        {
            if self.ml_marker_understanding {
                out.push(raw::MarkerUnderstandingML::NAME.into());
            }
        }
        {
            if self.ml_localization_map {
                out.push(raw::LocalizationMapML::NAME.into());
            }
        }
        {
            if self.ml_user_calibration {
                out.push(raw::UserCalibrationML::NAME.into());
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
            if self.msft_perception_anchor_interop {
                out.push(raw::PerceptionAnchorInteropMSFT::NAME.into());
            }
        }
        #[cfg(windows)]
        {
            if self.msft_holographic_window_attachment {
                out.push(raw::HolographicWindowAttachmentMSFT::NAME.into());
            }
        }
        {
            if self.msft_composition_layer_reprojection {
                out.push(raw::CompositionLayerReprojectionMSFT::NAME.into());
            }
        }
        {
            if self.msft_spatial_anchor_persistence {
                out.push(raw::SpatialAnchorPersistenceMSFT::NAME.into());
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.oculus_android_session_state_enable {
                out.push(raw::AndroidSessionStateEnableOCULUS::NAME.into());
            }
        }
        {
            if self.oculus_audio_device_guid {
                out.push(raw::AudioDeviceGuidOCULUS::NAME.into());
            }
        }
        {
            if self.oculus_external_camera {
                out.push(raw::ExternalCameraOCULUS::NAME.into());
            }
        }
        {
            if self.oppo_controller_interaction {
                out.push(raw::ControllerInteractionOPPO::NAME.into());
            }
        }
        {
            if self.qcom_tracking_optimization_settings {
                out.push(raw::TrackingOptimizationSettingsQCOM::NAME.into());
            }
        }
        {
            if self.ultraleap_hand_tracking_forearm {
                out.push(raw::HandTrackingForearmULTRALEAP::NAME.into());
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
        {
            if self.varjo_foveated_rendering {
                out.push(raw::FoveatedRenderingVARJO::NAME.into());
            }
        }
        {
            if self.varjo_composition_layer_depth_test {
                out.push(raw::CompositionLayerDepthTestVARJO::NAME.into());
            }
        }
        {
            if self.varjo_environment_depth_estimation {
                out.push(raw::EnvironmentDepthEstimationVARJO::NAME.into());
            }
        }
        {
            if self.varjo_marker_tracking {
                out.push(raw::MarkerTrackingVARJO::NAME.into());
            }
        }
        {
            if self.varjo_view_offset {
                out.push(raw::ViewOffsetVARJO::NAME.into());
            }
        }
        {
            if self.varjo_xr4_controller_interaction {
                out.push(raw::Xr4ControllerInteractionVARJO::NAME.into());
            }
        }
        {
            if self.yvr_controller_interaction {
                out.push(raw::ControllerInteractionYVR::NAME.into());
            }
        }
        {
            if self.extx_overlay {
                out.push(raw::OverlayEXTX::NAME.into());
            }
        }
        {
            if self.mndx_egl_enable {
                out.push(raw::EglEnableMNDX::NAME.into());
            }
        }
        {
            if self.mndx_force_feedback_curl {
                out.push(raw::ForceFeedbackCurlMNDX::NAME.into());
            }
        }
        {
            if self.htcx_vive_tracker_interaction {
                out.push(raw::ViveTrackerInteractionHTCX::NAME.into());
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
    pub almalence_digital_lens_control: Option<raw::DigitalLensControlALMALENCE>,
    pub bd_controller_interaction: Option<raw::ControllerInteractionBD>,
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
    pub ext_dpad_binding: Option<raw::DpadBindingEXT>,
    pub ext_hand_joints_motion_range: Option<raw::HandJointsMotionRangeEXT>,
    pub ext_samsung_odyssey_controller: Option<raw::SamsungOdysseyControllerEXT>,
    pub ext_hp_mixed_reality_controller: Option<raw::HpMixedRealityControllerEXT>,
    pub ext_palm_pose: Option<raw::PalmPoseEXT>,
    pub ext_uuid: Option<raw::UuidEXT>,
    pub ext_hand_interaction: Option<raw::HandInteractionEXT>,
    pub ext_active_action_set_priority: Option<raw::ActiveActionSetPriorityEXT>,
    pub ext_local_floor: Option<raw::LocalFloorEXT>,
    pub ext_hand_tracking_data_source: Option<raw::HandTrackingDataSourceEXT>,
    pub ext_plane_detection: Option<raw::PlaneDetectionEXT>,
    pub ext_future: Option<raw::FutureEXT>,
    pub ext_user_presence: Option<raw::UserPresenceEXT>,
    pub ext_composition_layer_inverted_alpha: Option<raw::CompositionLayerInvertedAlphaEXT>,
    pub fb_composition_layer_image_layout: Option<raw::CompositionLayerImageLayoutFB>,
    pub fb_composition_layer_alpha_blend: Option<raw::CompositionLayerAlphaBlendFB>,
    #[cfg(target_os = "android")]
    pub fb_android_surface_swapchain_create: Option<raw::AndroidSurfaceSwapchainCreateFB>,
    pub fb_swapchain_update_state: Option<raw::SwapchainUpdateStateFB>,
    pub fb_composition_layer_secure_content: Option<raw::CompositionLayerSecureContentFB>,
    pub fb_body_tracking: Option<raw::BodyTrackingFB>,
    pub fb_display_refresh_rate: Option<raw::DisplayRefreshRateFB>,
    pub fb_color_space: Option<raw::ColorSpaceFB>,
    pub fb_hand_tracking_mesh: Option<raw::HandTrackingMeshFB>,
    pub fb_hand_tracking_aim: Option<raw::HandTrackingAimFB>,
    pub fb_hand_tracking_capsules: Option<raw::HandTrackingCapsulesFB>,
    pub fb_spatial_entity: Option<raw::SpatialEntityFB>,
    pub fb_foveation: Option<raw::FoveationFB>,
    pub fb_foveation_configuration: Option<raw::FoveationConfigurationFB>,
    pub fb_keyboard_tracking: Option<raw::KeyboardTrackingFB>,
    pub fb_triangle_mesh: Option<raw::TriangleMeshFB>,
    pub fb_passthrough: Option<raw::PassthroughFB>,
    pub fb_render_model: Option<raw::RenderModelFB>,
    pub fb_spatial_entity_query: Option<raw::SpatialEntityQueryFB>,
    pub fb_spatial_entity_storage: Option<raw::SpatialEntityStorageFB>,
    pub fb_foveation_vulkan: Option<raw::FoveationVulkanFB>,
    #[cfg(target_os = "android")]
    pub fb_swapchain_update_state_android_surface:
        Option<raw::SwapchainUpdateStateAndroidSurfaceFB>,
    pub fb_swapchain_update_state_opengl_es: Option<raw::SwapchainUpdateStateOpenglEsFB>,
    pub fb_swapchain_update_state_vulkan: Option<raw::SwapchainUpdateStateVulkanFB>,
    pub fb_touch_controller_pro: Option<raw::TouchControllerProFB>,
    pub fb_spatial_entity_sharing: Option<raw::SpatialEntitySharingFB>,
    pub fb_space_warp: Option<raw::SpaceWarpFB>,
    pub fb_haptic_amplitude_envelope: Option<raw::HapticAmplitudeEnvelopeFB>,
    pub fb_scene: Option<raw::SceneFB>,
    pub fb_scene_capture: Option<raw::SceneCaptureFB>,
    pub fb_spatial_entity_container: Option<raw::SpatialEntityContainerFB>,
    pub fb_face_tracking: Option<raw::FaceTrackingFB>,
    pub fb_eye_tracking_social: Option<raw::EyeTrackingSocialFB>,
    pub fb_passthrough_keyboard_hands: Option<raw::PassthroughKeyboardHandsFB>,
    pub fb_composition_layer_settings: Option<raw::CompositionLayerSettingsFB>,
    pub fb_touch_controller_proximity: Option<raw::TouchControllerProximityFB>,
    pub fb_haptic_pcm: Option<raw::HapticPcmFB>,
    pub fb_composition_layer_depth_test: Option<raw::CompositionLayerDepthTestFB>,
    pub fb_spatial_entity_storage_batch: Option<raw::SpatialEntityStorageBatchFB>,
    pub fb_spatial_entity_user: Option<raw::SpatialEntityUserFB>,
    pub fb_face_tracking2: Option<raw::FaceTracking2FB>,
    pub htc_vive_cosmos_controller_interaction: Option<raw::ViveCosmosControllerInteractionHTC>,
    pub htc_facial_tracking: Option<raw::FacialTrackingHTC>,
    pub htc_vive_focus3_controller_interaction: Option<raw::ViveFocus3ControllerInteractionHTC>,
    pub htc_hand_interaction: Option<raw::HandInteractionHTC>,
    pub htc_vive_wrist_tracker_interaction: Option<raw::ViveWristTrackerInteractionHTC>,
    pub htc_passthrough: Option<raw::PassthroughHTC>,
    pub htc_foveation: Option<raw::FoveationHTC>,
    pub htc_anchor: Option<raw::AnchorHTC>,
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
    #[cfg(target_vendor = "apple")]
    pub khr_metal_enable: Option<raw::MetalEnableKHR>,
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
    pub khr_binding_modification: Option<raw::BindingModificationKHR>,
    pub khr_swapchain_usage_input_attachment_bit: Option<raw::SwapchainUsageInputAttachmentBitKHR>,
    pub khr_locate_spaces: Option<raw::LocateSpacesKHR>,
    pub khr_maintenance1: Option<raw::Maintenance1KHR>,
    pub meta_foveation_eye_tracked: Option<raw::FoveationEyeTrackedMETA>,
    pub meta_local_dimming: Option<raw::LocalDimmingMETA>,
    pub meta_passthrough_preferences: Option<raw::PassthroughPreferencesMETA>,
    pub meta_virtual_keyboard: Option<raw::VirtualKeyboardMETA>,
    pub meta_vulkan_swapchain_create_info: Option<raw::VulkanSwapchainCreateInfoMETA>,
    pub meta_performance_metrics: Option<raw::PerformanceMetricsMETA>,
    pub meta_headset_id: Option<raw::HeadsetIdMETA>,
    pub meta_recommended_layer_resolution: Option<raw::RecommendedLayerResolutionMETA>,
    pub meta_passthrough_color_lut: Option<raw::PassthroughColorLutMETA>,
    pub meta_spatial_entity_mesh: Option<raw::SpatialEntityMeshMETA>,
    pub meta_automatic_layer_filter: Option<raw::AutomaticLayerFilterMETA>,
    pub meta_touch_controller_plus: Option<raw::TouchControllerPlusMETA>,
    pub meta_environment_depth: Option<raw::EnvironmentDepthMETA>,
    pub ml_ml2_controller_interaction: Option<raw::Ml2ControllerInteractionML>,
    pub ml_frame_end_info: Option<raw::FrameEndInfoML>,
    pub ml_global_dimmer: Option<raw::GlobalDimmerML>,
    pub ml_compat: Option<raw::CompatML>,
    pub ml_marker_understanding: Option<raw::MarkerUnderstandingML>,
    pub ml_localization_map: Option<raw::LocalizationMapML>,
    pub ml_user_calibration: Option<raw::UserCalibrationML>,
    pub mnd_headless: Option<raw::HeadlessMND>,
    pub mnd_swapchain_usage_input_attachment_bit: Option<raw::SwapchainUsageInputAttachmentBitMND>,
    pub msft_unbounded_reference_space: Option<raw::UnboundedReferenceSpaceMSFT>,
    pub msft_spatial_anchor: Option<raw::SpatialAnchorMSFT>,
    pub msft_spatial_graph_bridge: Option<raw::SpatialGraphBridgeMSFT>,
    pub msft_hand_interaction: Option<raw::HandInteractionMSFT>,
    pub msft_hand_tracking_mesh: Option<raw::HandTrackingMeshMSFT>,
    pub msft_secondary_view_configuration: Option<raw::SecondaryViewConfigurationMSFT>,
    pub msft_first_person_observer: Option<raw::FirstPersonObserverMSFT>,
    pub msft_controller_model: Option<raw::ControllerModelMSFT>,
    #[cfg(windows)]
    pub msft_perception_anchor_interop: Option<raw::PerceptionAnchorInteropMSFT>,
    #[cfg(windows)]
    pub msft_holographic_window_attachment: Option<raw::HolographicWindowAttachmentMSFT>,
    pub msft_composition_layer_reprojection: Option<raw::CompositionLayerReprojectionMSFT>,
    pub msft_spatial_anchor_persistence: Option<raw::SpatialAnchorPersistenceMSFT>,
    #[cfg(target_os = "android")]
    pub oculus_android_session_state_enable: Option<raw::AndroidSessionStateEnableOCULUS>,
    pub oculus_audio_device_guid: Option<raw::AudioDeviceGuidOCULUS>,
    pub oculus_external_camera: Option<raw::ExternalCameraOCULUS>,
    pub oppo_controller_interaction: Option<raw::ControllerInteractionOPPO>,
    pub qcom_tracking_optimization_settings: Option<raw::TrackingOptimizationSettingsQCOM>,
    pub ultraleap_hand_tracking_forearm: Option<raw::HandTrackingForearmULTRALEAP>,
    pub valve_analog_threshold: Option<raw::AnalogThresholdVALVE>,
    pub varjo_quad_views: Option<raw::QuadViewsVARJO>,
    pub varjo_foveated_rendering: Option<raw::FoveatedRenderingVARJO>,
    pub varjo_composition_layer_depth_test: Option<raw::CompositionLayerDepthTestVARJO>,
    pub varjo_environment_depth_estimation: Option<raw::EnvironmentDepthEstimationVARJO>,
    pub varjo_marker_tracking: Option<raw::MarkerTrackingVARJO>,
    pub varjo_view_offset: Option<raw::ViewOffsetVARJO>,
    pub varjo_xr4_controller_interaction: Option<raw::Xr4ControllerInteractionVARJO>,
    pub yvr_controller_interaction: Option<raw::ControllerInteractionYVR>,
    pub extx_overlay: Option<raw::OverlayEXTX>,
    pub mndx_egl_enable: Option<raw::EglEnableMNDX>,
    pub mndx_force_feedback_curl: Option<raw::ForceFeedbackCurlMNDX>,
    pub htcx_vive_tracker_interaction: Option<raw::ViveTrackerInteractionHTCX>,
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
            almalence_digital_lens_control: if required.almalence_digital_lens_control {
                Some(raw::DigitalLensControlALMALENCE::load(entry, instance)?)
            } else {
                None
            },
            bd_controller_interaction: if required.bd_controller_interaction {
                Some(raw::ControllerInteractionBD {})
            } else {
                None
            },
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
            ext_dpad_binding: if required.ext_dpad_binding {
                Some(raw::DpadBindingEXT {})
            } else {
                None
            },
            ext_hand_joints_motion_range: if required.ext_hand_joints_motion_range {
                Some(raw::HandJointsMotionRangeEXT {})
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
            ext_palm_pose: if required.ext_palm_pose {
                Some(raw::PalmPoseEXT {})
            } else {
                None
            },
            ext_uuid: if required.ext_uuid {
                Some(raw::UuidEXT {})
            } else {
                None
            },
            ext_hand_interaction: if required.ext_hand_interaction {
                Some(raw::HandInteractionEXT {})
            } else {
                None
            },
            ext_active_action_set_priority: if required.ext_active_action_set_priority {
                Some(raw::ActiveActionSetPriorityEXT {})
            } else {
                None
            },
            ext_local_floor: if required.ext_local_floor {
                Some(raw::LocalFloorEXT {})
            } else {
                None
            },
            ext_hand_tracking_data_source: if required.ext_hand_tracking_data_source {
                Some(raw::HandTrackingDataSourceEXT {})
            } else {
                None
            },
            ext_plane_detection: if required.ext_plane_detection {
                Some(raw::PlaneDetectionEXT::load(entry, instance)?)
            } else {
                None
            },
            ext_future: if required.ext_future {
                Some(raw::FutureEXT::load(entry, instance)?)
            } else {
                None
            },
            ext_user_presence: if required.ext_user_presence {
                Some(raw::UserPresenceEXT {})
            } else {
                None
            },
            ext_composition_layer_inverted_alpha: if required.ext_composition_layer_inverted_alpha {
                Some(raw::CompositionLayerInvertedAlphaEXT {})
            } else {
                None
            },
            fb_composition_layer_image_layout: if required.fb_composition_layer_image_layout {
                Some(raw::CompositionLayerImageLayoutFB {})
            } else {
                None
            },
            fb_composition_layer_alpha_blend: if required.fb_composition_layer_alpha_blend {
                Some(raw::CompositionLayerAlphaBlendFB {})
            } else {
                None
            },
            #[cfg(target_os = "android")]
            fb_android_surface_swapchain_create: if required.fb_android_surface_swapchain_create {
                Some(raw::AndroidSurfaceSwapchainCreateFB {})
            } else {
                None
            },
            fb_swapchain_update_state: if required.fb_swapchain_update_state {
                Some(raw::SwapchainUpdateStateFB::load(entry, instance)?)
            } else {
                None
            },
            fb_composition_layer_secure_content: if required.fb_composition_layer_secure_content {
                Some(raw::CompositionLayerSecureContentFB {})
            } else {
                None
            },
            fb_body_tracking: if required.fb_body_tracking {
                Some(raw::BodyTrackingFB::load(entry, instance)?)
            } else {
                None
            },
            fb_display_refresh_rate: if required.fb_display_refresh_rate {
                Some(raw::DisplayRefreshRateFB::load(entry, instance)?)
            } else {
                None
            },
            fb_color_space: if required.fb_color_space {
                Some(raw::ColorSpaceFB::load(entry, instance)?)
            } else {
                None
            },
            fb_hand_tracking_mesh: if required.fb_hand_tracking_mesh {
                Some(raw::HandTrackingMeshFB::load(entry, instance)?)
            } else {
                None
            },
            fb_hand_tracking_aim: if required.fb_hand_tracking_aim {
                Some(raw::HandTrackingAimFB {})
            } else {
                None
            },
            fb_hand_tracking_capsules: if required.fb_hand_tracking_capsules {
                Some(raw::HandTrackingCapsulesFB {})
            } else {
                None
            },
            fb_spatial_entity: if required.fb_spatial_entity {
                Some(raw::SpatialEntityFB::load(entry, instance)?)
            } else {
                None
            },
            fb_foveation: if required.fb_foveation {
                Some(raw::FoveationFB::load(entry, instance)?)
            } else {
                None
            },
            fb_foveation_configuration: if required.fb_foveation_configuration {
                Some(raw::FoveationConfigurationFB {})
            } else {
                None
            },
            fb_keyboard_tracking: if required.fb_keyboard_tracking {
                Some(raw::KeyboardTrackingFB::load(entry, instance)?)
            } else {
                None
            },
            fb_triangle_mesh: if required.fb_triangle_mesh {
                Some(raw::TriangleMeshFB::load(entry, instance)?)
            } else {
                None
            },
            fb_passthrough: if required.fb_passthrough {
                Some(raw::PassthroughFB::load(entry, instance)?)
            } else {
                None
            },
            fb_render_model: if required.fb_render_model {
                Some(raw::RenderModelFB::load(entry, instance)?)
            } else {
                None
            },
            fb_spatial_entity_query: if required.fb_spatial_entity_query {
                Some(raw::SpatialEntityQueryFB::load(entry, instance)?)
            } else {
                None
            },
            fb_spatial_entity_storage: if required.fb_spatial_entity_storage {
                Some(raw::SpatialEntityStorageFB::load(entry, instance)?)
            } else {
                None
            },
            fb_foveation_vulkan: if required.fb_foveation_vulkan {
                Some(raw::FoveationVulkanFB {})
            } else {
                None
            },
            #[cfg(target_os = "android")]
            fb_swapchain_update_state_android_surface: if required
                .fb_swapchain_update_state_android_surface
            {
                Some(raw::SwapchainUpdateStateAndroidSurfaceFB {})
            } else {
                None
            },
            fb_swapchain_update_state_opengl_es: if required.fb_swapchain_update_state_opengl_es {
                Some(raw::SwapchainUpdateStateOpenglEsFB {})
            } else {
                None
            },
            fb_swapchain_update_state_vulkan: if required.fb_swapchain_update_state_vulkan {
                Some(raw::SwapchainUpdateStateVulkanFB {})
            } else {
                None
            },
            fb_touch_controller_pro: if required.fb_touch_controller_pro {
                Some(raw::TouchControllerProFB {})
            } else {
                None
            },
            fb_spatial_entity_sharing: if required.fb_spatial_entity_sharing {
                Some(raw::SpatialEntitySharingFB::load(entry, instance)?)
            } else {
                None
            },
            fb_space_warp: if required.fb_space_warp {
                Some(raw::SpaceWarpFB {})
            } else {
                None
            },
            fb_haptic_amplitude_envelope: if required.fb_haptic_amplitude_envelope {
                Some(raw::HapticAmplitudeEnvelopeFB {})
            } else {
                None
            },
            fb_scene: if required.fb_scene {
                Some(raw::SceneFB::load(entry, instance)?)
            } else {
                None
            },
            fb_scene_capture: if required.fb_scene_capture {
                Some(raw::SceneCaptureFB::load(entry, instance)?)
            } else {
                None
            },
            fb_spatial_entity_container: if required.fb_spatial_entity_container {
                Some(raw::SpatialEntityContainerFB::load(entry, instance)?)
            } else {
                None
            },
            fb_face_tracking: if required.fb_face_tracking {
                Some(raw::FaceTrackingFB::load(entry, instance)?)
            } else {
                None
            },
            fb_eye_tracking_social: if required.fb_eye_tracking_social {
                Some(raw::EyeTrackingSocialFB::load(entry, instance)?)
            } else {
                None
            },
            fb_passthrough_keyboard_hands: if required.fb_passthrough_keyboard_hands {
                Some(raw::PassthroughKeyboardHandsFB::load(entry, instance)?)
            } else {
                None
            },
            fb_composition_layer_settings: if required.fb_composition_layer_settings {
                Some(raw::CompositionLayerSettingsFB {})
            } else {
                None
            },
            fb_touch_controller_proximity: if required.fb_touch_controller_proximity {
                Some(raw::TouchControllerProximityFB {})
            } else {
                None
            },
            fb_haptic_pcm: if required.fb_haptic_pcm {
                Some(raw::HapticPcmFB::load(entry, instance)?)
            } else {
                None
            },
            fb_composition_layer_depth_test: if required.fb_composition_layer_depth_test {
                Some(raw::CompositionLayerDepthTestFB {})
            } else {
                None
            },
            fb_spatial_entity_storage_batch: if required.fb_spatial_entity_storage_batch {
                Some(raw::SpatialEntityStorageBatchFB::load(entry, instance)?)
            } else {
                None
            },
            fb_spatial_entity_user: if required.fb_spatial_entity_user {
                Some(raw::SpatialEntityUserFB::load(entry, instance)?)
            } else {
                None
            },
            fb_face_tracking2: if required.fb_face_tracking2 {
                Some(raw::FaceTracking2FB::load(entry, instance)?)
            } else {
                None
            },
            htc_vive_cosmos_controller_interaction: if required
                .htc_vive_cosmos_controller_interaction
            {
                Some(raw::ViveCosmosControllerInteractionHTC {})
            } else {
                None
            },
            htc_facial_tracking: if required.htc_facial_tracking {
                Some(raw::FacialTrackingHTC::load(entry, instance)?)
            } else {
                None
            },
            htc_vive_focus3_controller_interaction: if required
                .htc_vive_focus3_controller_interaction
            {
                Some(raw::ViveFocus3ControllerInteractionHTC {})
            } else {
                None
            },
            htc_hand_interaction: if required.htc_hand_interaction {
                Some(raw::HandInteractionHTC {})
            } else {
                None
            },
            htc_vive_wrist_tracker_interaction: if required.htc_vive_wrist_tracker_interaction {
                Some(raw::ViveWristTrackerInteractionHTC {})
            } else {
                None
            },
            htc_passthrough: if required.htc_passthrough {
                Some(raw::PassthroughHTC::load(entry, instance)?)
            } else {
                None
            },
            htc_foveation: if required.htc_foveation {
                Some(raw::FoveationHTC::load(entry, instance)?)
            } else {
                None
            },
            htc_anchor: if required.htc_anchor {
                Some(raw::AnchorHTC::load(entry, instance)?)
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
            #[cfg(target_vendor = "apple")]
            khr_metal_enable: if required.khr_metal_enable {
                Some(raw::MetalEnableKHR::load(entry, instance)?)
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
            khr_binding_modification: if required.khr_binding_modification {
                Some(raw::BindingModificationKHR {})
            } else {
                None
            },
            khr_swapchain_usage_input_attachment_bit: if required
                .khr_swapchain_usage_input_attachment_bit
            {
                Some(raw::SwapchainUsageInputAttachmentBitKHR {})
            } else {
                None
            },
            khr_locate_spaces: if required.khr_locate_spaces {
                Some(raw::LocateSpacesKHR::load(entry, instance)?)
            } else {
                None
            },
            khr_maintenance1: if required.khr_maintenance1 {
                Some(raw::Maintenance1KHR {})
            } else {
                None
            },
            meta_foveation_eye_tracked: if required.meta_foveation_eye_tracked {
                Some(raw::FoveationEyeTrackedMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_local_dimming: if required.meta_local_dimming {
                Some(raw::LocalDimmingMETA {})
            } else {
                None
            },
            meta_passthrough_preferences: if required.meta_passthrough_preferences {
                Some(raw::PassthroughPreferencesMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_virtual_keyboard: if required.meta_virtual_keyboard {
                Some(raw::VirtualKeyboardMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_vulkan_swapchain_create_info: if required.meta_vulkan_swapchain_create_info {
                Some(raw::VulkanSwapchainCreateInfoMETA {})
            } else {
                None
            },
            meta_performance_metrics: if required.meta_performance_metrics {
                Some(raw::PerformanceMetricsMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_headset_id: if required.meta_headset_id {
                Some(raw::HeadsetIdMETA {})
            } else {
                None
            },
            meta_recommended_layer_resolution: if required.meta_recommended_layer_resolution {
                Some(raw::RecommendedLayerResolutionMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_passthrough_color_lut: if required.meta_passthrough_color_lut {
                Some(raw::PassthroughColorLutMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_spatial_entity_mesh: if required.meta_spatial_entity_mesh {
                Some(raw::SpatialEntityMeshMETA::load(entry, instance)?)
            } else {
                None
            },
            meta_automatic_layer_filter: if required.meta_automatic_layer_filter {
                Some(raw::AutomaticLayerFilterMETA {})
            } else {
                None
            },
            meta_touch_controller_plus: if required.meta_touch_controller_plus {
                Some(raw::TouchControllerPlusMETA {})
            } else {
                None
            },
            meta_environment_depth: if required.meta_environment_depth {
                Some(raw::EnvironmentDepthMETA::load(entry, instance)?)
            } else {
                None
            },
            ml_ml2_controller_interaction: if required.ml_ml2_controller_interaction {
                Some(raw::Ml2ControllerInteractionML {})
            } else {
                None
            },
            ml_frame_end_info: if required.ml_frame_end_info {
                Some(raw::FrameEndInfoML {})
            } else {
                None
            },
            ml_global_dimmer: if required.ml_global_dimmer {
                Some(raw::GlobalDimmerML {})
            } else {
                None
            },
            ml_compat: if required.ml_compat {
                Some(raw::CompatML::load(entry, instance)?)
            } else {
                None
            },
            ml_marker_understanding: if required.ml_marker_understanding {
                Some(raw::MarkerUnderstandingML::load(entry, instance)?)
            } else {
                None
            },
            ml_localization_map: if required.ml_localization_map {
                Some(raw::LocalizationMapML::load(entry, instance)?)
            } else {
                None
            },
            ml_user_calibration: if required.ml_user_calibration {
                Some(raw::UserCalibrationML::load(entry, instance)?)
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
            msft_perception_anchor_interop: if required.msft_perception_anchor_interop {
                Some(raw::PerceptionAnchorInteropMSFT::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(windows)]
            msft_holographic_window_attachment: if required.msft_holographic_window_attachment {
                Some(raw::HolographicWindowAttachmentMSFT {})
            } else {
                None
            },
            msft_composition_layer_reprojection: if required.msft_composition_layer_reprojection {
                Some(raw::CompositionLayerReprojectionMSFT::load(
                    entry, instance,
                )?)
            } else {
                None
            },
            msft_spatial_anchor_persistence: if required.msft_spatial_anchor_persistence {
                Some(raw::SpatialAnchorPersistenceMSFT::load(entry, instance)?)
            } else {
                None
            },
            #[cfg(target_os = "android")]
            oculus_android_session_state_enable: if required.oculus_android_session_state_enable {
                Some(raw::AndroidSessionStateEnableOCULUS {})
            } else {
                None
            },
            oculus_audio_device_guid: if required.oculus_audio_device_guid {
                Some(raw::AudioDeviceGuidOCULUS::load(entry, instance)?)
            } else {
                None
            },
            oculus_external_camera: if required.oculus_external_camera {
                Some(raw::ExternalCameraOCULUS::load(entry, instance)?)
            } else {
                None
            },
            oppo_controller_interaction: if required.oppo_controller_interaction {
                Some(raw::ControllerInteractionOPPO {})
            } else {
                None
            },
            qcom_tracking_optimization_settings: if required.qcom_tracking_optimization_settings {
                Some(raw::TrackingOptimizationSettingsQCOM::load(
                    entry, instance,
                )?)
            } else {
                None
            },
            ultraleap_hand_tracking_forearm: if required.ultraleap_hand_tracking_forearm {
                Some(raw::HandTrackingForearmULTRALEAP {})
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
            varjo_foveated_rendering: if required.varjo_foveated_rendering {
                Some(raw::FoveatedRenderingVARJO {})
            } else {
                None
            },
            varjo_composition_layer_depth_test: if required.varjo_composition_layer_depth_test {
                Some(raw::CompositionLayerDepthTestVARJO {})
            } else {
                None
            },
            varjo_environment_depth_estimation: if required.varjo_environment_depth_estimation {
                Some(raw::EnvironmentDepthEstimationVARJO::load(entry, instance)?)
            } else {
                None
            },
            varjo_marker_tracking: if required.varjo_marker_tracking {
                Some(raw::MarkerTrackingVARJO::load(entry, instance)?)
            } else {
                None
            },
            varjo_view_offset: if required.varjo_view_offset {
                Some(raw::ViewOffsetVARJO::load(entry, instance)?)
            } else {
                None
            },
            varjo_xr4_controller_interaction: if required.varjo_xr4_controller_interaction {
                Some(raw::Xr4ControllerInteractionVARJO {})
            } else {
                None
            },
            yvr_controller_interaction: if required.yvr_controller_interaction {
                Some(raw::ControllerInteractionYVR {})
            } else {
                None
            },
            extx_overlay: if required.extx_overlay {
                Some(raw::OverlayEXTX {})
            } else {
                None
            },
            mndx_egl_enable: if required.mndx_egl_enable {
                Some(raw::EglEnableMNDX {})
            } else {
                None
            },
            mndx_force_feedback_curl: if required.mndx_force_feedback_curl {
                Some(raw::ForceFeedbackCurlMNDX::load(entry, instance)?)
            } else {
                None
            },
            htcx_vive_tracker_interaction: if required.htcx_vive_tracker_interaction {
                Some(raw::ViveTrackerInteractionHTCX::load(entry, instance)?)
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
    DisplayRefreshRateChangedFB(DisplayRefreshRateChangedFB<'a>),
    SpatialAnchorCreateCompleteFB(SpatialAnchorCreateCompleteFB<'a>),
    SpaceSetStatusCompleteFB(SpaceSetStatusCompleteFB<'a>),
    SpaceQueryResultsAvailableFB(SpaceQueryResultsAvailableFB<'a>),
    SpaceQueryCompleteFB(SpaceQueryCompleteFB<'a>),
    SpaceSaveCompleteFB(SpaceSaveCompleteFB<'a>),
    SpaceEraseCompleteFB(SpaceEraseCompleteFB<'a>),
    SpaceShareCompleteFB(SpaceShareCompleteFB<'a>),
    SpaceListSaveCompleteFB(SpaceListSaveCompleteFB<'a>),
    SceneCaptureCompleteFB(SceneCaptureCompleteFB<'a>),
    PassthroughStateChangedFB(PassthroughStateChangedFB<'a>),
    ViveTrackerConnectedHTCX(ViveTrackerConnectedHTCX<'a>),
    MarkerTrackingUpdateVARJO(MarkerTrackingUpdateVARJO<'a>),
    VirtualKeyboardCommitTextMETA(VirtualKeyboardCommitTextMETA<'a>),
    VirtualKeyboardBackspaceMETA(VirtualKeyboardBackspaceMETA<'a>),
    VirtualKeyboardEnterMETA(VirtualKeyboardEnterMETA<'a>),
    VirtualKeyboardShownMETA(VirtualKeyboardShownMETA<'a>),
    VirtualKeyboardHiddenMETA(VirtualKeyboardHiddenMETA<'a>),
    HeadsetFitChangedML(HeadsetFitChangedML<'a>),
    EyeCalibrationChangedML(EyeCalibrationChangedML<'a>),
    LocalizationChangedML(LocalizationChangedML<'a>),
    UserPresenceChangedEXT(UserPresenceChangedEXT<'a>),
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
            sys::StructureType::EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB => {
                let typed = &*(raw as *const sys::EventDataDisplayRefreshRateChangedFB);
                Event::DisplayRefreshRateChangedFB(DisplayRefreshRateChangedFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpatialAnchorCreateCompleteFB);
                Event::SpatialAnchorCreateCompleteFB(SpatialAnchorCreateCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceSetStatusCompleteFB);
                Event::SpaceSetStatusCompleteFB(SpaceSetStatusCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceQueryResultsAvailableFB);
                Event::SpaceQueryResultsAvailableFB(SpaceQueryResultsAvailableFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_QUERY_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceQueryCompleteFB);
                Event::SpaceQueryCompleteFB(SpaceQueryCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_SAVE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceSaveCompleteFB);
                Event::SpaceSaveCompleteFB(SpaceSaveCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_ERASE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceEraseCompleteFB);
                Event::SpaceEraseCompleteFB(SpaceEraseCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_SHARE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceShareCompleteFB);
                Event::SpaceShareCompleteFB(SpaceShareCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SPACE_LIST_SAVE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSpaceListSaveCompleteFB);
                Event::SpaceListSaveCompleteFB(SpaceListSaveCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_SCENE_CAPTURE_COMPLETE_FB => {
                let typed = &*(raw as *const sys::EventDataSceneCaptureCompleteFB);
                Event::SceneCaptureCompleteFB(SceneCaptureCompleteFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB => {
                let typed = &*(raw as *const sys::EventDataPassthroughStateChangedFB);
                Event::PassthroughStateChangedFB(PassthroughStateChangedFB::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX => {
                let typed = &*(raw as *const sys::EventDataViveTrackerConnectedHTCX);
                Event::ViveTrackerConnectedHTCX(ViveTrackerConnectedHTCX::new(typed))
            }
            sys::StructureType::EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO => {
                let typed = &*(raw as *const sys::EventDataMarkerTrackingUpdateVARJO);
                Event::MarkerTrackingUpdateVARJO(MarkerTrackingUpdateVARJO::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_COMMIT_TEXT_META => {
                let typed = &*(raw as *const sys::EventDataVirtualKeyboardCommitTextMETA);
                Event::VirtualKeyboardCommitTextMETA(VirtualKeyboardCommitTextMETA::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_BACKSPACE_META => {
                let typed = &*(raw as *const sys::EventDataVirtualKeyboardBackspaceMETA);
                Event::VirtualKeyboardBackspaceMETA(VirtualKeyboardBackspaceMETA::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_ENTER_META => {
                let typed = &*(raw as *const sys::EventDataVirtualKeyboardEnterMETA);
                Event::VirtualKeyboardEnterMETA(VirtualKeyboardEnterMETA::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_SHOWN_META => {
                let typed = &*(raw as *const sys::EventDataVirtualKeyboardShownMETA);
                Event::VirtualKeyboardShownMETA(VirtualKeyboardShownMETA::new(typed))
            }
            sys::StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_HIDDEN_META => {
                let typed = &*(raw as *const sys::EventDataVirtualKeyboardHiddenMETA);
                Event::VirtualKeyboardHiddenMETA(VirtualKeyboardHiddenMETA::new(typed))
            }
            sys::StructureType::EVENT_DATA_HEADSET_FIT_CHANGED_ML => {
                let typed = &*(raw as *const sys::EventDataHeadsetFitChangedML);
                Event::HeadsetFitChangedML(HeadsetFitChangedML::new(typed))
            }
            sys::StructureType::EVENT_DATA_EYE_CALIBRATION_CHANGED_ML => {
                let typed = &*(raw as *const sys::EventDataEyeCalibrationChangedML);
                Event::EyeCalibrationChangedML(EyeCalibrationChangedML::new(typed))
            }
            sys::StructureType::EVENT_DATA_LOCALIZATION_CHANGED_ML => {
                let typed = &*(raw as *const sys::EventDataLocalizationChangedML);
                Event::LocalizationChangedML(LocalizationChangedML::new(typed))
            }
            sys::StructureType::EVENT_DATA_USER_PRESENCE_CHANGED_EXT => {
                let typed = &*(raw as *const sys::EventDataUserPresenceChangedEXT);
                Event::UserPresenceChangedEXT(UserPresenceChangedEXT::new(typed))
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataEventsLost]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataEventsLost) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataInstanceLossPending]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataInstanceLossPending) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSessionStateChanged]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSessionStateChanged) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataReferenceSpaceChangePending]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataReferenceSpaceChangePending) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataPerfSettingsEXT]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataPerfSettingsEXT) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVisibilityMaskChangedKHR]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVisibilityMaskChangedKHR) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataInteractionProfileChanged]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataInteractionProfileChanged) -> Self {
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
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataMainSessionVisibilityChangedEXTX]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataMainSessionVisibilityChangedEXTX) -> Self {
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
#[derive(Copy, Clone)]
pub struct DisplayRefreshRateChangedFB<'a>(&'a sys::EventDataDisplayRefreshRateChangedFB);
impl<'a> DisplayRefreshRateChangedFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataDisplayRefreshRateChangedFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataDisplayRefreshRateChangedFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn from_display_refresh_rate(self) -> f32 {
        (self.0).from_display_refresh_rate
    }
    #[inline]
    pub fn to_display_refresh_rate(self) -> f32 {
        (self.0).to_display_refresh_rate
    }
}
#[derive(Copy, Clone)]
pub struct SpatialAnchorCreateCompleteFB<'a>(&'a sys::EventDataSpatialAnchorCreateCompleteFB);
impl<'a> SpatialAnchorCreateCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpatialAnchorCreateCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpatialAnchorCreateCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
    #[inline]
    pub fn space(self) -> sys::Space {
        (self.0).space
    }
    #[inline]
    pub fn uuid(self) -> UuidEXT {
        (self.0).uuid
    }
}
#[derive(Copy, Clone)]
pub struct SpaceSetStatusCompleteFB<'a>(&'a sys::EventDataSpaceSetStatusCompleteFB);
impl<'a> SpaceSetStatusCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceSetStatusCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceSetStatusCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
    #[inline]
    pub fn space(self) -> sys::Space {
        (self.0).space
    }
    #[inline]
    pub fn uuid(self) -> UuidEXT {
        (self.0).uuid
    }
    #[inline]
    pub fn component_type(self) -> SpaceComponentTypeFB {
        (self.0).component_type
    }
    #[inline]
    pub fn enabled(self) -> bool {
        (self.0).enabled.into()
    }
}
#[derive(Copy, Clone)]
pub struct SpaceQueryResultsAvailableFB<'a>(&'a sys::EventDataSpaceQueryResultsAvailableFB);
impl<'a> SpaceQueryResultsAvailableFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceQueryResultsAvailableFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceQueryResultsAvailableFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
}
#[derive(Copy, Clone)]
pub struct SpaceQueryCompleteFB<'a>(&'a sys::EventDataSpaceQueryCompleteFB);
impl<'a> SpaceQueryCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceQueryCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceQueryCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct SpaceSaveCompleteFB<'a>(&'a sys::EventDataSpaceSaveCompleteFB);
impl<'a> SpaceSaveCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceSaveCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceSaveCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
    #[inline]
    pub fn space(self) -> sys::Space {
        (self.0).space
    }
    #[inline]
    pub fn uuid(self) -> UuidEXT {
        (self.0).uuid
    }
    #[inline]
    pub fn location(self) -> SpaceStorageLocationFB {
        (self.0).location
    }
}
#[derive(Copy, Clone)]
pub struct SpaceEraseCompleteFB<'a>(&'a sys::EventDataSpaceEraseCompleteFB);
impl<'a> SpaceEraseCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceEraseCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceEraseCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
    #[inline]
    pub fn space(self) -> sys::Space {
        (self.0).space
    }
    #[inline]
    pub fn uuid(self) -> UuidEXT {
        (self.0).uuid
    }
    #[inline]
    pub fn location(self) -> SpaceStorageLocationFB {
        (self.0).location
    }
}
#[derive(Copy, Clone)]
pub struct SpaceShareCompleteFB<'a>(&'a sys::EventDataSpaceShareCompleteFB);
impl<'a> SpaceShareCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceShareCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceShareCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct SpaceListSaveCompleteFB<'a>(&'a sys::EventDataSpaceListSaveCompleteFB);
impl<'a> SpaceListSaveCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceListSaveCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceListSaveCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct SceneCaptureCompleteFB<'a>(&'a sys::EventDataSceneCaptureCompleteFB);
impl<'a> SceneCaptureCompleteFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSceneCaptureCompleteFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSceneCaptureCompleteFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct PassthroughStateChangedFB<'a>(&'a sys::EventDataPassthroughStateChangedFB);
impl<'a> PassthroughStateChangedFB<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataPassthroughStateChangedFB]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataPassthroughStateChangedFB) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn flags(self) -> PassthroughStateChangedFlagsFB {
        (self.0).flags
    }
}
#[derive(Copy, Clone)]
pub struct ViveTrackerConnectedHTCX<'a>(&'a sys::EventDataViveTrackerConnectedHTCX);
impl<'a> ViveTrackerConnectedHTCX<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataViveTrackerConnectedHTCX]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataViveTrackerConnectedHTCX) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn paths(self) -> ViveTrackerPathsHTCX {
        (*unsafe { self.0.paths.as_ref() }.unwrap()).into()
    }
}
#[derive(Copy, Clone)]
pub struct MarkerTrackingUpdateVARJO<'a>(&'a sys::EventDataMarkerTrackingUpdateVARJO);
impl<'a> MarkerTrackingUpdateVARJO<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataMarkerTrackingUpdateVARJO]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataMarkerTrackingUpdateVARJO) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn marker_id(self) -> u64 {
        (self.0).marker_id
    }
    #[inline]
    pub fn is_active(self) -> bool {
        (self.0).is_active.into()
    }
    #[inline]
    pub fn is_predicted(self) -> bool {
        (self.0).is_predicted.into()
    }
    #[inline]
    pub fn time(self) -> Time {
        (self.0).time
    }
}
#[derive(Copy, Clone)]
pub struct VirtualKeyboardCommitTextMETA<'a>(&'a sys::EventDataVirtualKeyboardCommitTextMETA);
impl<'a> VirtualKeyboardCommitTextMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVirtualKeyboardCommitTextMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVirtualKeyboardCommitTextMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn keyboard(self) -> sys::VirtualKeyboardMETA {
        (self.0).keyboard
    }
    #[inline]
    pub fn text(self) -> [c_char; MAX_VIRTUAL_KEYBOARD_COMMIT_TEXT_SIZE_META] {
        (self.0).text
    }
}
#[derive(Copy, Clone)]
pub struct VirtualKeyboardBackspaceMETA<'a>(&'a sys::EventDataVirtualKeyboardBackspaceMETA);
impl<'a> VirtualKeyboardBackspaceMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVirtualKeyboardBackspaceMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVirtualKeyboardBackspaceMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn keyboard(self) -> sys::VirtualKeyboardMETA {
        (self.0).keyboard
    }
}
#[derive(Copy, Clone)]
pub struct VirtualKeyboardEnterMETA<'a>(&'a sys::EventDataVirtualKeyboardEnterMETA);
impl<'a> VirtualKeyboardEnterMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVirtualKeyboardEnterMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVirtualKeyboardEnterMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn keyboard(self) -> sys::VirtualKeyboardMETA {
        (self.0).keyboard
    }
}
#[derive(Copy, Clone)]
pub struct VirtualKeyboardShownMETA<'a>(&'a sys::EventDataVirtualKeyboardShownMETA);
impl<'a> VirtualKeyboardShownMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVirtualKeyboardShownMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVirtualKeyboardShownMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn keyboard(self) -> sys::VirtualKeyboardMETA {
        (self.0).keyboard
    }
}
#[derive(Copy, Clone)]
pub struct VirtualKeyboardHiddenMETA<'a>(&'a sys::EventDataVirtualKeyboardHiddenMETA);
impl<'a> VirtualKeyboardHiddenMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataVirtualKeyboardHiddenMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataVirtualKeyboardHiddenMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn keyboard(self) -> sys::VirtualKeyboardMETA {
        (self.0).keyboard
    }
}
#[derive(Copy, Clone)]
pub struct HeadsetFitChangedML<'a>(&'a sys::EventDataHeadsetFitChangedML);
impl<'a> HeadsetFitChangedML<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataHeadsetFitChangedML]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataHeadsetFitChangedML) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn status(self) -> HeadsetFitStatusML {
        (self.0).status
    }
    #[inline]
    pub fn time(self) -> Time {
        (self.0).time
    }
}
#[derive(Copy, Clone)]
pub struct EyeCalibrationChangedML<'a>(&'a sys::EventDataEyeCalibrationChangedML);
impl<'a> EyeCalibrationChangedML<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataEyeCalibrationChangedML]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataEyeCalibrationChangedML) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn status(self) -> EyeCalibrationStatusML {
        (self.0).status
    }
}
#[derive(Copy, Clone)]
pub struct LocalizationChangedML<'a>(&'a sys::EventDataLocalizationChangedML);
impl<'a> LocalizationChangedML<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataLocalizationChangedML]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataLocalizationChangedML) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn state(self) -> LocalizationMapStateML {
        (self.0).state
    }
    #[inline]
    pub fn map(self) -> LocalizationMapML {
        LocalizationMapML::from_raw(self.0.map)
    }
    #[inline]
    pub fn confidence(self) -> LocalizationMapConfidenceML {
        (self.0).confidence
    }
    #[inline]
    pub fn error_flags(self) -> LocalizationMapErrorFlagsML {
        (self.0).error_flags
    }
}
#[derive(Copy, Clone)]
pub struct UserPresenceChangedEXT<'a>(&'a sys::EventDataUserPresenceChangedEXT);
impl<'a> UserPresenceChangedEXT<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataUserPresenceChangedEXT]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataUserPresenceChangedEXT) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn session(self) -> sys::Session {
        (self.0).session
    }
    #[inline]
    pub fn is_user_present(self) -> bool {
        (self.0).is_user_present.into()
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
    pub struct DigitalLensControlALMALENCE {
        pub set_digital_lens_control: pfn::SetDigitalLensControlALMALENCE,
    }
    impl DigitalLensControlALMALENCE {
        pub const VERSION: u32 = sys::ALMALENCE_digital_lens_control_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ALMALENCE_DIGITAL_LENS_CONTROL_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ALMALENCE_DIGITAL_LENS_CONTROL_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_digital_lens_control: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetDigitalLensControlALMALENCE\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionBD {}
    impl ControllerInteractionBD {
        pub const VERSION: u32 = sys::BD_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::BD_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ViewConfigurationFovEPIC {}
    impl ViewConfigurationFovEPIC {
        pub const VERSION: u32 = sys::EPIC_view_configuration_fov_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PerformanceSettingsEXT {
        pub perf_settings_set_performance_level: pfn::PerfSettingsSetPerformanceLevelEXT,
    }
    impl PerformanceSettingsEXT {
        pub const VERSION: u32 = sys::EXT_performance_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::EXT_THERMAL_QUERY_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::EXT_DEBUG_UTILS_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::EXT_EYE_GAZE_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ViewConfigurationDepthRangeEXT {}
    impl ViewConfigurationDepthRangeEXT {
        pub const VERSION: u32 = sys::EXT_view_configuration_depth_range_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::EXT_CONFORMANCE_AUTOMATION_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::EXT_HAND_TRACKING_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str =
            sys::EXT_WIN32_APPCONTAINER_COMPATIBLE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct DpadBindingEXT {}
    impl DpadBindingEXT {
        pub const VERSION: u32 = sys::EXT_dpad_binding_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_DPAD_BINDING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_DPAD_BINDING_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandJointsMotionRangeEXT {}
    impl HandJointsMotionRangeEXT {
        pub const VERSION: u32 = sys::EXT_hand_joints_motion_range_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_JOINTS_MOTION_RANGE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_HAND_JOINTS_MOTION_RANGE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SamsungOdysseyControllerEXT {}
    impl SamsungOdysseyControllerEXT {
        pub const VERSION: u32 = sys::EXT_samsung_odyssey_controller_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HpMixedRealityControllerEXT {}
    impl HpMixedRealityControllerEXT {
        pub const VERSION: u32 = sys::EXT_hp_mixed_reality_controller_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PalmPoseEXT {}
    impl PalmPoseEXT {
        pub const VERSION: u32 = sys::EXT_palm_pose_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PALM_POSE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_PALM_POSE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct UuidEXT {}
    impl UuidEXT {
        pub const VERSION: u32 = sys::EXT_uuid_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_UUID_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_UUID_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionEXT {}
    impl HandInteractionEXT {
        pub const VERSION: u32 = sys::EXT_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_HAND_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ActiveActionSetPriorityEXT {}
    impl ActiveActionSetPriorityEXT {
        pub const VERSION: u32 = sys::EXT_active_action_set_priority_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_ACTIVE_ACTION_SET_PRIORITY_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_ACTIVE_ACTION_SET_PRIORITY_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct LocalFloorEXT {}
    impl LocalFloorEXT {
        pub const VERSION: u32 = sys::EXT_local_floor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_LOCAL_FLOOR_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_LOCAL_FLOOR_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingDataSourceEXT {}
    impl HandTrackingDataSourceEXT {
        pub const VERSION: u32 = sys::EXT_hand_tracking_data_source_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_TRACKING_DATA_SOURCE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_HAND_TRACKING_DATA_SOURCE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PlaneDetectionEXT {
        pub create_plane_detector: pfn::CreatePlaneDetectorEXT,
        pub destroy_plane_detector: pfn::DestroyPlaneDetectorEXT,
        pub begin_plane_detection: pfn::BeginPlaneDetectionEXT,
        pub get_plane_detection_state: pfn::GetPlaneDetectionStateEXT,
        pub get_plane_detections: pfn::GetPlaneDetectionsEXT,
        pub get_plane_polygon_buffer: pfn::GetPlanePolygonBufferEXT,
    }
    impl PlaneDetectionEXT {
        pub const VERSION: u32 = sys::EXT_plane_detection_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PLANE_DETECTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_PLANE_DETECTION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_plane_detector: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreatePlaneDetectorEXT\0"),
                )?),
                destroy_plane_detector: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyPlaneDetectorEXT\0"),
                )?),
                begin_plane_detection: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrBeginPlaneDetectionEXT\0"),
                )?),
                get_plane_detection_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetPlaneDetectionStateEXT\0"),
                )?),
                get_plane_detections: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetPlaneDetectionsEXT\0"),
                )?),
                get_plane_polygon_buffer: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetPlanePolygonBufferEXT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FutureEXT {
        pub poll_future: pfn::PollFutureEXT,
        pub cancel_future: pfn::CancelFutureEXT,
    }
    impl FutureEXT {
        pub const VERSION: u32 = sys::EXT_future_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_FUTURE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_FUTURE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                poll_future: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPollFutureEXT\0"),
                )?),
                cancel_future: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCancelFutureEXT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct UserPresenceEXT {}
    impl UserPresenceEXT {
        pub const VERSION: u32 = sys::EXT_user_presence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_USER_PRESENCE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXT_USER_PRESENCE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerInvertedAlphaEXT {}
    impl CompositionLayerInvertedAlphaEXT {
        pub const VERSION: u32 = sys::EXT_composition_layer_inverted_alpha_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_COMPOSITION_LAYER_INVERTED_ALPHA_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::EXT_COMPOSITION_LAYER_INVERTED_ALPHA_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerImageLayoutFB {}
    impl CompositionLayerImageLayoutFB {
        pub const VERSION: u32 = sys::FB_composition_layer_image_layout_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_IMAGE_LAYOUT_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::FB_COMPOSITION_LAYER_IMAGE_LAYOUT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerAlphaBlendFB {}
    impl CompositionLayerAlphaBlendFB {
        pub const VERSION: u32 = sys::FB_composition_layer_alpha_blend_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_ALPHA_BLEND_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_COMPOSITION_LAYER_ALPHA_BLEND_EXTENSION_NAME_STR;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidSurfaceSwapchainCreateFB {}
    #[cfg(target_os = "android")]
    impl AndroidSurfaceSwapchainCreateFB {
        pub const VERSION: u32 = sys::FB_android_surface_swapchain_create_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_ANDROID_SURFACE_SWAPCHAIN_CREATE_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::FB_ANDROID_SURFACE_SWAPCHAIN_CREATE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateFB {
        pub update_swapchain: pfn::UpdateSwapchainFB,
        pub get_swapchain_state: pfn::GetSwapchainStateFB,
    }
    impl SwapchainUpdateStateFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SWAPCHAIN_UPDATE_STATE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                update_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrUpdateSwapchainFB\0"),
                )?),
                get_swapchain_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSwapchainStateFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerSecureContentFB {}
    impl CompositionLayerSecureContentFB {
        pub const VERSION: u32 = sys::FB_composition_layer_secure_content_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_SECURE_CONTENT_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::FB_COMPOSITION_LAYER_SECURE_CONTENT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct BodyTrackingFB {
        pub create_body_tracker: pfn::CreateBodyTrackerFB,
        pub destroy_body_tracker: pfn::DestroyBodyTrackerFB,
        pub locate_body_joints: pfn::LocateBodyJointsFB,
        pub get_body_skeleton: pfn::GetBodySkeletonFB,
    }
    impl BodyTrackingFB {
        pub const VERSION: u32 = sys::FB_body_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_BODY_TRACKING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_BODY_TRACKING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_body_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateBodyTrackerFB\0"),
                )?),
                destroy_body_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyBodyTrackerFB\0"),
                )?),
                locate_body_joints: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateBodyJointsFB\0"),
                )?),
                get_body_skeleton: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetBodySkeletonFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct DisplayRefreshRateFB {
        pub enumerate_display_refresh_rates: pfn::EnumerateDisplayRefreshRatesFB,
        pub get_display_refresh_rate: pfn::GetDisplayRefreshRateFB,
        pub request_display_refresh_rate: pfn::RequestDisplayRefreshRateFB,
    }
    impl DisplayRefreshRateFB {
        pub const VERSION: u32 = sys::FB_display_refresh_rate_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_DISPLAY_REFRESH_RATE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_DISPLAY_REFRESH_RATE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_display_refresh_rates: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateDisplayRefreshRatesFB\0"),
                )?),
                get_display_refresh_rate: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetDisplayRefreshRateFB\0"),
                )?),
                request_display_refresh_rate: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrRequestDisplayRefreshRateFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ColorSpaceFB {
        pub enumerate_color_spaces: pfn::EnumerateColorSpacesFB,
        pub set_color_space: pfn::SetColorSpaceFB,
    }
    impl ColorSpaceFB {
        pub const VERSION: u32 = sys::FB_color_space_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COLOR_SPACE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_COLOR_SPACE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_color_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateColorSpacesFB\0"),
                )?),
                set_color_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetColorSpaceFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingMeshFB {
        pub get_hand_mesh: pfn::GetHandMeshFB,
    }
    impl HandTrackingMeshFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_MESH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_HAND_TRACKING_MESH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_hand_mesh: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetHandMeshFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingAimFB {}
    impl HandTrackingAimFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_aim_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_AIM_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_HAND_TRACKING_AIM_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingCapsulesFB {}
    impl HandTrackingCapsulesFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_capsules_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_CAPSULES_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_HAND_TRACKING_CAPSULES_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityFB {
        pub create_spatial_anchor: pfn::CreateSpatialAnchorFB,
        pub get_space_uuid: pfn::GetSpaceUuidFB,
        pub enumerate_space_supported_components: pfn::EnumerateSpaceSupportedComponentsFB,
        pub set_space_component_status: pfn::SetSpaceComponentStatusFB,
        pub get_space_component_status: pfn::GetSpaceComponentStatusFB,
    }
    impl SpatialEntityFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpatialAnchorFB\0"),
                )?),
                get_space_uuid: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceUuidFB\0"),
                )?),
                enumerate_space_supported_components: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumerateSpaceSupportedComponentsFB\0",
                        ),
                    )?,
                ),
                set_space_component_status: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetSpaceComponentStatusFB\0"),
                )?),
                get_space_component_status: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceComponentStatusFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationFB {
        pub create_foveation_profile: pfn::CreateFoveationProfileFB,
        pub destroy_foveation_profile: pfn::DestroyFoveationProfileFB,
    }
    impl FoveationFB {
        pub const VERSION: u32 = sys::FB_foveation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FOVEATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_FOVEATION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_foveation_profile: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateFoveationProfileFB\0"),
                )?),
                destroy_foveation_profile: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyFoveationProfileFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationConfigurationFB {}
    impl FoveationConfigurationFB {
        pub const VERSION: u32 = sys::FB_foveation_configuration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FOVEATION_CONFIGURATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_FOVEATION_CONFIGURATION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct KeyboardTrackingFB {
        pub query_system_tracked_keyboard: pfn::QuerySystemTrackedKeyboardFB,
        pub create_keyboard_space: pfn::CreateKeyboardSpaceFB,
    }
    impl KeyboardTrackingFB {
        pub const VERSION: u32 = sys::FB_keyboard_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_KEYBOARD_TRACKING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_KEYBOARD_TRACKING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                query_system_tracked_keyboard: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrQuerySystemTrackedKeyboardFB\0"),
                )?),
                create_keyboard_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateKeyboardSpaceFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct TriangleMeshFB {
        pub create_triangle_mesh: pfn::CreateTriangleMeshFB,
        pub destroy_triangle_mesh: pfn::DestroyTriangleMeshFB,
        pub triangle_mesh_get_vertex_buffer: pfn::TriangleMeshGetVertexBufferFB,
        pub triangle_mesh_get_index_buffer: pfn::TriangleMeshGetIndexBufferFB,
        pub triangle_mesh_begin_update: pfn::TriangleMeshBeginUpdateFB,
        pub triangle_mesh_end_update: pfn::TriangleMeshEndUpdateFB,
        pub triangle_mesh_begin_vertex_buffer_update: pfn::TriangleMeshBeginVertexBufferUpdateFB,
        pub triangle_mesh_end_vertex_buffer_update: pfn::TriangleMeshEndVertexBufferUpdateFB,
    }
    impl TriangleMeshFB {
        pub const VERSION: u32 = sys::FB_triangle_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_TRIANGLE_MESH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_TRIANGLE_MESH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_triangle_mesh: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateTriangleMeshFB\0"),
                )?),
                destroy_triangle_mesh: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyTriangleMeshFB\0"),
                )?),
                triangle_mesh_get_vertex_buffer: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrTriangleMeshGetVertexBufferFB\0"),
                )?),
                triangle_mesh_get_index_buffer: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrTriangleMeshGetIndexBufferFB\0"),
                )?),
                triangle_mesh_begin_update: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrTriangleMeshBeginUpdateFB\0"),
                )?),
                triangle_mesh_end_update: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrTriangleMeshEndUpdateFB\0"),
                )?),
                triangle_mesh_begin_vertex_buffer_update: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrTriangleMeshBeginVertexBufferUpdateFB\0",
                        ),
                    )?,
                ),
                triangle_mesh_end_vertex_buffer_update: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrTriangleMeshEndVertexBufferUpdateFB\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughFB {
        pub create_passthrough: pfn::CreatePassthroughFB,
        pub destroy_passthrough: pfn::DestroyPassthroughFB,
        pub passthrough_start: pfn::PassthroughStartFB,
        pub passthrough_pause: pfn::PassthroughPauseFB,
        pub create_passthrough_layer: pfn::CreatePassthroughLayerFB,
        pub destroy_passthrough_layer: pfn::DestroyPassthroughLayerFB,
        pub passthrough_layer_pause: pfn::PassthroughLayerPauseFB,
        pub passthrough_layer_resume: pfn::PassthroughLayerResumeFB,
        pub passthrough_layer_set_style: pfn::PassthroughLayerSetStyleFB,
        pub create_geometry_instance: pfn::CreateGeometryInstanceFB,
        pub destroy_geometry_instance: pfn::DestroyGeometryInstanceFB,
        pub geometry_instance_set_transform: pfn::GeometryInstanceSetTransformFB,
    }
    impl PassthroughFB {
        pub const VERSION: u32 = sys::FB_passthrough_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_PASSTHROUGH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_PASSTHROUGH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_passthrough: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreatePassthroughFB\0"),
                )?),
                destroy_passthrough: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyPassthroughFB\0"),
                )?),
                passthrough_start: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPassthroughStartFB\0"),
                )?),
                passthrough_pause: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPassthroughPauseFB\0"),
                )?),
                create_passthrough_layer: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreatePassthroughLayerFB\0"),
                )?),
                destroy_passthrough_layer: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyPassthroughLayerFB\0"),
                )?),
                passthrough_layer_pause: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPassthroughLayerPauseFB\0"),
                )?),
                passthrough_layer_resume: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPassthroughLayerResumeFB\0"),
                )?),
                passthrough_layer_set_style: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPassthroughLayerSetStyleFB\0"),
                )?),
                create_geometry_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateGeometryInstanceFB\0"),
                )?),
                destroy_geometry_instance: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyGeometryInstanceFB\0"),
                )?),
                geometry_instance_set_transform: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGeometryInstanceSetTransformFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct RenderModelFB {
        pub enumerate_render_model_paths: pfn::EnumerateRenderModelPathsFB,
        pub get_render_model_properties: pfn::GetRenderModelPropertiesFB,
        pub load_render_model: pfn::LoadRenderModelFB,
    }
    impl RenderModelFB {
        pub const VERSION: u32 = sys::FB_render_model_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_RENDER_MODEL_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_RENDER_MODEL_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_render_model_paths: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateRenderModelPathsFB\0"),
                )?),
                get_render_model_properties: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetRenderModelPropertiesFB\0"),
                )?),
                load_render_model: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLoadRenderModelFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityQueryFB {
        pub query_spaces: pfn::QuerySpacesFB,
        pub retrieve_space_query_results: pfn::RetrieveSpaceQueryResultsFB,
    }
    impl SpatialEntityQueryFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_query_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_QUERY_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_QUERY_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                query_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrQuerySpacesFB\0"),
                )?),
                retrieve_space_query_results: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrRetrieveSpaceQueryResultsFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityStorageFB {
        pub save_space: pfn::SaveSpaceFB,
        pub erase_space: pfn::EraseSpaceFB,
    }
    impl SpatialEntityStorageFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_storage_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_STORAGE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_STORAGE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                save_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSaveSpaceFB\0"),
                )?),
                erase_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEraseSpaceFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationVulkanFB {}
    impl FoveationVulkanFB {
        pub const VERSION: u32 = sys::FB_foveation_vulkan_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FOVEATION_VULKAN_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_FOVEATION_VULKAN_EXTENSION_NAME_STR;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateAndroidSurfaceFB {}
    #[cfg(target_os = "android")]
    impl SwapchainUpdateStateAndroidSurfaceFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_android_surface_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::FB_SWAPCHAIN_UPDATE_STATE_ANDROID_SURFACE_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::FB_SWAPCHAIN_UPDATE_STATE_ANDROID_SURFACE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateOpenglEsFB {}
    impl SwapchainUpdateStateOpenglEsFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_opengl_es_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_OPENGL_ES_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::FB_SWAPCHAIN_UPDATE_STATE_OPENGL_ES_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateVulkanFB {}
    impl SwapchainUpdateStateVulkanFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_vulkan_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_VULKAN_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SWAPCHAIN_UPDATE_STATE_VULKAN_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerProFB {}
    impl TouchControllerProFB {
        pub const VERSION: u32 = sys::FB_touch_controller_pro_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_TOUCH_CONTROLLER_PRO_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_TOUCH_CONTROLLER_PRO_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntitySharingFB {
        pub share_spaces: pfn::ShareSpacesFB,
    }
    impl SpatialEntitySharingFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_sharing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_SHARING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_SHARING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                share_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrShareSpacesFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpaceWarpFB {}
    impl SpaceWarpFB {
        pub const VERSION: u32 = sys::FB_space_warp_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPACE_WARP_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPACE_WARP_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HapticAmplitudeEnvelopeFB {}
    impl HapticAmplitudeEnvelopeFB {
        pub const VERSION: u32 = sys::FB_haptic_amplitude_envelope_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAPTIC_AMPLITUDE_ENVELOPE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_HAPTIC_AMPLITUDE_ENVELOPE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SceneFB {
        pub get_space_bounding_box2_d: pfn::GetSpaceBoundingBox2DFB,
        pub get_space_bounding_box3_d: pfn::GetSpaceBoundingBox3DFB,
        pub get_space_semantic_labels: pfn::GetSpaceSemanticLabelsFB,
        pub get_space_boundary2_d: pfn::GetSpaceBoundary2DFB,
        pub get_space_room_layout: pfn::GetSpaceRoomLayoutFB,
    }
    impl SceneFB {
        pub const VERSION: u32 = sys::FB_scene_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SCENE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SCENE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_space_bounding_box2_d: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceBoundingBox2DFB\0"),
                )?),
                get_space_bounding_box3_d: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceBoundingBox3DFB\0"),
                )?),
                get_space_semantic_labels: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceSemanticLabelsFB\0"),
                )?),
                get_space_boundary2_d: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceBoundary2DFB\0"),
                )?),
                get_space_room_layout: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceRoomLayoutFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SceneCaptureFB {
        pub request_scene_capture: pfn::RequestSceneCaptureFB,
    }
    impl SceneCaptureFB {
        pub const VERSION: u32 = sys::FB_scene_capture_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SCENE_CAPTURE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SCENE_CAPTURE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                request_scene_capture: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrRequestSceneCaptureFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityContainerFB {
        pub get_space_container: pfn::GetSpaceContainerFB,
    }
    impl SpatialEntityContainerFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_container_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_CONTAINER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_CONTAINER_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_space_container: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceContainerFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FaceTrackingFB {
        pub create_face_tracker: pfn::CreateFaceTrackerFB,
        pub destroy_face_tracker: pfn::DestroyFaceTrackerFB,
        pub get_face_expression_weights: pfn::GetFaceExpressionWeightsFB,
    }
    impl FaceTrackingFB {
        pub const VERSION: u32 = sys::FB_face_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FACE_TRACKING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_FACE_TRACKING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_face_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateFaceTrackerFB\0"),
                )?),
                destroy_face_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyFaceTrackerFB\0"),
                )?),
                get_face_expression_weights: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetFaceExpressionWeightsFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct EyeTrackingSocialFB {
        pub create_eye_tracker: pfn::CreateEyeTrackerFB,
        pub destroy_eye_tracker: pfn::DestroyEyeTrackerFB,
        pub get_eye_gazes: pfn::GetEyeGazesFB,
    }
    impl EyeTrackingSocialFB {
        pub const VERSION: u32 = sys::FB_eye_tracking_social_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_EYE_TRACKING_SOCIAL_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_EYE_TRACKING_SOCIAL_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_eye_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateEyeTrackerFB\0"),
                )?),
                destroy_eye_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyEyeTrackerFB\0"),
                )?),
                get_eye_gazes: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetEyeGazesFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughKeyboardHandsFB {
        pub passthrough_layer_set_keyboard_hands_intensity:
            pfn::PassthroughLayerSetKeyboardHandsIntensityFB,
    }
    impl PassthroughKeyboardHandsFB {
        pub const VERSION: u32 = sys::FB_passthrough_keyboard_hands_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_PASSTHROUGH_KEYBOARD_HANDS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_PASSTHROUGH_KEYBOARD_HANDS_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                passthrough_layer_set_keyboard_hands_intensity: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrPassthroughLayerSetKeyboardHandsIntensityFB\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerSettingsFB {}
    impl CompositionLayerSettingsFB {
        pub const VERSION: u32 = sys::FB_composition_layer_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_SETTINGS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_COMPOSITION_LAYER_SETTINGS_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerProximityFB {}
    impl TouchControllerProximityFB {
        pub const VERSION: u32 = sys::FB_touch_controller_proximity_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_TOUCH_CONTROLLER_PROXIMITY_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_TOUCH_CONTROLLER_PROXIMITY_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HapticPcmFB {
        pub get_device_sample_rate: pfn::GetDeviceSampleRateFB,
    }
    impl HapticPcmFB {
        pub const VERSION: u32 = sys::FB_haptic_pcm_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAPTIC_PCM_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_HAPTIC_PCM_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_device_sample_rate: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetDeviceSampleRateFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthTestFB {}
    impl CompositionLayerDepthTestFB {
        pub const VERSION: u32 = sys::FB_composition_layer_depth_test_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityStorageBatchFB {
        pub save_space_list: pfn::SaveSpaceListFB,
    }
    impl SpatialEntityStorageBatchFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_storage_batch_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_STORAGE_BATCH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_STORAGE_BATCH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                save_space_list: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSaveSpaceListFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityUserFB {
        pub create_space_user: pfn::CreateSpaceUserFB,
        pub get_space_user_id: pfn::GetSpaceUserIdFB,
        pub destroy_space_user: pfn::DestroySpaceUserFB,
    }
    impl SpatialEntityUserFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_user_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_USER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_SPATIAL_ENTITY_USER_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_space_user: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpaceUserFB\0"),
                )?),
                get_space_user_id: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceUserIdFB\0"),
                )?),
                destroy_space_user: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpaceUserFB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FaceTracking2FB {
        pub create_face_tracker2: pfn::CreateFaceTracker2FB,
        pub destroy_face_tracker2: pfn::DestroyFaceTracker2FB,
        pub get_face_expression_weights2: pfn::GetFaceExpressionWeights2FB,
    }
    impl FaceTracking2FB {
        pub const VERSION: u32 = sys::FB_face_tracking2_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FACE_TRACKING2_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::FB_FACE_TRACKING2_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_face_tracker2: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateFaceTracker2FB\0"),
                )?),
                destroy_face_tracker2: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyFaceTracker2FB\0"),
                )?),
                get_face_expression_weights2: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetFaceExpressionWeights2FB\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveCosmosControllerInteractionHTC {}
    impl ViveCosmosControllerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_cosmos_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct FacialTrackingHTC {
        pub create_facial_tracker: pfn::CreateFacialTrackerHTC,
        pub destroy_facial_tracker: pfn::DestroyFacialTrackerHTC,
        pub get_facial_expressions: pfn::GetFacialExpressionsHTC,
    }
    impl FacialTrackingHTC {
        pub const VERSION: u32 = sys::HTC_facial_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_FACIAL_TRACKING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTC_FACIAL_TRACKING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_facial_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateFacialTrackerHTC\0"),
                )?),
                destroy_facial_tracker: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyFacialTrackerHTC\0"),
                )?),
                get_facial_expressions: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetFacialExpressionsHTC\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveFocus3ControllerInteractionHTC {}
    impl ViveFocus3ControllerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_focus3_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_FOCUS3_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::HTC_VIVE_FOCUS3_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionHTC {}
    impl HandInteractionHTC {
        pub const VERSION: u32 = sys::HTC_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_HAND_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTC_HAND_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ViveWristTrackerInteractionHTC {}
    impl ViveWristTrackerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_wrist_tracker_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_WRIST_TRACKER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::HTC_VIVE_WRIST_TRACKER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughHTC {
        pub create_passthrough: pfn::CreatePassthroughHTC,
        pub destroy_passthrough: pfn::DestroyPassthroughHTC,
    }
    impl PassthroughHTC {
        pub const VERSION: u32 = sys::HTC_passthrough_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_PASSTHROUGH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTC_PASSTHROUGH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_passthrough: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreatePassthroughHTC\0"),
                )?),
                destroy_passthrough: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyPassthroughHTC\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationHTC {
        pub apply_foveation: pfn::ApplyFoveationHTC,
    }
    impl FoveationHTC {
        pub const VERSION: u32 = sys::HTC_foveation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_FOVEATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTC_FOVEATION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                apply_foveation: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrApplyFoveationHTC\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct AnchorHTC {
        pub create_spatial_anchor: pfn::CreateSpatialAnchorHTC,
        pub get_spatial_anchor_name: pfn::GetSpatialAnchorNameHTC,
    }
    impl AnchorHTC {
        pub const VERSION: u32 = sys::HTC_anchor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_ANCHOR_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTC_ANCHOR_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateSpatialAnchorHTC\0"),
                )?),
                get_spatial_anchor_name: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpatialAnchorNameHTC\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionHUAWEI {}
    impl ControllerInteractionHUAWEI {
        pub const VERSION: u32 = sys::HUAWEI_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_ANDROID_THREAD_SETTINGS_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_ANDROID_SURFACE_SWAPCHAIN_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_COMPOSITION_LAYER_CUBE_EXTENSION_NAME_STR;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidCreateInstanceKHR {}
    #[cfg(target_os = "android")]
    impl AndroidCreateInstanceKHR {
        pub const VERSION: u32 = sys::KHR_android_create_instance_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthKHR {}
    impl CompositionLayerDepthKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_depth_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct VulkanSwapchainFormatListKHR {}
    impl VulkanSwapchainFormatListKHR {
        pub const VERSION: u32 = sys::KHR_vulkan_swapchain_format_list_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerCylinderKHR {}
    impl CompositionLayerCylinderKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_cylinder_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerEquirectKHR {}
    impl CompositionLayerEquirectKHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_equirect_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct OpenglEnableKHR {
        pub get_open_gl_graphics_requirements: pfn::GetOpenGLGraphicsRequirementsKHR,
    }
    impl OpenglEnableKHR {
        pub const VERSION: u32 = sys::KHR_opengl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_OPENGL_ENABLE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_OPENGL_ENABLE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_OPENGL_ES_ENABLE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_VULKAN_ENABLE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_D3D11_ENABLE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_D3D12_ENABLE_EXTENSION_NAME_STR;
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
    #[cfg(target_vendor = "apple")]
    #[derive(Copy, Clone)]
    pub struct MetalEnableKHR {
        pub get_metal_graphics_requirements: pfn::GetMetalGraphicsRequirementsKHR,
    }
    #[cfg(target_vendor = "apple")]
    impl MetalEnableKHR {
        pub const VERSION: u32 = sys::KHR_metal_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_METAL_ENABLE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_METAL_ENABLE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_metal_graphics_requirements: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMetalGraphicsRequirementsKHR\0"),
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
        pub const NAME_STR: &'static str = sys::KHR_VISIBILITY_MASK_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str =
            sys::KHR_COMPOSITION_LAYER_COLOR_SCALE_BIAS_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str =
            sys::KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_LOADER_INIT_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_LOADER_INIT_ANDROID_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_VULKAN_ENABLE2_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::KHR_COMPOSITION_LAYER_EQUIRECT2_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct BindingModificationKHR {}
    impl BindingModificationKHR {
        pub const VERSION: u32 = sys::KHR_binding_modification_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_BINDING_MODIFICATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_BINDING_MODIFICATION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUsageInputAttachmentBitKHR {}
    impl SwapchainUsageInputAttachmentBitKHR {
        pub const VERSION: u32 = sys::KHR_swapchain_usage_input_attachment_bit_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::KHR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::KHR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct LocateSpacesKHR {
        pub locate_spaces: pfn::LocateSpacesKHR,
    }
    impl LocateSpacesKHR {
        pub const VERSION: u32 = sys::KHR_locate_spaces_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_LOCATE_SPACES_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_LOCATE_SPACES_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                locate_spaces: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrLocateSpacesKHR\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct Maintenance1KHR {}
    impl Maintenance1KHR {
        pub const VERSION: u32 = sys::KHR_maintenance1_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_MAINTENANCE1_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::KHR_MAINTENANCE1_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct FoveationEyeTrackedMETA {
        pub get_foveation_eye_tracked_state: pfn::GetFoveationEyeTrackedStateMETA,
    }
    impl FoveationEyeTrackedMETA {
        pub const VERSION: u32 = sys::META_foveation_eye_tracked_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_FOVEATION_EYE_TRACKED_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_FOVEATION_EYE_TRACKED_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_foveation_eye_tracked_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetFoveationEyeTrackedStateMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct LocalDimmingMETA {}
    impl LocalDimmingMETA {
        pub const VERSION: u32 = sys::META_local_dimming_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_LOCAL_DIMMING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_LOCAL_DIMMING_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughPreferencesMETA {
        pub get_passthrough_preferences: pfn::GetPassthroughPreferencesMETA,
    }
    impl PassthroughPreferencesMETA {
        pub const VERSION: u32 = sys::META_passthrough_preferences_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_PASSTHROUGH_PREFERENCES_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_PASSTHROUGH_PREFERENCES_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_passthrough_preferences: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetPassthroughPreferencesMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct VirtualKeyboardMETA {
        pub create_virtual_keyboard: pfn::CreateVirtualKeyboardMETA,
        pub destroy_virtual_keyboard: pfn::DestroyVirtualKeyboardMETA,
        pub create_virtual_keyboard_space: pfn::CreateVirtualKeyboardSpaceMETA,
        pub suggest_virtual_keyboard_location: pfn::SuggestVirtualKeyboardLocationMETA,
        pub get_virtual_keyboard_scale: pfn::GetVirtualKeyboardScaleMETA,
        pub set_virtual_keyboard_model_visibility: pfn::SetVirtualKeyboardModelVisibilityMETA,
        pub get_virtual_keyboard_model_animation_states:
            pfn::GetVirtualKeyboardModelAnimationStatesMETA,
        pub get_virtual_keyboard_dirty_textures: pfn::GetVirtualKeyboardDirtyTexturesMETA,
        pub get_virtual_keyboard_texture_data: pfn::GetVirtualKeyboardTextureDataMETA,
        pub send_virtual_keyboard_input: pfn::SendVirtualKeyboardInputMETA,
        pub change_virtual_keyboard_text_context: pfn::ChangeVirtualKeyboardTextContextMETA,
    }
    impl VirtualKeyboardMETA {
        pub const VERSION: u32 = sys::META_virtual_keyboard_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_VIRTUAL_KEYBOARD_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_VIRTUAL_KEYBOARD_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_virtual_keyboard: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateVirtualKeyboardMETA\0"),
                )?),
                destroy_virtual_keyboard: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyVirtualKeyboardMETA\0"),
                )?),
                create_virtual_keyboard_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateVirtualKeyboardSpaceMETA\0"),
                )?),
                suggest_virtual_keyboard_location: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSuggestVirtualKeyboardLocationMETA\0"),
                )?),
                get_virtual_keyboard_scale: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVirtualKeyboardScaleMETA\0"),
                )?),
                set_virtual_keyboard_model_visibility: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSetVirtualKeyboardModelVisibilityMETA\0",
                        ),
                    )?,
                ),
                get_virtual_keyboard_model_animation_states: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrGetVirtualKeyboardModelAnimationStatesMETA\0",
                        ),
                    )?,
                ),
                get_virtual_keyboard_dirty_textures: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVirtualKeyboardDirtyTexturesMETA\0"),
                )?),
                get_virtual_keyboard_texture_data: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetVirtualKeyboardTextureDataMETA\0"),
                )?),
                send_virtual_keyboard_input: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSendVirtualKeyboardInputMETA\0"),
                )?),
                change_virtual_keyboard_text_context: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrChangeVirtualKeyboardTextContextMETA\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct VulkanSwapchainCreateInfoMETA {}
    impl VulkanSwapchainCreateInfoMETA {
        pub const VERSION: u32 = sys::META_vulkan_swapchain_create_info_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_VULKAN_SWAPCHAIN_CREATE_INFO_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::META_VULKAN_SWAPCHAIN_CREATE_INFO_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct PerformanceMetricsMETA {
        pub enumerate_performance_metrics_counter_paths:
            pfn::EnumeratePerformanceMetricsCounterPathsMETA,
        pub set_performance_metrics_state: pfn::SetPerformanceMetricsStateMETA,
        pub get_performance_metrics_state: pfn::GetPerformanceMetricsStateMETA,
        pub query_performance_metrics_counter: pfn::QueryPerformanceMetricsCounterMETA,
    }
    impl PerformanceMetricsMETA {
        pub const VERSION: u32 = sys::META_performance_metrics_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_PERFORMANCE_METRICS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_PERFORMANCE_METRICS_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_performance_metrics_counter_paths: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumeratePerformanceMetricsCounterPathsMETA\0",
                        ),
                    )?,
                ),
                set_performance_metrics_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetPerformanceMetricsStateMETA\0"),
                )?),
                get_performance_metrics_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetPerformanceMetricsStateMETA\0"),
                )?),
                query_performance_metrics_counter: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrQueryPerformanceMetricsCounterMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HeadsetIdMETA {}
    impl HeadsetIdMETA {
        pub const VERSION: u32 = sys::META_headset_id_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_HEADSET_ID_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_HEADSET_ID_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct RecommendedLayerResolutionMETA {
        pub get_recommended_layer_resolution: pfn::GetRecommendedLayerResolutionMETA,
    }
    impl RecommendedLayerResolutionMETA {
        pub const VERSION: u32 = sys::META_recommended_layer_resolution_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_RECOMMENDED_LAYER_RESOLUTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::META_RECOMMENDED_LAYER_RESOLUTION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_recommended_layer_resolution: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetRecommendedLayerResolutionMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughColorLutMETA {
        pub create_passthrough_color_lut: pfn::CreatePassthroughColorLutMETA,
        pub destroy_passthrough_color_lut: pfn::DestroyPassthroughColorLutMETA,
        pub update_passthrough_color_lut: pfn::UpdatePassthroughColorLutMETA,
    }
    impl PassthroughColorLutMETA {
        pub const VERSION: u32 = sys::META_passthrough_color_lut_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_PASSTHROUGH_COLOR_LUT_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_PASSTHROUGH_COLOR_LUT_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_passthrough_color_lut: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreatePassthroughColorLutMETA\0"),
                )?),
                destroy_passthrough_color_lut: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyPassthroughColorLutMETA\0"),
                )?),
                update_passthrough_color_lut: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrUpdatePassthroughColorLutMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityMeshMETA {
        pub get_space_triangle_mesh: pfn::GetSpaceTriangleMeshMETA,
    }
    impl SpatialEntityMeshMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_MESH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_SPATIAL_ENTITY_MESH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_space_triangle_mesh: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetSpaceTriangleMeshMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct AutomaticLayerFilterMETA {}
    impl AutomaticLayerFilterMETA {
        pub const VERSION: u32 = sys::META_automatic_layer_filter_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_AUTOMATIC_LAYER_FILTER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_AUTOMATIC_LAYER_FILTER_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerPlusMETA {}
    impl TouchControllerPlusMETA {
        pub const VERSION: u32 = sys::META_touch_controller_plus_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_TOUCH_CONTROLLER_PLUS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_TOUCH_CONTROLLER_PLUS_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct EnvironmentDepthMETA {
        pub create_environment_depth_provider: pfn::CreateEnvironmentDepthProviderMETA,
        pub destroy_environment_depth_provider: pfn::DestroyEnvironmentDepthProviderMETA,
        pub start_environment_depth_provider: pfn::StartEnvironmentDepthProviderMETA,
        pub stop_environment_depth_provider: pfn::StopEnvironmentDepthProviderMETA,
        pub create_environment_depth_swapchain: pfn::CreateEnvironmentDepthSwapchainMETA,
        pub destroy_environment_depth_swapchain: pfn::DestroyEnvironmentDepthSwapchainMETA,
        pub enumerate_environment_depth_swapchain_images:
            pfn::EnumerateEnvironmentDepthSwapchainImagesMETA,
        pub get_environment_depth_swapchain_state: pfn::GetEnvironmentDepthSwapchainStateMETA,
        pub acquire_environment_depth_image: pfn::AcquireEnvironmentDepthImageMETA,
        pub set_environment_depth_hand_removal: pfn::SetEnvironmentDepthHandRemovalMETA,
    }
    impl EnvironmentDepthMETA {
        pub const VERSION: u32 = sys::META_environment_depth_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_ENVIRONMENT_DEPTH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::META_ENVIRONMENT_DEPTH_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_environment_depth_provider: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateEnvironmentDepthProviderMETA\0"),
                )?),
                destroy_environment_depth_provider: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyEnvironmentDepthProviderMETA\0"),
                )?),
                start_environment_depth_provider: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStartEnvironmentDepthProviderMETA\0"),
                )?),
                stop_environment_depth_provider: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrStopEnvironmentDepthProviderMETA\0"),
                )?),
                create_environment_depth_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateEnvironmentDepthSwapchainMETA\0"),
                )?),
                destroy_environment_depth_swapchain: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(
                        b"xrDestroyEnvironmentDepthSwapchainMETA\0",
                    ),
                )?),
                enumerate_environment_depth_swapchain_images: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumerateEnvironmentDepthSwapchainImagesMETA\0",
                        ),
                    )?,
                ),
                get_environment_depth_swapchain_state: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrGetEnvironmentDepthSwapchainStateMETA\0",
                        ),
                    )?,
                ),
                acquire_environment_depth_image: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrAcquireEnvironmentDepthImageMETA\0"),
                )?),
                set_environment_depth_hand_removal: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetEnvironmentDepthHandRemovalMETA\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct Ml2ControllerInteractionML {}
    impl Ml2ControllerInteractionML {
        pub const VERSION: u32 = sys::ML_ml2_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_ML2_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_ML2_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct FrameEndInfoML {}
    impl FrameEndInfoML {
        pub const VERSION: u32 = sys::ML_frame_end_info_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_FRAME_END_INFO_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_FRAME_END_INFO_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct GlobalDimmerML {}
    impl GlobalDimmerML {
        pub const VERSION: u32 = sys::ML_global_dimmer_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_GLOBAL_DIMMER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_GLOBAL_DIMMER_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompatML {
        pub create_space_from_coordinate_frame_uid: pfn::CreateSpaceFromCoordinateFrameUIDML,
    }
    impl CompatML {
        pub const VERSION: u32 = sys::ML_compat_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_COMPAT_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_COMPAT_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_space_from_coordinate_frame_uid: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrCreateSpaceFromCoordinateFrameUIDML\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct MarkerUnderstandingML {
        pub create_marker_detector: pfn::CreateMarkerDetectorML,
        pub destroy_marker_detector: pfn::DestroyMarkerDetectorML,
        pub snapshot_marker_detector: pfn::SnapshotMarkerDetectorML,
        pub get_marker_detector_state: pfn::GetMarkerDetectorStateML,
        pub get_markers: pfn::GetMarkersML,
        pub get_marker_reprojection_error: pfn::GetMarkerReprojectionErrorML,
        pub get_marker_length: pfn::GetMarkerLengthML,
        pub get_marker_number: pfn::GetMarkerNumberML,
        pub get_marker_string: pfn::GetMarkerStringML,
        pub create_marker_space: pfn::CreateMarkerSpaceML,
    }
    impl MarkerUnderstandingML {
        pub const VERSION: u32 = sys::ML_marker_understanding_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_MARKER_UNDERSTANDING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_MARKER_UNDERSTANDING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_marker_detector: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateMarkerDetectorML\0"),
                )?),
                destroy_marker_detector: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyMarkerDetectorML\0"),
                )?),
                snapshot_marker_detector: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSnapshotMarkerDetectorML\0"),
                )?),
                get_marker_detector_state: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerDetectorStateML\0"),
                )?),
                get_markers: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkersML\0"),
                )?),
                get_marker_reprojection_error: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerReprojectionErrorML\0"),
                )?),
                get_marker_length: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerLengthML\0"),
                )?),
                get_marker_number: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerNumberML\0"),
                )?),
                get_marker_string: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerStringML\0"),
                )?),
                create_marker_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateMarkerSpaceML\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct LocalizationMapML {
        pub enable_localization_events: pfn::EnableLocalizationEventsML,
        pub query_localization_maps: pfn::QueryLocalizationMapsML,
        pub request_map_localization: pfn::RequestMapLocalizationML,
        pub import_localization_map: pfn::ImportLocalizationMapML,
        pub create_exported_localization_map: pfn::CreateExportedLocalizationMapML,
        pub destroy_exported_localization_map: pfn::DestroyExportedLocalizationMapML,
        pub get_exported_localization_map_data: pfn::GetExportedLocalizationMapDataML,
    }
    impl LocalizationMapML {
        pub const VERSION: u32 = sys::ML_localization_map_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_LOCALIZATION_MAP_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_LOCALIZATION_MAP_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enable_localization_events: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnableLocalizationEventsML\0"),
                )?),
                query_localization_maps: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrQueryLocalizationMapsML\0"),
                )?),
                request_map_localization: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrRequestMapLocalizationML\0"),
                )?),
                import_localization_map: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrImportLocalizationMapML\0"),
                )?),
                create_exported_localization_map: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateExportedLocalizationMapML\0"),
                )?),
                destroy_exported_localization_map: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroyExportedLocalizationMapML\0"),
                )?),
                get_exported_localization_map_data: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetExportedLocalizationMapDataML\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct UserCalibrationML {
        pub enable_user_calibration_events: pfn::EnableUserCalibrationEventsML,
    }
    impl UserCalibrationML {
        pub const VERSION: u32 = sys::ML_user_calibration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_USER_CALIBRATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ML_USER_CALIBRATION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enable_user_calibration_events: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnableUserCalibrationEventsML\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HeadlessMND {}
    impl HeadlessMND {
        pub const VERSION: u32 = sys::MND_headless_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MND_HEADLESS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MND_HEADLESS_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUsageInputAttachmentBitMND {}
    impl SwapchainUsageInputAttachmentBitMND {
        pub const VERSION: u32 = sys::MND_swapchain_usage_input_attachment_bit_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct UnboundedReferenceSpaceMSFT {}
    impl UnboundedReferenceSpaceMSFT {
        pub const VERSION: u32 = sys::MSFT_unbounded_reference_space_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::MSFT_SPATIAL_ANCHOR_EXTENSION_NAME_STR;
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
        pub try_create_spatial_graph_static_node_binding:
            pfn::TryCreateSpatialGraphStaticNodeBindingMSFT,
        pub destroy_spatial_graph_node_binding: pfn::DestroySpatialGraphNodeBindingMSFT,
        pub get_spatial_graph_node_binding_properties:
            pfn::GetSpatialGraphNodeBindingPropertiesMSFT,
    }
    impl SpatialGraphBridgeMSFT {
        pub const VERSION: u32 = sys::MSFT_spatial_graph_bridge_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_SPATIAL_GRAPH_BRIDGE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_SPATIAL_GRAPH_BRIDGE_EXTENSION_NAME_STR;
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
                try_create_spatial_graph_static_node_binding: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrTryCreateSpatialGraphStaticNodeBindingMSFT\0",
                        ),
                    )?,
                ),
                destroy_spatial_graph_node_binding: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrDestroySpatialGraphNodeBindingMSFT\0"),
                )?),
                get_spatial_graph_node_binding_properties: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrGetSpatialGraphNodeBindingPropertiesMSFT\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionMSFT {}
    impl HandInteractionMSFT {
        pub const VERSION: u32 = sys::MSFT_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_HAND_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_HAND_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingMeshMSFT {
        pub create_hand_mesh_space: pfn::CreateHandMeshSpaceMSFT,
        pub update_hand_mesh: pfn::UpdateHandMeshMSFT,
    }
    impl HandTrackingMeshMSFT {
        pub const VERSION: u32 = sys::MSFT_hand_tracking_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_HAND_TRACKING_MESH_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_HAND_TRACKING_MESH_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str =
            sys::MSFT_SECONDARY_VIEW_CONFIGURATION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct FirstPersonObserverMSFT {}
    impl FirstPersonObserverMSFT {
        pub const VERSION: u32 = sys::MSFT_first_person_observer_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_FIRST_PERSON_OBSERVER_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_FIRST_PERSON_OBSERVER_EXTENSION_NAME_STR;
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
        pub const NAME_STR: &'static str = sys::MSFT_CONTROLLER_MODEL_EXTENSION_NAME_STR;
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
    pub struct PerceptionAnchorInteropMSFT {
        pub create_spatial_anchor_from_perception_anchor:
            pfn::CreateSpatialAnchorFromPerceptionAnchorMSFT,
        pub try_get_perception_anchor_from_spatial_anchor:
            pfn::TryGetPerceptionAnchorFromSpatialAnchorMSFT,
    }
    #[cfg(windows)]
    impl PerceptionAnchorInteropMSFT {
        pub const VERSION: u32 = sys::MSFT_perception_anchor_interop_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_PERCEPTION_ANCHOR_INTEROP_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_PERCEPTION_ANCHOR_INTEROP_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_anchor_from_perception_anchor: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrCreateSpatialAnchorFromPerceptionAnchorMSFT\0",
                        ),
                    )?,
                ),
                try_get_perception_anchor_from_spatial_anchor: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrTryGetPerceptionAnchorFromSpatialAnchorMSFT\0",
                        ),
                    )?,
                ),
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
        pub const NAME_STR: &'static str =
            sys::MSFT_HOLOGRAPHIC_WINDOW_ATTACHMENT_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerReprojectionMSFT {
        pub enumerate_reprojection_modes: pfn::EnumerateReprojectionModesMSFT,
    }
    impl CompositionLayerReprojectionMSFT {
        pub const VERSION: u32 = sys::MSFT_composition_layer_reprojection_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_COMPOSITION_LAYER_REPROJECTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::MSFT_COMPOSITION_LAYER_REPROJECTION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_reprojection_modes: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateReprojectionModesMSFT\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorPersistenceMSFT {
        pub create_spatial_anchor_store_connection: pfn::CreateSpatialAnchorStoreConnectionMSFT,
        pub destroy_spatial_anchor_store_connection: pfn::DestroySpatialAnchorStoreConnectionMSFT,
        pub persist_spatial_anchor: pfn::PersistSpatialAnchorMSFT,
        pub enumerate_persisted_spatial_anchor_names: pfn::EnumeratePersistedSpatialAnchorNamesMSFT,
        pub create_spatial_anchor_from_persisted_name:
            pfn::CreateSpatialAnchorFromPersistedNameMSFT,
        pub unpersist_spatial_anchor: pfn::UnpersistSpatialAnchorMSFT,
        pub clear_spatial_anchor_store: pfn::ClearSpatialAnchorStoreMSFT,
    }
    impl SpatialAnchorPersistenceMSFT {
        pub const VERSION: u32 = sys::MSFT_spatial_anchor_persistence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_SPATIAL_ANCHOR_PERSISTENCE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MSFT_SPATIAL_ANCHOR_PERSISTENCE_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                create_spatial_anchor_store_connection: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrCreateSpatialAnchorStoreConnectionMSFT\0",
                        ),
                    )?,
                ),
                destroy_spatial_anchor_store_connection: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrDestroySpatialAnchorStoreConnectionMSFT\0",
                        ),
                    )?,
                ),
                persist_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrPersistSpatialAnchorMSFT\0"),
                )?),
                enumerate_persisted_spatial_anchor_names: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrEnumeratePersistedSpatialAnchorNamesMSFT\0",
                        ),
                    )?,
                ),
                create_spatial_anchor_from_persisted_name: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrCreateSpatialAnchorFromPersistedNameMSFT\0",
                        ),
                    )?,
                ),
                unpersist_spatial_anchor: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrUnpersistSpatialAnchorMSFT\0"),
                )?),
                clear_spatial_anchor_store: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrClearSpatialAnchorStoreMSFT\0"),
                )?),
            })
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidSessionStateEnableOCULUS {}
    #[cfg(target_os = "android")]
    impl AndroidSessionStateEnableOCULUS {
        pub const VERSION: u32 = sys::OCULUS_android_session_state_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct AudioDeviceGuidOCULUS {
        pub get_audio_output_device_guid: pfn::GetAudioOutputDeviceGuidOculus,
        pub get_audio_input_device_guid: pfn::GetAudioInputDeviceGuidOculus,
    }
    impl AudioDeviceGuidOCULUS {
        pub const VERSION: u32 = sys::OCULUS_audio_device_guid_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_AUDIO_DEVICE_GUID_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::OCULUS_AUDIO_DEVICE_GUID_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                get_audio_output_device_guid: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetAudioOutputDeviceGuidOculus\0"),
                )?),
                get_audio_input_device_guid: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetAudioInputDeviceGuidOculus\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ExternalCameraOCULUS {
        pub enumerate_external_cameras: pfn::EnumerateExternalCamerasOCULUS,
    }
    impl ExternalCameraOCULUS {
        pub const VERSION: u32 = sys::OCULUS_external_camera_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_EXTERNAL_CAMERA_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::OCULUS_EXTERNAL_CAMERA_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_external_cameras: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateExternalCamerasOCULUS\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionOPPO {}
    impl ControllerInteractionOPPO {
        pub const VERSION: u32 = sys::OPPO_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OPPO_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::OPPO_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct TrackingOptimizationSettingsQCOM {
        pub set_tracking_optimization_settings_hint: pfn::SetTrackingOptimizationSettingsHintQCOM,
    }
    impl TrackingOptimizationSettingsQCOM {
        pub const VERSION: u32 = sys::QCOM_tracking_optimization_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::QCOM_TRACKING_OPTIMIZATION_SETTINGS_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::QCOM_TRACKING_OPTIMIZATION_SETTINGS_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_tracking_optimization_settings_hint: mem::transmute(
                    entry.get_instance_proc_addr(
                        instance,
                        CStr::from_bytes_with_nul_unchecked(
                            b"xrSetTrackingOptimizationSettingsHintQCOM\0",
                        ),
                    )?,
                ),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingForearmULTRALEAP {}
    impl HandTrackingForearmULTRALEAP {
        pub const VERSION: u32 = sys::ULTRALEAP_hand_tracking_forearm_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ULTRALEAP_HAND_TRACKING_FOREARM_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::ULTRALEAP_HAND_TRACKING_FOREARM_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct AnalogThresholdVALVE {}
    impl AnalogThresholdVALVE {
        pub const VERSION: u32 = sys::VALVE_analog_threshold_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VALVE_ANALOG_THRESHOLD_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VALVE_ANALOG_THRESHOLD_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct QuadViewsVARJO {}
    impl QuadViewsVARJO {
        pub const VERSION: u32 = sys::VARJO_quad_views_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_QUAD_VIEWS_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VARJO_QUAD_VIEWS_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct FoveatedRenderingVARJO {}
    impl FoveatedRenderingVARJO {
        pub const VERSION: u32 = sys::VARJO_foveated_rendering_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_FOVEATED_RENDERING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VARJO_FOVEATED_RENDERING_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthTestVARJO {}
    impl CompositionLayerDepthTestVARJO {
        pub const VERSION: u32 = sys::VARJO_composition_layer_depth_test_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::VARJO_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct EnvironmentDepthEstimationVARJO {
        pub set_environment_depth_estimation: pfn::SetEnvironmentDepthEstimationVARJO,
    }
    impl EnvironmentDepthEstimationVARJO {
        pub const VERSION: u32 = sys::VARJO_environment_depth_estimation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_ENVIRONMENT_DEPTH_ESTIMATION_EXTENSION_NAME;
        pub const NAME_STR: &'static str =
            sys::VARJO_ENVIRONMENT_DEPTH_ESTIMATION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_environment_depth_estimation: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetEnvironmentDepthEstimationVARJO\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct MarkerTrackingVARJO {
        pub set_marker_tracking: pfn::SetMarkerTrackingVARJO,
        pub set_marker_tracking_timeout: pfn::SetMarkerTrackingTimeoutVARJO,
        pub set_marker_tracking_prediction: pfn::SetMarkerTrackingPredictionVARJO,
        pub get_marker_size: pfn::GetMarkerSizeVARJO,
        pub create_marker_space: pfn::CreateMarkerSpaceVARJO,
    }
    impl MarkerTrackingVARJO {
        pub const VERSION: u32 = sys::VARJO_marker_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_MARKER_TRACKING_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VARJO_MARKER_TRACKING_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_marker_tracking: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetMarkerTrackingVARJO\0"),
                )?),
                set_marker_tracking_timeout: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetMarkerTrackingTimeoutVARJO\0"),
                )?),
                set_marker_tracking_prediction: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetMarkerTrackingPredictionVARJO\0"),
                )?),
                get_marker_size: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrGetMarkerSizeVARJO\0"),
                )?),
                create_marker_space: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrCreateMarkerSpaceVARJO\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViewOffsetVARJO {
        pub set_view_offset: pfn::SetViewOffsetVARJO,
    }
    impl ViewOffsetVARJO {
        pub const VERSION: u32 = sys::VARJO_view_offset_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_VIEW_OFFSET_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VARJO_VIEW_OFFSET_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                set_view_offset: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrSetViewOffsetVARJO\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct Xr4ControllerInteractionVARJO {}
    impl Xr4ControllerInteractionVARJO {
        pub const VERSION: u32 = sys::VARJO_xr4_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_XR4_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::VARJO_XR4_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionYVR {}
    impl ControllerInteractionYVR {
        pub const VERSION: u32 = sys::YVR_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::YVR_CONTROLLER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::YVR_CONTROLLER_INTERACTION_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct OverlayEXTX {}
    impl OverlayEXTX {
        pub const VERSION: u32 = sys::EXTX_overlay_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXTX_OVERLAY_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::EXTX_OVERLAY_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct EglEnableMNDX {}
    impl EglEnableMNDX {
        pub const VERSION: u32 = sys::MNDX_egl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MNDX_EGL_ENABLE_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MNDX_EGL_ENABLE_EXTENSION_NAME_STR;
    }
    #[derive(Copy, Clone)]
    pub struct ForceFeedbackCurlMNDX {
        pub apply_force_feedback_curl: pfn::ApplyForceFeedbackCurlMNDX,
    }
    impl ForceFeedbackCurlMNDX {
        pub const VERSION: u32 = sys::MNDX_force_feedback_curl_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MNDX_FORCE_FEEDBACK_CURL_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::MNDX_FORCE_FEEDBACK_CURL_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                apply_force_feedback_curl: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrApplyForceFeedbackCurlMNDX\0"),
                )?),
            })
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveTrackerInteractionHTCX {
        pub enumerate_vive_tracker_paths: pfn::EnumerateViveTrackerPathsHTCX,
    }
    impl ViveTrackerInteractionHTCX {
        pub const VERSION: u32 = sys::HTCX_vive_tracker_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTCX_VIVE_TRACKER_INTERACTION_EXTENSION_NAME;
        pub const NAME_STR: &'static str = sys::HTCX_VIVE_TRACKER_INTERACTION_EXTENSION_NAME_STR;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            Ok(Self {
                enumerate_vive_tracker_paths: mem::transmute(entry.get_instance_proc_addr(
                    instance,
                    CStr::from_bytes_with_nul_unchecked(b"xrEnumerateViveTrackerPathsHTCX\0"),
                )?),
            })
        }
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
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct CompositionLayerPassthroughFB<'a, G: Graphics> {
        inner: sys::CompositionLayerPassthroughFB,
        _marker: PhantomData<&'a G>,
    }
    impl<'a, G: Graphics> CompositionLayerPassthroughFB<'a, G> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::CompositionLayerPassthroughFB {
                    ty: sys::StructureType::COMPOSITION_LAYER_PASSTHROUGH_FB,
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
        pub unsafe fn from_raw(inner: sys::CompositionLayerPassthroughFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::CompositionLayerPassthroughFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::CompositionLayerPassthroughFB {
            &self.inner
        }
        #[inline]
        pub fn flags(mut self, value: CompositionLayerFlags) -> Self {
            self.inner.flags = value;
            self
        }
        #[inline]
        pub fn space(mut self, value: &'a Space) -> Self {
            self.inner.space = value.as_raw();
            self
        }
        #[inline]
        pub fn layer_handle(mut self, value: &'a PassthroughLayerFB) -> Self {
            self.inner.layer_handle = value.as_raw();
            self
        }
    }
    impl<'a, G: Graphics> Deref for CompositionLayerPassthroughFB<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a, G: Graphics> Default for CompositionLayerPassthroughFB<'a, G> {
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
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct HapticAmplitudeEnvelopeVibrationFB<'a> {
        inner: sys::HapticAmplitudeEnvelopeVibrationFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> HapticAmplitudeEnvelopeVibrationFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::HapticAmplitudeEnvelopeVibrationFB {
                    ty: sys::StructureType::HAPTIC_AMPLITUDE_ENVELOPE_VIBRATION_FB,
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
        pub unsafe fn from_raw(inner: sys::HapticAmplitudeEnvelopeVibrationFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::HapticAmplitudeEnvelopeVibrationFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::HapticAmplitudeEnvelopeVibrationFB {
            &self.inner
        }
        #[inline]
        pub fn duration(mut self, value: Duration) -> Self {
            self.inner.duration = value;
            self
        }
        #[inline]
        pub fn amplitudes(mut self, value: &'a [f32]) -> Self {
            self.inner.amplitudes = value.as_ptr() as *const _ as _;
            self.inner.amplitude_count = value.len() as u32;
            self
        }
    }
    impl<'a> Deref for HapticAmplitudeEnvelopeVibrationFB<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for HapticAmplitudeEnvelopeVibrationFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct HapticPcmVibrationFB<'a> {
        inner: sys::HapticPcmVibrationFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> HapticPcmVibrationFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::HapticPcmVibrationFB {
                    ty: sys::StructureType::HAPTIC_PCM_VIBRATION_FB,
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
        pub unsafe fn from_raw(inner: sys::HapticPcmVibrationFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::HapticPcmVibrationFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::HapticPcmVibrationFB {
            &self.inner
        }
        #[inline]
        pub fn buffer(mut self, value: &'a [f32]) -> Self {
            self.inner.buffer = value.as_ptr() as *const _ as _;
            self.inner.buffer_size = value.len() as u32;
            self
        }
        #[inline]
        pub fn sample_rate(mut self, value: f32) -> Self {
            self.inner.sample_rate = value;
            self
        }
        #[inline]
        pub fn append(mut self, value: bool) -> Self {
            self.inner.append = value.into();
            self
        }
        #[inline]
        pub fn samples_consumed(mut self, value: &'a mut u32) -> Self {
            self.inner.samples_consumed = value as *mut _ as _;
            self
        }
    }
    impl<'a> Deref for HapticPcmVibrationFB<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for HapticPcmVibrationFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct BindingModificationBase<'a> {
        _inner: sys::BindingModificationBaseHeaderKHR,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct InteractionProfileDpadBindingEXT<'a> {
        inner: sys::InteractionProfileDpadBindingEXT,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> InteractionProfileDpadBindingEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::InteractionProfileDpadBindingEXT {
                    ty: sys::StructureType::INTERACTION_PROFILE_DPAD_BINDING_EXT,
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
        pub unsafe fn from_raw(inner: sys::InteractionProfileDpadBindingEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::InteractionProfileDpadBindingEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::InteractionProfileDpadBindingEXT {
            &self.inner
        }
        #[inline]
        pub fn binding(mut self, value: Path) -> Self {
            self.inner.binding = value;
            self
        }
        #[inline]
        pub fn action_set(mut self, value: &'a ActionSet) -> Self {
            self.inner.action_set = value.as_raw();
            self
        }
        #[inline]
        pub fn force_threshold(mut self, value: f32) -> Self {
            self.inner.force_threshold = value;
            self
        }
        #[inline]
        pub fn force_threshold_released(mut self, value: f32) -> Self {
            self.inner.force_threshold_released = value;
            self
        }
        #[inline]
        pub fn center_region(mut self, value: f32) -> Self {
            self.inner.center_region = value;
            self
        }
        #[inline]
        pub fn wedge_angle(mut self, value: f32) -> Self {
            self.inner.wedge_angle = value;
            self
        }
        #[inline]
        pub fn is_sticky(mut self, value: bool) -> Self {
            self.inner.is_sticky = value.into();
            self
        }
        #[inline]
        pub fn on_haptic(mut self, value: &'a HapticBase<'a>) -> Self {
            self.inner.on_haptic = value as *const _ as _;
            self
        }
        #[inline]
        pub fn off_haptic(mut self, value: &'a HapticBase<'a>) -> Self {
            self.inner.off_haptic = value as *const _ as _;
            self
        }
    }
    impl<'a> Deref for InteractionProfileDpadBindingEXT<'a> {
        type Target = BindingModificationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for InteractionProfileDpadBindingEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct InteractionProfileAnalogThresholdVALVE<'a> {
        inner: sys::InteractionProfileAnalogThresholdVALVE,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> InteractionProfileAnalogThresholdVALVE<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::InteractionProfileAnalogThresholdVALVE {
                    ty: sys::StructureType::INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE,
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
        pub unsafe fn from_raw(inner: sys::InteractionProfileAnalogThresholdVALVE) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::InteractionProfileAnalogThresholdVALVE {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::InteractionProfileAnalogThresholdVALVE {
            &self.inner
        }
        #[inline]
        pub fn action<ATY: ActionTy>(mut self, value: &'a Action<ATY>) -> Self {
            self.inner.action = value.as_raw();
            self
        }
        #[inline]
        pub fn binding(mut self, value: Path) -> Self {
            self.inner.binding = value;
            self
        }
        #[inline]
        pub fn on_threshold(mut self, value: f32) -> Self {
            self.inner.on_threshold = value;
            self
        }
        #[inline]
        pub fn off_threshold(mut self, value: f32) -> Self {
            self.inner.off_threshold = value;
            self
        }
        #[inline]
        pub fn on_haptic(mut self, value: &'a HapticBase<'a>) -> Self {
            self.inner.on_haptic = value as *const _ as _;
            self
        }
        #[inline]
        pub fn off_haptic(mut self, value: &'a HapticBase<'a>) -> Self {
            self.inner.off_haptic = value as *const _ as _;
            self
        }
    }
    impl<'a> Deref for InteractionProfileAnalogThresholdVALVE<'a> {
        type Target = BindingModificationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for InteractionProfileAnalogThresholdVALVE<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SwapchainStateBase<'a> {
        _inner: sys::SwapchainStateBaseHeaderFB,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainStateAndroidSurfaceDimensionsFB<'a> {
        inner: sys::SwapchainStateAndroidSurfaceDimensionsFB,
        _marker: PhantomData<&'a ()>,
    }
    #[cfg(target_os = "android")]
    impl<'a> SwapchainStateAndroidSurfaceDimensionsFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainStateAndroidSurfaceDimensionsFB {
                    ty: sys::StructureType::SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB,
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
        pub unsafe fn from_raw(inner: sys::SwapchainStateAndroidSurfaceDimensionsFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainStateAndroidSurfaceDimensionsFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainStateAndroidSurfaceDimensionsFB {
            &self.inner
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
    }
    #[cfg(target_os = "android")]
    impl<'a> Deref for SwapchainStateAndroidSurfaceDimensionsFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[cfg(target_os = "android")]
    impl<'a> Default for SwapchainStateAndroidSurfaceDimensionsFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainStateSamplerOpenGLESFB<'a> {
        inner: sys::SwapchainStateSamplerOpenGLESFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainStateSamplerOpenGLESFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainStateSamplerOpenGLESFB {
                    ty: sys::StructureType::SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB,
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
        pub unsafe fn from_raw(inner: sys::SwapchainStateSamplerOpenGLESFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainStateSamplerOpenGLESFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainStateSamplerOpenGLESFB {
            &self.inner
        }
        #[inline]
        pub fn min_filter(mut self, value: EGLenum) -> Self {
            self.inner.min_filter = value;
            self
        }
        #[inline]
        pub fn mag_filter(mut self, value: EGLenum) -> Self {
            self.inner.mag_filter = value;
            self
        }
        #[inline]
        pub fn wrap_mode_s(mut self, value: EGLenum) -> Self {
            self.inner.wrap_mode_s = value;
            self
        }
        #[inline]
        pub fn wrap_mode_t(mut self, value: EGLenum) -> Self {
            self.inner.wrap_mode_t = value;
            self
        }
        #[inline]
        pub fn swizzle_red(mut self, value: EGLenum) -> Self {
            self.inner.swizzle_red = value;
            self
        }
        #[inline]
        pub fn swizzle_green(mut self, value: EGLenum) -> Self {
            self.inner.swizzle_green = value;
            self
        }
        #[inline]
        pub fn swizzle_blue(mut self, value: EGLenum) -> Self {
            self.inner.swizzle_blue = value;
            self
        }
        #[inline]
        pub fn swizzle_alpha(mut self, value: EGLenum) -> Self {
            self.inner.swizzle_alpha = value;
            self
        }
        #[inline]
        pub fn max_anisotropy(mut self, value: f32) -> Self {
            self.inner.max_anisotropy = value;
            self
        }
        #[inline]
        pub fn border_color(mut self, value: Color4f) -> Self {
            self.inner.border_color = value;
            self
        }
    }
    impl<'a> Deref for SwapchainStateSamplerOpenGLESFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SwapchainStateSamplerOpenGLESFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainStateSamplerVulkanFB<'a> {
        inner: sys::SwapchainStateSamplerVulkanFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainStateSamplerVulkanFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainStateSamplerVulkanFB {
                    ty: sys::StructureType::SWAPCHAIN_STATE_SAMPLER_VULKAN_FB,
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
        pub unsafe fn from_raw(inner: sys::SwapchainStateSamplerVulkanFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainStateSamplerVulkanFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainStateSamplerVulkanFB {
            &self.inner
        }
        #[inline]
        pub fn min_filter(mut self, value: VkFilter) -> Self {
            self.inner.min_filter = value;
            self
        }
        #[inline]
        pub fn mag_filter(mut self, value: VkFilter) -> Self {
            self.inner.mag_filter = value;
            self
        }
        #[inline]
        pub fn mipmap_mode(mut self, value: VkSamplerMipmapMode) -> Self {
            self.inner.mipmap_mode = value;
            self
        }
        #[inline]
        pub fn wrap_mode_s(mut self, value: VkSamplerAddressMode) -> Self {
            self.inner.wrap_mode_s = value;
            self
        }
        #[inline]
        pub fn wrap_mode_t(mut self, value: VkSamplerAddressMode) -> Self {
            self.inner.wrap_mode_t = value;
            self
        }
        #[inline]
        pub fn swizzle_red(mut self, value: VkComponentSwizzle) -> Self {
            self.inner.swizzle_red = value;
            self
        }
        #[inline]
        pub fn swizzle_green(mut self, value: VkComponentSwizzle) -> Self {
            self.inner.swizzle_green = value;
            self
        }
        #[inline]
        pub fn swizzle_blue(mut self, value: VkComponentSwizzle) -> Self {
            self.inner.swizzle_blue = value;
            self
        }
        #[inline]
        pub fn swizzle_alpha(mut self, value: VkComponentSwizzle) -> Self {
            self.inner.swizzle_alpha = value;
            self
        }
        #[inline]
        pub fn max_anisotropy(mut self, value: f32) -> Self {
            self.inner.max_anisotropy = value;
            self
        }
        #[inline]
        pub fn border_color(mut self, value: Color4f) -> Self {
            self.inner.border_color = value;
            self
        }
    }
    impl<'a> Deref for SwapchainStateSamplerVulkanFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SwapchainStateSamplerVulkanFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SwapchainStateFoveationFB<'a> {
        inner: sys::SwapchainStateFoveationFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SwapchainStateFoveationFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SwapchainStateFoveationFB {
                    ty: sys::StructureType::SWAPCHAIN_STATE_FOVEATION_FB,
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
        pub unsafe fn from_raw(inner: sys::SwapchainStateFoveationFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SwapchainStateFoveationFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SwapchainStateFoveationFB {
            &self.inner
        }
        #[inline]
        pub fn flags(mut self, value: SwapchainStateFoveationFlagsFB) -> Self {
            self.inner.flags = value;
            self
        }
        #[inline]
        pub fn profile(mut self, value: &'a FoveationProfileFB) -> Self {
            self.inner.profile = value.as_raw();
            self
        }
    }
    impl<'a> Deref for SwapchainStateFoveationFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SwapchainStateFoveationFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SpaceQueryInfoBase<'a> {
        _inner: sys::SpaceQueryInfoBaseHeaderFB,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceQueryInfoFB<'a> {
        inner: sys::SpaceQueryInfoFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SpaceQueryInfoFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceQueryInfoFB {
                    ty: sys::StructureType::SPACE_QUERY_INFO_FB,
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
        pub unsafe fn from_raw(inner: sys::SpaceQueryInfoFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceQueryInfoFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceQueryInfoFB {
            &self.inner
        }
        #[inline]
        pub fn query_action(mut self, value: SpaceQueryActionFB) -> Self {
            self.inner.query_action = value;
            self
        }
        #[inline]
        pub fn max_result_count(mut self, value: u32) -> Self {
            self.inner.max_result_count = value;
            self
        }
        #[inline]
        pub fn timeout(mut self, value: Duration) -> Self {
            self.inner.timeout = value;
            self
        }
        #[inline]
        pub fn filter(mut self, value: &'a SpaceFilterInfoBase<'a>) -> Self {
            self.inner.filter = value as *const _ as _;
            self
        }
        #[inline]
        pub fn exclude_filter(mut self, value: &'a SpaceFilterInfoBase<'a>) -> Self {
            self.inner.exclude_filter = value as *const _ as _;
            self
        }
    }
    impl<'a> Deref for SpaceQueryInfoFB<'a> {
        type Target = SpaceQueryInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SpaceQueryInfoFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SpaceFilterInfoBase<'a> {
        _inner: sys::SpaceFilterInfoBaseHeaderFB,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceUuidFilterInfoFB<'a> {
        inner: sys::SpaceUuidFilterInfoFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SpaceUuidFilterInfoFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceUuidFilterInfoFB {
                    ty: sys::StructureType::SPACE_UUID_FILTER_INFO_FB,
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
        pub unsafe fn from_raw(inner: sys::SpaceUuidFilterInfoFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceUuidFilterInfoFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceUuidFilterInfoFB {
            &self.inner
        }
        #[inline]
        pub fn uuids(mut self, value: &'a [UuidEXT]) -> Self {
            self.inner.uuids = value.as_ptr() as *const _ as _;
            self.inner.uuid_count = value.len() as u32;
            self
        }
    }
    impl<'a> Deref for SpaceUuidFilterInfoFB<'a> {
        type Target = SpaceFilterInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SpaceUuidFilterInfoFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceComponentFilterInfoFB<'a> {
        inner: sys::SpaceComponentFilterInfoFB,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> SpaceComponentFilterInfoFB<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceComponentFilterInfoFB {
                    ty: sys::StructureType::SPACE_COMPONENT_FILTER_INFO_FB,
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
        pub unsafe fn from_raw(inner: sys::SpaceComponentFilterInfoFB) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceComponentFilterInfoFB {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceComponentFilterInfoFB {
            &self.inner
        }
        #[inline]
        pub fn component_type(mut self, value: SpaceComponentTypeFB) -> Self {
            self.inner.component_type = value;
            self
        }
    }
    impl<'a> Deref for SpaceComponentFilterInfoFB<'a> {
        type Target = SpaceFilterInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for SpaceComponentFilterInfoFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct FutureCompletionBase<'a> {
        _inner: sys::FutureCompletionBaseHeaderEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct FutureCompletionEXT<'a> {
        inner: sys::FutureCompletionEXT,
        _marker: PhantomData<&'a ()>,
    }
    impl<'a> FutureCompletionEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::FutureCompletionEXT {
                    ty: sys::StructureType::FUTURE_COMPLETION_EXT,
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
        pub unsafe fn from_raw(inner: sys::FutureCompletionEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::FutureCompletionEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::FutureCompletionEXT {
            &self.inner
        }
        #[inline]
        pub fn future_result(mut self, value: sys::Result) -> Self {
            self.inner.future_result = value;
            self
        }
    }
    impl<'a> Deref for FutureCompletionEXT<'a> {
        type Target = FutureCompletionBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    impl<'a> Default for FutureCompletionEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
}

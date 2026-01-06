#![doc = r" Automatically generated code; do not edit!"]
#![allow(
    clippy::wrong_self_convention,
    clippy::transmute_ptr_to_ptr,
    clippy::missing_transmute_annotations
)]
use crate::*;
use std::iter::FromIterator;
use std::mem::MaybeUninit;
pub use sys::platform::{
    EGLenum, VkComponentSwizzle, VkFilter, VkSamplerAddressMode, VkSamplerMipmapMode,
};
pub use sys::{
    ActionType, AnchorPersistStateANDROID, AndroidSurfaceSwapchainFlagsFB, AndroidThreadTypeKHR,
    BlendFactorFB, BodyJointBD, BodyJointConfidenceHTC, BodyJointFB, BodyJointHTC,
    BodyJointLocationBD, BodyJointLocationFB, BodyJointLocationHTC, BodyJointSetBD, BodyJointSetFB,
    BodyJointSetHTC, BodySkeletonJointFB, BodySkeletonJointHTC, BodyTrackingCalibrationStateMETA,
    Boxf, Color3f, Color4f, ColorSpaceFB, CompareOpFB, CompositionLayerFlags,
    CompositionLayerImageLayoutFlagsFB, CompositionLayerSecureContentFlagsFB,
    CompositionLayerSettingsFlagsFB, CompositionLayerSpaceWarpInfoFlagsFB,
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DigitalLensControlFlagsALMALENCE, EnvironmentBlendMode,
    EnvironmentDepthProviderCreateFlagsMETA, EnvironmentDepthSwapchainCreateFlagsMETA, Extent2Df,
    Extent2Di, Extent3Df, ExternalCameraAttachedToDeviceOCULUS, ExternalCameraExtrinsicsOCULUS,
    ExternalCameraIntrinsicsOCULUS, ExternalCameraStatusFlagsOCULUS, EyeCalibrationStatusML,
    EyeExpressionHTC, EyePositionFB, EyeVisibility, FaceConfidence2FB, FaceConfidenceFB,
    FaceConfidenceRegionsANDROID, FaceExpression2FB, FaceExpressionBD, FaceExpressionFB,
    FaceExpressionSet2FB, FaceExpressionSetFB, FaceParameterIndicesANDROID,
    FaceTrackingDataSource2FB, FaceTrackingStateANDROID, FacialBlendShapeML,
    FacialExpressionBlendShapePropertiesFlagsML, FacialSimulationModeBD, FacialTrackingTypeHTC,
    ForceFeedbackCurlApplyLocationMNDX, ForceFeedbackCurlLocationMNDX, FormFactor,
    FoveationConfigurationHTC, FoveationDynamicFB, FoveationDynamicFlagsHTC,
    FoveationEyeTrackedProfileCreateFlagsMETA, FoveationEyeTrackedStateFlagsMETA, FoveationLevelFB,
    FoveationLevelHTC, FoveationModeHTC, Fovf, FrameEndInfoFlagsML, FrameSynthesisInfoFlagsEXT,
    Frustumf, FullBodyJointMETA, FutureStateEXT, GlobalDimmerFrameEndInfoFlagsML, HandEXT,
    HandForearmJointULTRALEAP, HandJointEXT, HandJointLocationEXT, HandJointSetEXT,
    HandJointVelocityEXT, HandJointsMotionRangeEXT, HandMeshVertexMSFT, HandPoseTypeMSFT,
    HandTrackingAimFlagsFB, HandTrackingDataSourceEXT, HeadsetFitStatusML,
    InputSourceLocalizedNameFlags, InstanceCreateFlags, KeyboardTrackingFlagsFB,
    KeyboardTrackingQueryFlagsFB, LipExpressionBD, LipExpressionHTC, LoaderInterfaceStructs,
    LocalDimmingModeMETA, LocalizationMapConfidenceML, LocalizationMapErrorFlagsML,
    LocalizationMapStateML, LocalizationMapTypeML, MarkerAprilTagDictML, MarkerArucoDictML,
    MarkerDetectorCameraML, MarkerDetectorCornerRefineMethodML, MarkerDetectorFpsML,
    MarkerDetectorFullAnalysisIntervalML, MarkerDetectorProfileML, MarkerDetectorResolutionML,
    MarkerDetectorStatusML, MarkerTypeML, MeshComputeLodMSFT, NegotiateApiLayerRequest,
    NegotiateLoaderInfo, NegotiateRuntimeRequest, ObjectLabelANDROID, ObjectType, Offset2Df,
    Offset2Di, Offset3DfFB, OverlayMainSessionFlagsEXTX, OverlaySessionCreateFlagsEXTX,
    PassthroughCameraStateANDROID, PassthroughCapabilityFlagsFB, PassthroughColorLutChannelsMETA,
    PassthroughFlagsFB, PassthroughFormHTC, PassthroughLayerPurposeFB,
    PassthroughPreferenceFlagsMETA, PassthroughStateChangedFlagsFB, PerfSettingsDomainEXT,
    PerfSettingsLevelEXT, PerfSettingsNotificationLevelEXT, PerfSettingsSubDomainEXT,
    PerformanceMetricsCounterFlagsMETA, PerformanceMetricsCounterUnitMETA, PersistenceLocationBD,
    PlaneDetectionCapabilityFlagsEXT, PlaneDetectionStateEXT, PlaneDetectorFlagsEXT,
    PlaneDetectorOrientationEXT, PlaneDetectorSemanticTypeEXT, PlaneLabelANDROID,
    PlaneOrientationBD, PlaneTypeANDROID, Posef, Quaternionf, Rect2Df, Rect2Di, Rect3DfFB,
    ReferenceSpaceType, RenderModelFlagsFB, ReprojectionModeMSFT, SceneComponentTypeMSFT,
    SceneComputeConsistencyMSFT, SceneComputeFeatureMSFT, SceneComputeStateMSFT,
    SceneMarkerQRCodeSymbolTypeMSFT, SceneMarkerTypeMSFT, SceneObjectTypeMSFT,
    ScenePlaneAlignmentTypeMSFT, SemanticLabelBD, SemanticLabelsSupportFlagsFB,
    SenseDataProviderStateBD, SenseDataProviderTypeBD, SessionCreateFlags, SessionState,
    SpaceComponentTypeFB, SpaceLocationData, SpaceLocationFlags, SpacePersistenceModeFB,
    SpaceQueryActionFB, SpaceStorageLocationFB, SpaceVelocityData, SpaceVelocityFlags,
    SpatialAnchorCompletionResultML, SpatialAnchorConfidenceML, SpatialBounded2DDataEXT,
    SpatialBufferEXT, SpatialBufferTypeEXT, SpatialCapabilityEXT, SpatialCapabilityFeatureEXT,
    SpatialComponentTypeEXT, SpatialEntityComponentTypeBD, SpatialEntityTrackingStateEXT,
    SpatialGraphNodeTypeMSFT, SpatialMarkerAprilTagDictEXT, SpatialMarkerArucoDictEXT,
    SpatialMarkerDataEXT, SpatialMeshConfigFlagsBD, SpatialMeshDataEXT, SpatialMeshLodBD,
    SpatialPersistenceContextResultEXT, SpatialPersistenceScopeEXT, SpatialPersistenceStateEXT,
    SpatialPlaneAlignmentEXT, SpatialPlaneSemanticLabelEXT, SpatialPolygon2DDataEXT, Spheref,
    StructureType, SwapchainCreateFlags, SwapchainCreateFoveationFlagsFB,
    SwapchainStateFoveationFlagsFB, SwapchainUsageFlags, SystemGraphicsProperties,
    TrackableMarkerDictionaryANDROID, TrackableMarkerTrackingModeANDROID, TrackableTypeANDROID,
    TrackingOptimizationSettingsDomainQCOM, TrackingOptimizationSettingsHintQCOM,
    TrackingStateANDROID, TriangleMeshFlagsFB, Vector2f, Vector3f, Vector4f, Vector4sFB,
    ViewConfigurationType, ViewStateFlags, VirtualKeyboardInputSourceMETA,
    VirtualKeyboardInputStateFlagsMETA, VirtualKeyboardLocationTypeMETA, VisibilityMaskTypeKHR,
    VulkanDeviceCreateFlagsKHR, VulkanInstanceCreateFlagsKHR, WindingOrderFB,
    WorldMeshBlockResultML, WorldMeshBlockStatusML, WorldMeshDetectorFlagsML,
    WorldMeshDetectorLodML,
};
#[doc = r" A subset of known extensions"]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[non_exhaustive]
pub struct ExtensionSet {
    pub almalence_digital_lens_control: bool,
    #[cfg(target_os = "android")]
    pub android_trackables: bool,
    #[cfg(target_os = "android")]
    pub android_device_anchor_persistence: bool,
    #[cfg(target_os = "android")]
    pub android_face_tracking: bool,
    #[cfg(target_os = "android")]
    pub android_passthrough_camera_state: bool,
    #[cfg(target_os = "android")]
    pub android_raycast: bool,
    #[cfg(target_os = "android")]
    pub android_trackables_object: bool,
    #[cfg(target_os = "android")]
    pub android_anchor_sharing_export: bool,
    #[cfg(target_os = "android")]
    pub android_trackables_marker: bool,
    pub bd_controller_interaction: bool,
    pub bd_body_tracking: bool,
    pub bd_facial_simulation: bool,
    pub bd_spatial_sensing: bool,
    pub bd_spatial_anchor: bool,
    pub bd_spatial_anchor_sharing: bool,
    pub bd_spatial_scene: bool,
    pub bd_spatial_mesh: bool,
    pub bd_future_progress: bool,
    pub bd_spatial_plane: bool,
    pub bd_ultra_controller_interaction: bool,
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
    pub ext_frame_synthesis: bool,
    pub ext_uuid: bool,
    pub ext_render_model: bool,
    pub ext_interaction_render_model: bool,
    pub ext_hand_interaction: bool,
    pub ext_active_action_set_priority: bool,
    pub ext_local_floor: bool,
    pub ext_hand_tracking_data_source: bool,
    pub ext_plane_detection: bool,
    pub ext_future: bool,
    pub ext_user_presence: bool,
    pub ext_composition_layer_inverted_alpha: bool,
    pub ext_spatial_entity: bool,
    pub ext_spatial_plane_tracking: bool,
    pub ext_spatial_marker_tracking: bool,
    pub ext_spatial_anchor: bool,
    pub ext_spatial_persistence: bool,
    pub ext_spatial_persistence_operations: bool,
    pub ext_loader_init_properties: bool,
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
    pub htc_body_tracking: bool,
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
    pub khr_extended_struct_name_lengths: bool,
    pub khr_swapchain_usage_input_attachment_bit: bool,
    pub khr_locate_spaces: bool,
    pub khr_maintenance1: bool,
    pub khr_generic_controller: bool,
    pub logitech_mx_ink_stylus_interaction: bool,
    pub meta_foveation_eye_tracked: bool,
    pub meta_local_dimming: bool,
    pub meta_passthrough_preferences: bool,
    pub meta_virtual_keyboard: bool,
    pub meta_vulkan_swapchain_create_info: bool,
    pub meta_performance_metrics: bool,
    pub meta_detached_controllers: bool,
    pub meta_headset_id: bool,
    pub meta_spatial_entity_discovery: bool,
    pub meta_hand_tracking_microgestures: bool,
    pub meta_recommended_layer_resolution: bool,
    pub meta_spatial_entity_persistence: bool,
    pub meta_passthrough_color_lut: bool,
    pub meta_spatial_entity_mesh: bool,
    pub meta_automatic_layer_filter: bool,
    pub meta_body_tracking_full_body: bool,
    pub meta_touch_controller_plus: bool,
    pub meta_passthrough_layer_resumed_event: bool,
    pub meta_body_tracking_calibration: bool,
    pub meta_spatial_entity_sharing: bool,
    pub meta_environment_depth: bool,
    pub meta_simultaneous_hands_and_controllers: bool,
    pub meta_colocation_discovery: bool,
    pub meta_spatial_entity_group_sharing: bool,
    pub ml_ml2_controller_interaction: bool,
    pub ml_frame_end_info: bool,
    pub ml_global_dimmer: bool,
    pub ml_compat: bool,
    pub ml_marker_understanding: bool,
    pub ml_localization_map: bool,
    pub ml_spatial_anchors: bool,
    pub ml_spatial_anchors_storage: bool,
    pub ml_user_calibration: bool,
    pub ml_system_notifications: bool,
    pub ml_world_mesh_detection: bool,
    pub ml_facial_expression: bool,
    pub ml_view_configuration_depth_range_change: bool,
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
    pub other: Vec<Vec<u8>>,
}
#[doc = r" Create a ExtensionSet from a list of nul-terminated extension names"]
impl<'a> FromIterator<&'a [u8]> for ExtensionSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a [u8]>,
    {
        let mut out = Self::default();
        for name in iter {
            match name {
                raw::DigitalLensControlALMALENCE::NAME => {
                    out.almalence_digital_lens_control = true;
                }
                #[cfg(target_os = "android")]
                raw::TrackablesANDROID::NAME => {
                    out.android_trackables = true;
                }
                #[cfg(target_os = "android")]
                raw::DeviceAnchorPersistenceANDROID::NAME => {
                    out.android_device_anchor_persistence = true;
                }
                #[cfg(target_os = "android")]
                raw::FaceTrackingANDROID::NAME => {
                    out.android_face_tracking = true;
                }
                #[cfg(target_os = "android")]
                raw::PassthroughCameraStateANDROID::NAME => {
                    out.android_passthrough_camera_state = true;
                }
                #[cfg(target_os = "android")]
                raw::RaycastANDROID::NAME => {
                    out.android_raycast = true;
                }
                #[cfg(target_os = "android")]
                raw::TrackablesObjectANDROID::NAME => {
                    out.android_trackables_object = true;
                }
                #[cfg(target_os = "android")]
                raw::AnchorSharingExportANDROID::NAME => {
                    out.android_anchor_sharing_export = true;
                }
                #[cfg(target_os = "android")]
                raw::TrackablesMarkerANDROID::NAME => {
                    out.android_trackables_marker = true;
                }
                raw::ControllerInteractionBD::NAME => {
                    out.bd_controller_interaction = true;
                }
                raw::BodyTrackingBD::NAME => {
                    out.bd_body_tracking = true;
                }
                raw::FacialSimulationBD::NAME => {
                    out.bd_facial_simulation = true;
                }
                raw::SpatialSensingBD::NAME => {
                    out.bd_spatial_sensing = true;
                }
                raw::SpatialAnchorBD::NAME => {
                    out.bd_spatial_anchor = true;
                }
                raw::SpatialAnchorSharingBD::NAME => {
                    out.bd_spatial_anchor_sharing = true;
                }
                raw::SpatialSceneBD::NAME => {
                    out.bd_spatial_scene = true;
                }
                raw::SpatialMeshBD::NAME => {
                    out.bd_spatial_mesh = true;
                }
                raw::FutureProgressBD::NAME => {
                    out.bd_future_progress = true;
                }
                raw::SpatialPlaneBD::NAME => {
                    out.bd_spatial_plane = true;
                }
                raw::UltraControllerInteractionBD::NAME => {
                    out.bd_ultra_controller_interaction = true;
                }
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
                raw::DpadBindingEXT::NAME => {
                    out.ext_dpad_binding = true;
                }
                raw::HandJointsMotionRangeEXT::NAME => {
                    out.ext_hand_joints_motion_range = true;
                }
                raw::SamsungOdysseyControllerEXT::NAME => {
                    out.ext_samsung_odyssey_controller = true;
                }
                raw::HpMixedRealityControllerEXT::NAME => {
                    out.ext_hp_mixed_reality_controller = true;
                }
                raw::PalmPoseEXT::NAME => {
                    out.ext_palm_pose = true;
                }
                raw::FrameSynthesisEXT::NAME => {
                    out.ext_frame_synthesis = true;
                }
                raw::UuidEXT::NAME => {
                    out.ext_uuid = true;
                }
                raw::RenderModelEXT::NAME => {
                    out.ext_render_model = true;
                }
                raw::InteractionRenderModelEXT::NAME => {
                    out.ext_interaction_render_model = true;
                }
                raw::HandInteractionEXT::NAME => {
                    out.ext_hand_interaction = true;
                }
                raw::ActiveActionSetPriorityEXT::NAME => {
                    out.ext_active_action_set_priority = true;
                }
                raw::LocalFloorEXT::NAME => {
                    out.ext_local_floor = true;
                }
                raw::HandTrackingDataSourceEXT::NAME => {
                    out.ext_hand_tracking_data_source = true;
                }
                raw::PlaneDetectionEXT::NAME => {
                    out.ext_plane_detection = true;
                }
                raw::FutureEXT::NAME => {
                    out.ext_future = true;
                }
                raw::UserPresenceEXT::NAME => {
                    out.ext_user_presence = true;
                }
                raw::CompositionLayerInvertedAlphaEXT::NAME => {
                    out.ext_composition_layer_inverted_alpha = true;
                }
                raw::SpatialEntityEXT::NAME => {
                    out.ext_spatial_entity = true;
                }
                raw::SpatialPlaneTrackingEXT::NAME => {
                    out.ext_spatial_plane_tracking = true;
                }
                raw::SpatialMarkerTrackingEXT::NAME => {
                    out.ext_spatial_marker_tracking = true;
                }
                raw::SpatialAnchorEXT::NAME => {
                    out.ext_spatial_anchor = true;
                }
                raw::SpatialPersistenceEXT::NAME => {
                    out.ext_spatial_persistence = true;
                }
                raw::SpatialPersistenceOperationsEXT::NAME => {
                    out.ext_spatial_persistence_operations = true;
                }
                raw::LoaderInitPropertiesEXT::NAME => {
                    out.ext_loader_init_properties = true;
                }
                raw::CompositionLayerImageLayoutFB::NAME => {
                    out.fb_composition_layer_image_layout = true;
                }
                raw::CompositionLayerAlphaBlendFB::NAME => {
                    out.fb_composition_layer_alpha_blend = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSurfaceSwapchainCreateFB::NAME => {
                    out.fb_android_surface_swapchain_create = true;
                }
                raw::SwapchainUpdateStateFB::NAME => {
                    out.fb_swapchain_update_state = true;
                }
                raw::CompositionLayerSecureContentFB::NAME => {
                    out.fb_composition_layer_secure_content = true;
                }
                raw::BodyTrackingFB::NAME => {
                    out.fb_body_tracking = true;
                }
                raw::DisplayRefreshRateFB::NAME => {
                    out.fb_display_refresh_rate = true;
                }
                raw::ColorSpaceFB::NAME => {
                    out.fb_color_space = true;
                }
                raw::HandTrackingMeshFB::NAME => {
                    out.fb_hand_tracking_mesh = true;
                }
                raw::HandTrackingAimFB::NAME => {
                    out.fb_hand_tracking_aim = true;
                }
                raw::HandTrackingCapsulesFB::NAME => {
                    out.fb_hand_tracking_capsules = true;
                }
                raw::SpatialEntityFB::NAME => {
                    out.fb_spatial_entity = true;
                }
                raw::FoveationFB::NAME => {
                    out.fb_foveation = true;
                }
                raw::FoveationConfigurationFB::NAME => {
                    out.fb_foveation_configuration = true;
                }
                raw::KeyboardTrackingFB::NAME => {
                    out.fb_keyboard_tracking = true;
                }
                raw::TriangleMeshFB::NAME => {
                    out.fb_triangle_mesh = true;
                }
                raw::PassthroughFB::NAME => {
                    out.fb_passthrough = true;
                }
                raw::RenderModelFB::NAME => {
                    out.fb_render_model = true;
                }
                raw::SpatialEntityQueryFB::NAME => {
                    out.fb_spatial_entity_query = true;
                }
                raw::SpatialEntityStorageFB::NAME => {
                    out.fb_spatial_entity_storage = true;
                }
                raw::FoveationVulkanFB::NAME => {
                    out.fb_foveation_vulkan = true;
                }
                #[cfg(target_os = "android")]
                raw::SwapchainUpdateStateAndroidSurfaceFB::NAME => {
                    out.fb_swapchain_update_state_android_surface = true;
                }
                raw::SwapchainUpdateStateOpenglEsFB::NAME => {
                    out.fb_swapchain_update_state_opengl_es = true;
                }
                raw::SwapchainUpdateStateVulkanFB::NAME => {
                    out.fb_swapchain_update_state_vulkan = true;
                }
                raw::TouchControllerProFB::NAME => {
                    out.fb_touch_controller_pro = true;
                }
                raw::SpatialEntitySharingFB::NAME => {
                    out.fb_spatial_entity_sharing = true;
                }
                raw::SpaceWarpFB::NAME => {
                    out.fb_space_warp = true;
                }
                raw::HapticAmplitudeEnvelopeFB::NAME => {
                    out.fb_haptic_amplitude_envelope = true;
                }
                raw::SceneFB::NAME => {
                    out.fb_scene = true;
                }
                raw::SceneCaptureFB::NAME => {
                    out.fb_scene_capture = true;
                }
                raw::SpatialEntityContainerFB::NAME => {
                    out.fb_spatial_entity_container = true;
                }
                raw::FaceTrackingFB::NAME => {
                    out.fb_face_tracking = true;
                }
                raw::EyeTrackingSocialFB::NAME => {
                    out.fb_eye_tracking_social = true;
                }
                raw::PassthroughKeyboardHandsFB::NAME => {
                    out.fb_passthrough_keyboard_hands = true;
                }
                raw::CompositionLayerSettingsFB::NAME => {
                    out.fb_composition_layer_settings = true;
                }
                raw::TouchControllerProximityFB::NAME => {
                    out.fb_touch_controller_proximity = true;
                }
                raw::HapticPcmFB::NAME => {
                    out.fb_haptic_pcm = true;
                }
                raw::CompositionLayerDepthTestFB::NAME => {
                    out.fb_composition_layer_depth_test = true;
                }
                raw::SpatialEntityStorageBatchFB::NAME => {
                    out.fb_spatial_entity_storage_batch = true;
                }
                raw::SpatialEntityUserFB::NAME => {
                    out.fb_spatial_entity_user = true;
                }
                raw::FaceTracking2FB::NAME => {
                    out.fb_face_tracking2 = true;
                }
                raw::ViveCosmosControllerInteractionHTC::NAME => {
                    out.htc_vive_cosmos_controller_interaction = true;
                }
                raw::FacialTrackingHTC::NAME => {
                    out.htc_facial_tracking = true;
                }
                raw::ViveFocus3ControllerInteractionHTC::NAME => {
                    out.htc_vive_focus3_controller_interaction = true;
                }
                raw::HandInteractionHTC::NAME => {
                    out.htc_hand_interaction = true;
                }
                raw::ViveWristTrackerInteractionHTC::NAME => {
                    out.htc_vive_wrist_tracker_interaction = true;
                }
                raw::PassthroughHTC::NAME => {
                    out.htc_passthrough = true;
                }
                raw::FoveationHTC::NAME => {
                    out.htc_foveation = true;
                }
                raw::AnchorHTC::NAME => {
                    out.htc_anchor = true;
                }
                raw::BodyTrackingHTC::NAME => {
                    out.htc_body_tracking = true;
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
                #[cfg(target_vendor = "apple")]
                raw::MetalEnableKHR::NAME => {
                    out.khr_metal_enable = true;
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
                raw::BindingModificationKHR::NAME => {
                    out.khr_binding_modification = true;
                }
                raw::ExtendedStructNameLengthsKHR::NAME => {
                    out.khr_extended_struct_name_lengths = true;
                }
                raw::SwapchainUsageInputAttachmentBitKHR::NAME => {
                    out.khr_swapchain_usage_input_attachment_bit = true;
                }
                raw::LocateSpacesKHR::NAME => {
                    out.khr_locate_spaces = true;
                }
                raw::Maintenance1KHR::NAME => {
                    out.khr_maintenance1 = true;
                }
                raw::GenericControllerKHR::NAME => {
                    out.khr_generic_controller = true;
                }
                raw::MxInkStylusInteractionLOGITECH::NAME => {
                    out.logitech_mx_ink_stylus_interaction = true;
                }
                raw::FoveationEyeTrackedMETA::NAME => {
                    out.meta_foveation_eye_tracked = true;
                }
                raw::LocalDimmingMETA::NAME => {
                    out.meta_local_dimming = true;
                }
                raw::PassthroughPreferencesMETA::NAME => {
                    out.meta_passthrough_preferences = true;
                }
                raw::VirtualKeyboardMETA::NAME => {
                    out.meta_virtual_keyboard = true;
                }
                raw::VulkanSwapchainCreateInfoMETA::NAME => {
                    out.meta_vulkan_swapchain_create_info = true;
                }
                raw::PerformanceMetricsMETA::NAME => {
                    out.meta_performance_metrics = true;
                }
                raw::DetachedControllersMETA::NAME => {
                    out.meta_detached_controllers = true;
                }
                raw::HeadsetIdMETA::NAME => {
                    out.meta_headset_id = true;
                }
                raw::SpatialEntityDiscoveryMETA::NAME => {
                    out.meta_spatial_entity_discovery = true;
                }
                raw::HandTrackingMicrogesturesMETA::NAME => {
                    out.meta_hand_tracking_microgestures = true;
                }
                raw::RecommendedLayerResolutionMETA::NAME => {
                    out.meta_recommended_layer_resolution = true;
                }
                raw::SpatialEntityPersistenceMETA::NAME => {
                    out.meta_spatial_entity_persistence = true;
                }
                raw::PassthroughColorLutMETA::NAME => {
                    out.meta_passthrough_color_lut = true;
                }
                raw::SpatialEntityMeshMETA::NAME => {
                    out.meta_spatial_entity_mesh = true;
                }
                raw::AutomaticLayerFilterMETA::NAME => {
                    out.meta_automatic_layer_filter = true;
                }
                raw::BodyTrackingFullBodyMETA::NAME => {
                    out.meta_body_tracking_full_body = true;
                }
                raw::TouchControllerPlusMETA::NAME => {
                    out.meta_touch_controller_plus = true;
                }
                raw::PassthroughLayerResumedEventMETA::NAME => {
                    out.meta_passthrough_layer_resumed_event = true;
                }
                raw::BodyTrackingCalibrationMETA::NAME => {
                    out.meta_body_tracking_calibration = true;
                }
                raw::SpatialEntitySharingMETA::NAME => {
                    out.meta_spatial_entity_sharing = true;
                }
                raw::EnvironmentDepthMETA::NAME => {
                    out.meta_environment_depth = true;
                }
                raw::SimultaneousHandsAndControllersMETA::NAME => {
                    out.meta_simultaneous_hands_and_controllers = true;
                }
                raw::ColocationDiscoveryMETA::NAME => {
                    out.meta_colocation_discovery = true;
                }
                raw::SpatialEntityGroupSharingMETA::NAME => {
                    out.meta_spatial_entity_group_sharing = true;
                }
                raw::Ml2ControllerInteractionML::NAME => {
                    out.ml_ml2_controller_interaction = true;
                }
                raw::FrameEndInfoML::NAME => {
                    out.ml_frame_end_info = true;
                }
                raw::GlobalDimmerML::NAME => {
                    out.ml_global_dimmer = true;
                }
                raw::CompatML::NAME => {
                    out.ml_compat = true;
                }
                raw::MarkerUnderstandingML::NAME => {
                    out.ml_marker_understanding = true;
                }
                raw::LocalizationMapML::NAME => {
                    out.ml_localization_map = true;
                }
                raw::SpatialAnchorsML::NAME => {
                    out.ml_spatial_anchors = true;
                }
                raw::SpatialAnchorsStorageML::NAME => {
                    out.ml_spatial_anchors_storage = true;
                }
                raw::UserCalibrationML::NAME => {
                    out.ml_user_calibration = true;
                }
                raw::SystemNotificationsML::NAME => {
                    out.ml_system_notifications = true;
                }
                raw::WorldMeshDetectionML::NAME => {
                    out.ml_world_mesh_detection = true;
                }
                raw::FacialExpressionML::NAME => {
                    out.ml_facial_expression = true;
                }
                raw::ViewConfigurationDepthRangeChangeML::NAME => {
                    out.ml_view_configuration_depth_range_change = true;
                }
                raw::HeadlessMND::NAME => {
                    out.mnd_headless = true;
                }
                raw::SwapchainUsageInputAttachmentBitMND::NAME => {
                    out.mnd_swapchain_usage_input_attachment_bit = true;
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
                raw::PerceptionAnchorInteropMSFT::NAME => {
                    out.msft_perception_anchor_interop = true;
                }
                #[cfg(windows)]
                raw::HolographicWindowAttachmentMSFT::NAME => {
                    out.msft_holographic_window_attachment = true;
                }
                raw::CompositionLayerReprojectionMSFT::NAME => {
                    out.msft_composition_layer_reprojection = true;
                }
                raw::SpatialAnchorPersistenceMSFT::NAME => {
                    out.msft_spatial_anchor_persistence = true;
                }
                #[cfg(target_os = "android")]
                raw::AndroidSessionStateEnableOCULUS::NAME => {
                    out.oculus_android_session_state_enable = true;
                }
                raw::AudioDeviceGuidOCULUS::NAME => {
                    out.oculus_audio_device_guid = true;
                }
                raw::ExternalCameraOCULUS::NAME => {
                    out.oculus_external_camera = true;
                }
                raw::ControllerInteractionOPPO::NAME => {
                    out.oppo_controller_interaction = true;
                }
                raw::TrackingOptimizationSettingsQCOM::NAME => {
                    out.qcom_tracking_optimization_settings = true;
                }
                raw::HandTrackingForearmULTRALEAP::NAME => {
                    out.ultraleap_hand_tracking_forearm = true;
                }
                raw::AnalogThresholdVALVE::NAME => {
                    out.valve_analog_threshold = true;
                }
                raw::QuadViewsVARJO::NAME => {
                    out.varjo_quad_views = true;
                }
                raw::FoveatedRenderingVARJO::NAME => {
                    out.varjo_foveated_rendering = true;
                }
                raw::CompositionLayerDepthTestVARJO::NAME => {
                    out.varjo_composition_layer_depth_test = true;
                }
                raw::EnvironmentDepthEstimationVARJO::NAME => {
                    out.varjo_environment_depth_estimation = true;
                }
                raw::MarkerTrackingVARJO::NAME => {
                    out.varjo_marker_tracking = true;
                }
                raw::ViewOffsetVARJO::NAME => {
                    out.varjo_view_offset = true;
                }
                raw::Xr4ControllerInteractionVARJO::NAME => {
                    out.varjo_xr4_controller_interaction = true;
                }
                raw::ControllerInteractionYVR::NAME => {
                    out.yvr_controller_interaction = true;
                }
                raw::OverlayEXTX::NAME => {
                    out.extx_overlay = true;
                }
                raw::EglEnableMNDX::NAME => {
                    out.mndx_egl_enable = true;
                }
                raw::ForceFeedbackCurlMNDX::NAME => {
                    out.mndx_force_feedback_curl = true;
                }
                raw::ViveTrackerInteractionHTCX::NAME => {
                    out.htcx_vive_tracker_interaction = true;
                }
                bytes => out.other.push(bytes.to_vec()),
            }
        }
        out
    }
}
impl ExtensionSet {
    #[doc = r" Return `self` without the members set in `other`."]
    #[inline]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            almalence_digital_lens_control: self.almalence_digital_lens_control
                && !other.almalence_digital_lens_control,
            #[cfg(target_os = "android")]
            android_trackables: self.android_trackables && !other.android_trackables,
            #[cfg(target_os = "android")]
            android_device_anchor_persistence: self.android_device_anchor_persistence
                && !other.android_device_anchor_persistence,
            #[cfg(target_os = "android")]
            android_face_tracking: self.android_face_tracking && !other.android_face_tracking,
            #[cfg(target_os = "android")]
            android_passthrough_camera_state: self.android_passthrough_camera_state
                && !other.android_passthrough_camera_state,
            #[cfg(target_os = "android")]
            android_raycast: self.android_raycast && !other.android_raycast,
            #[cfg(target_os = "android")]
            android_trackables_object: self.android_trackables_object
                && !other.android_trackables_object,
            #[cfg(target_os = "android")]
            android_anchor_sharing_export: self.android_anchor_sharing_export
                && !other.android_anchor_sharing_export,
            #[cfg(target_os = "android")]
            android_trackables_marker: self.android_trackables_marker
                && !other.android_trackables_marker,
            bd_controller_interaction: self.bd_controller_interaction
                && !other.bd_controller_interaction,
            bd_body_tracking: self.bd_body_tracking && !other.bd_body_tracking,
            bd_facial_simulation: self.bd_facial_simulation && !other.bd_facial_simulation,
            bd_spatial_sensing: self.bd_spatial_sensing && !other.bd_spatial_sensing,
            bd_spatial_anchor: self.bd_spatial_anchor && !other.bd_spatial_anchor,
            bd_spatial_anchor_sharing: self.bd_spatial_anchor_sharing
                && !other.bd_spatial_anchor_sharing,
            bd_spatial_scene: self.bd_spatial_scene && !other.bd_spatial_scene,
            bd_spatial_mesh: self.bd_spatial_mesh && !other.bd_spatial_mesh,
            bd_future_progress: self.bd_future_progress && !other.bd_future_progress,
            bd_spatial_plane: self.bd_spatial_plane && !other.bd_spatial_plane,
            bd_ultra_controller_interaction: self.bd_ultra_controller_interaction
                && !other.bd_ultra_controller_interaction,
            epic_view_configuration_fov: self.epic_view_configuration_fov
                && !other.epic_view_configuration_fov,
            ext_performance_settings: self.ext_performance_settings
                && !other.ext_performance_settings,
            ext_thermal_query: self.ext_thermal_query && !other.ext_thermal_query,
            ext_debug_utils: self.ext_debug_utils && !other.ext_debug_utils,
            ext_eye_gaze_interaction: self.ext_eye_gaze_interaction
                && !other.ext_eye_gaze_interaction,
            ext_view_configuration_depth_range: self.ext_view_configuration_depth_range
                && !other.ext_view_configuration_depth_range,
            ext_conformance_automation: self.ext_conformance_automation
                && !other.ext_conformance_automation,
            ext_hand_tracking: self.ext_hand_tracking && !other.ext_hand_tracking,
            #[cfg(windows)]
            ext_win32_appcontainer_compatible: self.ext_win32_appcontainer_compatible
                && !other.ext_win32_appcontainer_compatible,
            ext_dpad_binding: self.ext_dpad_binding && !other.ext_dpad_binding,
            ext_hand_joints_motion_range: self.ext_hand_joints_motion_range
                && !other.ext_hand_joints_motion_range,
            ext_samsung_odyssey_controller: self.ext_samsung_odyssey_controller
                && !other.ext_samsung_odyssey_controller,
            ext_hp_mixed_reality_controller: self.ext_hp_mixed_reality_controller
                && !other.ext_hp_mixed_reality_controller,
            ext_palm_pose: self.ext_palm_pose && !other.ext_palm_pose,
            ext_frame_synthesis: self.ext_frame_synthesis && !other.ext_frame_synthesis,
            ext_uuid: self.ext_uuid && !other.ext_uuid,
            ext_render_model: self.ext_render_model && !other.ext_render_model,
            ext_interaction_render_model: self.ext_interaction_render_model
                && !other.ext_interaction_render_model,
            ext_hand_interaction: self.ext_hand_interaction && !other.ext_hand_interaction,
            ext_active_action_set_priority: self.ext_active_action_set_priority
                && !other.ext_active_action_set_priority,
            ext_local_floor: self.ext_local_floor && !other.ext_local_floor,
            ext_hand_tracking_data_source: self.ext_hand_tracking_data_source
                && !other.ext_hand_tracking_data_source,
            ext_plane_detection: self.ext_plane_detection && !other.ext_plane_detection,
            ext_future: self.ext_future && !other.ext_future,
            ext_user_presence: self.ext_user_presence && !other.ext_user_presence,
            ext_composition_layer_inverted_alpha: self.ext_composition_layer_inverted_alpha
                && !other.ext_composition_layer_inverted_alpha,
            ext_spatial_entity: self.ext_spatial_entity && !other.ext_spatial_entity,
            ext_spatial_plane_tracking: self.ext_spatial_plane_tracking
                && !other.ext_spatial_plane_tracking,
            ext_spatial_marker_tracking: self.ext_spatial_marker_tracking
                && !other.ext_spatial_marker_tracking,
            ext_spatial_anchor: self.ext_spatial_anchor && !other.ext_spatial_anchor,
            ext_spatial_persistence: self.ext_spatial_persistence && !other.ext_spatial_persistence,
            ext_spatial_persistence_operations: self.ext_spatial_persistence_operations
                && !other.ext_spatial_persistence_operations,
            ext_loader_init_properties: self.ext_loader_init_properties
                && !other.ext_loader_init_properties,
            fb_composition_layer_image_layout: self.fb_composition_layer_image_layout
                && !other.fb_composition_layer_image_layout,
            fb_composition_layer_alpha_blend: self.fb_composition_layer_alpha_blend
                && !other.fb_composition_layer_alpha_blend,
            #[cfg(target_os = "android")]
            fb_android_surface_swapchain_create: self.fb_android_surface_swapchain_create
                && !other.fb_android_surface_swapchain_create,
            fb_swapchain_update_state: self.fb_swapchain_update_state
                && !other.fb_swapchain_update_state,
            fb_composition_layer_secure_content: self.fb_composition_layer_secure_content
                && !other.fb_composition_layer_secure_content,
            fb_body_tracking: self.fb_body_tracking && !other.fb_body_tracking,
            fb_display_refresh_rate: self.fb_display_refresh_rate && !other.fb_display_refresh_rate,
            fb_color_space: self.fb_color_space && !other.fb_color_space,
            fb_hand_tracking_mesh: self.fb_hand_tracking_mesh && !other.fb_hand_tracking_mesh,
            fb_hand_tracking_aim: self.fb_hand_tracking_aim && !other.fb_hand_tracking_aim,
            fb_hand_tracking_capsules: self.fb_hand_tracking_capsules
                && !other.fb_hand_tracking_capsules,
            fb_spatial_entity: self.fb_spatial_entity && !other.fb_spatial_entity,
            fb_foveation: self.fb_foveation && !other.fb_foveation,
            fb_foveation_configuration: self.fb_foveation_configuration
                && !other.fb_foveation_configuration,
            fb_keyboard_tracking: self.fb_keyboard_tracking && !other.fb_keyboard_tracking,
            fb_triangle_mesh: self.fb_triangle_mesh && !other.fb_triangle_mesh,
            fb_passthrough: self.fb_passthrough && !other.fb_passthrough,
            fb_render_model: self.fb_render_model && !other.fb_render_model,
            fb_spatial_entity_query: self.fb_spatial_entity_query && !other.fb_spatial_entity_query,
            fb_spatial_entity_storage: self.fb_spatial_entity_storage
                && !other.fb_spatial_entity_storage,
            fb_foveation_vulkan: self.fb_foveation_vulkan && !other.fb_foveation_vulkan,
            #[cfg(target_os = "android")]
            fb_swapchain_update_state_android_surface: self
                .fb_swapchain_update_state_android_surface
                && !other.fb_swapchain_update_state_android_surface,
            fb_swapchain_update_state_opengl_es: self.fb_swapchain_update_state_opengl_es
                && !other.fb_swapchain_update_state_opengl_es,
            fb_swapchain_update_state_vulkan: self.fb_swapchain_update_state_vulkan
                && !other.fb_swapchain_update_state_vulkan,
            fb_touch_controller_pro: self.fb_touch_controller_pro && !other.fb_touch_controller_pro,
            fb_spatial_entity_sharing: self.fb_spatial_entity_sharing
                && !other.fb_spatial_entity_sharing,
            fb_space_warp: self.fb_space_warp && !other.fb_space_warp,
            fb_haptic_amplitude_envelope: self.fb_haptic_amplitude_envelope
                && !other.fb_haptic_amplitude_envelope,
            fb_scene: self.fb_scene && !other.fb_scene,
            fb_scene_capture: self.fb_scene_capture && !other.fb_scene_capture,
            fb_spatial_entity_container: self.fb_spatial_entity_container
                && !other.fb_spatial_entity_container,
            fb_face_tracking: self.fb_face_tracking && !other.fb_face_tracking,
            fb_eye_tracking_social: self.fb_eye_tracking_social && !other.fb_eye_tracking_social,
            fb_passthrough_keyboard_hands: self.fb_passthrough_keyboard_hands
                && !other.fb_passthrough_keyboard_hands,
            fb_composition_layer_settings: self.fb_composition_layer_settings
                && !other.fb_composition_layer_settings,
            fb_touch_controller_proximity: self.fb_touch_controller_proximity
                && !other.fb_touch_controller_proximity,
            fb_haptic_pcm: self.fb_haptic_pcm && !other.fb_haptic_pcm,
            fb_composition_layer_depth_test: self.fb_composition_layer_depth_test
                && !other.fb_composition_layer_depth_test,
            fb_spatial_entity_storage_batch: self.fb_spatial_entity_storage_batch
                && !other.fb_spatial_entity_storage_batch,
            fb_spatial_entity_user: self.fb_spatial_entity_user && !other.fb_spatial_entity_user,
            fb_face_tracking2: self.fb_face_tracking2 && !other.fb_face_tracking2,
            htc_vive_cosmos_controller_interaction: self.htc_vive_cosmos_controller_interaction
                && !other.htc_vive_cosmos_controller_interaction,
            htc_facial_tracking: self.htc_facial_tracking && !other.htc_facial_tracking,
            htc_vive_focus3_controller_interaction: self.htc_vive_focus3_controller_interaction
                && !other.htc_vive_focus3_controller_interaction,
            htc_hand_interaction: self.htc_hand_interaction && !other.htc_hand_interaction,
            htc_vive_wrist_tracker_interaction: self.htc_vive_wrist_tracker_interaction
                && !other.htc_vive_wrist_tracker_interaction,
            htc_passthrough: self.htc_passthrough && !other.htc_passthrough,
            htc_foveation: self.htc_foveation && !other.htc_foveation,
            htc_anchor: self.htc_anchor && !other.htc_anchor,
            htc_body_tracking: self.htc_body_tracking && !other.htc_body_tracking,
            huawei_controller_interaction: self.huawei_controller_interaction
                && !other.huawei_controller_interaction,
            #[cfg(target_os = "android")]
            khr_android_thread_settings: self.khr_android_thread_settings
                && !other.khr_android_thread_settings,
            #[cfg(target_os = "android")]
            khr_android_surface_swapchain: self.khr_android_surface_swapchain
                && !other.khr_android_surface_swapchain,
            khr_composition_layer_cube: self.khr_composition_layer_cube
                && !other.khr_composition_layer_cube,
            #[cfg(target_os = "android")]
            khr_android_create_instance: self.khr_android_create_instance
                && !other.khr_android_create_instance,
            khr_composition_layer_depth: self.khr_composition_layer_depth
                && !other.khr_composition_layer_depth,
            khr_vulkan_swapchain_format_list: self.khr_vulkan_swapchain_format_list
                && !other.khr_vulkan_swapchain_format_list,
            khr_composition_layer_cylinder: self.khr_composition_layer_cylinder
                && !other.khr_composition_layer_cylinder,
            khr_composition_layer_equirect: self.khr_composition_layer_equirect
                && !other.khr_composition_layer_equirect,
            khr_opengl_enable: self.khr_opengl_enable && !other.khr_opengl_enable,
            khr_opengl_es_enable: self.khr_opengl_es_enable && !other.khr_opengl_es_enable,
            khr_vulkan_enable: self.khr_vulkan_enable && !other.khr_vulkan_enable,
            #[cfg(windows)]
            khr_d3d11_enable: self.khr_d3d11_enable && !other.khr_d3d11_enable,
            #[cfg(windows)]
            khr_d3d12_enable: self.khr_d3d12_enable && !other.khr_d3d12_enable,
            #[cfg(target_vendor = "apple")]
            khr_metal_enable: self.khr_metal_enable && !other.khr_metal_enable,
            khr_visibility_mask: self.khr_visibility_mask && !other.khr_visibility_mask,
            khr_composition_layer_color_scale_bias: self.khr_composition_layer_color_scale_bias
                && !other.khr_composition_layer_color_scale_bias,
            #[cfg(windows)]
            khr_win32_convert_performance_counter_time: self
                .khr_win32_convert_performance_counter_time
                && !other.khr_win32_convert_performance_counter_time,
            khr_convert_timespec_time: self.khr_convert_timespec_time
                && !other.khr_convert_timespec_time,
            khr_loader_init: self.khr_loader_init && !other.khr_loader_init,
            #[cfg(target_os = "android")]
            khr_loader_init_android: self.khr_loader_init_android && !other.khr_loader_init_android,
            khr_vulkan_enable2: self.khr_vulkan_enable2 && !other.khr_vulkan_enable2,
            khr_composition_layer_equirect2: self.khr_composition_layer_equirect2
                && !other.khr_composition_layer_equirect2,
            khr_binding_modification: self.khr_binding_modification
                && !other.khr_binding_modification,
            khr_extended_struct_name_lengths: self.khr_extended_struct_name_lengths
                && !other.khr_extended_struct_name_lengths,
            khr_swapchain_usage_input_attachment_bit: self.khr_swapchain_usage_input_attachment_bit
                && !other.khr_swapchain_usage_input_attachment_bit,
            khr_locate_spaces: self.khr_locate_spaces && !other.khr_locate_spaces,
            khr_maintenance1: self.khr_maintenance1 && !other.khr_maintenance1,
            khr_generic_controller: self.khr_generic_controller && !other.khr_generic_controller,
            logitech_mx_ink_stylus_interaction: self.logitech_mx_ink_stylus_interaction
                && !other.logitech_mx_ink_stylus_interaction,
            meta_foveation_eye_tracked: self.meta_foveation_eye_tracked
                && !other.meta_foveation_eye_tracked,
            meta_local_dimming: self.meta_local_dimming && !other.meta_local_dimming,
            meta_passthrough_preferences: self.meta_passthrough_preferences
                && !other.meta_passthrough_preferences,
            meta_virtual_keyboard: self.meta_virtual_keyboard && !other.meta_virtual_keyboard,
            meta_vulkan_swapchain_create_info: self.meta_vulkan_swapchain_create_info
                && !other.meta_vulkan_swapchain_create_info,
            meta_performance_metrics: self.meta_performance_metrics
                && !other.meta_performance_metrics,
            meta_detached_controllers: self.meta_detached_controllers
                && !other.meta_detached_controllers,
            meta_headset_id: self.meta_headset_id && !other.meta_headset_id,
            meta_spatial_entity_discovery: self.meta_spatial_entity_discovery
                && !other.meta_spatial_entity_discovery,
            meta_hand_tracking_microgestures: self.meta_hand_tracking_microgestures
                && !other.meta_hand_tracking_microgestures,
            meta_recommended_layer_resolution: self.meta_recommended_layer_resolution
                && !other.meta_recommended_layer_resolution,
            meta_spatial_entity_persistence: self.meta_spatial_entity_persistence
                && !other.meta_spatial_entity_persistence,
            meta_passthrough_color_lut: self.meta_passthrough_color_lut
                && !other.meta_passthrough_color_lut,
            meta_spatial_entity_mesh: self.meta_spatial_entity_mesh
                && !other.meta_spatial_entity_mesh,
            meta_automatic_layer_filter: self.meta_automatic_layer_filter
                && !other.meta_automatic_layer_filter,
            meta_body_tracking_full_body: self.meta_body_tracking_full_body
                && !other.meta_body_tracking_full_body,
            meta_touch_controller_plus: self.meta_touch_controller_plus
                && !other.meta_touch_controller_plus,
            meta_passthrough_layer_resumed_event: self.meta_passthrough_layer_resumed_event
                && !other.meta_passthrough_layer_resumed_event,
            meta_body_tracking_calibration: self.meta_body_tracking_calibration
                && !other.meta_body_tracking_calibration,
            meta_spatial_entity_sharing: self.meta_spatial_entity_sharing
                && !other.meta_spatial_entity_sharing,
            meta_environment_depth: self.meta_environment_depth && !other.meta_environment_depth,
            meta_simultaneous_hands_and_controllers: self.meta_simultaneous_hands_and_controllers
                && !other.meta_simultaneous_hands_and_controllers,
            meta_colocation_discovery: self.meta_colocation_discovery
                && !other.meta_colocation_discovery,
            meta_spatial_entity_group_sharing: self.meta_spatial_entity_group_sharing
                && !other.meta_spatial_entity_group_sharing,
            ml_ml2_controller_interaction: self.ml_ml2_controller_interaction
                && !other.ml_ml2_controller_interaction,
            ml_frame_end_info: self.ml_frame_end_info && !other.ml_frame_end_info,
            ml_global_dimmer: self.ml_global_dimmer && !other.ml_global_dimmer,
            ml_compat: self.ml_compat && !other.ml_compat,
            ml_marker_understanding: self.ml_marker_understanding && !other.ml_marker_understanding,
            ml_localization_map: self.ml_localization_map && !other.ml_localization_map,
            ml_spatial_anchors: self.ml_spatial_anchors && !other.ml_spatial_anchors,
            ml_spatial_anchors_storage: self.ml_spatial_anchors_storage
                && !other.ml_spatial_anchors_storage,
            ml_user_calibration: self.ml_user_calibration && !other.ml_user_calibration,
            ml_system_notifications: self.ml_system_notifications && !other.ml_system_notifications,
            ml_world_mesh_detection: self.ml_world_mesh_detection && !other.ml_world_mesh_detection,
            ml_facial_expression: self.ml_facial_expression && !other.ml_facial_expression,
            ml_view_configuration_depth_range_change: self.ml_view_configuration_depth_range_change
                && !other.ml_view_configuration_depth_range_change,
            mnd_headless: self.mnd_headless && !other.mnd_headless,
            mnd_swapchain_usage_input_attachment_bit: self.mnd_swapchain_usage_input_attachment_bit
                && !other.mnd_swapchain_usage_input_attachment_bit,
            msft_unbounded_reference_space: self.msft_unbounded_reference_space
                && !other.msft_unbounded_reference_space,
            msft_spatial_anchor: self.msft_spatial_anchor && !other.msft_spatial_anchor,
            msft_spatial_graph_bridge: self.msft_spatial_graph_bridge
                && !other.msft_spatial_graph_bridge,
            msft_hand_interaction: self.msft_hand_interaction && !other.msft_hand_interaction,
            msft_hand_tracking_mesh: self.msft_hand_tracking_mesh && !other.msft_hand_tracking_mesh,
            msft_secondary_view_configuration: self.msft_secondary_view_configuration
                && !other.msft_secondary_view_configuration,
            msft_first_person_observer: self.msft_first_person_observer
                && !other.msft_first_person_observer,
            msft_controller_model: self.msft_controller_model && !other.msft_controller_model,
            #[cfg(windows)]
            msft_perception_anchor_interop: self.msft_perception_anchor_interop
                && !other.msft_perception_anchor_interop,
            #[cfg(windows)]
            msft_holographic_window_attachment: self.msft_holographic_window_attachment
                && !other.msft_holographic_window_attachment,
            msft_composition_layer_reprojection: self.msft_composition_layer_reprojection
                && !other.msft_composition_layer_reprojection,
            msft_spatial_anchor_persistence: self.msft_spatial_anchor_persistence
                && !other.msft_spatial_anchor_persistence,
            #[cfg(target_os = "android")]
            oculus_android_session_state_enable: self.oculus_android_session_state_enable
                && !other.oculus_android_session_state_enable,
            oculus_audio_device_guid: self.oculus_audio_device_guid
                && !other.oculus_audio_device_guid,
            oculus_external_camera: self.oculus_external_camera && !other.oculus_external_camera,
            oppo_controller_interaction: self.oppo_controller_interaction
                && !other.oppo_controller_interaction,
            qcom_tracking_optimization_settings: self.qcom_tracking_optimization_settings
                && !other.qcom_tracking_optimization_settings,
            ultraleap_hand_tracking_forearm: self.ultraleap_hand_tracking_forearm
                && !other.ultraleap_hand_tracking_forearm,
            valve_analog_threshold: self.valve_analog_threshold && !other.valve_analog_threshold,
            varjo_quad_views: self.varjo_quad_views && !other.varjo_quad_views,
            varjo_foveated_rendering: self.varjo_foveated_rendering
                && !other.varjo_foveated_rendering,
            varjo_composition_layer_depth_test: self.varjo_composition_layer_depth_test
                && !other.varjo_composition_layer_depth_test,
            varjo_environment_depth_estimation: self.varjo_environment_depth_estimation
                && !other.varjo_environment_depth_estimation,
            varjo_marker_tracking: self.varjo_marker_tracking && !other.varjo_marker_tracking,
            varjo_view_offset: self.varjo_view_offset && !other.varjo_view_offset,
            varjo_xr4_controller_interaction: self.varjo_xr4_controller_interaction
                && !other.varjo_xr4_controller_interaction,
            yvr_controller_interaction: self.yvr_controller_interaction
                && !other.yvr_controller_interaction,
            extx_overlay: self.extx_overlay && !other.extx_overlay,
            mndx_egl_enable: self.mndx_egl_enable && !other.mndx_egl_enable,
            mndx_force_feedback_curl: self.mndx_force_feedback_curl
                && !other.mndx_force_feedback_curl,
            htcx_vive_tracker_interaction: self.htcx_vive_tracker_interaction
                && !other.htcx_vive_tracker_interaction,
            other: self
                .other
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .difference(&other.other.iter().collect())
                .map(|x| x.to_vec())
                .collect(),
        }
    }
    #[doc = r" Return the intersection of `self` and `other`, i.e. fields set in both"]
    #[inline]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            almalence_digital_lens_control: self.almalence_digital_lens_control
                && other.almalence_digital_lens_control,
            #[cfg(target_os = "android")]
            android_trackables: self.android_trackables && other.android_trackables,
            #[cfg(target_os = "android")]
            android_device_anchor_persistence: self.android_device_anchor_persistence
                && other.android_device_anchor_persistence,
            #[cfg(target_os = "android")]
            android_face_tracking: self.android_face_tracking && other.android_face_tracking,
            #[cfg(target_os = "android")]
            android_passthrough_camera_state: self.android_passthrough_camera_state
                && other.android_passthrough_camera_state,
            #[cfg(target_os = "android")]
            android_raycast: self.android_raycast && other.android_raycast,
            #[cfg(target_os = "android")]
            android_trackables_object: self.android_trackables_object
                && other.android_trackables_object,
            #[cfg(target_os = "android")]
            android_anchor_sharing_export: self.android_anchor_sharing_export
                && other.android_anchor_sharing_export,
            #[cfg(target_os = "android")]
            android_trackables_marker: self.android_trackables_marker
                && other.android_trackables_marker,
            bd_controller_interaction: self.bd_controller_interaction
                && other.bd_controller_interaction,
            bd_body_tracking: self.bd_body_tracking && other.bd_body_tracking,
            bd_facial_simulation: self.bd_facial_simulation && other.bd_facial_simulation,
            bd_spatial_sensing: self.bd_spatial_sensing && other.bd_spatial_sensing,
            bd_spatial_anchor: self.bd_spatial_anchor && other.bd_spatial_anchor,
            bd_spatial_anchor_sharing: self.bd_spatial_anchor_sharing
                && other.bd_spatial_anchor_sharing,
            bd_spatial_scene: self.bd_spatial_scene && other.bd_spatial_scene,
            bd_spatial_mesh: self.bd_spatial_mesh && other.bd_spatial_mesh,
            bd_future_progress: self.bd_future_progress && other.bd_future_progress,
            bd_spatial_plane: self.bd_spatial_plane && other.bd_spatial_plane,
            bd_ultra_controller_interaction: self.bd_ultra_controller_interaction
                && other.bd_ultra_controller_interaction,
            epic_view_configuration_fov: self.epic_view_configuration_fov
                && other.epic_view_configuration_fov,
            ext_performance_settings: self.ext_performance_settings
                && other.ext_performance_settings,
            ext_thermal_query: self.ext_thermal_query && other.ext_thermal_query,
            ext_debug_utils: self.ext_debug_utils && other.ext_debug_utils,
            ext_eye_gaze_interaction: self.ext_eye_gaze_interaction
                && other.ext_eye_gaze_interaction,
            ext_view_configuration_depth_range: self.ext_view_configuration_depth_range
                && other.ext_view_configuration_depth_range,
            ext_conformance_automation: self.ext_conformance_automation
                && other.ext_conformance_automation,
            ext_hand_tracking: self.ext_hand_tracking && other.ext_hand_tracking,
            #[cfg(windows)]
            ext_win32_appcontainer_compatible: self.ext_win32_appcontainer_compatible
                && other.ext_win32_appcontainer_compatible,
            ext_dpad_binding: self.ext_dpad_binding && other.ext_dpad_binding,
            ext_hand_joints_motion_range: self.ext_hand_joints_motion_range
                && other.ext_hand_joints_motion_range,
            ext_samsung_odyssey_controller: self.ext_samsung_odyssey_controller
                && other.ext_samsung_odyssey_controller,
            ext_hp_mixed_reality_controller: self.ext_hp_mixed_reality_controller
                && other.ext_hp_mixed_reality_controller,
            ext_palm_pose: self.ext_palm_pose && other.ext_palm_pose,
            ext_frame_synthesis: self.ext_frame_synthesis && other.ext_frame_synthesis,
            ext_uuid: self.ext_uuid && other.ext_uuid,
            ext_render_model: self.ext_render_model && other.ext_render_model,
            ext_interaction_render_model: self.ext_interaction_render_model
                && other.ext_interaction_render_model,
            ext_hand_interaction: self.ext_hand_interaction && other.ext_hand_interaction,
            ext_active_action_set_priority: self.ext_active_action_set_priority
                && other.ext_active_action_set_priority,
            ext_local_floor: self.ext_local_floor && other.ext_local_floor,
            ext_hand_tracking_data_source: self.ext_hand_tracking_data_source
                && other.ext_hand_tracking_data_source,
            ext_plane_detection: self.ext_plane_detection && other.ext_plane_detection,
            ext_future: self.ext_future && other.ext_future,
            ext_user_presence: self.ext_user_presence && other.ext_user_presence,
            ext_composition_layer_inverted_alpha: self.ext_composition_layer_inverted_alpha
                && other.ext_composition_layer_inverted_alpha,
            ext_spatial_entity: self.ext_spatial_entity && other.ext_spatial_entity,
            ext_spatial_plane_tracking: self.ext_spatial_plane_tracking
                && other.ext_spatial_plane_tracking,
            ext_spatial_marker_tracking: self.ext_spatial_marker_tracking
                && other.ext_spatial_marker_tracking,
            ext_spatial_anchor: self.ext_spatial_anchor && other.ext_spatial_anchor,
            ext_spatial_persistence: self.ext_spatial_persistence && other.ext_spatial_persistence,
            ext_spatial_persistence_operations: self.ext_spatial_persistence_operations
                && other.ext_spatial_persistence_operations,
            ext_loader_init_properties: self.ext_loader_init_properties
                && other.ext_loader_init_properties,
            fb_composition_layer_image_layout: self.fb_composition_layer_image_layout
                && other.fb_composition_layer_image_layout,
            fb_composition_layer_alpha_blend: self.fb_composition_layer_alpha_blend
                && other.fb_composition_layer_alpha_blend,
            #[cfg(target_os = "android")]
            fb_android_surface_swapchain_create: self.fb_android_surface_swapchain_create
                && other.fb_android_surface_swapchain_create,
            fb_swapchain_update_state: self.fb_swapchain_update_state
                && other.fb_swapchain_update_state,
            fb_composition_layer_secure_content: self.fb_composition_layer_secure_content
                && other.fb_composition_layer_secure_content,
            fb_body_tracking: self.fb_body_tracking && other.fb_body_tracking,
            fb_display_refresh_rate: self.fb_display_refresh_rate && other.fb_display_refresh_rate,
            fb_color_space: self.fb_color_space && other.fb_color_space,
            fb_hand_tracking_mesh: self.fb_hand_tracking_mesh && other.fb_hand_tracking_mesh,
            fb_hand_tracking_aim: self.fb_hand_tracking_aim && other.fb_hand_tracking_aim,
            fb_hand_tracking_capsules: self.fb_hand_tracking_capsules
                && other.fb_hand_tracking_capsules,
            fb_spatial_entity: self.fb_spatial_entity && other.fb_spatial_entity,
            fb_foveation: self.fb_foveation && other.fb_foveation,
            fb_foveation_configuration: self.fb_foveation_configuration
                && other.fb_foveation_configuration,
            fb_keyboard_tracking: self.fb_keyboard_tracking && other.fb_keyboard_tracking,
            fb_triangle_mesh: self.fb_triangle_mesh && other.fb_triangle_mesh,
            fb_passthrough: self.fb_passthrough && other.fb_passthrough,
            fb_render_model: self.fb_render_model && other.fb_render_model,
            fb_spatial_entity_query: self.fb_spatial_entity_query && other.fb_spatial_entity_query,
            fb_spatial_entity_storage: self.fb_spatial_entity_storage
                && other.fb_spatial_entity_storage,
            fb_foveation_vulkan: self.fb_foveation_vulkan && other.fb_foveation_vulkan,
            #[cfg(target_os = "android")]
            fb_swapchain_update_state_android_surface: self
                .fb_swapchain_update_state_android_surface
                && other.fb_swapchain_update_state_android_surface,
            fb_swapchain_update_state_opengl_es: self.fb_swapchain_update_state_opengl_es
                && other.fb_swapchain_update_state_opengl_es,
            fb_swapchain_update_state_vulkan: self.fb_swapchain_update_state_vulkan
                && other.fb_swapchain_update_state_vulkan,
            fb_touch_controller_pro: self.fb_touch_controller_pro && other.fb_touch_controller_pro,
            fb_spatial_entity_sharing: self.fb_spatial_entity_sharing
                && other.fb_spatial_entity_sharing,
            fb_space_warp: self.fb_space_warp && other.fb_space_warp,
            fb_haptic_amplitude_envelope: self.fb_haptic_amplitude_envelope
                && other.fb_haptic_amplitude_envelope,
            fb_scene: self.fb_scene && other.fb_scene,
            fb_scene_capture: self.fb_scene_capture && other.fb_scene_capture,
            fb_spatial_entity_container: self.fb_spatial_entity_container
                && other.fb_spatial_entity_container,
            fb_face_tracking: self.fb_face_tracking && other.fb_face_tracking,
            fb_eye_tracking_social: self.fb_eye_tracking_social && other.fb_eye_tracking_social,
            fb_passthrough_keyboard_hands: self.fb_passthrough_keyboard_hands
                && other.fb_passthrough_keyboard_hands,
            fb_composition_layer_settings: self.fb_composition_layer_settings
                && other.fb_composition_layer_settings,
            fb_touch_controller_proximity: self.fb_touch_controller_proximity
                && other.fb_touch_controller_proximity,
            fb_haptic_pcm: self.fb_haptic_pcm && other.fb_haptic_pcm,
            fb_composition_layer_depth_test: self.fb_composition_layer_depth_test
                && other.fb_composition_layer_depth_test,
            fb_spatial_entity_storage_batch: self.fb_spatial_entity_storage_batch
                && other.fb_spatial_entity_storage_batch,
            fb_spatial_entity_user: self.fb_spatial_entity_user && other.fb_spatial_entity_user,
            fb_face_tracking2: self.fb_face_tracking2 && other.fb_face_tracking2,
            htc_vive_cosmos_controller_interaction: self.htc_vive_cosmos_controller_interaction
                && other.htc_vive_cosmos_controller_interaction,
            htc_facial_tracking: self.htc_facial_tracking && other.htc_facial_tracking,
            htc_vive_focus3_controller_interaction: self.htc_vive_focus3_controller_interaction
                && other.htc_vive_focus3_controller_interaction,
            htc_hand_interaction: self.htc_hand_interaction && other.htc_hand_interaction,
            htc_vive_wrist_tracker_interaction: self.htc_vive_wrist_tracker_interaction
                && other.htc_vive_wrist_tracker_interaction,
            htc_passthrough: self.htc_passthrough && other.htc_passthrough,
            htc_foveation: self.htc_foveation && other.htc_foveation,
            htc_anchor: self.htc_anchor && other.htc_anchor,
            htc_body_tracking: self.htc_body_tracking && other.htc_body_tracking,
            huawei_controller_interaction: self.huawei_controller_interaction
                && other.huawei_controller_interaction,
            #[cfg(target_os = "android")]
            khr_android_thread_settings: self.khr_android_thread_settings
                && other.khr_android_thread_settings,
            #[cfg(target_os = "android")]
            khr_android_surface_swapchain: self.khr_android_surface_swapchain
                && other.khr_android_surface_swapchain,
            khr_composition_layer_cube: self.khr_composition_layer_cube
                && other.khr_composition_layer_cube,
            #[cfg(target_os = "android")]
            khr_android_create_instance: self.khr_android_create_instance
                && other.khr_android_create_instance,
            khr_composition_layer_depth: self.khr_composition_layer_depth
                && other.khr_composition_layer_depth,
            khr_vulkan_swapchain_format_list: self.khr_vulkan_swapchain_format_list
                && other.khr_vulkan_swapchain_format_list,
            khr_composition_layer_cylinder: self.khr_composition_layer_cylinder
                && other.khr_composition_layer_cylinder,
            khr_composition_layer_equirect: self.khr_composition_layer_equirect
                && other.khr_composition_layer_equirect,
            khr_opengl_enable: self.khr_opengl_enable && other.khr_opengl_enable,
            khr_opengl_es_enable: self.khr_opengl_es_enable && other.khr_opengl_es_enable,
            khr_vulkan_enable: self.khr_vulkan_enable && other.khr_vulkan_enable,
            #[cfg(windows)]
            khr_d3d11_enable: self.khr_d3d11_enable && other.khr_d3d11_enable,
            #[cfg(windows)]
            khr_d3d12_enable: self.khr_d3d12_enable && other.khr_d3d12_enable,
            #[cfg(target_vendor = "apple")]
            khr_metal_enable: self.khr_metal_enable && other.khr_metal_enable,
            khr_visibility_mask: self.khr_visibility_mask && other.khr_visibility_mask,
            khr_composition_layer_color_scale_bias: self.khr_composition_layer_color_scale_bias
                && other.khr_composition_layer_color_scale_bias,
            #[cfg(windows)]
            khr_win32_convert_performance_counter_time: self
                .khr_win32_convert_performance_counter_time
                && other.khr_win32_convert_performance_counter_time,
            khr_convert_timespec_time: self.khr_convert_timespec_time
                && other.khr_convert_timespec_time,
            khr_loader_init: self.khr_loader_init && other.khr_loader_init,
            #[cfg(target_os = "android")]
            khr_loader_init_android: self.khr_loader_init_android && other.khr_loader_init_android,
            khr_vulkan_enable2: self.khr_vulkan_enable2 && other.khr_vulkan_enable2,
            khr_composition_layer_equirect2: self.khr_composition_layer_equirect2
                && other.khr_composition_layer_equirect2,
            khr_binding_modification: self.khr_binding_modification
                && other.khr_binding_modification,
            khr_extended_struct_name_lengths: self.khr_extended_struct_name_lengths
                && other.khr_extended_struct_name_lengths,
            khr_swapchain_usage_input_attachment_bit: self.khr_swapchain_usage_input_attachment_bit
                && other.khr_swapchain_usage_input_attachment_bit,
            khr_locate_spaces: self.khr_locate_spaces && other.khr_locate_spaces,
            khr_maintenance1: self.khr_maintenance1 && other.khr_maintenance1,
            khr_generic_controller: self.khr_generic_controller && other.khr_generic_controller,
            logitech_mx_ink_stylus_interaction: self.logitech_mx_ink_stylus_interaction
                && other.logitech_mx_ink_stylus_interaction,
            meta_foveation_eye_tracked: self.meta_foveation_eye_tracked
                && other.meta_foveation_eye_tracked,
            meta_local_dimming: self.meta_local_dimming && other.meta_local_dimming,
            meta_passthrough_preferences: self.meta_passthrough_preferences
                && other.meta_passthrough_preferences,
            meta_virtual_keyboard: self.meta_virtual_keyboard && other.meta_virtual_keyboard,
            meta_vulkan_swapchain_create_info: self.meta_vulkan_swapchain_create_info
                && other.meta_vulkan_swapchain_create_info,
            meta_performance_metrics: self.meta_performance_metrics
                && other.meta_performance_metrics,
            meta_detached_controllers: self.meta_detached_controllers
                && other.meta_detached_controllers,
            meta_headset_id: self.meta_headset_id && other.meta_headset_id,
            meta_spatial_entity_discovery: self.meta_spatial_entity_discovery
                && other.meta_spatial_entity_discovery,
            meta_hand_tracking_microgestures: self.meta_hand_tracking_microgestures
                && other.meta_hand_tracking_microgestures,
            meta_recommended_layer_resolution: self.meta_recommended_layer_resolution
                && other.meta_recommended_layer_resolution,
            meta_spatial_entity_persistence: self.meta_spatial_entity_persistence
                && other.meta_spatial_entity_persistence,
            meta_passthrough_color_lut: self.meta_passthrough_color_lut
                && other.meta_passthrough_color_lut,
            meta_spatial_entity_mesh: self.meta_spatial_entity_mesh
                && other.meta_spatial_entity_mesh,
            meta_automatic_layer_filter: self.meta_automatic_layer_filter
                && other.meta_automatic_layer_filter,
            meta_body_tracking_full_body: self.meta_body_tracking_full_body
                && other.meta_body_tracking_full_body,
            meta_touch_controller_plus: self.meta_touch_controller_plus
                && other.meta_touch_controller_plus,
            meta_passthrough_layer_resumed_event: self.meta_passthrough_layer_resumed_event
                && other.meta_passthrough_layer_resumed_event,
            meta_body_tracking_calibration: self.meta_body_tracking_calibration
                && other.meta_body_tracking_calibration,
            meta_spatial_entity_sharing: self.meta_spatial_entity_sharing
                && other.meta_spatial_entity_sharing,
            meta_environment_depth: self.meta_environment_depth && other.meta_environment_depth,
            meta_simultaneous_hands_and_controllers: self.meta_simultaneous_hands_and_controllers
                && other.meta_simultaneous_hands_and_controllers,
            meta_colocation_discovery: self.meta_colocation_discovery
                && other.meta_colocation_discovery,
            meta_spatial_entity_group_sharing: self.meta_spatial_entity_group_sharing
                && other.meta_spatial_entity_group_sharing,
            ml_ml2_controller_interaction: self.ml_ml2_controller_interaction
                && other.ml_ml2_controller_interaction,
            ml_frame_end_info: self.ml_frame_end_info && other.ml_frame_end_info,
            ml_global_dimmer: self.ml_global_dimmer && other.ml_global_dimmer,
            ml_compat: self.ml_compat && other.ml_compat,
            ml_marker_understanding: self.ml_marker_understanding && other.ml_marker_understanding,
            ml_localization_map: self.ml_localization_map && other.ml_localization_map,
            ml_spatial_anchors: self.ml_spatial_anchors && other.ml_spatial_anchors,
            ml_spatial_anchors_storage: self.ml_spatial_anchors_storage
                && other.ml_spatial_anchors_storage,
            ml_user_calibration: self.ml_user_calibration && other.ml_user_calibration,
            ml_system_notifications: self.ml_system_notifications && other.ml_system_notifications,
            ml_world_mesh_detection: self.ml_world_mesh_detection && other.ml_world_mesh_detection,
            ml_facial_expression: self.ml_facial_expression && other.ml_facial_expression,
            ml_view_configuration_depth_range_change: self.ml_view_configuration_depth_range_change
                && other.ml_view_configuration_depth_range_change,
            mnd_headless: self.mnd_headless && other.mnd_headless,
            mnd_swapchain_usage_input_attachment_bit: self.mnd_swapchain_usage_input_attachment_bit
                && other.mnd_swapchain_usage_input_attachment_bit,
            msft_unbounded_reference_space: self.msft_unbounded_reference_space
                && other.msft_unbounded_reference_space,
            msft_spatial_anchor: self.msft_spatial_anchor && other.msft_spatial_anchor,
            msft_spatial_graph_bridge: self.msft_spatial_graph_bridge
                && other.msft_spatial_graph_bridge,
            msft_hand_interaction: self.msft_hand_interaction && other.msft_hand_interaction,
            msft_hand_tracking_mesh: self.msft_hand_tracking_mesh && other.msft_hand_tracking_mesh,
            msft_secondary_view_configuration: self.msft_secondary_view_configuration
                && other.msft_secondary_view_configuration,
            msft_first_person_observer: self.msft_first_person_observer
                && other.msft_first_person_observer,
            msft_controller_model: self.msft_controller_model && other.msft_controller_model,
            #[cfg(windows)]
            msft_perception_anchor_interop: self.msft_perception_anchor_interop
                && other.msft_perception_anchor_interop,
            #[cfg(windows)]
            msft_holographic_window_attachment: self.msft_holographic_window_attachment
                && other.msft_holographic_window_attachment,
            msft_composition_layer_reprojection: self.msft_composition_layer_reprojection
                && other.msft_composition_layer_reprojection,
            msft_spatial_anchor_persistence: self.msft_spatial_anchor_persistence
                && other.msft_spatial_anchor_persistence,
            #[cfg(target_os = "android")]
            oculus_android_session_state_enable: self.oculus_android_session_state_enable
                && other.oculus_android_session_state_enable,
            oculus_audio_device_guid: self.oculus_audio_device_guid
                && other.oculus_audio_device_guid,
            oculus_external_camera: self.oculus_external_camera && other.oculus_external_camera,
            oppo_controller_interaction: self.oppo_controller_interaction
                && other.oppo_controller_interaction,
            qcom_tracking_optimization_settings: self.qcom_tracking_optimization_settings
                && other.qcom_tracking_optimization_settings,
            ultraleap_hand_tracking_forearm: self.ultraleap_hand_tracking_forearm
                && other.ultraleap_hand_tracking_forearm,
            valve_analog_threshold: self.valve_analog_threshold && other.valve_analog_threshold,
            varjo_quad_views: self.varjo_quad_views && other.varjo_quad_views,
            varjo_foveated_rendering: self.varjo_foveated_rendering
                && other.varjo_foveated_rendering,
            varjo_composition_layer_depth_test: self.varjo_composition_layer_depth_test
                && other.varjo_composition_layer_depth_test,
            varjo_environment_depth_estimation: self.varjo_environment_depth_estimation
                && other.varjo_environment_depth_estimation,
            varjo_marker_tracking: self.varjo_marker_tracking && other.varjo_marker_tracking,
            varjo_view_offset: self.varjo_view_offset && other.varjo_view_offset,
            varjo_xr4_controller_interaction: self.varjo_xr4_controller_interaction
                && other.varjo_xr4_controller_interaction,
            yvr_controller_interaction: self.yvr_controller_interaction
                && other.yvr_controller_interaction,
            extx_overlay: self.extx_overlay && other.extx_overlay,
            mndx_egl_enable: self.mndx_egl_enable && other.mndx_egl_enable,
            mndx_force_feedback_curl: self.mndx_force_feedback_curl
                && other.mndx_force_feedback_curl,
            htcx_vive_tracker_interaction: self.htcx_vive_tracker_interaction
                && other.htcx_vive_tracker_interaction,
            other: self
                .other
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .intersection(&other.other.iter().collect())
                .map(|x| x.to_vec())
                .collect(),
        }
    }
    #[doc = r" Return names of supported extensions, as a `Vec` of nul terminated byte slices."]
    pub fn names(&self) -> Vec<&[u8]> {
        let mut out = Vec::new();
        {
            if self.almalence_digital_lens_control {
                out.push(raw::DigitalLensControlALMALENCE::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_trackables {
                out.push(raw::TrackablesANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_device_anchor_persistence {
                out.push(raw::DeviceAnchorPersistenceANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_face_tracking {
                out.push(raw::FaceTrackingANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_passthrough_camera_state {
                out.push(raw::PassthroughCameraStateANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_raycast {
                out.push(raw::RaycastANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_trackables_object {
                out.push(raw::TrackablesObjectANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_anchor_sharing_export {
                out.push(raw::AnchorSharingExportANDROID::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.android_trackables_marker {
                out.push(raw::TrackablesMarkerANDROID::NAME);
            }
        }
        {
            if self.bd_controller_interaction {
                out.push(raw::ControllerInteractionBD::NAME);
            }
        }
        {
            if self.bd_body_tracking {
                out.push(raw::BodyTrackingBD::NAME);
            }
        }
        {
            if self.bd_facial_simulation {
                out.push(raw::FacialSimulationBD::NAME);
            }
        }
        {
            if self.bd_spatial_sensing {
                out.push(raw::SpatialSensingBD::NAME);
            }
        }
        {
            if self.bd_spatial_anchor {
                out.push(raw::SpatialAnchorBD::NAME);
            }
        }
        {
            if self.bd_spatial_anchor_sharing {
                out.push(raw::SpatialAnchorSharingBD::NAME);
            }
        }
        {
            if self.bd_spatial_scene {
                out.push(raw::SpatialSceneBD::NAME);
            }
        }
        {
            if self.bd_spatial_mesh {
                out.push(raw::SpatialMeshBD::NAME);
            }
        }
        {
            if self.bd_future_progress {
                out.push(raw::FutureProgressBD::NAME);
            }
        }
        {
            if self.bd_spatial_plane {
                out.push(raw::SpatialPlaneBD::NAME);
            }
        }
        {
            if self.bd_ultra_controller_interaction {
                out.push(raw::UltraControllerInteractionBD::NAME);
            }
        }
        {
            if self.epic_view_configuration_fov {
                out.push(raw::ViewConfigurationFovEPIC::NAME);
            }
        }
        {
            if self.ext_performance_settings {
                out.push(raw::PerformanceSettingsEXT::NAME);
            }
        }
        {
            if self.ext_thermal_query {
                out.push(raw::ThermalQueryEXT::NAME);
            }
        }
        {
            if self.ext_debug_utils {
                out.push(raw::DebugUtilsEXT::NAME);
            }
        }
        {
            if self.ext_eye_gaze_interaction {
                out.push(raw::EyeGazeInteractionEXT::NAME);
            }
        }
        {
            if self.ext_view_configuration_depth_range {
                out.push(raw::ViewConfigurationDepthRangeEXT::NAME);
            }
        }
        {
            if self.ext_conformance_automation {
                out.push(raw::ConformanceAutomationEXT::NAME);
            }
        }
        {
            if self.ext_hand_tracking {
                out.push(raw::HandTrackingEXT::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.ext_win32_appcontainer_compatible {
                out.push(raw::Win32AppcontainerCompatibleEXT::NAME);
            }
        }
        {
            if self.ext_dpad_binding {
                out.push(raw::DpadBindingEXT::NAME);
            }
        }
        {
            if self.ext_hand_joints_motion_range {
                out.push(raw::HandJointsMotionRangeEXT::NAME);
            }
        }
        {
            if self.ext_samsung_odyssey_controller {
                out.push(raw::SamsungOdysseyControllerEXT::NAME);
            }
        }
        {
            if self.ext_hp_mixed_reality_controller {
                out.push(raw::HpMixedRealityControllerEXT::NAME);
            }
        }
        {
            if self.ext_palm_pose {
                out.push(raw::PalmPoseEXT::NAME);
            }
        }
        {
            if self.ext_frame_synthesis {
                out.push(raw::FrameSynthesisEXT::NAME);
            }
        }
        {
            if self.ext_uuid {
                out.push(raw::UuidEXT::NAME);
            }
        }
        {
            if self.ext_render_model {
                out.push(raw::RenderModelEXT::NAME);
            }
        }
        {
            if self.ext_interaction_render_model {
                out.push(raw::InteractionRenderModelEXT::NAME);
            }
        }
        {
            if self.ext_hand_interaction {
                out.push(raw::HandInteractionEXT::NAME);
            }
        }
        {
            if self.ext_active_action_set_priority {
                out.push(raw::ActiveActionSetPriorityEXT::NAME);
            }
        }
        {
            if self.ext_local_floor {
                out.push(raw::LocalFloorEXT::NAME);
            }
        }
        {
            if self.ext_hand_tracking_data_source {
                out.push(raw::HandTrackingDataSourceEXT::NAME);
            }
        }
        {
            if self.ext_plane_detection {
                out.push(raw::PlaneDetectionEXT::NAME);
            }
        }
        {
            if self.ext_future {
                out.push(raw::FutureEXT::NAME);
            }
        }
        {
            if self.ext_user_presence {
                out.push(raw::UserPresenceEXT::NAME);
            }
        }
        {
            if self.ext_composition_layer_inverted_alpha {
                out.push(raw::CompositionLayerInvertedAlphaEXT::NAME);
            }
        }
        {
            if self.ext_spatial_entity {
                out.push(raw::SpatialEntityEXT::NAME);
            }
        }
        {
            if self.ext_spatial_plane_tracking {
                out.push(raw::SpatialPlaneTrackingEXT::NAME);
            }
        }
        {
            if self.ext_spatial_marker_tracking {
                out.push(raw::SpatialMarkerTrackingEXT::NAME);
            }
        }
        {
            if self.ext_spatial_anchor {
                out.push(raw::SpatialAnchorEXT::NAME);
            }
        }
        {
            if self.ext_spatial_persistence {
                out.push(raw::SpatialPersistenceEXT::NAME);
            }
        }
        {
            if self.ext_spatial_persistence_operations {
                out.push(raw::SpatialPersistenceOperationsEXT::NAME);
            }
        }
        {
            if self.ext_loader_init_properties {
                out.push(raw::LoaderInitPropertiesEXT::NAME);
            }
        }
        {
            if self.fb_composition_layer_image_layout {
                out.push(raw::CompositionLayerImageLayoutFB::NAME);
            }
        }
        {
            if self.fb_composition_layer_alpha_blend {
                out.push(raw::CompositionLayerAlphaBlendFB::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.fb_android_surface_swapchain_create {
                out.push(raw::AndroidSurfaceSwapchainCreateFB::NAME);
            }
        }
        {
            if self.fb_swapchain_update_state {
                out.push(raw::SwapchainUpdateStateFB::NAME);
            }
        }
        {
            if self.fb_composition_layer_secure_content {
                out.push(raw::CompositionLayerSecureContentFB::NAME);
            }
        }
        {
            if self.fb_body_tracking {
                out.push(raw::BodyTrackingFB::NAME);
            }
        }
        {
            if self.fb_display_refresh_rate {
                out.push(raw::DisplayRefreshRateFB::NAME);
            }
        }
        {
            if self.fb_color_space {
                out.push(raw::ColorSpaceFB::NAME);
            }
        }
        {
            if self.fb_hand_tracking_mesh {
                out.push(raw::HandTrackingMeshFB::NAME);
            }
        }
        {
            if self.fb_hand_tracking_aim {
                out.push(raw::HandTrackingAimFB::NAME);
            }
        }
        {
            if self.fb_hand_tracking_capsules {
                out.push(raw::HandTrackingCapsulesFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity {
                out.push(raw::SpatialEntityFB::NAME);
            }
        }
        {
            if self.fb_foveation {
                out.push(raw::FoveationFB::NAME);
            }
        }
        {
            if self.fb_foveation_configuration {
                out.push(raw::FoveationConfigurationFB::NAME);
            }
        }
        {
            if self.fb_keyboard_tracking {
                out.push(raw::KeyboardTrackingFB::NAME);
            }
        }
        {
            if self.fb_triangle_mesh {
                out.push(raw::TriangleMeshFB::NAME);
            }
        }
        {
            if self.fb_passthrough {
                out.push(raw::PassthroughFB::NAME);
            }
        }
        {
            if self.fb_render_model {
                out.push(raw::RenderModelFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_query {
                out.push(raw::SpatialEntityQueryFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_storage {
                out.push(raw::SpatialEntityStorageFB::NAME);
            }
        }
        {
            if self.fb_foveation_vulkan {
                out.push(raw::FoveationVulkanFB::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.fb_swapchain_update_state_android_surface {
                out.push(raw::SwapchainUpdateStateAndroidSurfaceFB::NAME);
            }
        }
        {
            if self.fb_swapchain_update_state_opengl_es {
                out.push(raw::SwapchainUpdateStateOpenglEsFB::NAME);
            }
        }
        {
            if self.fb_swapchain_update_state_vulkan {
                out.push(raw::SwapchainUpdateStateVulkanFB::NAME);
            }
        }
        {
            if self.fb_touch_controller_pro {
                out.push(raw::TouchControllerProFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_sharing {
                out.push(raw::SpatialEntitySharingFB::NAME);
            }
        }
        {
            if self.fb_space_warp {
                out.push(raw::SpaceWarpFB::NAME);
            }
        }
        {
            if self.fb_haptic_amplitude_envelope {
                out.push(raw::HapticAmplitudeEnvelopeFB::NAME);
            }
        }
        {
            if self.fb_scene {
                out.push(raw::SceneFB::NAME);
            }
        }
        {
            if self.fb_scene_capture {
                out.push(raw::SceneCaptureFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_container {
                out.push(raw::SpatialEntityContainerFB::NAME);
            }
        }
        {
            if self.fb_face_tracking {
                out.push(raw::FaceTrackingFB::NAME);
            }
        }
        {
            if self.fb_eye_tracking_social {
                out.push(raw::EyeTrackingSocialFB::NAME);
            }
        }
        {
            if self.fb_passthrough_keyboard_hands {
                out.push(raw::PassthroughKeyboardHandsFB::NAME);
            }
        }
        {
            if self.fb_composition_layer_settings {
                out.push(raw::CompositionLayerSettingsFB::NAME);
            }
        }
        {
            if self.fb_touch_controller_proximity {
                out.push(raw::TouchControllerProximityFB::NAME);
            }
        }
        {
            if self.fb_haptic_pcm {
                out.push(raw::HapticPcmFB::NAME);
            }
        }
        {
            if self.fb_composition_layer_depth_test {
                out.push(raw::CompositionLayerDepthTestFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_storage_batch {
                out.push(raw::SpatialEntityStorageBatchFB::NAME);
            }
        }
        {
            if self.fb_spatial_entity_user {
                out.push(raw::SpatialEntityUserFB::NAME);
            }
        }
        {
            if self.fb_face_tracking2 {
                out.push(raw::FaceTracking2FB::NAME);
            }
        }
        {
            if self.htc_vive_cosmos_controller_interaction {
                out.push(raw::ViveCosmosControllerInteractionHTC::NAME);
            }
        }
        {
            if self.htc_facial_tracking {
                out.push(raw::FacialTrackingHTC::NAME);
            }
        }
        {
            if self.htc_vive_focus3_controller_interaction {
                out.push(raw::ViveFocus3ControllerInteractionHTC::NAME);
            }
        }
        {
            if self.htc_hand_interaction {
                out.push(raw::HandInteractionHTC::NAME);
            }
        }
        {
            if self.htc_vive_wrist_tracker_interaction {
                out.push(raw::ViveWristTrackerInteractionHTC::NAME);
            }
        }
        {
            if self.htc_passthrough {
                out.push(raw::PassthroughHTC::NAME);
            }
        }
        {
            if self.htc_foveation {
                out.push(raw::FoveationHTC::NAME);
            }
        }
        {
            if self.htc_anchor {
                out.push(raw::AnchorHTC::NAME);
            }
        }
        {
            if self.htc_body_tracking {
                out.push(raw::BodyTrackingHTC::NAME);
            }
        }
        {
            if self.huawei_controller_interaction {
                out.push(raw::ControllerInteractionHUAWEI::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_thread_settings {
                out.push(raw::AndroidThreadSettingsKHR::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_surface_swapchain {
                out.push(raw::AndroidSurfaceSwapchainKHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_cube {
                out.push(raw::CompositionLayerCubeKHR::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_android_create_instance {
                out.push(raw::AndroidCreateInstanceKHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_depth {
                out.push(raw::CompositionLayerDepthKHR::NAME);
            }
        }
        {
            if self.khr_vulkan_swapchain_format_list {
                out.push(raw::VulkanSwapchainFormatListKHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_cylinder {
                out.push(raw::CompositionLayerCylinderKHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_equirect {
                out.push(raw::CompositionLayerEquirectKHR::NAME);
            }
        }
        {
            if self.khr_opengl_enable {
                out.push(raw::OpenglEnableKHR::NAME);
            }
        }
        {
            if self.khr_opengl_es_enable {
                out.push(raw::OpenglEsEnableKHR::NAME);
            }
        }
        {
            if self.khr_vulkan_enable {
                out.push(raw::VulkanEnableKHR::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.khr_d3d11_enable {
                out.push(raw::D3d11EnableKHR::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.khr_d3d12_enable {
                out.push(raw::D3d12EnableKHR::NAME);
            }
        }
        #[cfg(target_vendor = "apple")]
        {
            if self.khr_metal_enable {
                out.push(raw::MetalEnableKHR::NAME);
            }
        }
        {
            if self.khr_visibility_mask {
                out.push(raw::VisibilityMaskKHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_color_scale_bias {
                out.push(raw::CompositionLayerColorScaleBiasKHR::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.khr_win32_convert_performance_counter_time {
                out.push(raw::Win32ConvertPerformanceCounterTimeKHR::NAME);
            }
        }
        {
            if self.khr_convert_timespec_time {
                out.push(raw::ConvertTimespecTimeKHR::NAME);
            }
        }
        {
            if self.khr_loader_init {
                out.push(raw::LoaderInitKHR::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.khr_loader_init_android {
                out.push(raw::LoaderInitAndroidKHR::NAME);
            }
        }
        {
            if self.khr_vulkan_enable2 {
                out.push(raw::VulkanEnable2KHR::NAME);
            }
        }
        {
            if self.khr_composition_layer_equirect2 {
                out.push(raw::CompositionLayerEquirect2KHR::NAME);
            }
        }
        {
            if self.khr_binding_modification {
                out.push(raw::BindingModificationKHR::NAME);
            }
        }
        {
            if self.khr_extended_struct_name_lengths {
                out.push(raw::ExtendedStructNameLengthsKHR::NAME);
            }
        }
        {
            if self.khr_swapchain_usage_input_attachment_bit {
                out.push(raw::SwapchainUsageInputAttachmentBitKHR::NAME);
            }
        }
        {
            if self.khr_locate_spaces {
                out.push(raw::LocateSpacesKHR::NAME);
            }
        }
        {
            if self.khr_maintenance1 {
                out.push(raw::Maintenance1KHR::NAME);
            }
        }
        {
            if self.khr_generic_controller {
                out.push(raw::GenericControllerKHR::NAME);
            }
        }
        {
            if self.logitech_mx_ink_stylus_interaction {
                out.push(raw::MxInkStylusInteractionLOGITECH::NAME);
            }
        }
        {
            if self.meta_foveation_eye_tracked {
                out.push(raw::FoveationEyeTrackedMETA::NAME);
            }
        }
        {
            if self.meta_local_dimming {
                out.push(raw::LocalDimmingMETA::NAME);
            }
        }
        {
            if self.meta_passthrough_preferences {
                out.push(raw::PassthroughPreferencesMETA::NAME);
            }
        }
        {
            if self.meta_virtual_keyboard {
                out.push(raw::VirtualKeyboardMETA::NAME);
            }
        }
        {
            if self.meta_vulkan_swapchain_create_info {
                out.push(raw::VulkanSwapchainCreateInfoMETA::NAME);
            }
        }
        {
            if self.meta_performance_metrics {
                out.push(raw::PerformanceMetricsMETA::NAME);
            }
        }
        {
            if self.meta_detached_controllers {
                out.push(raw::DetachedControllersMETA::NAME);
            }
        }
        {
            if self.meta_headset_id {
                out.push(raw::HeadsetIdMETA::NAME);
            }
        }
        {
            if self.meta_spatial_entity_discovery {
                out.push(raw::SpatialEntityDiscoveryMETA::NAME);
            }
        }
        {
            if self.meta_hand_tracking_microgestures {
                out.push(raw::HandTrackingMicrogesturesMETA::NAME);
            }
        }
        {
            if self.meta_recommended_layer_resolution {
                out.push(raw::RecommendedLayerResolutionMETA::NAME);
            }
        }
        {
            if self.meta_spatial_entity_persistence {
                out.push(raw::SpatialEntityPersistenceMETA::NAME);
            }
        }
        {
            if self.meta_passthrough_color_lut {
                out.push(raw::PassthroughColorLutMETA::NAME);
            }
        }
        {
            if self.meta_spatial_entity_mesh {
                out.push(raw::SpatialEntityMeshMETA::NAME);
            }
        }
        {
            if self.meta_automatic_layer_filter {
                out.push(raw::AutomaticLayerFilterMETA::NAME);
            }
        }
        {
            if self.meta_body_tracking_full_body {
                out.push(raw::BodyTrackingFullBodyMETA::NAME);
            }
        }
        {
            if self.meta_touch_controller_plus {
                out.push(raw::TouchControllerPlusMETA::NAME);
            }
        }
        {
            if self.meta_passthrough_layer_resumed_event {
                out.push(raw::PassthroughLayerResumedEventMETA::NAME);
            }
        }
        {
            if self.meta_body_tracking_calibration {
                out.push(raw::BodyTrackingCalibrationMETA::NAME);
            }
        }
        {
            if self.meta_spatial_entity_sharing {
                out.push(raw::SpatialEntitySharingMETA::NAME);
            }
        }
        {
            if self.meta_environment_depth {
                out.push(raw::EnvironmentDepthMETA::NAME);
            }
        }
        {
            if self.meta_simultaneous_hands_and_controllers {
                out.push(raw::SimultaneousHandsAndControllersMETA::NAME);
            }
        }
        {
            if self.meta_colocation_discovery {
                out.push(raw::ColocationDiscoveryMETA::NAME);
            }
        }
        {
            if self.meta_spatial_entity_group_sharing {
                out.push(raw::SpatialEntityGroupSharingMETA::NAME);
            }
        }
        {
            if self.ml_ml2_controller_interaction {
                out.push(raw::Ml2ControllerInteractionML::NAME);
            }
        }
        {
            if self.ml_frame_end_info {
                out.push(raw::FrameEndInfoML::NAME);
            }
        }
        {
            if self.ml_global_dimmer {
                out.push(raw::GlobalDimmerML::NAME);
            }
        }
        {
            if self.ml_compat {
                out.push(raw::CompatML::NAME);
            }
        }
        {
            if self.ml_marker_understanding {
                out.push(raw::MarkerUnderstandingML::NAME);
            }
        }
        {
            if self.ml_localization_map {
                out.push(raw::LocalizationMapML::NAME);
            }
        }
        {
            if self.ml_spatial_anchors {
                out.push(raw::SpatialAnchorsML::NAME);
            }
        }
        {
            if self.ml_spatial_anchors_storage {
                out.push(raw::SpatialAnchorsStorageML::NAME);
            }
        }
        {
            if self.ml_user_calibration {
                out.push(raw::UserCalibrationML::NAME);
            }
        }
        {
            if self.ml_system_notifications {
                out.push(raw::SystemNotificationsML::NAME);
            }
        }
        {
            if self.ml_world_mesh_detection {
                out.push(raw::WorldMeshDetectionML::NAME);
            }
        }
        {
            if self.ml_facial_expression {
                out.push(raw::FacialExpressionML::NAME);
            }
        }
        {
            if self.ml_view_configuration_depth_range_change {
                out.push(raw::ViewConfigurationDepthRangeChangeML::NAME);
            }
        }
        {
            if self.mnd_headless {
                out.push(raw::HeadlessMND::NAME);
            }
        }
        {
            if self.mnd_swapchain_usage_input_attachment_bit {
                out.push(raw::SwapchainUsageInputAttachmentBitMND::NAME);
            }
        }
        {
            if self.msft_unbounded_reference_space {
                out.push(raw::UnboundedReferenceSpaceMSFT::NAME);
            }
        }
        {
            if self.msft_spatial_anchor {
                out.push(raw::SpatialAnchorMSFT::NAME);
            }
        }
        {
            if self.msft_spatial_graph_bridge {
                out.push(raw::SpatialGraphBridgeMSFT::NAME);
            }
        }
        {
            if self.msft_hand_interaction {
                out.push(raw::HandInteractionMSFT::NAME);
            }
        }
        {
            if self.msft_hand_tracking_mesh {
                out.push(raw::HandTrackingMeshMSFT::NAME);
            }
        }
        {
            if self.msft_secondary_view_configuration {
                out.push(raw::SecondaryViewConfigurationMSFT::NAME);
            }
        }
        {
            if self.msft_first_person_observer {
                out.push(raw::FirstPersonObserverMSFT::NAME);
            }
        }
        {
            if self.msft_controller_model {
                out.push(raw::ControllerModelMSFT::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.msft_perception_anchor_interop {
                out.push(raw::PerceptionAnchorInteropMSFT::NAME);
            }
        }
        #[cfg(windows)]
        {
            if self.msft_holographic_window_attachment {
                out.push(raw::HolographicWindowAttachmentMSFT::NAME);
            }
        }
        {
            if self.msft_composition_layer_reprojection {
                out.push(raw::CompositionLayerReprojectionMSFT::NAME);
            }
        }
        {
            if self.msft_spatial_anchor_persistence {
                out.push(raw::SpatialAnchorPersistenceMSFT::NAME);
            }
        }
        #[cfg(target_os = "android")]
        {
            if self.oculus_android_session_state_enable {
                out.push(raw::AndroidSessionStateEnableOCULUS::NAME);
            }
        }
        {
            if self.oculus_audio_device_guid {
                out.push(raw::AudioDeviceGuidOCULUS::NAME);
            }
        }
        {
            if self.oculus_external_camera {
                out.push(raw::ExternalCameraOCULUS::NAME);
            }
        }
        {
            if self.oppo_controller_interaction {
                out.push(raw::ControllerInteractionOPPO::NAME);
            }
        }
        {
            if self.qcom_tracking_optimization_settings {
                out.push(raw::TrackingOptimizationSettingsQCOM::NAME);
            }
        }
        {
            if self.ultraleap_hand_tracking_forearm {
                out.push(raw::HandTrackingForearmULTRALEAP::NAME);
            }
        }
        {
            if self.valve_analog_threshold {
                out.push(raw::AnalogThresholdVALVE::NAME);
            }
        }
        {
            if self.varjo_quad_views {
                out.push(raw::QuadViewsVARJO::NAME);
            }
        }
        {
            if self.varjo_foveated_rendering {
                out.push(raw::FoveatedRenderingVARJO::NAME);
            }
        }
        {
            if self.varjo_composition_layer_depth_test {
                out.push(raw::CompositionLayerDepthTestVARJO::NAME);
            }
        }
        {
            if self.varjo_environment_depth_estimation {
                out.push(raw::EnvironmentDepthEstimationVARJO::NAME);
            }
        }
        {
            if self.varjo_marker_tracking {
                out.push(raw::MarkerTrackingVARJO::NAME);
            }
        }
        {
            if self.varjo_view_offset {
                out.push(raw::ViewOffsetVARJO::NAME);
            }
        }
        {
            if self.varjo_xr4_controller_interaction {
                out.push(raw::Xr4ControllerInteractionVARJO::NAME);
            }
        }
        {
            if self.yvr_controller_interaction {
                out.push(raw::ControllerInteractionYVR::NAME);
            }
        }
        {
            if self.extx_overlay {
                out.push(raw::OverlayEXTX::NAME);
            }
        }
        {
            if self.mndx_egl_enable {
                out.push(raw::EglEnableMNDX::NAME);
            }
        }
        {
            if self.mndx_force_feedback_curl {
                out.push(raw::ForceFeedbackCurlMNDX::NAME);
            }
        }
        {
            if self.htcx_vive_tracker_interaction {
                out.push(raw::ViveTrackerInteractionHTCX::NAME);
            }
        }
        out.extend(self.other.iter().map(|x| x.as_slice()));
        out
    }
}
#[doc = r" Extensions used internally by the bindings"]
#[derive(Default, Copy, Clone)]
pub struct InstanceExtensions {
    pub almalence_digital_lens_control: Option<raw::DigitalLensControlALMALENCE>,
    #[cfg(target_os = "android")]
    pub android_trackables: Option<raw::TrackablesANDROID>,
    #[cfg(target_os = "android")]
    pub android_device_anchor_persistence: Option<raw::DeviceAnchorPersistenceANDROID>,
    #[cfg(target_os = "android")]
    pub android_face_tracking: Option<raw::FaceTrackingANDROID>,
    #[cfg(target_os = "android")]
    pub android_passthrough_camera_state: Option<raw::PassthroughCameraStateANDROID>,
    #[cfg(target_os = "android")]
    pub android_raycast: Option<raw::RaycastANDROID>,
    #[cfg(target_os = "android")]
    pub android_trackables_object: Option<raw::TrackablesObjectANDROID>,
    #[cfg(target_os = "android")]
    pub android_anchor_sharing_export: Option<raw::AnchorSharingExportANDROID>,
    #[cfg(target_os = "android")]
    pub android_trackables_marker: Option<raw::TrackablesMarkerANDROID>,
    pub bd_controller_interaction: Option<raw::ControllerInteractionBD>,
    pub bd_body_tracking: Option<raw::BodyTrackingBD>,
    pub bd_facial_simulation: Option<raw::FacialSimulationBD>,
    pub bd_spatial_sensing: Option<raw::SpatialSensingBD>,
    pub bd_spatial_anchor: Option<raw::SpatialAnchorBD>,
    pub bd_spatial_anchor_sharing: Option<raw::SpatialAnchorSharingBD>,
    pub bd_spatial_scene: Option<raw::SpatialSceneBD>,
    pub bd_spatial_mesh: Option<raw::SpatialMeshBD>,
    pub bd_future_progress: Option<raw::FutureProgressBD>,
    pub bd_spatial_plane: Option<raw::SpatialPlaneBD>,
    pub bd_ultra_controller_interaction: Option<raw::UltraControllerInteractionBD>,
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
    pub ext_frame_synthesis: Option<raw::FrameSynthesisEXT>,
    pub ext_uuid: Option<raw::UuidEXT>,
    pub ext_render_model: Option<raw::RenderModelEXT>,
    pub ext_interaction_render_model: Option<raw::InteractionRenderModelEXT>,
    pub ext_hand_interaction: Option<raw::HandInteractionEXT>,
    pub ext_active_action_set_priority: Option<raw::ActiveActionSetPriorityEXT>,
    pub ext_local_floor: Option<raw::LocalFloorEXT>,
    pub ext_hand_tracking_data_source: Option<raw::HandTrackingDataSourceEXT>,
    pub ext_plane_detection: Option<raw::PlaneDetectionEXT>,
    pub ext_future: Option<raw::FutureEXT>,
    pub ext_user_presence: Option<raw::UserPresenceEXT>,
    pub ext_composition_layer_inverted_alpha: Option<raw::CompositionLayerInvertedAlphaEXT>,
    pub ext_spatial_entity: Option<raw::SpatialEntityEXT>,
    pub ext_spatial_plane_tracking: Option<raw::SpatialPlaneTrackingEXT>,
    pub ext_spatial_marker_tracking: Option<raw::SpatialMarkerTrackingEXT>,
    pub ext_spatial_anchor: Option<raw::SpatialAnchorEXT>,
    pub ext_spatial_persistence: Option<raw::SpatialPersistenceEXT>,
    pub ext_spatial_persistence_operations: Option<raw::SpatialPersistenceOperationsEXT>,
    pub ext_loader_init_properties: Option<raw::LoaderInitPropertiesEXT>,
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
    pub htc_body_tracking: Option<raw::BodyTrackingHTC>,
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
    pub khr_extended_struct_name_lengths: Option<raw::ExtendedStructNameLengthsKHR>,
    pub khr_swapchain_usage_input_attachment_bit: Option<raw::SwapchainUsageInputAttachmentBitKHR>,
    pub khr_locate_spaces: Option<raw::LocateSpacesKHR>,
    pub khr_maintenance1: Option<raw::Maintenance1KHR>,
    pub khr_generic_controller: Option<raw::GenericControllerKHR>,
    pub logitech_mx_ink_stylus_interaction: Option<raw::MxInkStylusInteractionLOGITECH>,
    pub meta_foveation_eye_tracked: Option<raw::FoveationEyeTrackedMETA>,
    pub meta_local_dimming: Option<raw::LocalDimmingMETA>,
    pub meta_passthrough_preferences: Option<raw::PassthroughPreferencesMETA>,
    pub meta_virtual_keyboard: Option<raw::VirtualKeyboardMETA>,
    pub meta_vulkan_swapchain_create_info: Option<raw::VulkanSwapchainCreateInfoMETA>,
    pub meta_performance_metrics: Option<raw::PerformanceMetricsMETA>,
    pub meta_detached_controllers: Option<raw::DetachedControllersMETA>,
    pub meta_headset_id: Option<raw::HeadsetIdMETA>,
    pub meta_spatial_entity_discovery: Option<raw::SpatialEntityDiscoveryMETA>,
    pub meta_hand_tracking_microgestures: Option<raw::HandTrackingMicrogesturesMETA>,
    pub meta_recommended_layer_resolution: Option<raw::RecommendedLayerResolutionMETA>,
    pub meta_spatial_entity_persistence: Option<raw::SpatialEntityPersistenceMETA>,
    pub meta_passthrough_color_lut: Option<raw::PassthroughColorLutMETA>,
    pub meta_spatial_entity_mesh: Option<raw::SpatialEntityMeshMETA>,
    pub meta_automatic_layer_filter: Option<raw::AutomaticLayerFilterMETA>,
    pub meta_body_tracking_full_body: Option<raw::BodyTrackingFullBodyMETA>,
    pub meta_touch_controller_plus: Option<raw::TouchControllerPlusMETA>,
    pub meta_passthrough_layer_resumed_event: Option<raw::PassthroughLayerResumedEventMETA>,
    pub meta_body_tracking_calibration: Option<raw::BodyTrackingCalibrationMETA>,
    pub meta_spatial_entity_sharing: Option<raw::SpatialEntitySharingMETA>,
    pub meta_environment_depth: Option<raw::EnvironmentDepthMETA>,
    pub meta_simultaneous_hands_and_controllers: Option<raw::SimultaneousHandsAndControllersMETA>,
    pub meta_colocation_discovery: Option<raw::ColocationDiscoveryMETA>,
    pub meta_spatial_entity_group_sharing: Option<raw::SpatialEntityGroupSharingMETA>,
    pub ml_ml2_controller_interaction: Option<raw::Ml2ControllerInteractionML>,
    pub ml_frame_end_info: Option<raw::FrameEndInfoML>,
    pub ml_global_dimmer: Option<raw::GlobalDimmerML>,
    pub ml_compat: Option<raw::CompatML>,
    pub ml_marker_understanding: Option<raw::MarkerUnderstandingML>,
    pub ml_localization_map: Option<raw::LocalizationMapML>,
    pub ml_spatial_anchors: Option<raw::SpatialAnchorsML>,
    pub ml_spatial_anchors_storage: Option<raw::SpatialAnchorsStorageML>,
    pub ml_user_calibration: Option<raw::UserCalibrationML>,
    pub ml_system_notifications: Option<raw::SystemNotificationsML>,
    pub ml_world_mesh_detection: Option<raw::WorldMeshDetectionML>,
    pub ml_facial_expression: Option<raw::FacialExpressionML>,
    pub ml_view_configuration_depth_range_change: Option<raw::ViewConfigurationDepthRangeChangeML>,
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
        unsafe {
            Ok(Self {
                almalence_digital_lens_control: if required.almalence_digital_lens_control {
                    Some(raw::DigitalLensControlALMALENCE::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_trackables: if required.android_trackables {
                    Some(raw::TrackablesANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_device_anchor_persistence: if required.android_device_anchor_persistence {
                    Some(raw::DeviceAnchorPersistenceANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_face_tracking: if required.android_face_tracking {
                    Some(raw::FaceTrackingANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_passthrough_camera_state: if required.android_passthrough_camera_state {
                    Some(raw::PassthroughCameraStateANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_raycast: if required.android_raycast {
                    Some(raw::RaycastANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_trackables_object: if required.android_trackables_object {
                    Some(raw::TrackablesObjectANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_anchor_sharing_export: if required.android_anchor_sharing_export {
                    Some(raw::AnchorSharingExportANDROID::load(entry, instance)?)
                } else {
                    None
                },
                #[cfg(target_os = "android")]
                android_trackables_marker: if required.android_trackables_marker {
                    Some(raw::TrackablesMarkerANDROID::load(entry, instance)?)
                } else {
                    None
                },
                bd_controller_interaction: if required.bd_controller_interaction {
                    Some(raw::ControllerInteractionBD {})
                } else {
                    None
                },
                bd_body_tracking: if required.bd_body_tracking {
                    Some(raw::BodyTrackingBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_facial_simulation: if required.bd_facial_simulation {
                    Some(raw::FacialSimulationBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_spatial_sensing: if required.bd_spatial_sensing {
                    Some(raw::SpatialSensingBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_spatial_anchor: if required.bd_spatial_anchor {
                    Some(raw::SpatialAnchorBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_spatial_anchor_sharing: if required.bd_spatial_anchor_sharing {
                    Some(raw::SpatialAnchorSharingBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_spatial_scene: if required.bd_spatial_scene {
                    Some(raw::SpatialSceneBD::load(entry, instance)?)
                } else {
                    None
                },
                bd_spatial_mesh: if required.bd_spatial_mesh {
                    Some(raw::SpatialMeshBD {})
                } else {
                    None
                },
                bd_future_progress: if required.bd_future_progress {
                    Some(raw::FutureProgressBD {})
                } else {
                    None
                },
                bd_spatial_plane: if required.bd_spatial_plane {
                    Some(raw::SpatialPlaneBD {})
                } else {
                    None
                },
                bd_ultra_controller_interaction: if required.bd_ultra_controller_interaction {
                    Some(raw::UltraControllerInteractionBD {})
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
                ext_frame_synthesis: if required.ext_frame_synthesis {
                    Some(raw::FrameSynthesisEXT {})
                } else {
                    None
                },
                ext_uuid: if required.ext_uuid {
                    Some(raw::UuidEXT {})
                } else {
                    None
                },
                ext_render_model: if required.ext_render_model {
                    Some(raw::RenderModelEXT::load(entry, instance)?)
                } else {
                    None
                },
                ext_interaction_render_model: if required.ext_interaction_render_model {
                    Some(raw::InteractionRenderModelEXT::load(entry, instance)?)
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
                ext_composition_layer_inverted_alpha: if required
                    .ext_composition_layer_inverted_alpha
                {
                    Some(raw::CompositionLayerInvertedAlphaEXT {})
                } else {
                    None
                },
                ext_spatial_entity: if required.ext_spatial_entity {
                    Some(raw::SpatialEntityEXT::load(entry, instance)?)
                } else {
                    None
                },
                ext_spatial_plane_tracking: if required.ext_spatial_plane_tracking {
                    Some(raw::SpatialPlaneTrackingEXT {})
                } else {
                    None
                },
                ext_spatial_marker_tracking: if required.ext_spatial_marker_tracking {
                    Some(raw::SpatialMarkerTrackingEXT {})
                } else {
                    None
                },
                ext_spatial_anchor: if required.ext_spatial_anchor {
                    Some(raw::SpatialAnchorEXT::load(entry, instance)?)
                } else {
                    None
                },
                ext_spatial_persistence: if required.ext_spatial_persistence {
                    Some(raw::SpatialPersistenceEXT::load(entry, instance)?)
                } else {
                    None
                },
                ext_spatial_persistence_operations: if required.ext_spatial_persistence_operations {
                    Some(raw::SpatialPersistenceOperationsEXT::load(entry, instance)?)
                } else {
                    None
                },
                ext_loader_init_properties: if required.ext_loader_init_properties {
                    Some(raw::LoaderInitPropertiesEXT {})
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
                fb_android_surface_swapchain_create: if required.fb_android_surface_swapchain_create
                {
                    Some(raw::AndroidSurfaceSwapchainCreateFB {})
                } else {
                    None
                },
                fb_swapchain_update_state: if required.fb_swapchain_update_state {
                    Some(raw::SwapchainUpdateStateFB::load(entry, instance)?)
                } else {
                    None
                },
                fb_composition_layer_secure_content: if required.fb_composition_layer_secure_content
                {
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
                fb_swapchain_update_state_opengl_es: if required.fb_swapchain_update_state_opengl_es
                {
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
                htc_body_tracking: if required.htc_body_tracking {
                    Some(raw::BodyTrackingHTC::load(entry, instance)?)
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
                khr_extended_struct_name_lengths: if required.khr_extended_struct_name_lengths {
                    Some(raw::ExtendedStructNameLengthsKHR::load(entry, instance)?)
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
                khr_generic_controller: if required.khr_generic_controller {
                    Some(raw::GenericControllerKHR {})
                } else {
                    None
                },
                logitech_mx_ink_stylus_interaction: if required.logitech_mx_ink_stylus_interaction {
                    Some(raw::MxInkStylusInteractionLOGITECH {})
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
                meta_detached_controllers: if required.meta_detached_controllers {
                    Some(raw::DetachedControllersMETA {})
                } else {
                    None
                },
                meta_headset_id: if required.meta_headset_id {
                    Some(raw::HeadsetIdMETA {})
                } else {
                    None
                },
                meta_spatial_entity_discovery: if required.meta_spatial_entity_discovery {
                    Some(raw::SpatialEntityDiscoveryMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_hand_tracking_microgestures: if required.meta_hand_tracking_microgestures {
                    Some(raw::HandTrackingMicrogesturesMETA {})
                } else {
                    None
                },
                meta_recommended_layer_resolution: if required.meta_recommended_layer_resolution {
                    Some(raw::RecommendedLayerResolutionMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_spatial_entity_persistence: if required.meta_spatial_entity_persistence {
                    Some(raw::SpatialEntityPersistenceMETA::load(entry, instance)?)
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
                meta_body_tracking_full_body: if required.meta_body_tracking_full_body {
                    Some(raw::BodyTrackingFullBodyMETA {})
                } else {
                    None
                },
                meta_touch_controller_plus: if required.meta_touch_controller_plus {
                    Some(raw::TouchControllerPlusMETA {})
                } else {
                    None
                },
                meta_passthrough_layer_resumed_event: if required
                    .meta_passthrough_layer_resumed_event
                {
                    Some(raw::PassthroughLayerResumedEventMETA {})
                } else {
                    None
                },
                meta_body_tracking_calibration: if required.meta_body_tracking_calibration {
                    Some(raw::BodyTrackingCalibrationMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_spatial_entity_sharing: if required.meta_spatial_entity_sharing {
                    Some(raw::SpatialEntitySharingMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_environment_depth: if required.meta_environment_depth {
                    Some(raw::EnvironmentDepthMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_simultaneous_hands_and_controllers: if required
                    .meta_simultaneous_hands_and_controllers
                {
                    Some(raw::SimultaneousHandsAndControllersMETA::load(
                        entry, instance,
                    )?)
                } else {
                    None
                },
                meta_colocation_discovery: if required.meta_colocation_discovery {
                    Some(raw::ColocationDiscoveryMETA::load(entry, instance)?)
                } else {
                    None
                },
                meta_spatial_entity_group_sharing: if required.meta_spatial_entity_group_sharing {
                    Some(raw::SpatialEntityGroupSharingMETA {})
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
                ml_spatial_anchors: if required.ml_spatial_anchors {
                    Some(raw::SpatialAnchorsML::load(entry, instance)?)
                } else {
                    None
                },
                ml_spatial_anchors_storage: if required.ml_spatial_anchors_storage {
                    Some(raw::SpatialAnchorsStorageML::load(entry, instance)?)
                } else {
                    None
                },
                ml_user_calibration: if required.ml_user_calibration {
                    Some(raw::UserCalibrationML::load(entry, instance)?)
                } else {
                    None
                },
                ml_system_notifications: if required.ml_system_notifications {
                    Some(raw::SystemNotificationsML::load(entry, instance)?)
                } else {
                    None
                },
                ml_world_mesh_detection: if required.ml_world_mesh_detection {
                    Some(raw::WorldMeshDetectionML::load(entry, instance)?)
                } else {
                    None
                },
                ml_facial_expression: if required.ml_facial_expression {
                    Some(raw::FacialExpressionML::load(entry, instance)?)
                } else {
                    None
                },
                ml_view_configuration_depth_range_change: if required
                    .ml_view_configuration_depth_range_change
                {
                    Some(raw::ViewConfigurationDepthRangeChangeML {})
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
                msft_composition_layer_reprojection: if required.msft_composition_layer_reprojection
                {
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
                oculus_android_session_state_enable: if required.oculus_android_session_state_enable
                {
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
                qcom_tracking_optimization_settings: if required.qcom_tracking_optimization_settings
                {
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
    StartColocationAdvertisementCompleteMETA(StartColocationAdvertisementCompleteMETA<'a>),
    ColocationAdvertisementCompleteMETA(ColocationAdvertisementCompleteMETA<'a>),
    StopColocationAdvertisementCompleteMETA(StopColocationAdvertisementCompleteMETA<'a>),
    StartColocationDiscoveryCompleteMETA(StartColocationDiscoveryCompleteMETA<'a>),
    ColocationDiscoveryResultMETA(ColocationDiscoveryResultMETA<'a>),
    ColocationDiscoveryCompleteMETA(ColocationDiscoveryCompleteMETA<'a>),
    StopColocationDiscoveryCompleteMETA(StopColocationDiscoveryCompleteMETA<'a>),
    ShareSpacesCompleteMETA(ShareSpacesCompleteMETA<'a>),
    PassthroughStateChangedFB(PassthroughStateChangedFB<'a>),
    ViveTrackerConnectedHTCX(ViveTrackerConnectedHTCX<'a>),
    MarkerTrackingUpdateVARJO(MarkerTrackingUpdateVARJO<'a>),
    SpacesSaveResultMETA(SpacesSaveResultMETA<'a>),
    SpacesEraseResultMETA(SpacesEraseResultMETA<'a>),
    SpaceDiscoveryResultsAvailableMETA(SpaceDiscoveryResultsAvailableMETA<'a>),
    SpaceDiscoveryCompleteMETA(SpaceDiscoveryCompleteMETA<'a>),
    VirtualKeyboardCommitTextMETA(VirtualKeyboardCommitTextMETA<'a>),
    VirtualKeyboardBackspaceMETA(VirtualKeyboardBackspaceMETA<'a>),
    VirtualKeyboardEnterMETA(VirtualKeyboardEnterMETA<'a>),
    VirtualKeyboardShownMETA(VirtualKeyboardShownMETA<'a>),
    VirtualKeyboardHiddenMETA(VirtualKeyboardHiddenMETA<'a>),
    HeadsetFitChangedML(HeadsetFitChangedML<'a>),
    EyeCalibrationChangedML(EyeCalibrationChangedML<'a>),
    LocalizationChangedML(LocalizationChangedML<'a>),
    UserPresenceChangedEXT(UserPresenceChangedEXT<'a>),
    SpatialDiscoveryRecommendedEXT(SpatialDiscoveryRecommendedEXT<'a>),
    PassthroughLayerResumedMETA(PassthroughLayerResumedMETA<'a>),
    InteractionRenderModelsChangedEXT,
    SenseDataProviderStateChangedBD(SenseDataProviderStateChangedBD<'a>),
    SenseDataUpdatedBD(SenseDataUpdatedBD<'a>),
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
        unsafe {
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
                sys::StructureType::EVENT_DATA_START_COLOCATION_ADVERTISEMENT_COMPLETE_META => {
                    let typed =
                        &*(raw as *const sys::EventDataStartColocationAdvertisementCompleteMETA);
                    Event::StartColocationAdvertisementCompleteMETA(
                        StartColocationAdvertisementCompleteMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_COLOCATION_ADVERTISEMENT_COMPLETE_META => {
                    let typed = &*(raw as *const sys::EventDataColocationAdvertisementCompleteMETA);
                    Event::ColocationAdvertisementCompleteMETA(
                        ColocationAdvertisementCompleteMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_STOP_COLOCATION_ADVERTISEMENT_COMPLETE_META => {
                    let typed =
                        &*(raw as *const sys::EventDataStopColocationAdvertisementCompleteMETA);
                    Event::StopColocationAdvertisementCompleteMETA(
                        StopColocationAdvertisementCompleteMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_START_COLOCATION_DISCOVERY_COMPLETE_META => {
                    let typed =
                        &*(raw as *const sys::EventDataStartColocationDiscoveryCompleteMETA);
                    Event::StartColocationDiscoveryCompleteMETA(
                        StartColocationDiscoveryCompleteMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_COLOCATION_DISCOVERY_RESULT_META => {
                    let typed = &*(raw as *const sys::EventDataColocationDiscoveryResultMETA);
                    Event::ColocationDiscoveryResultMETA(ColocationDiscoveryResultMETA::new(typed))
                }
                sys::StructureType::EVENT_DATA_COLOCATION_DISCOVERY_COMPLETE_META => {
                    let typed = &*(raw as *const sys::EventDataColocationDiscoveryCompleteMETA);
                    Event::ColocationDiscoveryCompleteMETA(ColocationDiscoveryCompleteMETA::new(
                        typed,
                    ))
                }
                sys::StructureType::EVENT_DATA_STOP_COLOCATION_DISCOVERY_COMPLETE_META => {
                    let typed = &*(raw as *const sys::EventDataStopColocationDiscoveryCompleteMETA);
                    Event::StopColocationDiscoveryCompleteMETA(
                        StopColocationDiscoveryCompleteMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_SHARE_SPACES_COMPLETE_META => {
                    let typed = &*(raw as *const sys::EventDataShareSpacesCompleteMETA);
                    Event::ShareSpacesCompleteMETA(ShareSpacesCompleteMETA::new(typed))
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
                sys::StructureType::EVENT_DATA_SPACES_SAVE_RESULT_META => {
                    let typed = &*(raw as *const sys::EventDataSpacesSaveResultMETA);
                    Event::SpacesSaveResultMETA(SpacesSaveResultMETA::new(typed))
                }
                sys::StructureType::EVENT_DATA_SPACES_ERASE_RESULT_META => {
                    let typed = &*(raw as *const sys::EventDataSpacesEraseResultMETA);
                    Event::SpacesEraseResultMETA(SpacesEraseResultMETA::new(typed))
                }
                sys::StructureType::EVENT_DATA_SPACE_DISCOVERY_RESULTS_AVAILABLE_META => {
                    let typed = &*(raw as *const sys::EventDataSpaceDiscoveryResultsAvailableMETA);
                    Event::SpaceDiscoveryResultsAvailableMETA(
                        SpaceDiscoveryResultsAvailableMETA::new(typed),
                    )
                }
                sys::StructureType::EVENT_DATA_SPACE_DISCOVERY_COMPLETE_META => {
                    let typed = &*(raw as *const sys::EventDataSpaceDiscoveryCompleteMETA);
                    Event::SpaceDiscoveryCompleteMETA(SpaceDiscoveryCompleteMETA::new(typed))
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
                sys::StructureType::EVENT_DATA_SPATIAL_DISCOVERY_RECOMMENDED_EXT => {
                    let typed = &*(raw as *const sys::EventDataSpatialDiscoveryRecommendedEXT);
                    Event::SpatialDiscoveryRecommendedEXT(SpatialDiscoveryRecommendedEXT::new(
                        typed,
                    ))
                }
                sys::StructureType::EVENT_DATA_PASSTHROUGH_LAYER_RESUMED_META => {
                    let typed = &*(raw as *const sys::EventDataPassthroughLayerResumedMETA);
                    Event::PassthroughLayerResumedMETA(PassthroughLayerResumedMETA::new(typed))
                }
                sys::StructureType::EVENT_DATA_INTERACTION_RENDER_MODELS_CHANGED_EXT => {
                    Event::InteractionRenderModelsChangedEXT
                }
                sys::StructureType::EVENT_DATA_SENSE_DATA_PROVIDER_STATE_CHANGED_BD => {
                    let typed = &*(raw as *const sys::EventDataSenseDataProviderStateChangedBD);
                    Event::SenseDataProviderStateChangedBD(SenseDataProviderStateChangedBD::new(
                        typed,
                    ))
                }
                sys::StructureType::EVENT_DATA_SENSE_DATA_UPDATED_BD => {
                    let typed = &*(raw as *const sys::EventDataSenseDataUpdatedBD);
                    Event::SenseDataUpdatedBD(SenseDataUpdatedBD::new(typed))
                }
                _ => {
                    return None;
                }
            })
        }
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
pub struct StartColocationAdvertisementCompleteMETA<'a>(
    &'a sys::EventDataStartColocationAdvertisementCompleteMETA,
);
impl<'a> StartColocationAdvertisementCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataStartColocationAdvertisementCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataStartColocationAdvertisementCompleteMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn advertisement_request_id(self) -> AsyncRequestIdFB {
        (self.0).advertisement_request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
    #[inline]
    pub fn advertisement_uuid(self) -> Uuid {
        (self.0).advertisement_uuid
    }
}
#[derive(Copy, Clone)]
pub struct ColocationAdvertisementCompleteMETA<'a>(
    &'a sys::EventDataColocationAdvertisementCompleteMETA,
);
impl<'a> ColocationAdvertisementCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataColocationAdvertisementCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataColocationAdvertisementCompleteMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn advertisement_request_id(self) -> AsyncRequestIdFB {
        (self.0).advertisement_request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct StopColocationAdvertisementCompleteMETA<'a>(
    &'a sys::EventDataStopColocationAdvertisementCompleteMETA,
);
impl<'a> StopColocationAdvertisementCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataStopColocationAdvertisementCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataStopColocationAdvertisementCompleteMETA) -> Self {
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
pub struct StartColocationDiscoveryCompleteMETA<'a>(
    &'a sys::EventDataStartColocationDiscoveryCompleteMETA,
);
impl<'a> StartColocationDiscoveryCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataStartColocationDiscoveryCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataStartColocationDiscoveryCompleteMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn discovery_request_id(self) -> AsyncRequestIdFB {
        (self.0).discovery_request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct ColocationDiscoveryResultMETA<'a>(&'a sys::EventDataColocationDiscoveryResultMETA);
impl<'a> ColocationDiscoveryResultMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataColocationDiscoveryResultMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataColocationDiscoveryResultMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn discovery_request_id(self) -> AsyncRequestIdFB {
        (self.0).discovery_request_id
    }
    #[inline]
    pub fn advertisement_uuid(self) -> Uuid {
        (self.0).advertisement_uuid
    }
    #[inline]
    pub fn buffer(self) -> [u8; MAX_COLOCATION_DISCOVERY_BUFFER_SIZE_META] {
        (self.0).buffer
    }
}
#[derive(Copy, Clone)]
pub struct ColocationDiscoveryCompleteMETA<'a>(&'a sys::EventDataColocationDiscoveryCompleteMETA);
impl<'a> ColocationDiscoveryCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataColocationDiscoveryCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataColocationDiscoveryCompleteMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn discovery_request_id(self) -> AsyncRequestIdFB {
        (self.0).discovery_request_id
    }
    #[inline]
    pub fn result(self) -> sys::Result {
        (self.0).result
    }
}
#[derive(Copy, Clone)]
pub struct StopColocationDiscoveryCompleteMETA<'a>(
    &'a sys::EventDataStopColocationDiscoveryCompleteMETA,
);
impl<'a> StopColocationDiscoveryCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataStopColocationDiscoveryCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataStopColocationDiscoveryCompleteMETA) -> Self {
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
pub struct ShareSpacesCompleteMETA<'a>(&'a sys::EventDataShareSpacesCompleteMETA);
impl<'a> ShareSpacesCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataShareSpacesCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataShareSpacesCompleteMETA) -> Self {
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
pub struct SpacesSaveResultMETA<'a>(&'a sys::EventDataSpacesSaveResultMETA);
impl<'a> SpacesSaveResultMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpacesSaveResultMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpacesSaveResultMETA) -> Self {
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
pub struct SpacesEraseResultMETA<'a>(&'a sys::EventDataSpacesEraseResultMETA);
impl<'a> SpacesEraseResultMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpacesEraseResultMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpacesEraseResultMETA) -> Self {
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
pub struct SpaceDiscoveryResultsAvailableMETA<'a>(
    &'a sys::EventDataSpaceDiscoveryResultsAvailableMETA,
);
impl<'a> SpaceDiscoveryResultsAvailableMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceDiscoveryResultsAvailableMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceDiscoveryResultsAvailableMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn request_id(self) -> AsyncRequestIdFB {
        (self.0).request_id
    }
}
#[derive(Copy, Clone)]
pub struct SpaceDiscoveryCompleteMETA<'a>(&'a sys::EventDataSpaceDiscoveryCompleteMETA);
impl<'a> SpaceDiscoveryCompleteMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpaceDiscoveryCompleteMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpaceDiscoveryCompleteMETA) -> Self {
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
#[derive(Copy, Clone)]
pub struct SpatialDiscoveryRecommendedEXT<'a>(&'a sys::EventDataSpatialDiscoveryRecommendedEXT);
impl<'a> SpatialDiscoveryRecommendedEXT<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSpatialDiscoveryRecommendedEXT]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSpatialDiscoveryRecommendedEXT) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn spatial_context(self) -> sys::SpatialContextEXT {
        (self.0).spatial_context
    }
}
#[derive(Copy, Clone)]
pub struct PassthroughLayerResumedMETA<'a>(&'a sys::EventDataPassthroughLayerResumedMETA);
impl<'a> PassthroughLayerResumedMETA<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataPassthroughLayerResumedMETA]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataPassthroughLayerResumedMETA) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn layer(self) -> sys::PassthroughLayerFB {
        (self.0).layer
    }
}
#[derive(Copy, Clone)]
pub struct InteractionRenderModelsChangedEXT<'a>(
    &'a sys::EventDataInteractionRenderModelsChangedEXT,
);
impl<'a> InteractionRenderModelsChangedEXT<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataInteractionRenderModelsChangedEXT]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataInteractionRenderModelsChangedEXT) -> Self {
        Self(inner)
    }
}
#[derive(Copy, Clone)]
pub struct SenseDataProviderStateChangedBD<'a>(&'a sys::EventDataSenseDataProviderStateChangedBD);
impl<'a> SenseDataProviderStateChangedBD<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSenseDataProviderStateChangedBD]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSenseDataProviderStateChangedBD) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn provider(self) -> sys::SenseDataProviderBD {
        (self.0).provider
    }
    #[inline]
    pub fn new_state(self) -> SenseDataProviderStateBD {
        (self.0).new_state
    }
}
#[derive(Copy, Clone)]
pub struct SenseDataUpdatedBD<'a>(&'a sys::EventDataSenseDataUpdatedBD);
impl<'a> SenseDataUpdatedBD<'a> {
    #[inline]
    #[doc = r" # Safety"]
    #[doc = r" `inner` must be valid event data according to the OpenXR spec. Refer to"]
    #[doc = "[sys::EventDataSenseDataUpdatedBD]"]
    #[doc = r" for more information."]
    pub unsafe fn new(inner: &'a sys::EventDataSenseDataUpdatedBD) -> Self {
        Self(inner)
    }
    #[inline]
    pub fn provider(self) -> sys::SenseDataProviderBD {
        (self.0).provider
    }
}
pub mod raw {
    use crate::{Entry, Result};
    use std::mem;
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
            unsafe {
                Ok(Self {
                    get_instance_proc_addr: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetInstanceProcAddr")?,
                    ),
                    enumerate_api_layer_properties: entry
                        .get_instance_proc_addr(instance, c"xrEnumerateApiLayerProperties")
                        .map(|s| mem::transmute(s))
                        .unwrap_or(crate::stub_enumerate_api_layer_properties),
                    enumerate_instance_extension_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateInstanceExtensionProperties",
                        )?,
                    ),
                    create_instance: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateInstance")?,
                    ),
                    destroy_instance: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyInstance")?,
                    ),
                    result_to_string: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrResultToString")?,
                    ),
                    structure_type_to_string: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStructureTypeToString")?,
                    ),
                    get_instance_properties: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetInstanceProperties")?,
                    ),
                    get_system: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSystem")?,
                    ),
                    get_system_properties: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSystemProperties")?,
                    ),
                    create_session: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSession")?,
                    ),
                    destroy_session: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySession")?,
                    ),
                    destroy_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpace")?,
                    ),
                    enumerate_swapchain_formats: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateSwapchainFormats")?,
                    ),
                    create_swapchain: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSwapchain")?,
                    ),
                    destroy_swapchain: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySwapchain")?,
                    ),
                    enumerate_swapchain_images: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateSwapchainImages")?,
                    ),
                    acquire_swapchain_image: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrAcquireSwapchainImage")?,
                    ),
                    wait_swapchain_image: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrWaitSwapchainImage")?,
                    ),
                    release_swapchain_image: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrReleaseSwapchainImage")?,
                    ),
                    begin_session: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrBeginSession")?,
                    ),
                    end_session: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEndSession")?,
                    ),
                    request_exit_session: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestExitSession")?,
                    ),
                    enumerate_reference_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateReferenceSpaces")?,
                    ),
                    create_reference_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateReferenceSpace")?,
                    ),
                    create_action_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateActionSpace")?,
                    ),
                    locate_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateSpace")?,
                    ),
                    enumerate_view_configurations: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateViewConfigurations")?,
                    ),
                    enumerate_environment_blend_modes: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateEnvironmentBlendModes",
                        )?,
                    ),
                    get_view_configuration_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetViewConfigurationProperties",
                        )?,
                    ),
                    enumerate_view_configuration_views: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateViewConfigurationViews",
                        )?,
                    ),
                    begin_frame: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrBeginFrame")?,
                    ),
                    locate_views: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateViews")?,
                    ),
                    end_frame: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEndFrame")?,
                    ),
                    wait_frame: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrWaitFrame")?,
                    ),
                    apply_haptic_feedback: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrApplyHapticFeedback")?,
                    ),
                    stop_haptic_feedback: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStopHapticFeedback")?,
                    ),
                    poll_event: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPollEvent")?,
                    ),
                    string_to_path: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStringToPath")?,
                    ),
                    path_to_string: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPathToString")?,
                    ),
                    get_reference_space_bounds_rect: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetReferenceSpaceBoundsRect")?,
                    ),
                    get_action_state_boolean: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetActionStateBoolean")?,
                    ),
                    get_action_state_float: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetActionStateFloat")?,
                    ),
                    get_action_state_vector2f: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetActionStateVector2f")?,
                    ),
                    get_action_state_pose: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetActionStatePose")?,
                    ),
                    create_action_set: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateActionSet")?,
                    ),
                    destroy_action_set: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyActionSet")?,
                    ),
                    create_action: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateAction")?,
                    ),
                    destroy_action: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyAction")?,
                    ),
                    suggest_interaction_profile_bindings: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSuggestInteractionProfileBindings",
                        )?,
                    ),
                    attach_session_action_sets: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrAttachSessionActionSets")?,
                    ),
                    get_current_interaction_profile: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetCurrentInteractionProfile")?,
                    ),
                    sync_actions: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSyncActions")?,
                    ),
                    enumerate_bound_sources_for_action: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateBoundSourcesForAction",
                        )?,
                    ),
                    get_input_source_localized_name: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetInputSourceLocalizedName")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct DigitalLensControlALMALENCE {
        pub set_digital_lens_control: pfn::SetDigitalLensControlALMALENCE,
    }
    impl DigitalLensControlALMALENCE {
        pub const VERSION: u32 = sys::ALMALENCE_digital_lens_control_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ALMALENCE_DIGITAL_LENS_CONTROL_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_digital_lens_control: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetDigitalLensControlALMALENCE",
                        )?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct TrackablesANDROID {
        pub enumerate_supported_trackable_types: pfn::EnumerateSupportedTrackableTypesANDROID,
        pub enumerate_supported_anchor_trackable_types:
            pfn::EnumerateSupportedAnchorTrackableTypesANDROID,
        pub create_trackable_tracker: pfn::CreateTrackableTrackerANDROID,
        pub destroy_trackable_tracker: pfn::DestroyTrackableTrackerANDROID,
        pub get_all_trackables: pfn::GetAllTrackablesANDROID,
        pub get_trackable_plane: pfn::GetTrackablePlaneANDROID,
        pub create_anchor_space: pfn::CreateAnchorSpaceANDROID,
    }
    #[cfg(target_os = "android")]
    impl TrackablesANDROID {
        pub const VERSION: u32 = sys::ANDROID_trackables_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_TRACKABLES_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_supported_trackable_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSupportedTrackableTypesANDROID",
                        )?,
                    ),
                    enumerate_supported_anchor_trackable_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSupportedAnchorTrackableTypesANDROID",
                        )?,
                    ),
                    create_trackable_tracker: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateTrackableTrackerANDROID")?,
                    ),
                    destroy_trackable_tracker: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyTrackableTrackerANDROID",
                        )?,
                    ),
                    get_all_trackables: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetAllTrackablesANDROID")?,
                    ),
                    get_trackable_plane: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetTrackablePlaneANDROID")?,
                    ),
                    create_anchor_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateAnchorSpaceANDROID")?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct DeviceAnchorPersistenceANDROID {
        pub enumerate_supported_persistence_anchor_types:
            pfn::EnumerateSupportedPersistenceAnchorTypesANDROID,
        pub create_device_anchor_persistence: pfn::CreateDeviceAnchorPersistenceANDROID,
        pub destroy_device_anchor_persistence: pfn::DestroyDeviceAnchorPersistenceANDROID,
        pub persist_anchor: pfn::PersistAnchorANDROID,
        pub get_anchor_persist_state: pfn::GetAnchorPersistStateANDROID,
        pub create_persisted_anchor_space: pfn::CreatePersistedAnchorSpaceANDROID,
        pub enumerate_persisted_anchors: pfn::EnumeratePersistedAnchorsANDROID,
        pub unpersist_anchor: pfn::UnpersistAnchorANDROID,
    }
    #[cfg(target_os = "android")]
    impl DeviceAnchorPersistenceANDROID {
        pub const VERSION: u32 = sys::ANDROID_device_anchor_persistence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_DEVICE_ANCHOR_PERSISTENCE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_supported_persistence_anchor_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSupportedPersistenceAnchorTypesANDROID",
                        )?,
                    ),
                    create_device_anchor_persistence: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateDeviceAnchorPersistenceANDROID",
                        )?,
                    ),
                    destroy_device_anchor_persistence: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyDeviceAnchorPersistenceANDROID",
                        )?,
                    ),
                    persist_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPersistAnchorANDROID")?,
                    ),
                    get_anchor_persist_state: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetAnchorPersistStateANDROID")?,
                    ),
                    create_persisted_anchor_space: mem::transmute(entry.get_instance_proc_addr(
                        instance,
                        c"xrCreatePersistedAnchorSpaceANDROID",
                    )?),
                    enumerate_persisted_anchors: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumeratePersistedAnchorsANDROID",
                        )?,
                    ),
                    unpersist_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrUnpersistAnchorANDROID")?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct FaceTrackingANDROID {
        pub create_face_tracker: pfn::CreateFaceTrackerANDROID,
        pub destroy_face_tracker: pfn::DestroyFaceTrackerANDROID,
        pub get_face_state: pfn::GetFaceStateANDROID,
        pub get_face_calibration_state: pfn::GetFaceCalibrationStateANDROID,
    }
    #[cfg(target_os = "android")]
    impl FaceTrackingANDROID {
        pub const VERSION: u32 = sys::ANDROID_face_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_FACE_TRACKING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFaceTrackerANDROID")?,
                    ),
                    destroy_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFaceTrackerANDROID")?,
                    ),
                    get_face_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFaceStateANDROID")?,
                    ),
                    get_face_calibration_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetFaceCalibrationStateANDROID",
                        )?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct PassthroughCameraStateANDROID {
        pub get_passthrough_camera_state: pfn::GetPassthroughCameraStateANDROID,
    }
    #[cfg(target_os = "android")]
    impl PassthroughCameraStateANDROID {
        pub const VERSION: u32 = sys::ANDROID_passthrough_camera_state_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_PASSTHROUGH_CAMERA_STATE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_passthrough_camera_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetPassthroughCameraStateANDROID",
                        )?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct RaycastANDROID {
        pub enumerate_raycast_supported_trackable_types:
            pfn::EnumerateRaycastSupportedTrackableTypesANDROID,
        pub raycast: pfn::RaycastANDROID,
    }
    #[cfg(target_os = "android")]
    impl RaycastANDROID {
        pub const VERSION: u32 = sys::ANDROID_raycast_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_RAYCAST_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_raycast_supported_trackable_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateRaycastSupportedTrackableTypesANDROID",
                        )?,
                    ),
                    raycast: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRaycastANDROID")?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct TrackablesObjectANDROID {
        pub get_trackable_object: pfn::GetTrackableObjectANDROID,
    }
    #[cfg(target_os = "android")]
    impl TrackablesObjectANDROID {
        pub const VERSION: u32 = sys::ANDROID_trackables_object_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_TRACKABLES_OBJECT_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_trackable_object: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetTrackableObjectANDROID")?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AnchorSharingExportANDROID {
        pub share_anchor: pfn::ShareAnchorANDROID,
        pub unshare_anchor: pfn::UnshareAnchorANDROID,
    }
    #[cfg(target_os = "android")]
    impl AnchorSharingExportANDROID {
        pub const VERSION: u32 = sys::ANDROID_anchor_sharing_export_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_ANCHOR_SHARING_EXPORT_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    share_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrShareAnchorANDROID")?,
                    ),
                    unshare_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrUnshareAnchorANDROID")?,
                    ),
                })
            }
        }
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct TrackablesMarkerANDROID {
        pub get_trackable_marker: pfn::GetTrackableMarkerANDROID,
    }
    #[cfg(target_os = "android")]
    impl TrackablesMarkerANDROID {
        pub const VERSION: u32 = sys::ANDROID_trackables_marker_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ANDROID_TRACKABLES_MARKER_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_trackable_marker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetTrackableMarkerANDROID")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionBD {}
    impl ControllerInteractionBD {
        pub const VERSION: u32 = sys::BD_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct BodyTrackingBD {
        pub create_body_tracker: pfn::CreateBodyTrackerBD,
        pub destroy_body_tracker: pfn::DestroyBodyTrackerBD,
        pub locate_body_joints: pfn::LocateBodyJointsBD,
    }
    impl BodyTrackingBD {
        pub const VERSION: u32 = sys::BD_body_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_BODY_TRACKING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateBodyTrackerBD")?,
                    ),
                    destroy_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyBodyTrackerBD")?,
                    ),
                    locate_body_joints: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateBodyJointsBD")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct FacialSimulationBD {
        pub enumerate_facial_simulation_modes: pfn::EnumerateFacialSimulationModesBD,
        pub create_face_tracker: pfn::CreateFaceTrackerBD,
        pub destroy_face_tracker: pfn::DestroyFaceTrackerBD,
        pub get_facial_simulation_data: pfn::GetFacialSimulationDataBD,
        pub set_facial_simulation_mode: pfn::SetFacialSimulationModeBD,
        pub get_facial_simulation_mode: pfn::GetFacialSimulationModeBD,
    }
    impl FacialSimulationBD {
        pub const VERSION: u32 = sys::BD_facial_simulation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_FACIAL_SIMULATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_facial_simulation_modes: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateFacialSimulationModesBD",
                        )?,
                    ),
                    create_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFaceTrackerBD")?,
                    ),
                    destroy_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFaceTrackerBD")?,
                    ),
                    get_facial_simulation_data: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFacialSimulationDataBD")?,
                    ),
                    set_facial_simulation_mode: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetFacialSimulationModeBD")?,
                    ),
                    get_facial_simulation_mode: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFacialSimulationModeBD")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialSensingBD {
        pub enumerate_spatial_entity_component_types: pfn::EnumerateSpatialEntityComponentTypesBD,
        pub get_spatial_entity_uuid: pfn::GetSpatialEntityUuidBD,
        pub get_spatial_entity_component_data: pfn::GetSpatialEntityComponentDataBD,
        pub create_sense_data_provider: pfn::CreateSenseDataProviderBD,
        pub start_sense_data_provider_async: pfn::StartSenseDataProviderAsyncBD,
        pub start_sense_data_provider_complete: pfn::StartSenseDataProviderCompleteBD,
        pub get_sense_data_provider_state: pfn::GetSenseDataProviderStateBD,
        pub query_sense_data_async: pfn::QuerySenseDataAsyncBD,
        pub query_sense_data_complete: pfn::QuerySenseDataCompleteBD,
        pub destroy_sense_data_snapshot: pfn::DestroySenseDataSnapshotBD,
        pub get_queried_sense_data: pfn::GetQueriedSenseDataBD,
        pub stop_sense_data_provider: pfn::StopSenseDataProviderBD,
        pub destroy_sense_data_provider: pfn::DestroySenseDataProviderBD,
        pub create_spatial_entity_anchor: pfn::CreateSpatialEntityAnchorBD,
        pub destroy_anchor: pfn::DestroyAnchorBD,
        pub get_anchor_uuid: pfn::GetAnchorUuidBD,
        pub create_anchor_space: pfn::CreateAnchorSpaceBD,
    }
    impl SpatialSensingBD {
        pub const VERSION: u32 = sys::BD_spatial_sensing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_SENSING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_spatial_entity_component_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpatialEntityComponentTypesBD",
                        )?,
                    ),
                    get_spatial_entity_uuid: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialEntityUuidBD")?,
                    ),
                    get_spatial_entity_component_data: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetSpatialEntityComponentDataBD",
                        )?,
                    ),
                    create_sense_data_provider: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSenseDataProviderBD")?,
                    ),
                    start_sense_data_provider_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrStartSenseDataProviderAsyncBD")?,
                    ),
                    start_sense_data_provider_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrStartSenseDataProviderCompleteBD",
                        )?,
                    ),
                    get_sense_data_provider_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSenseDataProviderStateBD")?,
                    ),
                    query_sense_data_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrQuerySenseDataAsyncBD")?,
                    ),
                    query_sense_data_complete: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrQuerySenseDataCompleteBD")?,
                    ),
                    destroy_sense_data_snapshot: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySenseDataSnapshotBD")?,
                    ),
                    get_queried_sense_data: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetQueriedSenseDataBD")?,
                    ),
                    stop_sense_data_provider: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStopSenseDataProviderBD")?,
                    ),
                    destroy_sense_data_provider: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySenseDataProviderBD")?,
                    ),
                    create_spatial_entity_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialEntityAnchorBD")?,
                    ),
                    destroy_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyAnchorBD")?,
                    ),
                    get_anchor_uuid: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetAnchorUuidBD")?,
                    ),
                    create_anchor_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateAnchorSpaceBD")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorBD {
        pub create_spatial_anchor_async: pfn::CreateSpatialAnchorAsyncBD,
        pub create_spatial_anchor_complete: pfn::CreateSpatialAnchorCompleteBD,
        pub persist_spatial_anchor_async: pfn::PersistSpatialAnchorAsyncBD,
        pub persist_spatial_anchor_complete: pfn::PersistSpatialAnchorCompleteBD,
        pub unpersist_spatial_anchor_async: pfn::UnpersistSpatialAnchorAsyncBD,
        pub unpersist_spatial_anchor_complete: pfn::UnpersistSpatialAnchorCompleteBD,
    }
    impl SpatialAnchorBD {
        pub const VERSION: u32 = sys::BD_spatial_anchor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_ANCHOR_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorAsyncBD")?,
                    ),
                    create_spatial_anchor_complete: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateSpatialAnchorCompleteBD")?,
                    ),
                    persist_spatial_anchor_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPersistSpatialAnchorAsyncBD")?,
                    ),
                    persist_spatial_anchor_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPersistSpatialAnchorCompleteBD",
                        )?,
                    ),
                    unpersist_spatial_anchor_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrUnpersistSpatialAnchorAsyncBD")?,
                    ),
                    unpersist_spatial_anchor_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrUnpersistSpatialAnchorCompleteBD",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorSharingBD {
        pub share_spatial_anchor_async: pfn::ShareSpatialAnchorAsyncBD,
        pub share_spatial_anchor_complete: pfn::ShareSpatialAnchorCompleteBD,
        pub download_shared_spatial_anchor_async: pfn::DownloadSharedSpatialAnchorAsyncBD,
        pub download_shared_spatial_anchor_complete: pfn::DownloadSharedSpatialAnchorCompleteBD,
    }
    impl SpatialAnchorSharingBD {
        pub const VERSION: u32 = sys::BD_spatial_anchor_sharing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_ANCHOR_SHARING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    share_spatial_anchor_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrShareSpatialAnchorAsyncBD")?,
                    ),
                    share_spatial_anchor_complete: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrShareSpatialAnchorCompleteBD")?,
                    ),
                    download_shared_spatial_anchor_async: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDownloadSharedSpatialAnchorAsyncBD",
                        )?,
                    ),
                    download_shared_spatial_anchor_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDownloadSharedSpatialAnchorCompleteBD",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialSceneBD {
        pub capture_scene_async: pfn::CaptureSceneAsyncBD,
        pub capture_scene_complete: pfn::CaptureSceneCompleteBD,
    }
    impl SpatialSceneBD {
        pub const VERSION: u32 = sys::BD_spatial_scene_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_SCENE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    capture_scene_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCaptureSceneAsyncBD")?,
                    ),
                    capture_scene_complete: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCaptureSceneCompleteBD")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialMeshBD {}
    impl SpatialMeshBD {
        pub const VERSION: u32 = sys::BD_spatial_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_MESH_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct FutureProgressBD {}
    impl FutureProgressBD {
        pub const VERSION: u32 = sys::BD_future_progress_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_FUTURE_PROGRESS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialPlaneBD {}
    impl SpatialPlaneBD {
        pub const VERSION: u32 = sys::BD_spatial_plane_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_SPATIAL_PLANE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct UltraControllerInteractionBD {}
    impl UltraControllerInteractionBD {
        pub const VERSION: u32 = sys::BD_ultra_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::BD_ULTRA_CONTROLLER_INTERACTION_EXTENSION_NAME;
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
            unsafe {
                Ok(Self {
                    perf_settings_set_performance_level: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPerfSettingsSetPerformanceLevelEXT",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    thermal_get_temperature_trend: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrThermalGetTemperatureTrendEXT")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    set_debug_utils_object_name: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetDebugUtilsObjectNameEXT")?,
                    ),
                    create_debug_utils_messenger: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateDebugUtilsMessengerEXT")?,
                    ),
                    destroy_debug_utils_messenger: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrDestroyDebugUtilsMessengerEXT")?,
                    ),
                    submit_debug_utils_message: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSubmitDebugUtilsMessageEXT")?,
                    ),
                    session_begin_debug_utils_label_region: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSessionBeginDebugUtilsLabelRegionEXT",
                        )?,
                    ),
                    session_end_debug_utils_label_region: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSessionEndDebugUtilsLabelRegionEXT",
                        )?,
                    ),
                    session_insert_debug_utils_label: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSessionInsertDebugUtilsLabelEXT",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    set_input_device_active: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetInputDeviceActiveEXT")?,
                    ),
                    set_input_device_state_bool: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetInputDeviceStateBoolEXT")?,
                    ),
                    set_input_device_state_float: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetInputDeviceStateFloatEXT")?,
                    ),
                    set_input_device_state_vector2f: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetInputDeviceStateVector2fEXT",
                        )?,
                    ),
                    set_input_device_location: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetInputDeviceLocationEXT")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    create_hand_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateHandTrackerEXT")?,
                    ),
                    destroy_hand_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyHandTrackerEXT")?,
                    ),
                    locate_hand_joints: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateHandJointsEXT")?,
                    ),
                })
            }
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
    pub struct DpadBindingEXT {}
    impl DpadBindingEXT {
        pub const VERSION: u32 = sys::EXT_dpad_binding_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_DPAD_BINDING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HandJointsMotionRangeEXT {}
    impl HandJointsMotionRangeEXT {
        pub const VERSION: u32 = sys::EXT_hand_joints_motion_range_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_JOINTS_MOTION_RANGE_EXTENSION_NAME;
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
    pub struct PalmPoseEXT {}
    impl PalmPoseEXT {
        pub const VERSION: u32 = sys::EXT_palm_pose_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_PALM_POSE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct FrameSynthesisEXT {}
    impl FrameSynthesisEXT {
        pub const VERSION: u32 = sys::EXT_frame_synthesis_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_FRAME_SYNTHESIS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct UuidEXT {}
    impl UuidEXT {
        pub const VERSION: u32 = sys::EXT_uuid_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_UUID_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct RenderModelEXT {
        pub create_render_model: pfn::CreateRenderModelEXT,
        pub destroy_render_model: pfn::DestroyRenderModelEXT,
        pub get_render_model_properties: pfn::GetRenderModelPropertiesEXT,
        pub create_render_model_space: pfn::CreateRenderModelSpaceEXT,
        pub create_render_model_asset: pfn::CreateRenderModelAssetEXT,
        pub destroy_render_model_asset: pfn::DestroyRenderModelAssetEXT,
        pub get_render_model_asset_data: pfn::GetRenderModelAssetDataEXT,
        pub get_render_model_asset_properties: pfn::GetRenderModelAssetPropertiesEXT,
        pub get_render_model_state: pfn::GetRenderModelStateEXT,
    }
    impl RenderModelEXT {
        pub const VERSION: u32 = sys::EXT_render_model_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_RENDER_MODEL_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_render_model: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateRenderModelEXT")?,
                    ),
                    destroy_render_model: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyRenderModelEXT")?,
                    ),
                    get_render_model_properties: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetRenderModelPropertiesEXT")?,
                    ),
                    create_render_model_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateRenderModelSpaceEXT")?,
                    ),
                    create_render_model_asset: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateRenderModelAssetEXT")?,
                    ),
                    destroy_render_model_asset: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyRenderModelAssetEXT")?,
                    ),
                    get_render_model_asset_data: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetRenderModelAssetDataEXT")?,
                    ),
                    get_render_model_asset_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetRenderModelAssetPropertiesEXT",
                        )?,
                    ),
                    get_render_model_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetRenderModelStateEXT")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct InteractionRenderModelEXT {
        pub enumerate_interaction_render_model_ids: pfn::EnumerateInteractionRenderModelIdsEXT,
        pub enumerate_render_model_subaction_paths: pfn::EnumerateRenderModelSubactionPathsEXT,
        pub get_render_model_pose_top_level_user_path: pfn::GetRenderModelPoseTopLevelUserPathEXT,
    }
    impl InteractionRenderModelEXT {
        pub const VERSION: u32 = sys::EXT_interaction_render_model_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_INTERACTION_RENDER_MODEL_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_interaction_render_model_ids: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateInteractionRenderModelIdsEXT",
                        )?,
                    ),
                    enumerate_render_model_subaction_paths: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateRenderModelSubactionPathsEXT",
                        )?,
                    ),
                    get_render_model_pose_top_level_user_path: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetRenderModelPoseTopLevelUserPathEXT",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionEXT {}
    impl HandInteractionEXT {
        pub const VERSION: u32 = sys::EXT_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ActiveActionSetPriorityEXT {}
    impl ActiveActionSetPriorityEXT {
        pub const VERSION: u32 = sys::EXT_active_action_set_priority_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_ACTIVE_ACTION_SET_PRIORITY_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct LocalFloorEXT {}
    impl LocalFloorEXT {
        pub const VERSION: u32 = sys::EXT_local_floor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_LOCAL_FLOOR_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingDataSourceEXT {}
    impl HandTrackingDataSourceEXT {
        pub const VERSION: u32 = sys::EXT_hand_tracking_data_source_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_HAND_TRACKING_DATA_SOURCE_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_plane_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreatePlaneDetectorEXT")?,
                    ),
                    destroy_plane_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyPlaneDetectorEXT")?,
                    ),
                    begin_plane_detection: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrBeginPlaneDetectionEXT")?,
                    ),
                    get_plane_detection_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetPlaneDetectionStateEXT")?,
                    ),
                    get_plane_detections: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetPlaneDetectionsEXT")?,
                    ),
                    get_plane_polygon_buffer: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetPlanePolygonBufferEXT")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    poll_future: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPollFutureEXT")?,
                    ),
                    cancel_future: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCancelFutureEXT")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct UserPresenceEXT {}
    impl UserPresenceEXT {
        pub const VERSION: u32 = sys::EXT_user_presence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_USER_PRESENCE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerInvertedAlphaEXT {}
    impl CompositionLayerInvertedAlphaEXT {
        pub const VERSION: u32 = sys::EXT_composition_layer_inverted_alpha_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_COMPOSITION_LAYER_INVERTED_ALPHA_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityEXT {
        pub enumerate_spatial_capabilities: pfn::EnumerateSpatialCapabilitiesEXT,
        pub enumerate_spatial_capability_component_types:
            pfn::EnumerateSpatialCapabilityComponentTypesEXT,
        pub enumerate_spatial_capability_features: pfn::EnumerateSpatialCapabilityFeaturesEXT,
        pub create_spatial_context_async: pfn::CreateSpatialContextAsyncEXT,
        pub create_spatial_context_complete: pfn::CreateSpatialContextCompleteEXT,
        pub destroy_spatial_context: pfn::DestroySpatialContextEXT,
        pub create_spatial_discovery_snapshot_async: pfn::CreateSpatialDiscoverySnapshotAsyncEXT,
        pub create_spatial_discovery_snapshot_complete:
            pfn::CreateSpatialDiscoverySnapshotCompleteEXT,
        pub query_spatial_component_data: pfn::QuerySpatialComponentDataEXT,
        pub destroy_spatial_snapshot: pfn::DestroySpatialSnapshotEXT,
        pub create_spatial_entity_from_id: pfn::CreateSpatialEntityFromIdEXT,
        pub destroy_spatial_entity: pfn::DestroySpatialEntityEXT,
        pub create_spatial_update_snapshot: pfn::CreateSpatialUpdateSnapshotEXT,
        pub get_spatial_buffer_string: pfn::GetSpatialBufferStringEXT,
        pub get_spatial_buffer_uint8: pfn::GetSpatialBufferUint8EXT,
        pub get_spatial_buffer_uint16: pfn::GetSpatialBufferUint16EXT,
        pub get_spatial_buffer_uint32: pfn::GetSpatialBufferUint32EXT,
        pub get_spatial_buffer_float: pfn::GetSpatialBufferFloatEXT,
        pub get_spatial_buffer_vector2f: pfn::GetSpatialBufferVector2fEXT,
        pub get_spatial_buffer_vector3f: pfn::GetSpatialBufferVector3fEXT,
    }
    impl SpatialEntityEXT {
        pub const VERSION: u32 = sys::EXT_spatial_entity_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_ENTITY_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_spatial_capabilities: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpatialCapabilitiesEXT",
                        )?,
                    ),
                    enumerate_spatial_capability_component_types: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpatialCapabilityComponentTypesEXT",
                        )?,
                    ),
                    enumerate_spatial_capability_features: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpatialCapabilityFeaturesEXT",
                        )?,
                    ),
                    create_spatial_context_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateSpatialContextAsyncEXT")?,
                    ),
                    create_spatial_context_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialContextCompleteEXT",
                        )?,
                    ),
                    destroy_spatial_context: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpatialContextEXT")?,
                    ),
                    create_spatial_discovery_snapshot_async: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialDiscoverySnapshotAsyncEXT",
                        )?,
                    ),
                    create_spatial_discovery_snapshot_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialDiscoverySnapshotCompleteEXT",
                        )?,
                    ),
                    query_spatial_component_data: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrQuerySpatialComponentDataEXT")?,
                    ),
                    destroy_spatial_snapshot: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpatialSnapshotEXT")?,
                    ),
                    create_spatial_entity_from_id: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateSpatialEntityFromIdEXT")?,
                    ),
                    destroy_spatial_entity: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpatialEntityEXT")?,
                    ),
                    create_spatial_update_snapshot: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialUpdateSnapshotEXT",
                        )?,
                    ),
                    get_spatial_buffer_string: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferStringEXT")?,
                    ),
                    get_spatial_buffer_uint8: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferUint8EXT")?,
                    ),
                    get_spatial_buffer_uint16: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferUint16EXT")?,
                    ),
                    get_spatial_buffer_uint32: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferUint32EXT")?,
                    ),
                    get_spatial_buffer_float: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferFloatEXT")?,
                    ),
                    get_spatial_buffer_vector2f: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferVector2fEXT")?,
                    ),
                    get_spatial_buffer_vector3f: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialBufferVector3fEXT")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialPlaneTrackingEXT {}
    impl SpatialPlaneTrackingEXT {
        pub const VERSION: u32 = sys::EXT_spatial_plane_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_PLANE_TRACKING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialMarkerTrackingEXT {}
    impl SpatialMarkerTrackingEXT {
        pub const VERSION: u32 = sys::EXT_spatial_marker_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_MARKER_TRACKING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorEXT {
        pub create_spatial_anchor: pfn::CreateSpatialAnchorEXT,
    }
    impl SpatialAnchorEXT {
        pub const VERSION: u32 = sys::EXT_spatial_anchor_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_ANCHOR_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorEXT")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialPersistenceEXT {
        pub enumerate_spatial_persistence_scopes: pfn::EnumerateSpatialPersistenceScopesEXT,
        pub create_spatial_persistence_context_async: pfn::CreateSpatialPersistenceContextAsyncEXT,
        pub create_spatial_persistence_context_complete:
            pfn::CreateSpatialPersistenceContextCompleteEXT,
        pub destroy_spatial_persistence_context: pfn::DestroySpatialPersistenceContextEXT,
    }
    impl SpatialPersistenceEXT {
        pub const VERSION: u32 = sys::EXT_spatial_persistence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_PERSISTENCE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_spatial_persistence_scopes: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpatialPersistenceScopesEXT",
                        )?,
                    ),
                    create_spatial_persistence_context_async: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialPersistenceContextAsyncEXT",
                        )?,
                    ),
                    create_spatial_persistence_context_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialPersistenceContextCompleteEXT",
                        )?,
                    ),
                    destroy_spatial_persistence_context: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroySpatialPersistenceContextEXT",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialPersistenceOperationsEXT {
        pub persist_spatial_entity_async: pfn::PersistSpatialEntityAsyncEXT,
        pub persist_spatial_entity_complete: pfn::PersistSpatialEntityCompleteEXT,
        pub unpersist_spatial_entity_async: pfn::UnpersistSpatialEntityAsyncEXT,
        pub unpersist_spatial_entity_complete: pfn::UnpersistSpatialEntityCompleteEXT,
    }
    impl SpatialPersistenceOperationsEXT {
        pub const VERSION: u32 = sys::EXT_spatial_persistence_operations_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_SPATIAL_PERSISTENCE_OPERATIONS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    persist_spatial_entity_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrPersistSpatialEntityAsyncEXT")?,
                    ),
                    persist_spatial_entity_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPersistSpatialEntityCompleteEXT",
                        )?,
                    ),
                    unpersist_spatial_entity_async: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrUnpersistSpatialEntityAsyncEXT",
                        )?,
                    ),
                    unpersist_spatial_entity_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrUnpersistSpatialEntityCompleteEXT",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct LoaderInitPropertiesEXT {}
    impl LoaderInitPropertiesEXT {
        pub const VERSION: u32 = sys::EXT_loader_init_properties_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXT_LOADER_INIT_PROPERTIES_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerImageLayoutFB {}
    impl CompositionLayerImageLayoutFB {
        pub const VERSION: u32 = sys::FB_composition_layer_image_layout_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_IMAGE_LAYOUT_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerAlphaBlendFB {}
    impl CompositionLayerAlphaBlendFB {
        pub const VERSION: u32 = sys::FB_composition_layer_alpha_blend_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_ALPHA_BLEND_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct AndroidSurfaceSwapchainCreateFB {}
    #[cfg(target_os = "android")]
    impl AndroidSurfaceSwapchainCreateFB {
        pub const VERSION: u32 = sys::FB_android_surface_swapchain_create_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_ANDROID_SURFACE_SWAPCHAIN_CREATE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateFB {
        pub update_swapchain: pfn::UpdateSwapchainFB,
        pub get_swapchain_state: pfn::GetSwapchainStateFB,
    }
    impl SwapchainUpdateStateFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    update_swapchain: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrUpdateSwapchainFB")?,
                    ),
                    get_swapchain_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSwapchainStateFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerSecureContentFB {}
    impl CompositionLayerSecureContentFB {
        pub const VERSION: u32 = sys::FB_composition_layer_secure_content_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_SECURE_CONTENT_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateBodyTrackerFB")?,
                    ),
                    destroy_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyBodyTrackerFB")?,
                    ),
                    locate_body_joints: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateBodyJointsFB")?,
                    ),
                    get_body_skeleton: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetBodySkeletonFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_display_refresh_rates: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateDisplayRefreshRatesFB",
                        )?,
                    ),
                    get_display_refresh_rate: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetDisplayRefreshRateFB")?,
                    ),
                    request_display_refresh_rate: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestDisplayRefreshRateFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_color_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateColorSpacesFB")?,
                    ),
                    set_color_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetColorSpaceFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingMeshFB {
        pub get_hand_mesh: pfn::GetHandMeshFB,
    }
    impl HandTrackingMeshFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_MESH_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_hand_mesh: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetHandMeshFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingAimFB {}
    impl HandTrackingAimFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_aim_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_AIM_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingCapsulesFB {}
    impl HandTrackingCapsulesFB {
        pub const VERSION: u32 = sys::FB_hand_tracking_capsules_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAND_TRACKING_CAPSULES_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorFB")?,
                    ),
                    get_space_uuid: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceUuidFB")?,
                    ),
                    enumerate_space_supported_components: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateSpaceSupportedComponentsFB",
                        )?,
                    ),
                    set_space_component_status: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetSpaceComponentStatusFB")?,
                    ),
                    get_space_component_status: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceComponentStatusFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_foveation_profile: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFoveationProfileFB")?,
                    ),
                    destroy_foveation_profile: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFoveationProfileFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationConfigurationFB {}
    impl FoveationConfigurationFB {
        pub const VERSION: u32 = sys::FB_foveation_configuration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FOVEATION_CONFIGURATION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct KeyboardTrackingFB {
        pub query_system_tracked_keyboard: pfn::QuerySystemTrackedKeyboardFB,
        pub create_keyboard_space: pfn::CreateKeyboardSpaceFB,
    }
    impl KeyboardTrackingFB {
        pub const VERSION: u32 = sys::FB_keyboard_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_KEYBOARD_TRACKING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    query_system_tracked_keyboard: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrQuerySystemTrackedKeyboardFB")?,
                    ),
                    create_keyboard_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateKeyboardSpaceFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_triangle_mesh: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateTriangleMeshFB")?,
                    ),
                    destroy_triangle_mesh: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyTriangleMeshFB")?,
                    ),
                    triangle_mesh_get_vertex_buffer: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrTriangleMeshGetVertexBufferFB")?,
                    ),
                    triangle_mesh_get_index_buffer: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrTriangleMeshGetIndexBufferFB")?,
                    ),
                    triangle_mesh_begin_update: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrTriangleMeshBeginUpdateFB")?,
                    ),
                    triangle_mesh_end_update: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrTriangleMeshEndUpdateFB")?,
                    ),
                    triangle_mesh_begin_vertex_buffer_update: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrTriangleMeshBeginVertexBufferUpdateFB",
                        )?,
                    ),
                    triangle_mesh_end_vertex_buffer_update: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrTriangleMeshEndVertexBufferUpdateFB",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_passthrough: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreatePassthroughFB")?,
                    ),
                    destroy_passthrough: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyPassthroughFB")?,
                    ),
                    passthrough_start: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPassthroughStartFB")?,
                    ),
                    passthrough_pause: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPassthroughPauseFB")?,
                    ),
                    create_passthrough_layer: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreatePassthroughLayerFB")?,
                    ),
                    destroy_passthrough_layer: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyPassthroughLayerFB")?,
                    ),
                    passthrough_layer_pause: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPassthroughLayerPauseFB")?,
                    ),
                    passthrough_layer_resume: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPassthroughLayerResumeFB")?,
                    ),
                    passthrough_layer_set_style: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPassthroughLayerSetStyleFB")?,
                    ),
                    create_geometry_instance: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateGeometryInstanceFB")?,
                    ),
                    destroy_geometry_instance: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyGeometryInstanceFB")?,
                    ),
                    geometry_instance_set_transform: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGeometryInstanceSetTransformFB",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_render_model_paths: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnumerateRenderModelPathsFB")?,
                    ),
                    get_render_model_properties: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetRenderModelPropertiesFB")?,
                    ),
                    load_render_model: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLoadRenderModelFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    query_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrQuerySpacesFB")?,
                    ),
                    retrieve_space_query_results: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRetrieveSpaceQueryResultsFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    save_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSaveSpaceFB")?,
                    ),
                    erase_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEraseSpaceFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationVulkanFB {}
    impl FoveationVulkanFB {
        pub const VERSION: u32 = sys::FB_foveation_vulkan_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_FOVEATION_VULKAN_EXTENSION_NAME;
    }
    #[cfg(target_os = "android")]
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateAndroidSurfaceFB {}
    #[cfg(target_os = "android")]
    impl SwapchainUpdateStateAndroidSurfaceFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_android_surface_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::FB_SWAPCHAIN_UPDATE_STATE_ANDROID_SURFACE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateOpenglEsFB {}
    impl SwapchainUpdateStateOpenglEsFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_opengl_es_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_OPENGL_ES_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUpdateStateVulkanFB {}
    impl SwapchainUpdateStateVulkanFB {
        pub const VERSION: u32 = sys::FB_swapchain_update_state_vulkan_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SWAPCHAIN_UPDATE_STATE_VULKAN_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerProFB {}
    impl TouchControllerProFB {
        pub const VERSION: u32 = sys::FB_touch_controller_pro_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_TOUCH_CONTROLLER_PRO_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntitySharingFB {
        pub share_spaces: pfn::ShareSpacesFB,
    }
    impl SpatialEntitySharingFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_sharing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_SHARING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    share_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrShareSpacesFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpaceWarpFB {}
    impl SpaceWarpFB {
        pub const VERSION: u32 = sys::FB_space_warp_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPACE_WARP_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HapticAmplitudeEnvelopeFB {}
    impl HapticAmplitudeEnvelopeFB {
        pub const VERSION: u32 = sys::FB_haptic_amplitude_envelope_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAPTIC_AMPLITUDE_ENVELOPE_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_space_bounding_box2_d: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceBoundingBox2DFB")?,
                    ),
                    get_space_bounding_box3_d: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceBoundingBox3DFB")?,
                    ),
                    get_space_semantic_labels: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceSemanticLabelsFB")?,
                    ),
                    get_space_boundary2_d: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceBoundary2DFB")?,
                    ),
                    get_space_room_layout: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceRoomLayoutFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SceneCaptureFB {
        pub request_scene_capture: pfn::RequestSceneCaptureFB,
    }
    impl SceneCaptureFB {
        pub const VERSION: u32 = sys::FB_scene_capture_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SCENE_CAPTURE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    request_scene_capture: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestSceneCaptureFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityContainerFB {
        pub get_space_container: pfn::GetSpaceContainerFB,
    }
    impl SpatialEntityContainerFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_container_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_CONTAINER_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_space_container: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceContainerFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFaceTrackerFB")?,
                    ),
                    destroy_face_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFaceTrackerFB")?,
                    ),
                    get_face_expression_weights: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFaceExpressionWeightsFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_eye_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateEyeTrackerFB")?,
                    ),
                    destroy_eye_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyEyeTrackerFB")?,
                    ),
                    get_eye_gazes: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetEyeGazesFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    passthrough_layer_set_keyboard_hands_intensity: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPassthroughLayerSetKeyboardHandsIntensityFB",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerSettingsFB {}
    impl CompositionLayerSettingsFB {
        pub const VERSION: u32 = sys::FB_composition_layer_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_SETTINGS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerProximityFB {}
    impl TouchControllerProximityFB {
        pub const VERSION: u32 = sys::FB_touch_controller_proximity_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_TOUCH_CONTROLLER_PROXIMITY_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HapticPcmFB {
        pub get_device_sample_rate: pfn::GetDeviceSampleRateFB,
    }
    impl HapticPcmFB {
        pub const VERSION: u32 = sys::FB_haptic_pcm_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_HAPTIC_PCM_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_device_sample_rate: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetDeviceSampleRateFB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthTestFB {}
    impl CompositionLayerDepthTestFB {
        pub const VERSION: u32 = sys::FB_composition_layer_depth_test_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityStorageBatchFB {
        pub save_space_list: pfn::SaveSpaceListFB,
    }
    impl SpatialEntityStorageBatchFB {
        pub const VERSION: u32 = sys::FB_spatial_entity_storage_batch_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::FB_SPATIAL_ENTITY_STORAGE_BATCH_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    save_space_list: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSaveSpaceListFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_space_user: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpaceUserFB")?,
                    ),
                    get_space_user_id: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceUserIdFB")?,
                    ),
                    destroy_space_user: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpaceUserFB")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_face_tracker2: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFaceTracker2FB")?,
                    ),
                    destroy_face_tracker2: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFaceTracker2FB")?,
                    ),
                    get_face_expression_weights2: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFaceExpressionWeights2FB")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveCosmosControllerInteractionHTC {}
    impl ViveCosmosControllerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_cosmos_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_facial_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateFacialTrackerHTC")?,
                    ),
                    destroy_facial_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyFacialTrackerHTC")?,
                    ),
                    get_facial_expressions: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetFacialExpressionsHTC")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveFocus3ControllerInteractionHTC {}
    impl ViveFocus3ControllerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_focus3_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_FOCUS3_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HandInteractionHTC {}
    impl HandInteractionHTC {
        pub const VERSION: u32 = sys::HTC_hand_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_HAND_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ViveWristTrackerInteractionHTC {}
    impl ViveWristTrackerInteractionHTC {
        pub const VERSION: u32 = sys::HTC_vive_wrist_tracker_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_VIVE_WRIST_TRACKER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughHTC {
        pub create_passthrough: pfn::CreatePassthroughHTC,
        pub destroy_passthrough: pfn::DestroyPassthroughHTC,
    }
    impl PassthroughHTC {
        pub const VERSION: u32 = sys::HTC_passthrough_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_PASSTHROUGH_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_passthrough: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreatePassthroughHTC")?,
                    ),
                    destroy_passthrough: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyPassthroughHTC")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct FoveationHTC {
        pub apply_foveation: pfn::ApplyFoveationHTC,
    }
    impl FoveationHTC {
        pub const VERSION: u32 = sys::HTC_foveation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_FOVEATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    apply_foveation: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrApplyFoveationHTC")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorHTC")?,
                    ),
                    get_spatial_anchor_name: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialAnchorNameHTC")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct BodyTrackingHTC {
        pub create_body_tracker: pfn::CreateBodyTrackerHTC,
        pub destroy_body_tracker: pfn::DestroyBodyTrackerHTC,
        pub locate_body_joints: pfn::LocateBodyJointsHTC,
        pub get_body_skeleton: pfn::GetBodySkeletonHTC,
    }
    impl BodyTrackingHTC {
        pub const VERSION: u32 = sys::HTC_body_tracking_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTC_BODY_TRACKING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateBodyTrackerHTC")?,
                    ),
                    destroy_body_tracker: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyBodyTrackerHTC")?,
                    ),
                    locate_body_joints: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateBodyJointsHTC")?,
                    ),
                    get_body_skeleton: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetBodySkeletonHTC")?,
                    ),
                })
            }
        }
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
            unsafe {
                Ok(Self {
                    set_android_application_thread: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetAndroidApplicationThreadKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    create_swapchain_android_surface: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSwapchainAndroidSurfaceKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_open_gl_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetOpenGLGraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_open_gles_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetOpenGLESGraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_vulkan_instance_extensions: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVulkanInstanceExtensionsKHR",
                        )?,
                    ),
                    get_vulkan_device_extensions: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetVulkanDeviceExtensionsKHR")?,
                    ),
                    get_vulkan_graphics_device: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetVulkanGraphicsDeviceKHR")?,
                    ),
                    get_vulkan_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVulkanGraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_d3d11_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetD3D11GraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_d3d12_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetD3D12GraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_metal_graphics_requirements: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetMetalGraphicsRequirementsKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_visibility_mask: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetVisibilityMaskKHR")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    convert_win32_performance_counter_to_time: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrConvertWin32PerformanceCounterToTimeKHR",
                        )?,
                    ),
                    convert_time_to_win32_performance_counter: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrConvertTimeToWin32PerformanceCounterKHR",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    convert_timespec_time_to_time: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrConvertTimespecTimeToTimeKHR")?,
                    ),
                    convert_time_to_timespec_time: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrConvertTimeToTimespecTimeKHR")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    initialize_loader: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrInitializeLoaderKHR")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    create_vulkan_instance: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateVulkanInstanceKHR")?,
                    ),
                    create_vulkan_device: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateVulkanDeviceKHR")?,
                    ),
                    get_vulkan_graphics_device2: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetVulkanGraphicsDevice2KHR")?,
                    ),
                    get_vulkan_graphics_requirements2: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVulkanGraphicsRequirements2KHR",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerEquirect2KHR {}
    impl CompositionLayerEquirect2KHR {
        pub const VERSION: u32 = sys::KHR_composition_layer_equirect2_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_COMPOSITION_LAYER_EQUIRECT2_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct BindingModificationKHR {}
    impl BindingModificationKHR {
        pub const VERSION: u32 = sys::KHR_binding_modification_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_BINDING_MODIFICATION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ExtendedStructNameLengthsKHR {
        pub structure_type_to_string2: pfn::StructureTypeToString2KHR,
    }
    impl ExtendedStructNameLengthsKHR {
        pub const VERSION: u32 = sys::KHR_extended_struct_name_lengths_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_EXTENDED_STRUCT_NAME_LENGTHS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    structure_type_to_string2: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStructureTypeToString2KHR")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SwapchainUsageInputAttachmentBitKHR {}
    impl SwapchainUsageInputAttachmentBitKHR {
        pub const VERSION: u32 = sys::KHR_swapchain_usage_input_attachment_bit_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::KHR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct LocateSpacesKHR {
        pub locate_spaces: pfn::LocateSpacesKHR,
    }
    impl LocateSpacesKHR {
        pub const VERSION: u32 = sys::KHR_locate_spaces_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_LOCATE_SPACES_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    locate_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLocateSpacesKHR")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Maintenance1KHR {}
    impl Maintenance1KHR {
        pub const VERSION: u32 = sys::KHR_maintenance1_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_MAINTENANCE1_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct GenericControllerKHR {}
    impl GenericControllerKHR {
        pub const VERSION: u32 = sys::KHR_generic_controller_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::KHR_GENERIC_CONTROLLER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct MxInkStylusInteractionLOGITECH {}
    impl MxInkStylusInteractionLOGITECH {
        pub const VERSION: u32 = sys::LOGITECH_mx_ink_stylus_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::LOGITECH_MX_INK_STYLUS_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct FoveationEyeTrackedMETA {
        pub get_foveation_eye_tracked_state: pfn::GetFoveationEyeTrackedStateMETA,
    }
    impl FoveationEyeTrackedMETA {
        pub const VERSION: u32 = sys::META_foveation_eye_tracked_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_FOVEATION_EYE_TRACKED_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_foveation_eye_tracked_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetFoveationEyeTrackedStateMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct LocalDimmingMETA {}
    impl LocalDimmingMETA {
        pub const VERSION: u32 = sys::META_local_dimming_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_LOCAL_DIMMING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughPreferencesMETA {
        pub get_passthrough_preferences: pfn::GetPassthroughPreferencesMETA,
    }
    impl PassthroughPreferencesMETA {
        pub const VERSION: u32 = sys::META_passthrough_preferences_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_PASSTHROUGH_PREFERENCES_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_passthrough_preferences: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetPassthroughPreferencesMETA")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_virtual_keyboard: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateVirtualKeyboardMETA")?,
                    ),
                    destroy_virtual_keyboard: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyVirtualKeyboardMETA")?,
                    ),
                    create_virtual_keyboard_space: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateVirtualKeyboardSpaceMETA",
                        )?,
                    ),
                    suggest_virtual_keyboard_location: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSuggestVirtualKeyboardLocationMETA",
                        )?,
                    ),
                    get_virtual_keyboard_scale: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetVirtualKeyboardScaleMETA")?,
                    ),
                    set_virtual_keyboard_model_visibility: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetVirtualKeyboardModelVisibilityMETA",
                        )?,
                    ),
                    get_virtual_keyboard_model_animation_states: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVirtualKeyboardModelAnimationStatesMETA",
                        )?,
                    ),
                    get_virtual_keyboard_dirty_textures: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVirtualKeyboardDirtyTexturesMETA",
                        )?,
                    ),
                    get_virtual_keyboard_texture_data: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetVirtualKeyboardTextureDataMETA",
                        )?,
                    ),
                    send_virtual_keyboard_input: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrSendVirtualKeyboardInputMETA")?,
                    ),
                    change_virtual_keyboard_text_context: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrChangeVirtualKeyboardTextContextMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct VulkanSwapchainCreateInfoMETA {}
    impl VulkanSwapchainCreateInfoMETA {
        pub const VERSION: u32 = sys::META_vulkan_swapchain_create_info_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_VULKAN_SWAPCHAIN_CREATE_INFO_EXTENSION_NAME;
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_performance_metrics_counter_paths: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumeratePerformanceMetricsCounterPathsMETA",
                        )?,
                    ),
                    set_performance_metrics_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetPerformanceMetricsStateMETA",
                        )?,
                    ),
                    get_performance_metrics_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetPerformanceMetricsStateMETA",
                        )?,
                    ),
                    query_performance_metrics_counter: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrQueryPerformanceMetricsCounterMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct DetachedControllersMETA {}
    impl DetachedControllersMETA {
        pub const VERSION: u32 = sys::META_detached_controllers_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_DETACHED_CONTROLLERS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct HeadsetIdMETA {}
    impl HeadsetIdMETA {
        pub const VERSION: u32 = sys::META_headset_id_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_HEADSET_ID_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityDiscoveryMETA {
        pub discover_spaces: pfn::DiscoverSpacesMETA,
        pub retrieve_space_discovery_results: pfn::RetrieveSpaceDiscoveryResultsMETA,
    }
    impl SpatialEntityDiscoveryMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_discovery_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_DISCOVERY_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    discover_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDiscoverSpacesMETA")?,
                    ),
                    retrieve_space_discovery_results: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrRetrieveSpaceDiscoveryResultsMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingMicrogesturesMETA {}
    impl HandTrackingMicrogesturesMETA {
        pub const VERSION: u32 = sys::META_hand_tracking_microgestures_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_HAND_TRACKING_MICROGESTURES_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct RecommendedLayerResolutionMETA {
        pub get_recommended_layer_resolution: pfn::GetRecommendedLayerResolutionMETA,
    }
    impl RecommendedLayerResolutionMETA {
        pub const VERSION: u32 = sys::META_recommended_layer_resolution_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_RECOMMENDED_LAYER_RESOLUTION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_recommended_layer_resolution: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetRecommendedLayerResolutionMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityPersistenceMETA {
        pub save_spaces: pfn::SaveSpacesMETA,
        pub erase_spaces: pfn::EraseSpacesMETA,
    }
    impl SpatialEntityPersistenceMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_persistence_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_PERSISTENCE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    save_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSaveSpacesMETA")?,
                    ),
                    erase_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEraseSpacesMETA")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_passthrough_color_lut: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreatePassthroughColorLutMETA")?,
                    ),
                    destroy_passthrough_color_lut: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyPassthroughColorLutMETA",
                        )?,
                    ),
                    update_passthrough_color_lut: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrUpdatePassthroughColorLutMETA")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityMeshMETA {
        pub get_space_triangle_mesh: pfn::GetSpaceTriangleMeshMETA,
    }
    impl SpatialEntityMeshMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_mesh_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_MESH_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_space_triangle_mesh: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpaceTriangleMeshMETA")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct AutomaticLayerFilterMETA {}
    impl AutomaticLayerFilterMETA {
        pub const VERSION: u32 = sys::META_automatic_layer_filter_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_AUTOMATIC_LAYER_FILTER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct BodyTrackingFullBodyMETA {}
    impl BodyTrackingFullBodyMETA {
        pub const VERSION: u32 = sys::META_body_tracking_full_body_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_BODY_TRACKING_FULL_BODY_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct TouchControllerPlusMETA {}
    impl TouchControllerPlusMETA {
        pub const VERSION: u32 = sys::META_touch_controller_plus_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_TOUCH_CONTROLLER_PLUS_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct PassthroughLayerResumedEventMETA {}
    impl PassthroughLayerResumedEventMETA {
        pub const VERSION: u32 = sys::META_passthrough_layer_resumed_event_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_PASSTHROUGH_LAYER_RESUMED_EVENT_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct BodyTrackingCalibrationMETA {
        pub suggest_body_tracking_calibration_override:
            pfn::SuggestBodyTrackingCalibrationOverrideMETA,
        pub reset_body_tracking_calibration: pfn::ResetBodyTrackingCalibrationMETA,
    }
    impl BodyTrackingCalibrationMETA {
        pub const VERSION: u32 = sys::META_body_tracking_calibration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_BODY_TRACKING_CALIBRATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    suggest_body_tracking_calibration_override: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSuggestBodyTrackingCalibrationOverrideMETA",
                        )?,
                    ),
                    reset_body_tracking_calibration: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrResetBodyTrackingCalibrationMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntitySharingMETA {
        pub share_spaces: pfn::ShareSpacesMETA,
    }
    impl SpatialEntitySharingMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_sharing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_SHARING_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    share_spaces: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrShareSpacesMETA")?,
                    ),
                })
            }
        }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_environment_depth_provider: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateEnvironmentDepthProviderMETA",
                        )?,
                    ),
                    destroy_environment_depth_provider: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyEnvironmentDepthProviderMETA",
                        )?,
                    ),
                    start_environment_depth_provider: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrStartEnvironmentDepthProviderMETA",
                        )?,
                    ),
                    stop_environment_depth_provider: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrStopEnvironmentDepthProviderMETA",
                        )?,
                    ),
                    create_environment_depth_swapchain: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateEnvironmentDepthSwapchainMETA",
                        )?,
                    ),
                    destroy_environment_depth_swapchain: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyEnvironmentDepthSwapchainMETA",
                        )?,
                    ),
                    enumerate_environment_depth_swapchain_images: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateEnvironmentDepthSwapchainImagesMETA",
                        )?,
                    ),
                    get_environment_depth_swapchain_state: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetEnvironmentDepthSwapchainStateMETA",
                        )?,
                    ),
                    acquire_environment_depth_image: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrAcquireEnvironmentDepthImageMETA",
                        )?,
                    ),
                    set_environment_depth_hand_removal: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetEnvironmentDepthHandRemovalMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SimultaneousHandsAndControllersMETA {
        pub resume_simultaneous_hands_and_controllers_tracking:
            pfn::ResumeSimultaneousHandsAndControllersTrackingMETA,
        pub pause_simultaneous_hands_and_controllers_tracking:
            pfn::PauseSimultaneousHandsAndControllersTrackingMETA,
    }
    impl SimultaneousHandsAndControllersMETA {
        pub const VERSION: u32 = sys::META_simultaneous_hands_and_controllers_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SIMULTANEOUS_HANDS_AND_CONTROLLERS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    resume_simultaneous_hands_and_controllers_tracking: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrResumeSimultaneousHandsAndControllersTrackingMETA",
                        )?,
                    ),
                    pause_simultaneous_hands_and_controllers_tracking: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPauseSimultaneousHandsAndControllersTrackingMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ColocationDiscoveryMETA {
        pub start_colocation_discovery: pfn::StartColocationDiscoveryMETA,
        pub stop_colocation_discovery: pfn::StopColocationDiscoveryMETA,
        pub start_colocation_advertisement: pfn::StartColocationAdvertisementMETA,
        pub stop_colocation_advertisement: pfn::StopColocationAdvertisementMETA,
    }
    impl ColocationDiscoveryMETA {
        pub const VERSION: u32 = sys::META_colocation_discovery_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_COLOCATION_DISCOVERY_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    start_colocation_discovery: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrStartColocationDiscoveryMETA")?,
                    ),
                    stop_colocation_discovery: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrStopColocationDiscoveryMETA")?,
                    ),
                    start_colocation_advertisement: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrStartColocationAdvertisementMETA",
                        )?,
                    ),
                    stop_colocation_advertisement: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrStopColocationAdvertisementMETA",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialEntityGroupSharingMETA {}
    impl SpatialEntityGroupSharingMETA {
        pub const VERSION: u32 = sys::META_spatial_entity_group_sharing_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::META_SPATIAL_ENTITY_GROUP_SHARING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct Ml2ControllerInteractionML {}
    impl Ml2ControllerInteractionML {
        pub const VERSION: u32 = sys::ML_ml2_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_ML2_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct FrameEndInfoML {}
    impl FrameEndInfoML {
        pub const VERSION: u32 = sys::ML_frame_end_info_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_FRAME_END_INFO_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct GlobalDimmerML {}
    impl GlobalDimmerML {
        pub const VERSION: u32 = sys::ML_global_dimmer_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_GLOBAL_DIMMER_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompatML {
        pub create_space_from_coordinate_frame_uid: pfn::CreateSpaceFromCoordinateFrameUIDML,
    }
    impl CompatML {
        pub const VERSION: u32 = sys::ML_compat_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_COMPAT_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_space_from_coordinate_frame_uid: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpaceFromCoordinateFrameUIDML",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_marker_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateMarkerDetectorML")?,
                    ),
                    destroy_marker_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyMarkerDetectorML")?,
                    ),
                    snapshot_marker_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSnapshotMarkerDetectorML")?,
                    ),
                    get_marker_detector_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkerDetectorStateML")?,
                    ),
                    get_markers: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkersML")?,
                    ),
                    get_marker_reprojection_error: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetMarkerReprojectionErrorML")?,
                    ),
                    get_marker_length: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkerLengthML")?,
                    ),
                    get_marker_number: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkerNumberML")?,
                    ),
                    get_marker_string: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkerStringML")?,
                    ),
                    create_marker_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateMarkerSpaceML")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enable_localization_events: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrEnableLocalizationEventsML")?,
                    ),
                    query_localization_maps: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrQueryLocalizationMapsML")?,
                    ),
                    request_map_localization: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestMapLocalizationML")?,
                    ),
                    import_localization_map: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrImportLocalizationMapML")?,
                    ),
                    create_exported_localization_map: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateExportedLocalizationMapML",
                        )?,
                    ),
                    destroy_exported_localization_map: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyExportedLocalizationMapML",
                        )?,
                    ),
                    get_exported_localization_map_data: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetExportedLocalizationMapDataML",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorsML {
        pub create_spatial_anchors_async: pfn::CreateSpatialAnchorsAsyncML,
        pub create_spatial_anchors_complete: pfn::CreateSpatialAnchorsCompleteML,
        pub get_spatial_anchor_state: pfn::GetSpatialAnchorStateML,
    }
    impl SpatialAnchorsML {
        pub const VERSION: u32 = sys::ML_spatial_anchors_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_SPATIAL_ANCHORS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchors_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorsAsyncML")?,
                    ),
                    create_spatial_anchors_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialAnchorsCompleteML",
                        )?,
                    ),
                    get_spatial_anchor_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetSpatialAnchorStateML")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SpatialAnchorsStorageML {
        pub create_spatial_anchors_storage: pfn::CreateSpatialAnchorsStorageML,
        pub destroy_spatial_anchors_storage: pfn::DestroySpatialAnchorsStorageML,
        pub query_spatial_anchors_async: pfn::QuerySpatialAnchorsAsyncML,
        pub query_spatial_anchors_complete: pfn::QuerySpatialAnchorsCompleteML,
        pub publish_spatial_anchors_async: pfn::PublishSpatialAnchorsAsyncML,
        pub publish_spatial_anchors_complete: pfn::PublishSpatialAnchorsCompleteML,
        pub delete_spatial_anchors_async: pfn::DeleteSpatialAnchorsAsyncML,
        pub delete_spatial_anchors_complete: pfn::DeleteSpatialAnchorsCompleteML,
        pub update_spatial_anchors_expiration_async: pfn::UpdateSpatialAnchorsExpirationAsyncML,
        pub update_spatial_anchors_expiration_complete:
            pfn::UpdateSpatialAnchorsExpirationCompleteML,
    }
    impl SpatialAnchorsStorageML {
        pub const VERSION: u32 = sys::ML_spatial_anchors_storage_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_SPATIAL_ANCHORS_STORAGE_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchors_storage: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateSpatialAnchorsStorageML")?,
                    ),
                    destroy_spatial_anchors_storage: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroySpatialAnchorsStorageML",
                        )?,
                    ),
                    query_spatial_anchors_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrQuerySpatialAnchorsAsyncML")?,
                    ),
                    query_spatial_anchors_complete: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrQuerySpatialAnchorsCompleteML")?,
                    ),
                    publish_spatial_anchors_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrPublishSpatialAnchorsAsyncML")?,
                    ),
                    publish_spatial_anchors_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrPublishSpatialAnchorsCompleteML",
                        )?,
                    ),
                    delete_spatial_anchors_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDeleteSpatialAnchorsAsyncML")?,
                    ),
                    delete_spatial_anchors_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDeleteSpatialAnchorsCompleteML",
                        )?,
                    ),
                    update_spatial_anchors_expiration_async: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrUpdateSpatialAnchorsExpirationAsyncML",
                        )?,
                    ),
                    update_spatial_anchors_expiration_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrUpdateSpatialAnchorsExpirationCompleteML",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct UserCalibrationML {
        pub enable_user_calibration_events: pfn::EnableUserCalibrationEventsML,
    }
    impl UserCalibrationML {
        pub const VERSION: u32 = sys::ML_user_calibration_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_USER_CALIBRATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enable_user_calibration_events: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrEnableUserCalibrationEventsML")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct SystemNotificationsML {
        pub set_system_notifications: pfn::SetSystemNotificationsML,
    }
    impl SystemNotificationsML {
        pub const VERSION: u32 = sys::ML_system_notifications_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_SYSTEM_NOTIFICATIONS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_system_notifications: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetSystemNotificationsML")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct WorldMeshDetectionML {
        pub create_world_mesh_detector: pfn::CreateWorldMeshDetectorML,
        pub destroy_world_mesh_detector: pfn::DestroyWorldMeshDetectorML,
        pub request_world_mesh_state_async: pfn::RequestWorldMeshStateAsyncML,
        pub request_world_mesh_state_complete: pfn::RequestWorldMeshStateCompleteML,
        pub get_world_mesh_buffer_recommend_size: pfn::GetWorldMeshBufferRecommendSizeML,
        pub allocate_world_mesh_buffer: pfn::AllocateWorldMeshBufferML,
        pub free_world_mesh_buffer: pfn::FreeWorldMeshBufferML,
        pub request_world_mesh_async: pfn::RequestWorldMeshAsyncML,
        pub request_world_mesh_complete: pfn::RequestWorldMeshCompleteML,
    }
    impl WorldMeshDetectionML {
        pub const VERSION: u32 = sys::ML_world_mesh_detection_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_WORLD_MESH_DETECTION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_world_mesh_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateWorldMeshDetectorML")?,
                    ),
                    destroy_world_mesh_detector: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroyWorldMeshDetectorML")?,
                    ),
                    request_world_mesh_state_async: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrRequestWorldMeshStateAsyncML")?,
                    ),
                    request_world_mesh_state_complete: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrRequestWorldMeshStateCompleteML",
                        )?,
                    ),
                    get_world_mesh_buffer_recommend_size: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetWorldMeshBufferRecommendSizeML",
                        )?,
                    ),
                    allocate_world_mesh_buffer: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrAllocateWorldMeshBufferML")?,
                    ),
                    free_world_mesh_buffer: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrFreeWorldMeshBufferML")?,
                    ),
                    request_world_mesh_async: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestWorldMeshAsyncML")?,
                    ),
                    request_world_mesh_complete: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrRequestWorldMeshCompleteML")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct FacialExpressionML {
        pub create_facial_expression_client: pfn::CreateFacialExpressionClientML,
        pub destroy_facial_expression_client: pfn::DestroyFacialExpressionClientML,
        pub get_facial_expression_blend_shape_properties:
            pfn::GetFacialExpressionBlendShapePropertiesML,
    }
    impl FacialExpressionML {
        pub const VERSION: u32 = sys::ML_facial_expression_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ML_FACIAL_EXPRESSION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_facial_expression_client: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateFacialExpressionClientML",
                        )?,
                    ),
                    destroy_facial_expression_client: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroyFacialExpressionClientML",
                        )?,
                    ),
                    get_facial_expression_blend_shape_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetFacialExpressionBlendShapePropertiesML",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViewConfigurationDepthRangeChangeML {}
    impl ViewConfigurationDepthRangeChangeML {
        pub const VERSION: u32 = sys::ML_view_configuration_depth_range_change_SPEC_VERSION;
        pub const NAME: &'static [u8] =
            sys::ML_VIEW_CONFIGURATION_DEPTH_RANGE_CHANGE_EXTENSION_NAME;
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
            unsafe {
                Ok(Self {
                    create_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateSpatialAnchorMSFT")?,
                    ),
                    create_spatial_anchor_space: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrCreateSpatialAnchorSpaceMSFT")?,
                    ),
                    destroy_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrDestroySpatialAnchorMSFT")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_graph_node_space: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialGraphNodeSpaceMSFT",
                        )?,
                    ),
                    try_create_spatial_graph_static_node_binding: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrTryCreateSpatialGraphStaticNodeBindingMSFT",
                        )?,
                    ),
                    destroy_spatial_graph_node_binding: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroySpatialGraphNodeBindingMSFT",
                        )?,
                    ),
                    get_spatial_graph_node_binding_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetSpatialGraphNodeBindingPropertiesMSFT",
                        )?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    create_hand_mesh_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateHandMeshSpaceMSFT")?,
                    ),
                    update_hand_mesh: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrUpdateHandMeshMSFT")?,
                    ),
                })
            }
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
            unsafe {
                Ok(Self {
                    get_controller_model_key: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetControllerModelKeyMSFT")?,
                    ),
                    load_controller_model: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrLoadControllerModelMSFT")?,
                    ),
                    get_controller_model_properties: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetControllerModelPropertiesMSFT",
                        )?,
                    ),
                    get_controller_model_state: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetControllerModelStateMSFT")?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor_from_perception_anchor: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialAnchorFromPerceptionAnchorMSFT",
                        )?,
                    ),
                    try_get_perception_anchor_from_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrTryGetPerceptionAnchorFromSpatialAnchorMSFT",
                        )?,
                    ),
                })
            }
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
    #[derive(Copy, Clone)]
    pub struct CompositionLayerReprojectionMSFT {
        pub enumerate_reprojection_modes: pfn::EnumerateReprojectionModesMSFT,
    }
    impl CompositionLayerReprojectionMSFT {
        pub const VERSION: u32 = sys::MSFT_composition_layer_reprojection_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MSFT_COMPOSITION_LAYER_REPROJECTION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_reprojection_modes: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateReprojectionModesMSFT",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    create_spatial_anchor_store_connection: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialAnchorStoreConnectionMSFT",
                        )?,
                    ),
                    destroy_spatial_anchor_store_connection: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrDestroySpatialAnchorStoreConnectionMSFT",
                        )?,
                    ),
                    persist_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrPersistSpatialAnchorMSFT")?,
                    ),
                    enumerate_persisted_spatial_anchor_names: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumeratePersistedSpatialAnchorNamesMSFT",
                        )?,
                    ),
                    create_spatial_anchor_from_persisted_name: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrCreateSpatialAnchorFromPersistedNameMSFT",
                        )?,
                    ),
                    unpersist_spatial_anchor: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrUnpersistSpatialAnchorMSFT")?,
                    ),
                    clear_spatial_anchor_store: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrClearSpatialAnchorStoreMSFT")?,
                    ),
                })
            }
        }
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
    pub struct AudioDeviceGuidOCULUS {
        pub get_audio_output_device_guid: pfn::GetAudioOutputDeviceGuidOculus,
        pub get_audio_input_device_guid: pfn::GetAudioInputDeviceGuidOculus,
    }
    impl AudioDeviceGuidOCULUS {
        pub const VERSION: u32 = sys::OCULUS_audio_device_guid_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_AUDIO_DEVICE_GUID_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    get_audio_output_device_guid: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrGetAudioOutputDeviceGuidOculus",
                        )?,
                    ),
                    get_audio_input_device_guid: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrGetAudioInputDeviceGuidOculus")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ExternalCameraOCULUS {
        pub enumerate_external_cameras: pfn::EnumerateExternalCamerasOCULUS,
    }
    impl ExternalCameraOCULUS {
        pub const VERSION: u32 = sys::OCULUS_external_camera_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OCULUS_EXTERNAL_CAMERA_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_external_cameras: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrEnumerateExternalCamerasOCULUS",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionOPPO {}
    impl ControllerInteractionOPPO {
        pub const VERSION: u32 = sys::OPPO_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::OPPO_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct TrackingOptimizationSettingsQCOM {
        pub set_tracking_optimization_settings_hint: pfn::SetTrackingOptimizationSettingsHintQCOM,
    }
    impl TrackingOptimizationSettingsQCOM {
        pub const VERSION: u32 = sys::QCOM_tracking_optimization_settings_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::QCOM_TRACKING_OPTIMIZATION_SETTINGS_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_tracking_optimization_settings_hint: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetTrackingOptimizationSettingsHintQCOM",
                        )?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct HandTrackingForearmULTRALEAP {}
    impl HandTrackingForearmULTRALEAP {
        pub const VERSION: u32 = sys::ULTRALEAP_hand_tracking_forearm_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::ULTRALEAP_HAND_TRACKING_FOREARM_EXTENSION_NAME;
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
    #[derive(Copy, Clone)]
    pub struct FoveatedRenderingVARJO {}
    impl FoveatedRenderingVARJO {
        pub const VERSION: u32 = sys::VARJO_foveated_rendering_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_FOVEATED_RENDERING_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct CompositionLayerDepthTestVARJO {}
    impl CompositionLayerDepthTestVARJO {
        pub const VERSION: u32 = sys::VARJO_composition_layer_depth_test_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct EnvironmentDepthEstimationVARJO {
        pub set_environment_depth_estimation: pfn::SetEnvironmentDepthEstimationVARJO,
    }
    impl EnvironmentDepthEstimationVARJO {
        pub const VERSION: u32 = sys::VARJO_environment_depth_estimation_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_ENVIRONMENT_DEPTH_ESTIMATION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_environment_depth_estimation: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetEnvironmentDepthEstimationVARJO",
                        )?,
                    ),
                })
            }
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
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_marker_tracking: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetMarkerTrackingVARJO")?,
                    ),
                    set_marker_tracking_timeout: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrSetMarkerTrackingTimeoutVARJO")?,
                    ),
                    set_marker_tracking_prediction: mem::transmute(
                        entry.get_instance_proc_addr(
                            instance,
                            c"xrSetMarkerTrackingPredictionVARJO",
                        )?,
                    ),
                    get_marker_size: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrGetMarkerSizeVARJO")?,
                    ),
                    create_marker_space: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrCreateMarkerSpaceVARJO")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViewOffsetVARJO {
        pub set_view_offset: pfn::SetViewOffsetVARJO,
    }
    impl ViewOffsetVARJO {
        pub const VERSION: u32 = sys::VARJO_view_offset_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_VIEW_OFFSET_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    set_view_offset: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrSetViewOffsetVARJO")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct Xr4ControllerInteractionVARJO {}
    impl Xr4ControllerInteractionVARJO {
        pub const VERSION: u32 = sys::VARJO_xr4_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::VARJO_XR4_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ControllerInteractionYVR {}
    impl ControllerInteractionYVR {
        pub const VERSION: u32 = sys::YVR_controller_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::YVR_CONTROLLER_INTERACTION_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct OverlayEXTX {}
    impl OverlayEXTX {
        pub const VERSION: u32 = sys::EXTX_overlay_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::EXTX_OVERLAY_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct EglEnableMNDX {}
    impl EglEnableMNDX {
        pub const VERSION: u32 = sys::MNDX_egl_enable_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MNDX_EGL_ENABLE_EXTENSION_NAME;
    }
    #[derive(Copy, Clone)]
    pub struct ForceFeedbackCurlMNDX {
        pub apply_force_feedback_curl: pfn::ApplyForceFeedbackCurlMNDX,
    }
    impl ForceFeedbackCurlMNDX {
        pub const VERSION: u32 = sys::MNDX_force_feedback_curl_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::MNDX_FORCE_FEEDBACK_CURL_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    apply_force_feedback_curl: mem::transmute(
                        entry.get_instance_proc_addr(instance, c"xrApplyForceFeedbackCurlMNDX")?,
                    ),
                })
            }
        }
    }
    #[derive(Copy, Clone)]
    pub struct ViveTrackerInteractionHTCX {
        pub enumerate_vive_tracker_paths: pfn::EnumerateViveTrackerPathsHTCX,
    }
    impl ViveTrackerInteractionHTCX {
        pub const VERSION: u32 = sys::HTCX_vive_tracker_interaction_SPEC_VERSION;
        pub const NAME: &'static [u8] = sys::HTCX_VIVE_TRACKER_INTERACTION_EXTENSION_NAME;
        #[doc = r" Load the extension's function pointer table"]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" `instance` must be a valid instance handle."]
        pub unsafe fn load(entry: &Entry, instance: sys::Instance) -> Result<Self> {
            unsafe {
                Ok(Self {
                    enumerate_vive_tracker_paths: mem::transmute(
                        entry
                            .get_instance_proc_addr(instance, c"xrEnumerateViveTrackerPathsHTCX")?,
                    ),
                })
            }
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerProjection<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerQuad<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerCylinderKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerCubeKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerEquirectKHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerEquirect2KHR<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a, G: Graphics> Deref for CompositionLayerPassthroughFB<'a, G> {
        type Target = CompositionLayerBase<'a, G>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for HapticVibration<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for HapticAmplitudeEnvelopeVibrationFB<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for HapticPcmVibrationFB<'a> {
        type Target = HapticBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for InteractionProfileDpadBindingEXT<'a> {
        type Target = BindingModificationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for InteractionProfileAnalogThresholdVALVE<'a> {
        type Target = BindingModificationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SwapchainStateAndroidSurfaceDimensionsFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[cfg(target_os = "android")]
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SwapchainStateSamplerOpenGLESFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SwapchainStateSamplerVulkanFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SwapchainStateFoveationFB<'a> {
        type Target = SwapchainStateBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpaceQueryInfoFB<'a> {
        type Target = SpaceQueryInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpaceUuidFilterInfoFB<'a> {
        type Target = SpaceFilterInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
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
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpaceComponentFilterInfoFB<'a> {
        type Target = SpaceFilterInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpaceComponentFilterInfoFB<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct ShareSpacesRecipientBase<'a> {
        _inner: sys::ShareSpacesRecipientBaseHeaderMETA,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct ShareSpacesRecipientGroupsMETA<'a> {
        inner: sys::ShareSpacesRecipientGroupsMETA,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> ShareSpacesRecipientGroupsMETA<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::ShareSpacesRecipientGroupsMETA {
                    ty: sys::StructureType::SHARE_SPACES_RECIPIENT_GROUPS_META,
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
        pub unsafe fn from_raw(inner: sys::ShareSpacesRecipientGroupsMETA) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::ShareSpacesRecipientGroupsMETA {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::ShareSpacesRecipientGroupsMETA {
            &self.inner
        }
        #[inline]
        pub fn groups(mut self, value: &'a [Uuid]) -> Self {
            self.inner.groups = value.as_ptr() as *const _ as _;
            self.inner.group_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for ShareSpacesRecipientGroupsMETA<'a> {
        type Target = ShareSpacesRecipientBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for ShareSpacesRecipientGroupsMETA<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SpaceFilterBase<'a> {
        _inner: sys::SpaceFilterBaseHeaderMETA,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceFilterUuidMETA<'a> {
        inner: sys::SpaceFilterUuidMETA,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpaceFilterUuidMETA<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceFilterUuidMETA {
                    ty: sys::StructureType::SPACE_FILTER_UUID_META,
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
        pub unsafe fn from_raw(inner: sys::SpaceFilterUuidMETA) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceFilterUuidMETA {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceFilterUuidMETA {
            &self.inner
        }
        #[inline]
        pub fn uuids(mut self, value: &'a [UuidEXT]) -> Self {
            self.inner.uuids = value.as_ptr() as *const _ as _;
            self.inner.uuid_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpaceFilterUuidMETA<'a> {
        type Target = SpaceFilterBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpaceFilterUuidMETA<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpaceFilterComponentMETA<'a> {
        inner: sys::SpaceFilterComponentMETA,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpaceFilterComponentMETA<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpaceFilterComponentMETA {
                    ty: sys::StructureType::SPACE_FILTER_COMPONENT_META,
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
        pub unsafe fn from_raw(inner: sys::SpaceFilterComponentMETA) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpaceFilterComponentMETA {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpaceFilterComponentMETA {
            &self.inner
        }
        #[inline]
        pub fn component_type(mut self, value: SpaceComponentTypeFB) -> Self {
            self.inner.component_type = value;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpaceFilterComponentMETA<'a> {
        type Target = SpaceFilterBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpaceFilterComponentMETA<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationBase<'a> {
        _inner: sys::SpatialCapabilityConfigurationBaseHeaderEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationAnchorEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationAnchorEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationAnchorEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationAnchorEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_ANCHOR_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationAnchorEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationAnchorEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationAnchorEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationAnchorEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationAnchorEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationPlaneTrackingEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationPlaneTrackingEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationPlaneTrackingEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationPlaneTrackingEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_PLANE_TRACKING_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationPlaneTrackingEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationPlaneTrackingEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationPlaneTrackingEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationPlaneTrackingEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationPlaneTrackingEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationArucoMarkerEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationArucoMarkerEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationArucoMarkerEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationArucoMarkerEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_ARUCO_MARKER_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationArucoMarkerEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationArucoMarkerEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationArucoMarkerEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
        #[inline]
        pub fn ar_uco_dict(mut self, value: SpatialMarkerArucoDictEXT) -> Self {
            self.inner.ar_uco_dict = value;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationArucoMarkerEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationArucoMarkerEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationAprilTagEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationAprilTagEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationAprilTagEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationAprilTagEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_APRIL_TAG_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationAprilTagEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationAprilTagEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationAprilTagEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
        #[inline]
        pub fn april_dict(mut self, value: SpatialMarkerAprilTagDictEXT) -> Self {
            self.inner.april_dict = value;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationAprilTagEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationAprilTagEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationQrCodeEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationQrCodeEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationQrCodeEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationQrCodeEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_QR_CODE_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationQrCodeEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationQrCodeEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationQrCodeEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationQrCodeEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationQrCodeEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialCapabilityConfigurationMicroQrCodeEXT<'a> {
        inner: sys::SpatialCapabilityConfigurationMicroQrCodeEXT,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialCapabilityConfigurationMicroQrCodeEXT<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialCapabilityConfigurationMicroQrCodeEXT {
                    ty: sys::StructureType::SPATIAL_CAPABILITY_CONFIGURATION_MICRO_QR_CODE_EXT,
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
        pub unsafe fn from_raw(inner: sys::SpatialCapabilityConfigurationMicroQrCodeEXT) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialCapabilityConfigurationMicroQrCodeEXT {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialCapabilityConfigurationMicroQrCodeEXT {
            &self.inner
        }
        #[inline]
        pub fn capability(mut self, value: SpatialCapabilityEXT) -> Self {
            self.inner.capability = value;
            self
        }
        #[inline]
        pub fn enabled_components(mut self, value: &'a [SpatialComponentTypeEXT]) -> Self {
            self.inner.enabled_components = value.as_ptr() as *const _ as _;
            self.inner.enabled_component_count = value.len() as u32;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialCapabilityConfigurationMicroQrCodeEXT<'a> {
        type Target = SpatialCapabilityConfigurationBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialCapabilityConfigurationMicroQrCodeEXT<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
    #[repr(transparent)]
    pub struct SpatialAnchorsQueryInfoBase<'a> {
        _inner: sys::SpatialAnchorsQueryInfoBaseHeaderML,
        _marker: PhantomData<&'a ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(transparent)]
    pub struct SpatialAnchorsQueryInfoRadiusML<'a> {
        inner: sys::SpatialAnchorsQueryInfoRadiusML,
        _marker: PhantomData<&'a ()>,
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> SpatialAnchorsQueryInfoRadiusML<'a> {
        #[inline]
        pub fn new() -> Self {
            Self {
                inner: sys::SpatialAnchorsQueryInfoRadiusML {
                    ty: sys::StructureType::SPATIAL_ANCHORS_QUERY_INFO_RADIUS_ML,
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
        pub unsafe fn from_raw(inner: sys::SpatialAnchorsQueryInfoRadiusML) -> Self {
            Self {
                inner,
                _marker: PhantomData,
            }
        }
        #[inline]
        pub fn into_raw(self) -> sys::SpatialAnchorsQueryInfoRadiusML {
            self.inner
        }
        #[inline]
        pub fn as_raw(&self) -> &sys::SpatialAnchorsQueryInfoRadiusML {
            &self.inner
        }
        #[inline]
        pub fn base_space(mut self, value: &'a Space) -> Self {
            self.inner.base_space = value.as_raw();
            self
        }
        #[inline]
        pub fn center(mut self, value: Vector3f) -> Self {
            self.inner.center = value;
            self
        }
        #[inline]
        pub fn time(mut self, value: Time) -> Self {
            self.inner.time = value;
            self
        }
        #[inline]
        pub fn radius(mut self, value: f32) -> Self {
            self.inner.radius = value;
            self
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Deref for SpatialAnchorsQueryInfoRadiusML<'a> {
        type Target = SpatialAnchorsQueryInfoBase<'a>;
        #[inline]
        fn deref(&self) -> &Self::Target {
            unsafe { mem::transmute(&self.inner) }
        }
    }
    #[allow(clippy::needless_lifetimes)]
    impl<'a> Default for SpatialAnchorsQueryInfoRadiusML<'a> {
        fn default() -> Self {
            Self::new()
        }
    }
}

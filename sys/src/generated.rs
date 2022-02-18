#![doc = r" Automatically generated code; do not edit!"]
#![allow(
    non_upper_case_globals,
    clippy::unreadable_literal,
    clippy::identity_op,
    unused
)]
use crate::platform::*;
use crate::support::*;
use crate::*;
use libc::{timespec, wchar_t};
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};
pub const CURRENT_API_VERSION: Version = Version::new(1u16, 0u16, 22u32);
pub const NULL_PATH: usize = 0usize;
pub const NULL_SYSTEM_ID: usize = 0usize;
pub const NO_DURATION: usize = 0usize;
pub const FREQUENCY_UNSPECIFIED: usize = 0usize;
pub const HAND_JOINT_COUNT_EXT: usize = 26usize;
pub const NULL_CONTROLLER_MODEL_KEY_MSFT: usize = 0usize;
pub const NULL_RENDER_MODEL_KEY_FB: usize = 0usize;
pub const FACIAL_EXPRESSION_EYE_COUNT_HTC: usize = 14usize;
pub const FACIAL_EXPRESSION_LIP_COUNT_HTC: usize = 37usize;
pub const MAX_EXTENSION_NAME_SIZE: usize = 128usize;
pub const MAX_API_LAYER_NAME_SIZE: usize = 256usize;
pub const MAX_API_LAYER_DESCRIPTION_SIZE: usize = 256usize;
pub const MAX_SYSTEM_NAME_SIZE: usize = 256usize;
pub const MAX_APPLICATION_NAME_SIZE: usize = 128usize;
pub const MAX_ENGINE_NAME_SIZE: usize = 128usize;
pub const MAX_RUNTIME_NAME_SIZE: usize = 128usize;
pub const MAX_PATH_LENGTH: usize = 256usize;
pub const MAX_STRUCTURE_NAME_SIZE: usize = 64usize;
pub const MAX_RESULT_STRING_SIZE: usize = 64usize;
pub const MAX_GRAPHICS_APIS_SUPPORTED: usize = 32usize;
pub const MAX_ACTION_SET_NAME_SIZE: usize = 64usize;
pub const MAX_ACTION_NAME_SIZE: usize = 64usize;
pub const MAX_LOCALIZED_ACTION_SET_NAME_SIZE: usize = 128usize;
pub const MAX_LOCALIZED_ACTION_NAME_SIZE: usize = 128usize;
pub const MIN_COMPOSITION_LAYERS_SUPPORTED: usize = 16usize;
pub const MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT: usize = 64usize;
pub const HAND_TRACKING_CAPSULE_POINT_COUNT_FB: usize = 2usize;
pub const HAND_TRACKING_CAPSULE_COUNT_FB: usize = 19usize;
pub const MAX_KEYBOARD_TRACKING_NAME_SIZE_FB: usize = 128usize;
pub const PASSTHROUGH_COLOR_MAP_MONO_SIZE_FB: usize = 256usize;
pub const MAX_RENDER_MODEL_NAME_SIZE_FB: usize = 64usize;
pub const MAX_SPATIAL_ANCHOR_NAME_SIZE_MSFT: usize = 256usize;
pub const MAX_AUDIO_DEVICE_STR_SIZE_OCULUS: usize = 128usize;
pub const UUID_SIZE_EXT: usize = 16usize;
#[doc = "Structure type enumerant - see [XrStructureType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrStructureType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StructureType(i32);
impl StructureType {
    pub const UNKNOWN: StructureType = Self(0i32);
    pub const API_LAYER_PROPERTIES: StructureType = Self(1i32);
    pub const EXTENSION_PROPERTIES: StructureType = Self(2i32);
    pub const INSTANCE_CREATE_INFO: StructureType = Self(3i32);
    pub const SYSTEM_GET_INFO: StructureType = Self(4i32);
    pub const SYSTEM_PROPERTIES: StructureType = Self(5i32);
    pub const VIEW_LOCATE_INFO: StructureType = Self(6i32);
    pub const VIEW: StructureType = Self(7i32);
    pub const SESSION_CREATE_INFO: StructureType = Self(8i32);
    pub const SWAPCHAIN_CREATE_INFO: StructureType = Self(9i32);
    pub const SESSION_BEGIN_INFO: StructureType = Self(10i32);
    pub const VIEW_STATE: StructureType = Self(11i32);
    pub const FRAME_END_INFO: StructureType = Self(12i32);
    pub const HAPTIC_VIBRATION: StructureType = Self(13i32);
    pub const EVENT_DATA_BUFFER: StructureType = Self(16i32);
    pub const EVENT_DATA_INSTANCE_LOSS_PENDING: StructureType = Self(17i32);
    pub const EVENT_DATA_SESSION_STATE_CHANGED: StructureType = Self(18i32);
    pub const ACTION_STATE_BOOLEAN: StructureType = Self(23i32);
    pub const ACTION_STATE_FLOAT: StructureType = Self(24i32);
    pub const ACTION_STATE_VECTOR2F: StructureType = Self(25i32);
    pub const ACTION_STATE_POSE: StructureType = Self(27i32);
    pub const ACTION_SET_CREATE_INFO: StructureType = Self(28i32);
    pub const ACTION_CREATE_INFO: StructureType = Self(29i32);
    pub const INSTANCE_PROPERTIES: StructureType = Self(32i32);
    pub const FRAME_WAIT_INFO: StructureType = Self(33i32);
    pub const COMPOSITION_LAYER_PROJECTION: StructureType = Self(35i32);
    pub const COMPOSITION_LAYER_QUAD: StructureType = Self(36i32);
    pub const REFERENCE_SPACE_CREATE_INFO: StructureType = Self(37i32);
    pub const ACTION_SPACE_CREATE_INFO: StructureType = Self(38i32);
    pub const EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING: StructureType = Self(40i32);
    pub const VIEW_CONFIGURATION_VIEW: StructureType = Self(41i32);
    pub const SPACE_LOCATION: StructureType = Self(42i32);
    pub const SPACE_VELOCITY: StructureType = Self(43i32);
    pub const FRAME_STATE: StructureType = Self(44i32);
    pub const VIEW_CONFIGURATION_PROPERTIES: StructureType = Self(45i32);
    pub const FRAME_BEGIN_INFO: StructureType = Self(46i32);
    pub const COMPOSITION_LAYER_PROJECTION_VIEW: StructureType = Self(48i32);
    pub const EVENT_DATA_EVENTS_LOST: StructureType = Self(49i32);
    pub const INTERACTION_PROFILE_SUGGESTED_BINDING: StructureType = Self(51i32);
    pub const EVENT_DATA_INTERACTION_PROFILE_CHANGED: StructureType = Self(52i32);
    pub const INTERACTION_PROFILE_STATE: StructureType = Self(53i32);
    pub const SWAPCHAIN_IMAGE_ACQUIRE_INFO: StructureType = Self(55i32);
    pub const SWAPCHAIN_IMAGE_WAIT_INFO: StructureType = Self(56i32);
    pub const SWAPCHAIN_IMAGE_RELEASE_INFO: StructureType = Self(57i32);
    pub const ACTION_STATE_GET_INFO: StructureType = Self(58i32);
    pub const HAPTIC_ACTION_INFO: StructureType = Self(59i32);
    pub const SESSION_ACTION_SETS_ATTACH_INFO: StructureType = Self(60i32);
    pub const ACTIONS_SYNC_INFO: StructureType = Self(61i32);
    pub const BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO: StructureType = Self(62i32);
    pub const INPUT_SOURCE_LOCALIZED_NAME_GET_INFO: StructureType = Self(63i32);
    pub const COMPOSITION_LAYER_CUBE_KHR: StructureType = Self(1000006000i32);
    pub const INSTANCE_CREATE_INFO_ANDROID_KHR: StructureType = Self(1000008000i32);
    pub const COMPOSITION_LAYER_DEPTH_INFO_KHR: StructureType = Self(1000010000i32);
    pub const VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR: StructureType = Self(1000014000i32);
    pub const EVENT_DATA_PERF_SETTINGS_EXT: StructureType = Self(1000015000i32);
    pub const COMPOSITION_LAYER_CYLINDER_KHR: StructureType = Self(1000017000i32);
    pub const COMPOSITION_LAYER_EQUIRECT_KHR: StructureType = Self(1000018000i32);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: StructureType = Self(1000019000i32);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: StructureType = Self(1000019001i32);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: StructureType = Self(1000019002i32);
    pub const DEBUG_UTILS_LABEL_EXT: StructureType = Self(1000019003i32);
    pub const GRAPHICS_BINDING_OPENGL_WIN32_KHR: StructureType = Self(1000023000i32);
    pub const GRAPHICS_BINDING_OPENGL_XLIB_KHR: StructureType = Self(1000023001i32);
    pub const GRAPHICS_BINDING_OPENGL_XCB_KHR: StructureType = Self(1000023002i32);
    pub const GRAPHICS_BINDING_OPENGL_WAYLAND_KHR: StructureType = Self(1000023003i32);
    pub const SWAPCHAIN_IMAGE_OPENGL_KHR: StructureType = Self(1000023004i32);
    pub const GRAPHICS_REQUIREMENTS_OPENGL_KHR: StructureType = Self(1000023005i32);
    pub const GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR: StructureType = Self(1000024001i32);
    pub const SWAPCHAIN_IMAGE_OPENGL_ES_KHR: StructureType = Self(1000024002i32);
    pub const GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR: StructureType = Self(1000024003i32);
    pub const GRAPHICS_BINDING_VULKAN_KHR: StructureType = Self(1000025000i32);
    pub const SWAPCHAIN_IMAGE_VULKAN_KHR: StructureType = Self(1000025001i32);
    pub const GRAPHICS_REQUIREMENTS_VULKAN_KHR: StructureType = Self(1000025002i32);
    pub const GRAPHICS_BINDING_D3D11_KHR: StructureType = Self(1000027000i32);
    pub const SWAPCHAIN_IMAGE_D3D11_KHR: StructureType = Self(1000027001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D11_KHR: StructureType = Self(1000027002i32);
    pub const GRAPHICS_BINDING_D3D12_KHR: StructureType = Self(1000028000i32);
    pub const SWAPCHAIN_IMAGE_D3D12_KHR: StructureType = Self(1000028001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D12_KHR: StructureType = Self(1000028002i32);
    pub const SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT: StructureType = Self(1000030000i32);
    pub const EYE_GAZE_SAMPLE_TIME_EXT: StructureType = Self(1000030001i32);
    pub const VISIBILITY_MASK_KHR: StructureType = Self(1000031000i32);
    pub const EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR: StructureType = Self(1000031001i32);
    pub const SESSION_CREATE_INFO_OVERLAY_EXTX: StructureType = Self(1000033000i32);
    pub const EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX: StructureType = Self(1000033003i32);
    pub const COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR: StructureType = Self(1000034000i32);
    pub const SPATIAL_ANCHOR_CREATE_INFO_MSFT: StructureType = Self(1000039000i32);
    pub const SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT: StructureType = Self(1000039001i32);
    pub const COMPOSITION_LAYER_IMAGE_LAYOUT_FB: StructureType = Self(1000040000i32);
    pub const COMPOSITION_LAYER_ALPHA_BLEND_FB: StructureType = Self(1000041001i32);
    pub const VIEW_CONFIGURATION_DEPTH_RANGE_EXT: StructureType = Self(1000046000i32);
    pub const GRAPHICS_BINDING_EGL_MNDX: StructureType = Self(1000048004i32);
    pub const SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT: StructureType = Self(1000049000i32);
    pub const SYSTEM_HAND_TRACKING_PROPERTIES_EXT: StructureType = Self(1000051000i32);
    pub const HAND_TRACKER_CREATE_INFO_EXT: StructureType = Self(1000051001i32);
    pub const HAND_JOINTS_LOCATE_INFO_EXT: StructureType = Self(1000051002i32);
    pub const HAND_JOINT_LOCATIONS_EXT: StructureType = Self(1000051003i32);
    pub const HAND_JOINT_VELOCITIES_EXT: StructureType = Self(1000051004i32);
    pub const SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT: StructureType = Self(1000052000i32);
    pub const HAND_MESH_SPACE_CREATE_INFO_MSFT: StructureType = Self(1000052001i32);
    pub const HAND_MESH_UPDATE_INFO_MSFT: StructureType = Self(1000052002i32);
    pub const HAND_MESH_MSFT: StructureType = Self(1000052003i32);
    pub const HAND_POSE_TYPE_INFO_MSFT: StructureType = Self(1000052004i32);
    pub const SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT: StructureType =
        Self(1000053000i32);
    pub const SECONDARY_VIEW_CONFIGURATION_STATE_MSFT: StructureType = Self(1000053001i32);
    pub const SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT: StructureType = Self(1000053002i32);
    pub const SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT: StructureType = Self(1000053003i32);
    pub const SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT: StructureType = Self(1000053004i32);
    pub const SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT: StructureType =
        Self(1000053005i32);
    pub const CONTROLLER_MODEL_KEY_STATE_MSFT: StructureType = Self(1000055000i32);
    pub const CONTROLLER_MODEL_NODE_PROPERTIES_MSFT: StructureType = Self(1000055001i32);
    pub const CONTROLLER_MODEL_PROPERTIES_MSFT: StructureType = Self(1000055002i32);
    pub const CONTROLLER_MODEL_NODE_STATE_MSFT: StructureType = Self(1000055003i32);
    pub const CONTROLLER_MODEL_STATE_MSFT: StructureType = Self(1000055004i32);
    pub const VIEW_CONFIGURATION_VIEW_FOV_EPIC: StructureType = Self(1000059000i32);
    pub const HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT: StructureType = Self(1000063000i32);
    pub const COMPOSITION_LAYER_REPROJECTION_INFO_MSFT: StructureType = Self(1000066000i32);
    pub const COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT: StructureType =
        Self(1000066001i32);
    pub const ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB: StructureType = Self(1000070000i32);
    pub const COMPOSITION_LAYER_SECURE_CONTENT_FB: StructureType = Self(1000072000i32);
    pub const INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE: StructureType = Self(1000079000i32);
    pub const HAND_JOINTS_MOTION_RANGE_INFO_EXT: StructureType = Self(1000080000i32);
    pub const LOADER_INIT_INFO_ANDROID_KHR: StructureType = Self(1000089000i32);
    pub const VULKAN_INSTANCE_CREATE_INFO_KHR: StructureType = Self(1000090000i32);
    pub const VULKAN_DEVICE_CREATE_INFO_KHR: StructureType = Self(1000090001i32);
    pub const VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR: StructureType = Self(1000090003i32);
    pub const GRAPHICS_BINDING_VULKAN2_KHR: StructureType = Self::GRAPHICS_BINDING_VULKAN_KHR;
    pub const SWAPCHAIN_IMAGE_VULKAN2_KHR: StructureType = Self::SWAPCHAIN_IMAGE_VULKAN_KHR;
    pub const GRAPHICS_REQUIREMENTS_VULKAN2_KHR: StructureType =
        Self::GRAPHICS_REQUIREMENTS_VULKAN_KHR;
    pub const COMPOSITION_LAYER_EQUIRECT2_KHR: StructureType = Self(1000091000i32);
    pub const SCENE_OBSERVER_CREATE_INFO_MSFT: StructureType = Self(1000097000i32);
    pub const SCENE_CREATE_INFO_MSFT: StructureType = Self(1000097001i32);
    pub const NEW_SCENE_COMPUTE_INFO_MSFT: StructureType = Self(1000097002i32);
    pub const VISUAL_MESH_COMPUTE_LOD_INFO_MSFT: StructureType = Self(1000097003i32);
    pub const SCENE_COMPONENTS_MSFT: StructureType = Self(1000097004i32);
    pub const SCENE_COMPONENTS_GET_INFO_MSFT: StructureType = Self(1000097005i32);
    pub const SCENE_COMPONENT_LOCATIONS_MSFT: StructureType = Self(1000097006i32);
    pub const SCENE_COMPONENTS_LOCATE_INFO_MSFT: StructureType = Self(1000097007i32);
    pub const SCENE_OBJECTS_MSFT: StructureType = Self(1000097008i32);
    pub const SCENE_COMPONENT_PARENT_FILTER_INFO_MSFT: StructureType = Self(1000097009i32);
    pub const SCENE_OBJECT_TYPES_FILTER_INFO_MSFT: StructureType = Self(1000097010i32);
    pub const SCENE_PLANES_MSFT: StructureType = Self(1000097011i32);
    pub const SCENE_PLANE_ALIGNMENT_FILTER_INFO_MSFT: StructureType = Self(1000097012i32);
    pub const SCENE_MESHES_MSFT: StructureType = Self(1000097013i32);
    pub const SCENE_MESH_BUFFERS_GET_INFO_MSFT: StructureType = Self(1000097014i32);
    pub const SCENE_MESH_BUFFERS_MSFT: StructureType = Self(1000097015i32);
    pub const SCENE_MESH_VERTEX_BUFFER_MSFT: StructureType = Self(1000097016i32);
    pub const SCENE_MESH_INDICES_UINT32_MSFT: StructureType = Self(1000097017i32);
    pub const SCENE_MESH_INDICES_UINT16_MSFT: StructureType = Self(1000097018i32);
    pub const SERIALIZED_SCENE_FRAGMENT_DATA_GET_INFO_MSFT: StructureType = Self(1000098000i32);
    pub const SCENE_DESERIALIZE_INFO_MSFT: StructureType = Self(1000098001i32);
    pub const EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB: StructureType = Self(1000101000i32);
    pub const VIVE_TRACKER_PATHS_HTCX: StructureType = Self(1000103000i32);
    pub const EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX: StructureType = Self(1000103001i32);
    pub const SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC: StructureType = Self(1000104000i32);
    pub const FACIAL_TRACKER_CREATE_INFO_HTC: StructureType = Self(1000104001i32);
    pub const FACIAL_EXPRESSIONS_HTC: StructureType = Self(1000104002i32);
    pub const SYSTEM_COLOR_SPACE_PROPERTIES_FB: StructureType = Self(1000108000i32);
    pub const HAND_TRACKING_MESH_FB: StructureType = Self(1000110001i32);
    pub const HAND_TRACKING_SCALE_FB: StructureType = Self(1000110003i32);
    pub const HAND_TRACKING_AIM_STATE_FB: StructureType = Self(1000111001i32);
    pub const HAND_TRACKING_CAPSULES_STATE_FB: StructureType = Self(1000112000i32);
    pub const FOVEATION_PROFILE_CREATE_INFO_FB: StructureType = Self(1000114000i32);
    pub const SWAPCHAIN_CREATE_INFO_FOVEATION_FB: StructureType = Self(1000114001i32);
    pub const SWAPCHAIN_STATE_FOVEATION_FB: StructureType = Self(1000114002i32);
    pub const FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB: StructureType = Self(1000115000i32);
    pub const KEYBOARD_SPACE_CREATE_INFO_FB: StructureType = Self(1000116009i32);
    pub const KEYBOARD_TRACKING_QUERY_FB: StructureType = Self(1000116004i32);
    pub const SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB: StructureType = Self(1000116002i32);
    pub const TRIANGLE_MESH_CREATE_INFO_FB: StructureType = Self(1000117001i32);
    pub const SYSTEM_PASSTHROUGH_PROPERTIES_FB: StructureType = Self(1000118000i32);
    pub const PASSTHROUGH_CREATE_INFO_FB: StructureType = Self(1000118001i32);
    pub const PASSTHROUGH_LAYER_CREATE_INFO_FB: StructureType = Self(1000118002i32);
    pub const COMPOSITION_LAYER_PASSTHROUGH_FB: StructureType = Self(1000118003i32);
    pub const GEOMETRY_INSTANCE_CREATE_INFO_FB: StructureType = Self(1000118004i32);
    pub const GEOMETRY_INSTANCE_TRANSFORM_FB: StructureType = Self(1000118005i32);
    pub const PASSTHROUGH_STYLE_FB: StructureType = Self(1000118020i32);
    pub const PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB: StructureType = Self(1000118021i32);
    pub const PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB: StructureType = Self(1000118022i32);
    pub const EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB: StructureType = Self(1000118030i32);
    pub const RENDER_MODEL_PATH_INFO_FB: StructureType = Self(1000119000i32);
    pub const RENDER_MODEL_PROPERTIES_FB: StructureType = Self(1000119001i32);
    pub const RENDER_MODEL_BUFFER_FB: StructureType = Self(1000119002i32);
    pub const RENDER_MODEL_LOAD_INFO_FB: StructureType = Self(1000119003i32);
    pub const SYSTEM_RENDER_MODEL_PROPERTIES_FB: StructureType = Self(1000119004i32);
    pub const BINDING_MODIFICATIONS_KHR: StructureType = Self(1000120000i32);
    pub const VIEW_LOCATE_FOVEATED_RENDERING_VARJO: StructureType = Self(1000121000i32);
    pub const FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO: StructureType = Self(1000121001i32);
    pub const SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO: StructureType = Self(1000121002i32);
    pub const COMPOSITION_LAYER_DEPTH_TEST_VARJO: StructureType = Self(1000122000i32);
    pub const SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO: StructureType = Self(1000124000i32);
    pub const EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO: StructureType = Self(1000124001i32);
    pub const MARKER_SPACE_CREATE_INFO_VARJO: StructureType = Self(1000124002i32);
    pub const SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT: StructureType = Self(1000142000i32);
    pub const SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT: StructureType =
        Self(1000142001i32);
    pub const SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB: StructureType = Self(1000160000i32);
    pub const SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB: StructureType = Self(1000161000i32);
    pub const SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB: StructureType = Self(1000162000i32);
    pub const SWAPCHAIN_STATE_SAMPLER_VULKAN_FB: StructureType = Self(1000163000i32);
    pub const COMPOSITION_LAYER_SPACE_WARP_INFO_FB: StructureType = Self(1000171000i32);
    pub const SYSTEM_SPACE_WARP_PROPERTIES_FB: StructureType = Self(1000171001i32);
    pub const DIGITAL_LENS_CONTROL_ALMALENCE: StructureType = Self(1000196000i32);
    pub const PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB: StructureType = Self(1000203002i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for StructureType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::API_LAYER_PROPERTIES => Some("API_LAYER_PROPERTIES"),
            Self::EXTENSION_PROPERTIES => Some("EXTENSION_PROPERTIES"),
            Self::INSTANCE_CREATE_INFO => Some("INSTANCE_CREATE_INFO"),
            Self::SYSTEM_GET_INFO => Some("SYSTEM_GET_INFO"),
            Self::SYSTEM_PROPERTIES => Some("SYSTEM_PROPERTIES"),
            Self::VIEW_LOCATE_INFO => Some("VIEW_LOCATE_INFO"),
            Self::VIEW => Some("VIEW"),
            Self::SESSION_CREATE_INFO => Some("SESSION_CREATE_INFO"),
            Self::SWAPCHAIN_CREATE_INFO => Some("SWAPCHAIN_CREATE_INFO"),
            Self::SESSION_BEGIN_INFO => Some("SESSION_BEGIN_INFO"),
            Self::VIEW_STATE => Some("VIEW_STATE"),
            Self::FRAME_END_INFO => Some("FRAME_END_INFO"),
            Self::HAPTIC_VIBRATION => Some("HAPTIC_VIBRATION"),
            Self::EVENT_DATA_BUFFER => Some("EVENT_DATA_BUFFER"),
            Self::EVENT_DATA_INSTANCE_LOSS_PENDING => Some("EVENT_DATA_INSTANCE_LOSS_PENDING"),
            Self::EVENT_DATA_SESSION_STATE_CHANGED => Some("EVENT_DATA_SESSION_STATE_CHANGED"),
            Self::ACTION_STATE_BOOLEAN => Some("ACTION_STATE_BOOLEAN"),
            Self::ACTION_STATE_FLOAT => Some("ACTION_STATE_FLOAT"),
            Self::ACTION_STATE_VECTOR2F => Some("ACTION_STATE_VECTOR2F"),
            Self::ACTION_STATE_POSE => Some("ACTION_STATE_POSE"),
            Self::ACTION_SET_CREATE_INFO => Some("ACTION_SET_CREATE_INFO"),
            Self::ACTION_CREATE_INFO => Some("ACTION_CREATE_INFO"),
            Self::INSTANCE_PROPERTIES => Some("INSTANCE_PROPERTIES"),
            Self::FRAME_WAIT_INFO => Some("FRAME_WAIT_INFO"),
            Self::COMPOSITION_LAYER_PROJECTION => Some("COMPOSITION_LAYER_PROJECTION"),
            Self::COMPOSITION_LAYER_QUAD => Some("COMPOSITION_LAYER_QUAD"),
            Self::REFERENCE_SPACE_CREATE_INFO => Some("REFERENCE_SPACE_CREATE_INFO"),
            Self::ACTION_SPACE_CREATE_INFO => Some("ACTION_SPACE_CREATE_INFO"),
            Self::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                Some("EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING")
            }
            Self::VIEW_CONFIGURATION_VIEW => Some("VIEW_CONFIGURATION_VIEW"),
            Self::SPACE_LOCATION => Some("SPACE_LOCATION"),
            Self::SPACE_VELOCITY => Some("SPACE_VELOCITY"),
            Self::FRAME_STATE => Some("FRAME_STATE"),
            Self::VIEW_CONFIGURATION_PROPERTIES => Some("VIEW_CONFIGURATION_PROPERTIES"),
            Self::FRAME_BEGIN_INFO => Some("FRAME_BEGIN_INFO"),
            Self::COMPOSITION_LAYER_PROJECTION_VIEW => Some("COMPOSITION_LAYER_PROJECTION_VIEW"),
            Self::EVENT_DATA_EVENTS_LOST => Some("EVENT_DATA_EVENTS_LOST"),
            Self::INTERACTION_PROFILE_SUGGESTED_BINDING => {
                Some("INTERACTION_PROFILE_SUGGESTED_BINDING")
            }
            Self::EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
                Some("EVENT_DATA_INTERACTION_PROFILE_CHANGED")
            }
            Self::INTERACTION_PROFILE_STATE => Some("INTERACTION_PROFILE_STATE"),
            Self::SWAPCHAIN_IMAGE_ACQUIRE_INFO => Some("SWAPCHAIN_IMAGE_ACQUIRE_INFO"),
            Self::SWAPCHAIN_IMAGE_WAIT_INFO => Some("SWAPCHAIN_IMAGE_WAIT_INFO"),
            Self::SWAPCHAIN_IMAGE_RELEASE_INFO => Some("SWAPCHAIN_IMAGE_RELEASE_INFO"),
            Self::ACTION_STATE_GET_INFO => Some("ACTION_STATE_GET_INFO"),
            Self::HAPTIC_ACTION_INFO => Some("HAPTIC_ACTION_INFO"),
            Self::SESSION_ACTION_SETS_ATTACH_INFO => Some("SESSION_ACTION_SETS_ATTACH_INFO"),
            Self::ACTIONS_SYNC_INFO => Some("ACTIONS_SYNC_INFO"),
            Self::BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO => {
                Some("BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO")
            }
            Self::INPUT_SOURCE_LOCALIZED_NAME_GET_INFO => {
                Some("INPUT_SOURCE_LOCALIZED_NAME_GET_INFO")
            }
            Self::COMPOSITION_LAYER_CUBE_KHR => Some("COMPOSITION_LAYER_CUBE_KHR"),
            Self::INSTANCE_CREATE_INFO_ANDROID_KHR => Some("INSTANCE_CREATE_INFO_ANDROID_KHR"),
            Self::COMPOSITION_LAYER_DEPTH_INFO_KHR => Some("COMPOSITION_LAYER_DEPTH_INFO_KHR"),
            Self::VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR => {
                Some("VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR")
            }
            Self::EVENT_DATA_PERF_SETTINGS_EXT => Some("EVENT_DATA_PERF_SETTINGS_EXT"),
            Self::COMPOSITION_LAYER_CYLINDER_KHR => Some("COMPOSITION_LAYER_CYLINDER_KHR"),
            Self::COMPOSITION_LAYER_EQUIRECT_KHR => Some("COMPOSITION_LAYER_EQUIRECT_KHR"),
            Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => Some("DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
            }
            Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
            }
            Self::DEBUG_UTILS_LABEL_EXT => Some("DEBUG_UTILS_LABEL_EXT"),
            Self::GRAPHICS_BINDING_OPENGL_WIN32_KHR => Some("GRAPHICS_BINDING_OPENGL_WIN32_KHR"),
            Self::GRAPHICS_BINDING_OPENGL_XLIB_KHR => Some("GRAPHICS_BINDING_OPENGL_XLIB_KHR"),
            Self::GRAPHICS_BINDING_OPENGL_XCB_KHR => Some("GRAPHICS_BINDING_OPENGL_XCB_KHR"),
            Self::GRAPHICS_BINDING_OPENGL_WAYLAND_KHR => {
                Some("GRAPHICS_BINDING_OPENGL_WAYLAND_KHR")
            }
            Self::SWAPCHAIN_IMAGE_OPENGL_KHR => Some("SWAPCHAIN_IMAGE_OPENGL_KHR"),
            Self::GRAPHICS_REQUIREMENTS_OPENGL_KHR => Some("GRAPHICS_REQUIREMENTS_OPENGL_KHR"),
            Self::GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR => {
                Some("GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR")
            }
            Self::SWAPCHAIN_IMAGE_OPENGL_ES_KHR => Some("SWAPCHAIN_IMAGE_OPENGL_ES_KHR"),
            Self::GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR => {
                Some("GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR")
            }
            Self::GRAPHICS_BINDING_VULKAN_KHR => Some("GRAPHICS_BINDING_VULKAN_KHR"),
            Self::SWAPCHAIN_IMAGE_VULKAN_KHR => Some("SWAPCHAIN_IMAGE_VULKAN_KHR"),
            Self::GRAPHICS_REQUIREMENTS_VULKAN_KHR => Some("GRAPHICS_REQUIREMENTS_VULKAN_KHR"),
            Self::GRAPHICS_BINDING_D3D11_KHR => Some("GRAPHICS_BINDING_D3D11_KHR"),
            Self::SWAPCHAIN_IMAGE_D3D11_KHR => Some("SWAPCHAIN_IMAGE_D3D11_KHR"),
            Self::GRAPHICS_REQUIREMENTS_D3D11_KHR => Some("GRAPHICS_REQUIREMENTS_D3D11_KHR"),
            Self::GRAPHICS_BINDING_D3D12_KHR => Some("GRAPHICS_BINDING_D3D12_KHR"),
            Self::SWAPCHAIN_IMAGE_D3D12_KHR => Some("SWAPCHAIN_IMAGE_D3D12_KHR"),
            Self::GRAPHICS_REQUIREMENTS_D3D12_KHR => Some("GRAPHICS_REQUIREMENTS_D3D12_KHR"),
            Self::SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT => {
                Some("SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT")
            }
            Self::EYE_GAZE_SAMPLE_TIME_EXT => Some("EYE_GAZE_SAMPLE_TIME_EXT"),
            Self::VISIBILITY_MASK_KHR => Some("VISIBILITY_MASK_KHR"),
            Self::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
                Some("EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR")
            }
            Self::SESSION_CREATE_INFO_OVERLAY_EXTX => Some("SESSION_CREATE_INFO_OVERLAY_EXTX"),
            Self::EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX => {
                Some("EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX")
            }
            Self::COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR => {
                Some("COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR")
            }
            Self::SPATIAL_ANCHOR_CREATE_INFO_MSFT => Some("SPATIAL_ANCHOR_CREATE_INFO_MSFT"),
            Self::SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT")
            }
            Self::COMPOSITION_LAYER_IMAGE_LAYOUT_FB => Some("COMPOSITION_LAYER_IMAGE_LAYOUT_FB"),
            Self::COMPOSITION_LAYER_ALPHA_BLEND_FB => Some("COMPOSITION_LAYER_ALPHA_BLEND_FB"),
            Self::VIEW_CONFIGURATION_DEPTH_RANGE_EXT => Some("VIEW_CONFIGURATION_DEPTH_RANGE_EXT"),
            Self::GRAPHICS_BINDING_EGL_MNDX => Some("GRAPHICS_BINDING_EGL_MNDX"),
            Self::SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT => {
                Some("SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT")
            }
            Self::SYSTEM_HAND_TRACKING_PROPERTIES_EXT => {
                Some("SYSTEM_HAND_TRACKING_PROPERTIES_EXT")
            }
            Self::HAND_TRACKER_CREATE_INFO_EXT => Some("HAND_TRACKER_CREATE_INFO_EXT"),
            Self::HAND_JOINTS_LOCATE_INFO_EXT => Some("HAND_JOINTS_LOCATE_INFO_EXT"),
            Self::HAND_JOINT_LOCATIONS_EXT => Some("HAND_JOINT_LOCATIONS_EXT"),
            Self::HAND_JOINT_VELOCITIES_EXT => Some("HAND_JOINT_VELOCITIES_EXT"),
            Self::SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT => {
                Some("SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT")
            }
            Self::HAND_MESH_SPACE_CREATE_INFO_MSFT => Some("HAND_MESH_SPACE_CREATE_INFO_MSFT"),
            Self::HAND_MESH_UPDATE_INFO_MSFT => Some("HAND_MESH_UPDATE_INFO_MSFT"),
            Self::HAND_MESH_MSFT => Some("HAND_MESH_MSFT"),
            Self::HAND_POSE_TYPE_INFO_MSFT => Some("HAND_POSE_TYPE_INFO_MSFT"),
            Self::SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT")
            }
            Self::SECONDARY_VIEW_CONFIGURATION_STATE_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_STATE_MSFT")
            }
            Self::SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT")
            }
            Self::SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT")
            }
            Self::SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT")
            }
            Self::SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT => {
                Some("SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT")
            }
            Self::CONTROLLER_MODEL_KEY_STATE_MSFT => Some("CONTROLLER_MODEL_KEY_STATE_MSFT"),
            Self::CONTROLLER_MODEL_NODE_PROPERTIES_MSFT => {
                Some("CONTROLLER_MODEL_NODE_PROPERTIES_MSFT")
            }
            Self::CONTROLLER_MODEL_PROPERTIES_MSFT => Some("CONTROLLER_MODEL_PROPERTIES_MSFT"),
            Self::CONTROLLER_MODEL_NODE_STATE_MSFT => Some("CONTROLLER_MODEL_NODE_STATE_MSFT"),
            Self::CONTROLLER_MODEL_STATE_MSFT => Some("CONTROLLER_MODEL_STATE_MSFT"),
            Self::VIEW_CONFIGURATION_VIEW_FOV_EPIC => Some("VIEW_CONFIGURATION_VIEW_FOV_EPIC"),
            Self::HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT => Some("HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT"),
            Self::COMPOSITION_LAYER_REPROJECTION_INFO_MSFT => {
                Some("COMPOSITION_LAYER_REPROJECTION_INFO_MSFT")
            }
            Self::COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT => {
                Some("COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT")
            }
            Self::ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB => {
                Some("ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB")
            }
            Self::COMPOSITION_LAYER_SECURE_CONTENT_FB => {
                Some("COMPOSITION_LAYER_SECURE_CONTENT_FB")
            }
            Self::INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE => {
                Some("INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE")
            }
            Self::HAND_JOINTS_MOTION_RANGE_INFO_EXT => Some("HAND_JOINTS_MOTION_RANGE_INFO_EXT"),
            Self::LOADER_INIT_INFO_ANDROID_KHR => Some("LOADER_INIT_INFO_ANDROID_KHR"),
            Self::VULKAN_INSTANCE_CREATE_INFO_KHR => Some("VULKAN_INSTANCE_CREATE_INFO_KHR"),
            Self::VULKAN_DEVICE_CREATE_INFO_KHR => Some("VULKAN_DEVICE_CREATE_INFO_KHR"),
            Self::VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR => {
                Some("VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR")
            }
            Self::COMPOSITION_LAYER_EQUIRECT2_KHR => Some("COMPOSITION_LAYER_EQUIRECT2_KHR"),
            Self::SCENE_OBSERVER_CREATE_INFO_MSFT => Some("SCENE_OBSERVER_CREATE_INFO_MSFT"),
            Self::SCENE_CREATE_INFO_MSFT => Some("SCENE_CREATE_INFO_MSFT"),
            Self::NEW_SCENE_COMPUTE_INFO_MSFT => Some("NEW_SCENE_COMPUTE_INFO_MSFT"),
            Self::VISUAL_MESH_COMPUTE_LOD_INFO_MSFT => Some("VISUAL_MESH_COMPUTE_LOD_INFO_MSFT"),
            Self::SCENE_COMPONENTS_MSFT => Some("SCENE_COMPONENTS_MSFT"),
            Self::SCENE_COMPONENTS_GET_INFO_MSFT => Some("SCENE_COMPONENTS_GET_INFO_MSFT"),
            Self::SCENE_COMPONENT_LOCATIONS_MSFT => Some("SCENE_COMPONENT_LOCATIONS_MSFT"),
            Self::SCENE_COMPONENTS_LOCATE_INFO_MSFT => Some("SCENE_COMPONENTS_LOCATE_INFO_MSFT"),
            Self::SCENE_OBJECTS_MSFT => Some("SCENE_OBJECTS_MSFT"),
            Self::SCENE_COMPONENT_PARENT_FILTER_INFO_MSFT => {
                Some("SCENE_COMPONENT_PARENT_FILTER_INFO_MSFT")
            }
            Self::SCENE_OBJECT_TYPES_FILTER_INFO_MSFT => {
                Some("SCENE_OBJECT_TYPES_FILTER_INFO_MSFT")
            }
            Self::SCENE_PLANES_MSFT => Some("SCENE_PLANES_MSFT"),
            Self::SCENE_PLANE_ALIGNMENT_FILTER_INFO_MSFT => {
                Some("SCENE_PLANE_ALIGNMENT_FILTER_INFO_MSFT")
            }
            Self::SCENE_MESHES_MSFT => Some("SCENE_MESHES_MSFT"),
            Self::SCENE_MESH_BUFFERS_GET_INFO_MSFT => Some("SCENE_MESH_BUFFERS_GET_INFO_MSFT"),
            Self::SCENE_MESH_BUFFERS_MSFT => Some("SCENE_MESH_BUFFERS_MSFT"),
            Self::SCENE_MESH_VERTEX_BUFFER_MSFT => Some("SCENE_MESH_VERTEX_BUFFER_MSFT"),
            Self::SCENE_MESH_INDICES_UINT32_MSFT => Some("SCENE_MESH_INDICES_UINT32_MSFT"),
            Self::SCENE_MESH_INDICES_UINT16_MSFT => Some("SCENE_MESH_INDICES_UINT16_MSFT"),
            Self::SERIALIZED_SCENE_FRAGMENT_DATA_GET_INFO_MSFT => {
                Some("SERIALIZED_SCENE_FRAGMENT_DATA_GET_INFO_MSFT")
            }
            Self::SCENE_DESERIALIZE_INFO_MSFT => Some("SCENE_DESERIALIZE_INFO_MSFT"),
            Self::EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB => {
                Some("EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB")
            }
            Self::VIVE_TRACKER_PATHS_HTCX => Some("VIVE_TRACKER_PATHS_HTCX"),
            Self::EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX => {
                Some("EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX")
            }
            Self::SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC => {
                Some("SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC")
            }
            Self::FACIAL_TRACKER_CREATE_INFO_HTC => Some("FACIAL_TRACKER_CREATE_INFO_HTC"),
            Self::FACIAL_EXPRESSIONS_HTC => Some("FACIAL_EXPRESSIONS_HTC"),
            Self::SYSTEM_COLOR_SPACE_PROPERTIES_FB => Some("SYSTEM_COLOR_SPACE_PROPERTIES_FB"),
            Self::HAND_TRACKING_MESH_FB => Some("HAND_TRACKING_MESH_FB"),
            Self::HAND_TRACKING_SCALE_FB => Some("HAND_TRACKING_SCALE_FB"),
            Self::HAND_TRACKING_AIM_STATE_FB => Some("HAND_TRACKING_AIM_STATE_FB"),
            Self::HAND_TRACKING_CAPSULES_STATE_FB => Some("HAND_TRACKING_CAPSULES_STATE_FB"),
            Self::FOVEATION_PROFILE_CREATE_INFO_FB => Some("FOVEATION_PROFILE_CREATE_INFO_FB"),
            Self::SWAPCHAIN_CREATE_INFO_FOVEATION_FB => Some("SWAPCHAIN_CREATE_INFO_FOVEATION_FB"),
            Self::SWAPCHAIN_STATE_FOVEATION_FB => Some("SWAPCHAIN_STATE_FOVEATION_FB"),
            Self::FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB => {
                Some("FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB")
            }
            Self::KEYBOARD_SPACE_CREATE_INFO_FB => Some("KEYBOARD_SPACE_CREATE_INFO_FB"),
            Self::KEYBOARD_TRACKING_QUERY_FB => Some("KEYBOARD_TRACKING_QUERY_FB"),
            Self::SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB => {
                Some("SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB")
            }
            Self::TRIANGLE_MESH_CREATE_INFO_FB => Some("TRIANGLE_MESH_CREATE_INFO_FB"),
            Self::SYSTEM_PASSTHROUGH_PROPERTIES_FB => Some("SYSTEM_PASSTHROUGH_PROPERTIES_FB"),
            Self::PASSTHROUGH_CREATE_INFO_FB => Some("PASSTHROUGH_CREATE_INFO_FB"),
            Self::PASSTHROUGH_LAYER_CREATE_INFO_FB => Some("PASSTHROUGH_LAYER_CREATE_INFO_FB"),
            Self::COMPOSITION_LAYER_PASSTHROUGH_FB => Some("COMPOSITION_LAYER_PASSTHROUGH_FB"),
            Self::GEOMETRY_INSTANCE_CREATE_INFO_FB => Some("GEOMETRY_INSTANCE_CREATE_INFO_FB"),
            Self::GEOMETRY_INSTANCE_TRANSFORM_FB => Some("GEOMETRY_INSTANCE_TRANSFORM_FB"),
            Self::PASSTHROUGH_STYLE_FB => Some("PASSTHROUGH_STYLE_FB"),
            Self::PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB => {
                Some("PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB")
            }
            Self::PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB => {
                Some("PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB")
            }
            Self::EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB => {
                Some("EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB")
            }
            Self::RENDER_MODEL_PATH_INFO_FB => Some("RENDER_MODEL_PATH_INFO_FB"),
            Self::RENDER_MODEL_PROPERTIES_FB => Some("RENDER_MODEL_PROPERTIES_FB"),
            Self::RENDER_MODEL_BUFFER_FB => Some("RENDER_MODEL_BUFFER_FB"),
            Self::RENDER_MODEL_LOAD_INFO_FB => Some("RENDER_MODEL_LOAD_INFO_FB"),
            Self::SYSTEM_RENDER_MODEL_PROPERTIES_FB => Some("SYSTEM_RENDER_MODEL_PROPERTIES_FB"),
            Self::BINDING_MODIFICATIONS_KHR => Some("BINDING_MODIFICATIONS_KHR"),
            Self::VIEW_LOCATE_FOVEATED_RENDERING_VARJO => {
                Some("VIEW_LOCATE_FOVEATED_RENDERING_VARJO")
            }
            Self::FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO => {
                Some("FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO")
            }
            Self::SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO => {
                Some("SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO")
            }
            Self::COMPOSITION_LAYER_DEPTH_TEST_VARJO => Some("COMPOSITION_LAYER_DEPTH_TEST_VARJO"),
            Self::SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO => {
                Some("SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO")
            }
            Self::EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO => {
                Some("EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO")
            }
            Self::MARKER_SPACE_CREATE_INFO_VARJO => Some("MARKER_SPACE_CREATE_INFO_VARJO"),
            Self::SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT")
            }
            Self::SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT")
            }
            Self::SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB => {
                Some("SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB")
            }
            Self::SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB => {
                Some("SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB")
            }
            Self::SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB => {
                Some("SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB")
            }
            Self::SWAPCHAIN_STATE_SAMPLER_VULKAN_FB => Some("SWAPCHAIN_STATE_SAMPLER_VULKAN_FB"),
            Self::COMPOSITION_LAYER_SPACE_WARP_INFO_FB => {
                Some("COMPOSITION_LAYER_SPACE_WARP_INFO_FB")
            }
            Self::SYSTEM_SPACE_WARP_PROPERTIES_FB => Some("SYSTEM_SPACE_WARP_PROPERTIES_FB"),
            Self::DIGITAL_LENS_CONTROL_ALMALENCE => Some("DIGITAL_LENS_CONTROL_ALMALENCE"),
            Self::PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB => {
                Some("PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Error and return codes - see [XrResult](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrResult)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(i32);
impl Result {
    #[doc = "Function successfully completed."]
    pub const SUCCESS: Result = Self(0i32);
    #[doc = "The specified timeout time occurred before the operation could complete."]
    pub const TIMEOUT_EXPIRED: Result = Self(1i32);
    #[doc = "The session will be lost soon."]
    pub const SESSION_LOSS_PENDING: Result = Self(3i32);
    #[doc = "No event was available."]
    pub const EVENT_UNAVAILABLE: Result = Self(4i32);
    #[doc = "The space's bounds are not known at the moment."]
    pub const SPACE_BOUNDS_UNAVAILABLE: Result = Self(7i32);
    #[doc = "The session is not in the focused state."]
    pub const SESSION_NOT_FOCUSED: Result = Self(8i32);
    #[doc = "A frame has been discarded from composition."]
    pub const FRAME_DISCARDED: Result = Self(9i32);
    #[doc = "The function usage was invalid in some way."]
    pub const ERROR_VALIDATION_FAILURE: Result = Self(-1i32);
    #[doc = "The runtime failed to handle the function in an unexpected way that is not covered by another error result."]
    pub const ERROR_RUNTIME_FAILURE: Result = Self(-2i32);
    #[doc = "A memory allocation has failed."]
    pub const ERROR_OUT_OF_MEMORY: Result = Self(-3i32);
    #[doc = "The runtime does not support the requested API version."]
    pub const ERROR_API_VERSION_UNSUPPORTED: Result = Self(-4i32);
    #[doc = "Initialization of object could not be completed."]
    pub const ERROR_INITIALIZATION_FAILED: Result = Self(-6i32);
    #[doc = "The requested function was not found or is otherwise unsupported."]
    pub const ERROR_FUNCTION_UNSUPPORTED: Result = Self(-7i32);
    #[doc = "The requested feature is not supported."]
    pub const ERROR_FEATURE_UNSUPPORTED: Result = Self(-8i32);
    #[doc = "A requested extension is not supported."]
    pub const ERROR_EXTENSION_NOT_PRESENT: Result = Self(-9i32);
    #[doc = "The runtime supports no more of the requested resource."]
    pub const ERROR_LIMIT_REACHED: Result = Self(-10i32);
    #[doc = "The supplied size was smaller than required."]
    pub const ERROR_SIZE_INSUFFICIENT: Result = Self(-11i32);
    #[doc = "A supplied object handle was invalid."]
    pub const ERROR_HANDLE_INVALID: Result = Self(-12i32);
    #[doc = "The XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated."]
    pub const ERROR_INSTANCE_LOST: Result = Self(-13i32);
    #[doc = "The session is already running."]
    pub const ERROR_SESSION_RUNNING: Result = Self(-14i32);
    #[doc = "The session is not yet running."]
    pub const ERROR_SESSION_NOT_RUNNING: Result = Self(-16i32);
    #[doc = "The XrSession was lost. It will need to be destroyed and optionally recreated."]
    pub const ERROR_SESSION_LOST: Result = Self(-17i32);
    #[doc = "The provided XrSystemId was invalid."]
    pub const ERROR_SYSTEM_INVALID: Result = Self(-18i32);
    #[doc = "The provided XrPath was not valid."]
    pub const ERROR_PATH_INVALID: Result = Self(-19i32);
    #[doc = "The maximum number of supported semantic paths has been reached."]
    pub const ERROR_PATH_COUNT_EXCEEDED: Result = Self(-20i32);
    #[doc = "The semantic path character format is invalid."]
    pub const ERROR_PATH_FORMAT_INVALID: Result = Self(-21i32);
    #[doc = "The semantic path is unsupported."]
    pub const ERROR_PATH_UNSUPPORTED: Result = Self(-22i32);
    #[doc = "The layer was NULL or otherwise invalid."]
    pub const ERROR_LAYER_INVALID: Result = Self(-23i32);
    #[doc = "The number of specified layers is greater than the supported number."]
    pub const ERROR_LAYER_LIMIT_EXCEEDED: Result = Self(-24i32);
    #[doc = "The image rect was negatively sized or otherwise invalid."]
    pub const ERROR_SWAPCHAIN_RECT_INVALID: Result = Self(-25i32);
    #[doc = "The image format is not supported by the runtime or platform."]
    pub const ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED: Result = Self(-26i32);
    #[doc = "The API used to retrieve an action's state does not match the action's type."]
    pub const ERROR_ACTION_TYPE_MISMATCH: Result = Self(-27i32);
    #[doc = "The session is not in the ready state."]
    pub const ERROR_SESSION_NOT_READY: Result = Self(-28i32);
    #[doc = "The session is not in the stopping state."]
    pub const ERROR_SESSION_NOT_STOPPING: Result = Self(-29i32);
    #[doc = "The provided XrTime was zero, negative, or out of range."]
    pub const ERROR_TIME_INVALID: Result = Self(-30i32);
    #[doc = "The specified reference space is not supported by the runtime or system."]
    pub const ERROR_REFERENCE_SPACE_UNSUPPORTED: Result = Self(-31i32);
    #[doc = "The file could not be accessed."]
    pub const ERROR_FILE_ACCESS_ERROR: Result = Self(-32i32);
    #[doc = "The file's contents were invalid."]
    pub const ERROR_FILE_CONTENTS_INVALID: Result = Self(-33i32);
    #[doc = "The specified form factor is not supported by the current runtime or platform."]
    pub const ERROR_FORM_FACTOR_UNSUPPORTED: Result = Self(-34i32);
    #[doc = "The specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off."]
    pub const ERROR_FORM_FACTOR_UNAVAILABLE: Result = Self(-35i32);
    #[doc = "A requested API layer is not present or could not be loaded."]
    pub const ERROR_API_LAYER_NOT_PRESENT: Result = Self(-36i32);
    #[doc = "The call was made without having made a previously required call."]
    pub const ERROR_CALL_ORDER_INVALID: Result = Self(-37i32);
    #[doc = "The given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements."]
    pub const ERROR_GRAPHICS_DEVICE_INVALID: Result = Self(-38i32);
    #[doc = "The supplied pose was invalid with respect to the requirements."]
    pub const ERROR_POSE_INVALID: Result = Self(-39i32);
    #[doc = "The supplied index was outside the range of valid indices."]
    pub const ERROR_INDEX_OUT_OF_RANGE: Result = Self(-40i32);
    #[doc = "The specified view configuration type is not supported by the runtime or platform."]
    pub const ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED: Result = Self(-41i32);
    #[doc = "The specified environment blend mode is not supported by the runtime or platform."]
    pub const ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED: Result = Self(-42i32);
    #[doc = "The name provided was a duplicate of an already-existing resource."]
    pub const ERROR_NAME_DUPLICATED: Result = Self(-44i32);
    #[doc = "The name provided was invalid."]
    pub const ERROR_NAME_INVALID: Result = Self(-45i32);
    #[doc = "A referenced action set is not attached to the session."]
    pub const ERROR_ACTIONSET_NOT_ATTACHED: Result = Self(-46i32);
    #[doc = "The session already has attached action sets."]
    pub const ERROR_ACTIONSETS_ALREADY_ATTACHED: Result = Self(-47i32);
    #[doc = "The localized name provided was a duplicate of an already-existing resource."]
    pub const ERROR_LOCALIZED_NAME_DUPLICATED: Result = Self(-48i32);
    #[doc = "The localized name provided was invalid."]
    pub const ERROR_LOCALIZED_NAME_INVALID: Result = Self(-49i32);
    #[doc = "The xrGetGraphicsRequirements* call was not made before calling xrCreateSession."]
    pub const ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING: Result = Self(-50i32);
    #[doc = "The loader was unable to find or load a runtime."]
    pub const ERROR_RUNTIME_UNAVAILABLE: Result = Self(-51i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed as thread id is invalid."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR: Result = Self(-1000003000i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR: Result = Self(-1000003001i32);
    #[doc = "Spatial anchor could not be created at that location."]
    pub const ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT: Result = Self(-1000039001i32);
    #[doc = "The secondary view configuration was not enabled when creating the session."]
    pub const ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT: Result =
        Self(-1000053000i32);
    #[doc = "The controller model key is invalid."]
    pub const ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT: Result = Self(-1000055000i32);
    #[doc = "The reprojection mode is not supported."]
    pub const ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT: Result = Self(-1000066000i32);
    #[doc = "Compute new scene not completed."]
    pub const ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT: Result = Self(-1000097000i32);
    #[doc = "Scene component id invalid."]
    pub const ERROR_SCENE_COMPONENT_ID_INVALID_MSFT: Result = Self(-1000097001i32);
    #[doc = "Scene component type mismatch."]
    pub const ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT: Result = Self(-1000097002i32);
    #[doc = "Scene mesh buffer id invalid."]
    pub const ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT: Result = Self(-1000097003i32);
    #[doc = "Scene compute feature incompatible."]
    pub const ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT: Result = Self(-1000097004i32);
    #[doc = "Scene compute consistency mismatch."]
    pub const ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT: Result = Self(-1000097005i32);
    #[doc = "The display refresh rate is not supported by the platform."]
    pub const ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB: Result = Self(-1000101000i32);
    #[doc = "The color space is not supported by the runtime."]
    pub const ERROR_COLOR_SPACE_UNSUPPORTED_FB: Result = Self(-1000108000i32);
    #[doc = "The object state is unexpected for the issued command."]
    pub const ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB: Result = Self(-1000118000i32);
    #[doc = "Trying to create an MR feature when one was already created and only one instance is allowed."]
    pub const ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB: Result = Self(-1000118001i32);
    #[doc = "Requested functionality requires a feature to be created first."]
    pub const ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB: Result = Self(-1000118002i32);
    #[doc = "Requested functionality is not permitted - application is not allowed to perform the requested operation."]
    pub const ERROR_NOT_PERMITTED_PASSTHROUGH_FB: Result = Self(-1000118003i32);
    #[doc = "There weren't sufficient resources available to perform an operation."]
    pub const ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB: Result = Self(-1000118004i32);
    #[doc = "Unknown Passthrough error (no further details provided)."]
    pub const ERROR_UNKNOWN_PASSTHROUGH_FB: Result = Self(-1000118050i32);
    #[doc = "The model key is invalid."]
    pub const ERROR_RENDER_MODEL_KEY_INVALID_FB: Result = Self(-1000119000i32);
    #[doc = "The model is unavailable."]
    pub const RENDER_MODEL_UNAVAILABLE_FB: Result = Self(1000119020i32);
    #[doc = "Marker tracking is disabled or the specified marker is not currently tracked."]
    pub const ERROR_MARKER_NOT_TRACKED_VARJO: Result = Self(-1000124000i32);
    #[doc = "The specified marker ID is not valid."]
    pub const ERROR_MARKER_ID_INVALID_VARJO: Result = Self(-1000124001i32);
    #[doc = "A spatial anchor was not found associated with the spatial anchor name provided"]
    pub const ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT: Result = Self(-1000142001i32);
    #[doc = "The spatial anchor name provided was not valid"]
    pub const ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT: Result = Self(-1000142002i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::SUCCESS => Some("SUCCESS"),
            Self::TIMEOUT_EXPIRED => Some("TIMEOUT_EXPIRED"),
            Self::SESSION_LOSS_PENDING => Some("SESSION_LOSS_PENDING"),
            Self::EVENT_UNAVAILABLE => Some("EVENT_UNAVAILABLE"),
            Self::SPACE_BOUNDS_UNAVAILABLE => Some("SPACE_BOUNDS_UNAVAILABLE"),
            Self::SESSION_NOT_FOCUSED => Some("SESSION_NOT_FOCUSED"),
            Self::FRAME_DISCARDED => Some("FRAME_DISCARDED"),
            Self::ERROR_VALIDATION_FAILURE => Some("ERROR_VALIDATION_FAILURE"),
            Self::ERROR_RUNTIME_FAILURE => Some("ERROR_RUNTIME_FAILURE"),
            Self::ERROR_OUT_OF_MEMORY => Some("ERROR_OUT_OF_MEMORY"),
            Self::ERROR_API_VERSION_UNSUPPORTED => Some("ERROR_API_VERSION_UNSUPPORTED"),
            Self::ERROR_INITIALIZATION_FAILED => Some("ERROR_INITIALIZATION_FAILED"),
            Self::ERROR_FUNCTION_UNSUPPORTED => Some("ERROR_FUNCTION_UNSUPPORTED"),
            Self::ERROR_FEATURE_UNSUPPORTED => Some("ERROR_FEATURE_UNSUPPORTED"),
            Self::ERROR_EXTENSION_NOT_PRESENT => Some("ERROR_EXTENSION_NOT_PRESENT"),
            Self::ERROR_LIMIT_REACHED => Some("ERROR_LIMIT_REACHED"),
            Self::ERROR_SIZE_INSUFFICIENT => Some("ERROR_SIZE_INSUFFICIENT"),
            Self::ERROR_HANDLE_INVALID => Some("ERROR_HANDLE_INVALID"),
            Self::ERROR_INSTANCE_LOST => Some("ERROR_INSTANCE_LOST"),
            Self::ERROR_SESSION_RUNNING => Some("ERROR_SESSION_RUNNING"),
            Self::ERROR_SESSION_NOT_RUNNING => Some("ERROR_SESSION_NOT_RUNNING"),
            Self::ERROR_SESSION_LOST => Some("ERROR_SESSION_LOST"),
            Self::ERROR_SYSTEM_INVALID => Some("ERROR_SYSTEM_INVALID"),
            Self::ERROR_PATH_INVALID => Some("ERROR_PATH_INVALID"),
            Self::ERROR_PATH_COUNT_EXCEEDED => Some("ERROR_PATH_COUNT_EXCEEDED"),
            Self::ERROR_PATH_FORMAT_INVALID => Some("ERROR_PATH_FORMAT_INVALID"),
            Self::ERROR_PATH_UNSUPPORTED => Some("ERROR_PATH_UNSUPPORTED"),
            Self::ERROR_LAYER_INVALID => Some("ERROR_LAYER_INVALID"),
            Self::ERROR_LAYER_LIMIT_EXCEEDED => Some("ERROR_LAYER_LIMIT_EXCEEDED"),
            Self::ERROR_SWAPCHAIN_RECT_INVALID => Some("ERROR_SWAPCHAIN_RECT_INVALID"),
            Self::ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some("ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED"),
            Self::ERROR_ACTION_TYPE_MISMATCH => Some("ERROR_ACTION_TYPE_MISMATCH"),
            Self::ERROR_SESSION_NOT_READY => Some("ERROR_SESSION_NOT_READY"),
            Self::ERROR_SESSION_NOT_STOPPING => Some("ERROR_SESSION_NOT_STOPPING"),
            Self::ERROR_TIME_INVALID => Some("ERROR_TIME_INVALID"),
            Self::ERROR_REFERENCE_SPACE_UNSUPPORTED => Some("ERROR_REFERENCE_SPACE_UNSUPPORTED"),
            Self::ERROR_FILE_ACCESS_ERROR => Some("ERROR_FILE_ACCESS_ERROR"),
            Self::ERROR_FILE_CONTENTS_INVALID => Some("ERROR_FILE_CONTENTS_INVALID"),
            Self::ERROR_FORM_FACTOR_UNSUPPORTED => Some("ERROR_FORM_FACTOR_UNSUPPORTED"),
            Self::ERROR_FORM_FACTOR_UNAVAILABLE => Some("ERROR_FORM_FACTOR_UNAVAILABLE"),
            Self::ERROR_API_LAYER_NOT_PRESENT => Some("ERROR_API_LAYER_NOT_PRESENT"),
            Self::ERROR_CALL_ORDER_INVALID => Some("ERROR_CALL_ORDER_INVALID"),
            Self::ERROR_GRAPHICS_DEVICE_INVALID => Some("ERROR_GRAPHICS_DEVICE_INVALID"),
            Self::ERROR_POSE_INVALID => Some("ERROR_POSE_INVALID"),
            Self::ERROR_INDEX_OUT_OF_RANGE => Some("ERROR_INDEX_OUT_OF_RANGE"),
            Self::ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => {
                Some("ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED")
            }
            Self::ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => {
                Some("ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED")
            }
            Self::ERROR_NAME_DUPLICATED => Some("ERROR_NAME_DUPLICATED"),
            Self::ERROR_NAME_INVALID => Some("ERROR_NAME_INVALID"),
            Self::ERROR_ACTIONSET_NOT_ATTACHED => Some("ERROR_ACTIONSET_NOT_ATTACHED"),
            Self::ERROR_ACTIONSETS_ALREADY_ATTACHED => Some("ERROR_ACTIONSETS_ALREADY_ATTACHED"),
            Self::ERROR_LOCALIZED_NAME_DUPLICATED => Some("ERROR_LOCALIZED_NAME_DUPLICATED"),
            Self::ERROR_LOCALIZED_NAME_INVALID => Some("ERROR_LOCALIZED_NAME_INVALID"),
            Self::ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING => {
                Some("ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING")
            }
            Self::ERROR_RUNTIME_UNAVAILABLE => Some("ERROR_RUNTIME_UNAVAILABLE"),
            Self::ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR")
            }
            Self::ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR")
            }
            Self::ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => {
                Some("ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT")
            }
            Self::ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT => {
                Some("ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT")
            }
            Self::ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT => {
                Some("ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT")
            }
            Self::ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT => {
                Some("ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT")
            }
            Self::ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT => {
                Some("ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT")
            }
            Self::ERROR_SCENE_COMPONENT_ID_INVALID_MSFT => {
                Some("ERROR_SCENE_COMPONENT_ID_INVALID_MSFT")
            }
            Self::ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT => {
                Some("ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT")
            }
            Self::ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT => {
                Some("ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT")
            }
            Self::ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT => {
                Some("ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT")
            }
            Self::ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT => {
                Some("ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT")
            }
            Self::ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB => {
                Some("ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB")
            }
            Self::ERROR_COLOR_SPACE_UNSUPPORTED_FB => Some("ERROR_COLOR_SPACE_UNSUPPORTED_FB"),
            Self::ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB => {
                Some("ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB")
            }
            Self::ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB => {
                Some("ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB")
            }
            Self::ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB => {
                Some("ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB")
            }
            Self::ERROR_NOT_PERMITTED_PASSTHROUGH_FB => Some("ERROR_NOT_PERMITTED_PASSTHROUGH_FB"),
            Self::ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB => {
                Some("ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB")
            }
            Self::ERROR_UNKNOWN_PASSTHROUGH_FB => Some("ERROR_UNKNOWN_PASSTHROUGH_FB"),
            Self::ERROR_RENDER_MODEL_KEY_INVALID_FB => Some("ERROR_RENDER_MODEL_KEY_INVALID_FB"),
            Self::RENDER_MODEL_UNAVAILABLE_FB => Some("RENDER_MODEL_UNAVAILABLE_FB"),
            Self::ERROR_MARKER_NOT_TRACKED_VARJO => Some("ERROR_MARKER_NOT_TRACKED_VARJO"),
            Self::ERROR_MARKER_ID_INVALID_VARJO => Some("ERROR_MARKER_ID_INVALID_VARJO"),
            Self::ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => {
                Some("ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT")
            }
            Self::ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => {
                Some("ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let reason = match * self { Self :: SUCCESS => Some ("function successfully completed") , Self :: TIMEOUT_EXPIRED => Some ("the specified timeout time occurred before the operation could complete") , Self :: SESSION_LOSS_PENDING => Some ("the session will be lost soon") , Self :: EVENT_UNAVAILABLE => Some ("no event was available") , Self :: SPACE_BOUNDS_UNAVAILABLE => Some ("the space's bounds are not known at the moment") , Self :: SESSION_NOT_FOCUSED => Some ("the session is not in the focused state") , Self :: FRAME_DISCARDED => Some ("a frame has been discarded from composition") , Self :: ERROR_VALIDATION_FAILURE => Some ("the function usage was invalid in some way") , Self :: ERROR_RUNTIME_FAILURE => Some ("the runtime failed to handle the function in an unexpected way that is not covered by another error result") , Self :: ERROR_OUT_OF_MEMORY => Some ("a memory allocation has failed") , Self :: ERROR_API_VERSION_UNSUPPORTED => Some ("the runtime does not support the requested API version") , Self :: ERROR_INITIALIZATION_FAILED => Some ("initialization of object could not be completed") , Self :: ERROR_FUNCTION_UNSUPPORTED => Some ("the requested function was not found or is otherwise unsupported") , Self :: ERROR_FEATURE_UNSUPPORTED => Some ("the requested feature is not supported") , Self :: ERROR_EXTENSION_NOT_PRESENT => Some ("a requested extension is not supported") , Self :: ERROR_LIMIT_REACHED => Some ("the runtime supports no more of the requested resource") , Self :: ERROR_SIZE_INSUFFICIENT => Some ("the supplied size was smaller than required") , Self :: ERROR_HANDLE_INVALID => Some ("a supplied object handle was invalid") , Self :: ERROR_INSTANCE_LOST => Some ("the XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated") , Self :: ERROR_SESSION_RUNNING => Some ("the session is already running") , Self :: ERROR_SESSION_NOT_RUNNING => Some ("the session is not yet running") , Self :: ERROR_SESSION_LOST => Some ("the XrSession was lost. It will need to be destroyed and optionally recreated") , Self :: ERROR_SYSTEM_INVALID => Some ("the provided XrSystemId was invalid") , Self :: ERROR_PATH_INVALID => Some ("the provided XrPath was not valid") , Self :: ERROR_PATH_COUNT_EXCEEDED => Some ("the maximum number of supported semantic paths has been reached") , Self :: ERROR_PATH_FORMAT_INVALID => Some ("the semantic path character format is invalid") , Self :: ERROR_PATH_UNSUPPORTED => Some ("the semantic path is unsupported") , Self :: ERROR_LAYER_INVALID => Some ("the layer was NULL or otherwise invalid") , Self :: ERROR_LAYER_LIMIT_EXCEEDED => Some ("the number of specified layers is greater than the supported number") , Self :: ERROR_SWAPCHAIN_RECT_INVALID => Some ("the image rect was negatively sized or otherwise invalid") , Self :: ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some ("the image format is not supported by the runtime or platform") , Self :: ERROR_ACTION_TYPE_MISMATCH => Some ("the API used to retrieve an action's state does not match the action's type") , Self :: ERROR_SESSION_NOT_READY => Some ("the session is not in the ready state") , Self :: ERROR_SESSION_NOT_STOPPING => Some ("the session is not in the stopping state") , Self :: ERROR_TIME_INVALID => Some ("the provided XrTime was zero, negative, or out of range") , Self :: ERROR_REFERENCE_SPACE_UNSUPPORTED => Some ("the specified reference space is not supported by the runtime or system") , Self :: ERROR_FILE_ACCESS_ERROR => Some ("the file could not be accessed") , Self :: ERROR_FILE_CONTENTS_INVALID => Some ("the file's contents were invalid") , Self :: ERROR_FORM_FACTOR_UNSUPPORTED => Some ("the specified form factor is not supported by the current runtime or platform") , Self :: ERROR_FORM_FACTOR_UNAVAILABLE => Some ("the specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off") , Self :: ERROR_API_LAYER_NOT_PRESENT => Some ("a requested API layer is not present or could not be loaded") , Self :: ERROR_CALL_ORDER_INVALID => Some ("the call was made without having made a previously required call") , Self :: ERROR_GRAPHICS_DEVICE_INVALID => Some ("the given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements") , Self :: ERROR_POSE_INVALID => Some ("the supplied pose was invalid with respect to the requirements") , Self :: ERROR_INDEX_OUT_OF_RANGE => Some ("the supplied index was outside the range of valid indices") , Self :: ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => Some ("the specified view configuration type is not supported by the runtime or platform") , Self :: ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => Some ("the specified environment blend mode is not supported by the runtime or platform") , Self :: ERROR_NAME_DUPLICATED => Some ("the name provided was a duplicate of an already-existing resource") , Self :: ERROR_NAME_INVALID => Some ("the name provided was invalid") , Self :: ERROR_ACTIONSET_NOT_ATTACHED => Some ("a referenced action set is not attached to the session") , Self :: ERROR_ACTIONSETS_ALREADY_ATTACHED => Some ("the session already has attached action sets") , Self :: ERROR_LOCALIZED_NAME_DUPLICATED => Some ("the localized name provided was a duplicate of an already-existing resource") , Self :: ERROR_LOCALIZED_NAME_INVALID => Some ("the localized name provided was invalid") , Self :: ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING => Some ("the xrGetGraphicsRequirements* call was not made before calling xrCreateSession") , Self :: ERROR_RUNTIME_UNAVAILABLE => Some ("the loader was unable to find or load a runtime") , Self :: ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => Some ("xrSetAndroidApplicationThreadKHR failed as thread id is invalid") , Self :: ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => Some ("xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority") , Self :: ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => Some ("spatial anchor could not be created at that location") , Self :: ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT => Some ("the secondary view configuration was not enabled when creating the session") , Self :: ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT => Some ("the controller model key is invalid") , Self :: ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT => Some ("the reprojection mode is not supported") , Self :: ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT => Some ("compute new scene not completed") , Self :: ERROR_SCENE_COMPONENT_ID_INVALID_MSFT => Some ("scene component id invalid") , Self :: ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT => Some ("scene component type mismatch") , Self :: ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT => Some ("scene mesh buffer id invalid") , Self :: ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT => Some ("scene compute feature incompatible") , Self :: ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT => Some ("scene compute consistency mismatch") , Self :: ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB => Some ("the display refresh rate is not supported by the platform") , Self :: ERROR_COLOR_SPACE_UNSUPPORTED_FB => Some ("the color space is not supported by the runtime") , Self :: ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB => Some ("the object state is unexpected for the issued command") , Self :: ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB => Some ("trying to create an MR feature when one was already created and only one instance is allowed") , Self :: ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB => Some ("requested functionality requires a feature to be created first") , Self :: ERROR_NOT_PERMITTED_PASSTHROUGH_FB => Some ("requested functionality is not permitted - application is not allowed to perform the requested operation") , Self :: ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB => Some ("there weren't sufficient resources available to perform an operation") , Self :: ERROR_UNKNOWN_PASSTHROUGH_FB => Some ("unknown Passthrough error (no further details provided)") , Self :: ERROR_RENDER_MODEL_KEY_INVALID_FB => Some ("the model key is invalid") , Self :: RENDER_MODEL_UNAVAILABLE_FB => Some ("the model is unavailable") , Self :: ERROR_MARKER_NOT_TRACKED_VARJO => Some ("marker tracking is disabled or the specified marker is not currently tracked") , Self :: ERROR_MARKER_ID_INVALID_VARJO => Some ("the specified marker ID is not valid") , Self :: ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => Some ("a spatial anchor was not found associated with the spatial anchor name provided") , Self :: ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => Some ("the spatial anchor name provided was not valid") , _ => None , } ;
        if let Some(reason) = reason {
            fmt.pad(reason)
        } else {
            write!(fmt, "unknown error (code {})", self.0)
        }
    }
}
impl std::error::Error for Result {}
#[doc = "Enums to track objects of various types - see [XrObjectType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrObjectType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ObjectType(i32);
impl ObjectType {
    pub const UNKNOWN: ObjectType = Self(0i32);
    #[doc = "XrInstance"]
    pub const INSTANCE: ObjectType = Self(1i32);
    #[doc = "XrSession"]
    pub const SESSION: ObjectType = Self(2i32);
    #[doc = "XrSwapchain"]
    pub const SWAPCHAIN: ObjectType = Self(3i32);
    #[doc = "XrSpace"]
    pub const SPACE: ObjectType = Self(4i32);
    #[doc = "XrActionSet"]
    pub const ACTION_SET: ObjectType = Self(5i32);
    #[doc = "XrAction"]
    pub const ACTION: ObjectType = Self(6i32);
    #[doc = "XrDebugUtilsMessengerEXT"]
    pub const DEBUG_UTILS_MESSENGER_EXT: ObjectType = Self(1000019000i32);
    #[doc = "XrSpatialAnchorMSFT"]
    pub const SPATIAL_ANCHOR_MSFT: ObjectType = Self(1000039000i32);
    #[doc = "XrHandTrackerEXT"]
    pub const HAND_TRACKER_EXT: ObjectType = Self(1000051000i32);
    #[doc = "XrSceneObserverMSFT"]
    pub const SCENE_OBSERVER_MSFT: ObjectType = Self(1000097000i32);
    #[doc = "XrSceneMSFT"]
    pub const SCENE_MSFT: ObjectType = Self(1000097001i32);
    #[doc = "XrFacialTrackerHTC"]
    pub const FACIAL_TRACKER_HTC: ObjectType = Self(1000104000i32);
    #[doc = "XrFoveationProfileFB"]
    pub const FOVEATION_PROFILE_FB: ObjectType = Self(1000114000i32);
    #[doc = "XrTriangleMeshFB"]
    pub const TRIANGLE_MESH_FB: ObjectType = Self(1000117000i32);
    #[doc = "XrPassthroughFB"]
    pub const PASSTHROUGH_FB: ObjectType = Self(1000118000i32);
    #[doc = "XrPassthroughLayerFB"]
    pub const PASSTHROUGH_LAYER_FB: ObjectType = Self(1000118002i32);
    #[doc = "XrGeometryInstanceFB"]
    pub const GEOMETRY_INSTANCE_FB: ObjectType = Self(1000118004i32);
    #[doc = "XrSpatialAnchorStoreConnectionMSFT"]
    pub const SPATIAL_ANCHOR_STORE_CONNECTION_MSFT: ObjectType = Self(1000142000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ObjectType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::INSTANCE => Some("INSTANCE"),
            Self::SESSION => Some("SESSION"),
            Self::SWAPCHAIN => Some("SWAPCHAIN"),
            Self::SPACE => Some("SPACE"),
            Self::ACTION_SET => Some("ACTION_SET"),
            Self::ACTION => Some("ACTION"),
            Self::DEBUG_UTILS_MESSENGER_EXT => Some("DEBUG_UTILS_MESSENGER_EXT"),
            Self::SPATIAL_ANCHOR_MSFT => Some("SPATIAL_ANCHOR_MSFT"),
            Self::HAND_TRACKER_EXT => Some("HAND_TRACKER_EXT"),
            Self::SCENE_OBSERVER_MSFT => Some("SCENE_OBSERVER_MSFT"),
            Self::SCENE_MSFT => Some("SCENE_MSFT"),
            Self::FACIAL_TRACKER_HTC => Some("FACIAL_TRACKER_HTC"),
            Self::FOVEATION_PROFILE_FB => Some("FOVEATION_PROFILE_FB"),
            Self::TRIANGLE_MESH_FB => Some("TRIANGLE_MESH_FB"),
            Self::PASSTHROUGH_FB => Some("PASSTHROUGH_FB"),
            Self::PASSTHROUGH_LAYER_FB => Some("PASSTHROUGH_LAYER_FB"),
            Self::GEOMETRY_INSTANCE_FB => Some("GEOMETRY_INSTANCE_FB"),
            Self::SPATIAL_ANCHOR_STORE_CONNECTION_MSFT => {
                Some("SPATIAL_ANCHOR_STORE_CONNECTION_MSFT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Android Thread Types - see [XrAndroidThreadTypeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrAndroidThreadTypeKHR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AndroidThreadTypeKHR(i32);
impl AndroidThreadTypeKHR {
    pub const APPLICATION_MAIN: AndroidThreadTypeKHR = Self(1i32);
    pub const APPLICATION_WORKER: AndroidThreadTypeKHR = Self(2i32);
    pub const RENDERER_MAIN: AndroidThreadTypeKHR = Self(3i32);
    pub const RENDERER_WORKER: AndroidThreadTypeKHR = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for AndroidThreadTypeKHR {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::APPLICATION_MAIN => Some("APPLICATION_MAIN"),
            Self::APPLICATION_WORKER => Some("APPLICATION_WORKER"),
            Self::RENDERER_MAIN => Some("RENDERER_MAIN"),
            Self::RENDERER_WORKER => Some("RENDERER_WORKER"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "eye visibility selector - see [XrEyeVisibility](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEyeVisibility)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EyeVisibility(i32);
impl EyeVisibility {
    #[doc = "Display in both eyes."]
    pub const BOTH: EyeVisibility = Self(0i32);
    #[doc = "Display in the left eye only."]
    pub const LEFT: EyeVisibility = Self(1i32);
    #[doc = "Display in the right eye only."]
    pub const RIGHT: EyeVisibility = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for EyeVisibility {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::BOTH => Some("BOTH"),
            Self::LEFT => Some("LEFT"),
            Self::RIGHT => Some("RIGHT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrActionType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActionType(i32);
impl ActionType {
    pub const BOOLEAN_INPUT: ActionType = Self(1i32);
    pub const FLOAT_INPUT: ActionType = Self(2i32);
    pub const VECTOR2F_INPUT: ActionType = Self(3i32);
    pub const POSE_INPUT: ActionType = Self(4i32);
    pub const VIBRATION_OUTPUT: ActionType = Self(100i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ActionType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::BOOLEAN_INPUT => Some("BOOLEAN_INPUT"),
            Self::FLOAT_INPUT => Some("FLOAT_INPUT"),
            Self::VECTOR2F_INPUT => Some("VECTOR2F_INPUT"),
            Self::POSE_INPUT => Some("POSE_INPUT"),
            Self::VIBRATION_OUTPUT => Some("VIBRATION_OUTPUT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrReferenceSpaceType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrReferenceSpaceType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReferenceSpaceType(i32);
impl ReferenceSpaceType {
    pub const VIEW: ReferenceSpaceType = Self(1i32);
    pub const LOCAL: ReferenceSpaceType = Self(2i32);
    pub const STAGE: ReferenceSpaceType = Self(3i32);
    pub const UNBOUNDED_MSFT: ReferenceSpaceType = Self(1000038000i32);
    pub const COMBINED_EYE_VARJO: ReferenceSpaceType = Self(1000121000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ReferenceSpaceType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::VIEW => Some("VIEW"),
            Self::LOCAL => Some("LOCAL"),
            Self::STAGE => Some("STAGE"),
            Self::UNBOUNDED_MSFT => Some("UNBOUNDED_MSFT"),
            Self::COMBINED_EYE_VARJO => Some("COMBINED_EYE_VARJO"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFormFactor](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFormFactor)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FormFactor(i32);
impl FormFactor {
    pub const HEAD_MOUNTED_DISPLAY: FormFactor = Self(1i32);
    pub const HANDHELD_DISPLAY: FormFactor = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FormFactor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::HEAD_MOUNTED_DISPLAY => Some("HEAD_MOUNTED_DISPLAY"),
            Self::HANDHELD_DISPLAY => Some("HANDHELD_DISPLAY"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrViewConfigurationType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ViewConfigurationType(i32);
impl ViewConfigurationType {
    pub const PRIMARY_MONO: ViewConfigurationType = Self(1i32);
    pub const PRIMARY_STEREO: ViewConfigurationType = Self(2i32);
    pub const PRIMARY_QUAD_VARJO: ViewConfigurationType = Self(1000037000i32);
    pub const SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT: ViewConfigurationType =
        Self(1000054000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ViewConfigurationType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PRIMARY_MONO => Some("PRIMARY_MONO"),
            Self::PRIMARY_STEREO => Some("PRIMARY_STEREO"),
            Self::PRIMARY_QUAD_VARJO => Some("PRIMARY_QUAD_VARJO"),
            Self::SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT => {
                Some("SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrEnvironmentBlendMode](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEnvironmentBlendMode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnvironmentBlendMode(i32);
impl EnvironmentBlendMode {
    pub const OPAQUE: EnvironmentBlendMode = Self(1i32);
    pub const ADDITIVE: EnvironmentBlendMode = Self(2i32);
    pub const ALPHA_BLEND: EnvironmentBlendMode = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for EnvironmentBlendMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OPAQUE => Some("OPAQUE"),
            Self::ADDITIVE => Some("ADDITIVE"),
            Self::ALPHA_BLEND => Some("ALPHA_BLEND"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSessionState](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionState)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SessionState(i32);
impl SessionState {
    pub const UNKNOWN: SessionState = Self(0i32);
    pub const IDLE: SessionState = Self(1i32);
    pub const READY: SessionState = Self(2i32);
    pub const SYNCHRONIZED: SessionState = Self(3i32);
    pub const VISIBLE: SessionState = Self(4i32);
    pub const FOCUSED: SessionState = Self(5i32);
    pub const STOPPING: SessionState = Self(6i32);
    pub const LOSS_PENDING: SessionState = Self(7i32);
    pub const EXITING: SessionState = Self(8i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SessionState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::IDLE => Some("IDLE"),
            Self::READY => Some("READY"),
            Self::SYNCHRONIZED => Some("SYNCHRONIZED"),
            Self::VISIBLE => Some("VISIBLE"),
            Self::FOCUSED => Some("FOCUSED"),
            Self::STOPPING => Some("STOPPING"),
            Self::LOSS_PENDING => Some("LOSS_PENDING"),
            Self::EXITING => Some("EXITING"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPerfSettingsDomainEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPerfSettingsDomainEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsDomainEXT(i32);
impl PerfSettingsDomainEXT {
    #[doc = "Indicates that the performance settings or notification applies to CPU domain"]
    pub const CPU: PerfSettingsDomainEXT = Self(1i32);
    #[doc = "Indicates that the performance settings or notification applies to GPU domain"]
    pub const GPU: PerfSettingsDomainEXT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PerfSettingsDomainEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::CPU => Some("CPU"),
            Self::GPU => Some("GPU"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPerfSettingsSubDomainEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPerfSettingsSubDomainEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsSubDomainEXT(i32);
impl PerfSettingsSubDomainEXT {
    #[doc = "Indicates that the performance notification originates from the COMPOSITING sub-domain"]
    pub const COMPOSITING: PerfSettingsSubDomainEXT = Self(1i32);
    #[doc = "Indicates that the performance notification originates from the RENDERING sub-domain"]
    pub const RENDERING: PerfSettingsSubDomainEXT = Self(2i32);
    #[doc = "Indicates that the performance notification originates from the THERMAL sub-domain"]
    pub const THERMAL: PerfSettingsSubDomainEXT = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PerfSettingsSubDomainEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COMPOSITING => Some("COMPOSITING"),
            Self::RENDERING => Some("RENDERING"),
            Self::THERMAL => Some("THERMAL"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPerfSettingsLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPerfSettingsLevelEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsLevelEXT(i32);
impl PerfSettingsLevelEXT {
    #[doc = "Performance settings hint used by the application to indicate that it enters a non-XR\n                 section (head-locked / static screen), during which power savings are to be prioritized"]
    pub const POWER_SAVINGS: PerfSettingsLevelEXT = Self(0i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters a low\n                 and stable complexity section, during which reducing power is more important than\n                 occasional late rendering frames"]
    pub const SUSTAINED_LOW: PerfSettingsLevelEXT = Self(25i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters\n                 a high or dynamic complexity section, during which the XR Runtime strives for consistent\n                 XR compositing and frame rendering within a thermally sustainable range"]
    pub const SUSTAINED_HIGH: PerfSettingsLevelEXT = Self(50i32);
    #[doc = "Performance settings hint used by the application to indicate that the application enters\n                 a section with very high complexity, during which the XR Runtime is allowed to step\n                 up beyond the thermally sustainable range"]
    pub const BOOST: PerfSettingsLevelEXT = Self(75i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PerfSettingsLevelEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::POWER_SAVINGS => Some("POWER_SAVINGS"),
            Self::SUSTAINED_LOW => Some("SUSTAINED_LOW"),
            Self::SUSTAINED_HIGH => Some("SUSTAINED_HIGH"),
            Self::BOOST => Some("BOOST"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPerfSettingsNotificationLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPerfSettingsNotificationLevelEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsNotificationLevelEXT(i32);
impl PerfSettingsNotificationLevelEXT {
    #[doc = "Notifies that the sub-domain has reached a level\n                 where no further actions other than currently applied are necessary"]
    pub const NORMAL: PerfSettingsNotificationLevelEXT = Self(0i32);
    #[doc = "Notifies that the sub-domain has reached an early warning level\n                 where the application should start proactive mitigation actions\n                 with the goal to return to the XR_PERF_NOTIF_LEVEL_NORMAL level"]
    pub const WARNING: PerfSettingsNotificationLevelEXT = Self(25i32);
    #[doc = "Notifies that the sub-domain has reached a critical\n                 level with significant performance degradation.\n                 The application should take drastic mitigation action"]
    pub const IMPAIRED: PerfSettingsNotificationLevelEXT = Self(75i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PerfSettingsNotificationLevelEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NORMAL => Some("NORMAL"),
            Self::WARNING => Some("WARNING"),
            Self::IMPAIRED => Some("IMPAIRED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrVisibilityMaskTypeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVisibilityMaskTypeKHR)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VisibilityMaskTypeKHR(i32);
impl VisibilityMaskTypeKHR {
    #[doc = "exclusive mesh; indicates that which the viewer cannot see."]
    pub const HIDDEN_TRIANGLE_MESH: VisibilityMaskTypeKHR = Self(1i32);
    #[doc = "inclusive mesh; indicates strictly that which the viewer can see."]
    pub const VISIBLE_TRIANGLE_MESH: VisibilityMaskTypeKHR = Self(2i32);
    #[doc = "line loop; traces the outline of the area the viewer can see."]
    pub const LINE_LOOP: VisibilityMaskTypeKHR = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for VisibilityMaskTypeKHR {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::HIDDEN_TRIANGLE_MESH => Some("HIDDEN_TRIANGLE_MESH"),
            Self::VISIBLE_TRIANGLE_MESH => Some("VISIBLE_TRIANGLE_MESH"),
            Self::LINE_LOOP => Some("LINE_LOOP"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSpatialGraphNodeTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialGraphNodeTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpatialGraphNodeTypeMSFT(i32);
impl SpatialGraphNodeTypeMSFT {
    pub const STATIC: SpatialGraphNodeTypeMSFT = Self(1i32);
    pub const DYNAMIC: SpatialGraphNodeTypeMSFT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SpatialGraphNodeTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::STATIC => Some("STATIC"),
            Self::DYNAMIC => Some("DYNAMIC"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrBlendFactorFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBlendFactorFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlendFactorFB(i32);
impl BlendFactorFB {
    pub const ZERO: BlendFactorFB = Self(0i32);
    pub const ONE: BlendFactorFB = Self(1i32);
    pub const SRC_ALPHA: BlendFactorFB = Self(2i32);
    pub const ONE_MINUS_SRC_ALPHA: BlendFactorFB = Self(3i32);
    pub const DST_ALPHA: BlendFactorFB = Self(4i32);
    pub const ONE_MINUS_DST_ALPHA: BlendFactorFB = Self(5i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for BlendFactorFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ZERO => Some("ZERO"),
            Self::ONE => Some("ONE"),
            Self::SRC_ALPHA => Some("SRC_ALPHA"),
            Self::ONE_MINUS_SRC_ALPHA => Some("ONE_MINUS_SRC_ALPHA"),
            Self::DST_ALPHA => Some("DST_ALPHA"),
            Self::ONE_MINUS_DST_ALPHA => Some("ONE_MINUS_DST_ALPHA"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrWindingOrderFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrWindingOrderFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WindingOrderFB(i32);
impl WindingOrderFB {
    #[doc = "Winding order is unknown and the runtime cannot make any assumptions on the triangle orientation"]
    pub const UNKNOWN: WindingOrderFB = Self(0i32);
    #[doc = "Clockwise winding order"]
    pub const CW: WindingOrderFB = Self(1i32);
    #[doc = "Counter-clockwise winding order"]
    pub const CCW: WindingOrderFB = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for WindingOrderFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::CW => Some("CW"),
            Self::CCW => Some("CCW"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPassthroughLayerPurposeFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughLayerPurposeFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PassthroughLayerPurposeFB(i32);
impl PassthroughLayerPurposeFB {
    #[doc = "Reconstruction passthrough (full screen environment)"]
    pub const RECONSTRUCTION: PassthroughLayerPurposeFB = Self(0i32);
    #[doc = "Projected passthrough (using a custom surface)"]
    pub const PROJECTED: PassthroughLayerPurposeFB = Self(1i32);
    #[doc = "Passthrough layer purpose for keyboard hands presence."]
    pub const TRACKED_KEYBOARD_HANDS: PassthroughLayerPurposeFB = Self(1000203001i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PassthroughLayerPurposeFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::RECONSTRUCTION => Some("RECONSTRUCTION"),
            Self::PROJECTED => Some("PROJECTED"),
            Self::TRACKED_KEYBOARD_HANDS => Some("TRACKED_KEYBOARD_HANDS"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandEXT(i32);
impl HandEXT {
    pub const LEFT: HandEXT = Self(1i32);
    pub const RIGHT: HandEXT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LEFT => Some("LEFT"),
            Self::RIGHT => Some("RIGHT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandJointEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandJointEXT(i32);
impl HandJointEXT {
    pub const PALM: HandJointEXT = Self(0i32);
    pub const WRIST: HandJointEXT = Self(1i32);
    pub const THUMB_METACARPAL: HandJointEXT = Self(2i32);
    pub const THUMB_PROXIMAL: HandJointEXT = Self(3i32);
    pub const THUMB_DISTAL: HandJointEXT = Self(4i32);
    pub const THUMB_TIP: HandJointEXT = Self(5i32);
    pub const INDEX_METACARPAL: HandJointEXT = Self(6i32);
    pub const INDEX_PROXIMAL: HandJointEXT = Self(7i32);
    pub const INDEX_INTERMEDIATE: HandJointEXT = Self(8i32);
    pub const INDEX_DISTAL: HandJointEXT = Self(9i32);
    pub const INDEX_TIP: HandJointEXT = Self(10i32);
    pub const MIDDLE_METACARPAL: HandJointEXT = Self(11i32);
    pub const MIDDLE_PROXIMAL: HandJointEXT = Self(12i32);
    pub const MIDDLE_INTERMEDIATE: HandJointEXT = Self(13i32);
    pub const MIDDLE_DISTAL: HandJointEXT = Self(14i32);
    pub const MIDDLE_TIP: HandJointEXT = Self(15i32);
    pub const RING_METACARPAL: HandJointEXT = Self(16i32);
    pub const RING_PROXIMAL: HandJointEXT = Self(17i32);
    pub const RING_INTERMEDIATE: HandJointEXT = Self(18i32);
    pub const RING_DISTAL: HandJointEXT = Self(19i32);
    pub const RING_TIP: HandJointEXT = Self(20i32);
    pub const LITTLE_METACARPAL: HandJointEXT = Self(21i32);
    pub const LITTLE_PROXIMAL: HandJointEXT = Self(22i32);
    pub const LITTLE_INTERMEDIATE: HandJointEXT = Self(23i32);
    pub const LITTLE_DISTAL: HandJointEXT = Self(24i32);
    pub const LITTLE_TIP: HandJointEXT = Self(25i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandJointEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PALM => Some("PALM"),
            Self::WRIST => Some("WRIST"),
            Self::THUMB_METACARPAL => Some("THUMB_METACARPAL"),
            Self::THUMB_PROXIMAL => Some("THUMB_PROXIMAL"),
            Self::THUMB_DISTAL => Some("THUMB_DISTAL"),
            Self::THUMB_TIP => Some("THUMB_TIP"),
            Self::INDEX_METACARPAL => Some("INDEX_METACARPAL"),
            Self::INDEX_PROXIMAL => Some("INDEX_PROXIMAL"),
            Self::INDEX_INTERMEDIATE => Some("INDEX_INTERMEDIATE"),
            Self::INDEX_DISTAL => Some("INDEX_DISTAL"),
            Self::INDEX_TIP => Some("INDEX_TIP"),
            Self::MIDDLE_METACARPAL => Some("MIDDLE_METACARPAL"),
            Self::MIDDLE_PROXIMAL => Some("MIDDLE_PROXIMAL"),
            Self::MIDDLE_INTERMEDIATE => Some("MIDDLE_INTERMEDIATE"),
            Self::MIDDLE_DISTAL => Some("MIDDLE_DISTAL"),
            Self::MIDDLE_TIP => Some("MIDDLE_TIP"),
            Self::RING_METACARPAL => Some("RING_METACARPAL"),
            Self::RING_PROXIMAL => Some("RING_PROXIMAL"),
            Self::RING_INTERMEDIATE => Some("RING_INTERMEDIATE"),
            Self::RING_DISTAL => Some("RING_DISTAL"),
            Self::RING_TIP => Some("RING_TIP"),
            Self::LITTLE_METACARPAL => Some("LITTLE_METACARPAL"),
            Self::LITTLE_PROXIMAL => Some("LITTLE_PROXIMAL"),
            Self::LITTLE_INTERMEDIATE => Some("LITTLE_INTERMEDIATE"),
            Self::LITTLE_DISTAL => Some("LITTLE_DISTAL"),
            Self::LITTLE_TIP => Some("LITTLE_TIP"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandJointSetEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointSetEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandJointSetEXT(i32);
impl HandJointSetEXT {
    pub const DEFAULT: HandJointSetEXT = Self(0i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandJointSetEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandJointsMotionRangeEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointsMotionRangeEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandJointsMotionRangeEXT(i32);
impl HandJointsMotionRangeEXT {
    pub const UNOBSTRUCTED: HandJointsMotionRangeEXT = Self(1i32);
    pub const CONFORMING_TO_CONTROLLER: HandJointsMotionRangeEXT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandJointsMotionRangeEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNOBSTRUCTED => Some("UNOBSTRUCTED"),
            Self::CONFORMING_TO_CONTROLLER => Some("CONFORMING_TO_CONTROLLER"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandPoseTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandPoseTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandPoseTypeMSFT(i32);
impl HandPoseTypeMSFT {
    pub const TRACKED: HandPoseTypeMSFT = Self(0i32);
    pub const REFERENCE_OPEN_PALM: HandPoseTypeMSFT = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandPoseTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::TRACKED => Some("TRACKED"),
            Self::REFERENCE_OPEN_PALM => Some("REFERENCE_OPEN_PALM"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneObjectTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneObjectTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneObjectTypeMSFT(i32);
impl SceneObjectTypeMSFT {
    pub const UNCATEGORIZED: SceneObjectTypeMSFT = Self(-1i32);
    pub const BACKGROUND: SceneObjectTypeMSFT = Self(1i32);
    pub const WALL: SceneObjectTypeMSFT = Self(2i32);
    pub const FLOOR: SceneObjectTypeMSFT = Self(3i32);
    pub const CEILING: SceneObjectTypeMSFT = Self(4i32);
    pub const PLATFORM: SceneObjectTypeMSFT = Self(5i32);
    pub const INFERRED: SceneObjectTypeMSFT = Self(6i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneObjectTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNCATEGORIZED => Some("UNCATEGORIZED"),
            Self::BACKGROUND => Some("BACKGROUND"),
            Self::WALL => Some("WALL"),
            Self::FLOOR => Some("FLOOR"),
            Self::CEILING => Some("CEILING"),
            Self::PLATFORM => Some("PLATFORM"),
            Self::INFERRED => Some("INFERRED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrScenePlaneAlignmentTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrScenePlaneAlignmentTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScenePlaneAlignmentTypeMSFT(i32);
impl ScenePlaneAlignmentTypeMSFT {
    pub const NON_ORTHOGONAL: ScenePlaneAlignmentTypeMSFT = Self(0i32);
    pub const HORIZONTAL: ScenePlaneAlignmentTypeMSFT = Self(1i32);
    pub const VERTICAL: ScenePlaneAlignmentTypeMSFT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ScenePlaneAlignmentTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NON_ORTHOGONAL => Some("NON_ORTHOGONAL"),
            Self::HORIZONTAL => Some("HORIZONTAL"),
            Self::VERTICAL => Some("VERTICAL"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneComputeStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneComputeStateMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneComputeStateMSFT(i32);
impl SceneComputeStateMSFT {
    pub const NONE: SceneComputeStateMSFT = Self(0i32);
    pub const UPDATING: SceneComputeStateMSFT = Self(1i32);
    pub const COMPLETED: SceneComputeStateMSFT = Self(2i32);
    pub const COMPLETED_WITH_ERROR: SceneComputeStateMSFT = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneComputeStateMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::UPDATING => Some("UPDATING"),
            Self::COMPLETED => Some("COMPLETED"),
            Self::COMPLETED_WITH_ERROR => Some("COMPLETED_WITH_ERROR"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneComputeFeatureMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneComputeFeatureMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneComputeFeatureMSFT(i32);
impl SceneComputeFeatureMSFT {
    pub const PLANE: SceneComputeFeatureMSFT = Self(1i32);
    pub const PLANE_MESH: SceneComputeFeatureMSFT = Self(2i32);
    pub const VISUAL_MESH: SceneComputeFeatureMSFT = Self(3i32);
    pub const COLLIDER_MESH: SceneComputeFeatureMSFT = Self(4i32);
    pub const SERIALIZE_SCENE: SceneComputeFeatureMSFT = Self(1000098000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneComputeFeatureMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PLANE => Some("PLANE"),
            Self::PLANE_MESH => Some("PLANE_MESH"),
            Self::VISUAL_MESH => Some("VISUAL_MESH"),
            Self::COLLIDER_MESH => Some("COLLIDER_MESH"),
            Self::SERIALIZE_SCENE => Some("SERIALIZE_SCENE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneComputeConsistencyMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneComputeConsistencyMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneComputeConsistencyMSFT(i32);
impl SceneComputeConsistencyMSFT {
    pub const SNAPSHOT_COMPLETE: SceneComputeConsistencyMSFT = Self(1i32);
    pub const SNAPSHOT_INCOMPLETE_FAST: SceneComputeConsistencyMSFT = Self(2i32);
    pub const OCCLUSION_OPTIMIZED: SceneComputeConsistencyMSFT = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneComputeConsistencyMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::SNAPSHOT_COMPLETE => Some("SNAPSHOT_COMPLETE"),
            Self::SNAPSHOT_INCOMPLETE_FAST => Some("SNAPSHOT_INCOMPLETE_FAST"),
            Self::OCCLUSION_OPTIMIZED => Some("OCCLUSION_OPTIMIZED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneComponentTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneComponentTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneComponentTypeMSFT(i32);
impl SceneComponentTypeMSFT {
    pub const INVALID: SceneComponentTypeMSFT = Self(-1i32);
    pub const OBJECT: SceneComponentTypeMSFT = Self(1i32);
    pub const PLANE: SceneComponentTypeMSFT = Self(2i32);
    pub const VISUAL_MESH: SceneComponentTypeMSFT = Self(3i32);
    pub const COLLIDER_MESH: SceneComponentTypeMSFT = Self(4i32);
    pub const SERIALIZED_SCENE_FRAGMENT: SceneComponentTypeMSFT = Self(1000098000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneComponentTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::INVALID => Some("INVALID"),
            Self::OBJECT => Some("OBJECT"),
            Self::PLANE => Some("PLANE"),
            Self::VISUAL_MESH => Some("VISUAL_MESH"),
            Self::COLLIDER_MESH => Some("COLLIDER_MESH"),
            Self::SERIALIZED_SCENE_FRAGMENT => Some("SERIALIZED_SCENE_FRAGMENT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMeshComputeLodMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrMeshComputeLodMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MeshComputeLodMSFT(i32);
impl MeshComputeLodMSFT {
    pub const COARSE: MeshComputeLodMSFT = Self(1i32);
    pub const MEDIUM: MeshComputeLodMSFT = Self(2i32);
    pub const FINE: MeshComputeLodMSFT = Self(3i32);
    pub const UNLIMITED: MeshComputeLodMSFT = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MeshComputeLodMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COARSE => Some("COARSE"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::FINE => Some("FINE"),
            Self::UNLIMITED => Some("UNLIMITED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrColorSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrColorSpaceFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ColorSpaceFB(i32);
impl ColorSpaceFB {
    pub const UNMANAGED: ColorSpaceFB = Self(0i32);
    pub const REC2020: ColorSpaceFB = Self(1i32);
    pub const REC709: ColorSpaceFB = Self(2i32);
    pub const RIFT_CV1: ColorSpaceFB = Self(3i32);
    pub const RIFT_S: ColorSpaceFB = Self(4i32);
    pub const QUEST: ColorSpaceFB = Self(5i32);
    pub const P3: ColorSpaceFB = Self(6i32);
    pub const ADOBE_RGB: ColorSpaceFB = Self(7i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ColorSpaceFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNMANAGED => Some("UNMANAGED"),
            Self::REC2020 => Some("REC2020"),
            Self::REC709 => Some("REC709"),
            Self::RIFT_CV1 => Some("RIFT_CV1"),
            Self::RIFT_S => Some("RIFT_S"),
            Self::QUEST => Some("QUEST"),
            Self::P3 => Some("P3"),
            Self::ADOBE_RGB => Some("ADOBE_RGB"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFoveationLevelFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveationLevelFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FoveationLevelFB(i32);
impl FoveationLevelFB {
    #[doc = "No foveation"]
    pub const NONE: FoveationLevelFB = Self(0i32);
    #[doc = "Less foveation (higher periphery visual fidelity, lower performance)"]
    pub const LOW: FoveationLevelFB = Self(1i32);
    #[doc = "Medium foveation (medium periphery visual fidelity, medium performance)"]
    pub const MEDIUM: FoveationLevelFB = Self(2i32);
    #[doc = "High foveation (lower periphery visual fidelity, higher performance)"]
    pub const HIGH: FoveationLevelFB = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FoveationLevelFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::LOW => Some("LOW"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::HIGH => Some("HIGH"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFoveationDynamicFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveationDynamicFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FoveationDynamicFB(i32);
impl FoveationDynamicFB {
    #[doc = "Static foveation at the maximum desired level"]
    pub const DISABLED: FoveationDynamicFB = Self(0i32);
    #[doc = "Dynamic changing foveation based on performance headroom available up to the maximum desired level"]
    pub const LEVEL_ENABLED: FoveationDynamicFB = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FoveationDynamicFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DISABLED => Some("DISABLED"),
            Self::LEVEL_ENABLED => Some("LEVEL_ENABLED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrReprojectionModeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrReprojectionModeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReprojectionModeMSFT(i32);
impl ReprojectionModeMSFT {
    pub const DEPTH: ReprojectionModeMSFT = Self(1i32);
    pub const PLANAR_FROM_DEPTH: ReprojectionModeMSFT = Self(2i32);
    pub const PLANAR_MANUAL: ReprojectionModeMSFT = Self(3i32);
    pub const ORIENTATION_ONLY: ReprojectionModeMSFT = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ReprojectionModeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEPTH => Some("DEPTH"),
            Self::PLANAR_FROM_DEPTH => Some("PLANAR_FROM_DEPTH"),
            Self::PLANAR_MANUAL => Some("PLANAR_MANUAL"),
            Self::ORIENTATION_ONLY => Some("ORIENTATION_ONLY"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFacialTrackingTypeHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFacialTrackingTypeHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FacialTrackingTypeHTC(i32);
impl FacialTrackingTypeHTC {
    #[doc = "Specifies this handle will observe eye expressions, with values indexed by XrEyeExpressionHTC whose count is XR_FACIAL_EXPRESSION_EYE_COUNT_HTC."]
    pub const EYE_DEFAULT: FacialTrackingTypeHTC = Self(1i32);
    #[doc = "Specifies this handle will observe lip expressions, with values indexed by XrLipExpressionHTC whose count is XR_FACIAL_EXPRESSION_LIP_COUNT_HTC."]
    pub const LIP_DEFAULT: FacialTrackingTypeHTC = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FacialTrackingTypeHTC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::EYE_DEFAULT => Some("EYE_DEFAULT"),
            Self::LIP_DEFAULT => Some("LIP_DEFAULT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrEyeExpressionHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEyeExpressionHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EyeExpressionHTC(i32);
impl EyeExpressionHTC {
    pub const LEFT_BLINK: EyeExpressionHTC = Self(0i32);
    pub const LEFT_WIDE: EyeExpressionHTC = Self(1i32);
    pub const RIGHT_BLINK: EyeExpressionHTC = Self(2i32);
    pub const RIGHT_WIDE: EyeExpressionHTC = Self(3i32);
    pub const LEFT_SQUEEZE: EyeExpressionHTC = Self(4i32);
    pub const RIGHT_SQUEEZE: EyeExpressionHTC = Self(5i32);
    pub const LEFT_DOWN: EyeExpressionHTC = Self(6i32);
    pub const RIGHT_DOWN: EyeExpressionHTC = Self(7i32);
    pub const LEFT_OUT: EyeExpressionHTC = Self(8i32);
    pub const RIGHT_IN: EyeExpressionHTC = Self(9i32);
    pub const LEFT_IN: EyeExpressionHTC = Self(10i32);
    pub const RIGHT_OUT: EyeExpressionHTC = Self(11i32);
    pub const LEFT_UP: EyeExpressionHTC = Self(12i32);
    pub const RIGHT_UP: EyeExpressionHTC = Self(13i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for EyeExpressionHTC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LEFT_BLINK => Some("LEFT_BLINK"),
            Self::LEFT_WIDE => Some("LEFT_WIDE"),
            Self::RIGHT_BLINK => Some("RIGHT_BLINK"),
            Self::RIGHT_WIDE => Some("RIGHT_WIDE"),
            Self::LEFT_SQUEEZE => Some("LEFT_SQUEEZE"),
            Self::RIGHT_SQUEEZE => Some("RIGHT_SQUEEZE"),
            Self::LEFT_DOWN => Some("LEFT_DOWN"),
            Self::RIGHT_DOWN => Some("RIGHT_DOWN"),
            Self::LEFT_OUT => Some("LEFT_OUT"),
            Self::RIGHT_IN => Some("RIGHT_IN"),
            Self::LEFT_IN => Some("LEFT_IN"),
            Self::RIGHT_OUT => Some("RIGHT_OUT"),
            Self::LEFT_UP => Some("LEFT_UP"),
            Self::RIGHT_UP => Some("RIGHT_UP"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrLipExpressionHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrLipExpressionHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LipExpressionHTC(i32);
impl LipExpressionHTC {
    pub const JAW_RIGHT: LipExpressionHTC = Self(0i32);
    pub const JAW_LEFT: LipExpressionHTC = Self(1i32);
    pub const JAW_FORWARD: LipExpressionHTC = Self(2i32);
    pub const JAW_OPEN: LipExpressionHTC = Self(3i32);
    pub const MOUTH_APE_SHAPE: LipExpressionHTC = Self(4i32);
    pub const MOUTH_UPPER_RIGHT: LipExpressionHTC = Self(5i32);
    pub const MOUTH_UPPER_LEFT: LipExpressionHTC = Self(6i32);
    pub const MOUTH_LOWER_RIGHT: LipExpressionHTC = Self(7i32);
    pub const MOUTH_LOWER_LEFT: LipExpressionHTC = Self(8i32);
    pub const MOUTH_UPPER_OVERTURN: LipExpressionHTC = Self(9i32);
    pub const MOUTH_LOWER_OVERTURN: LipExpressionHTC = Self(10i32);
    pub const MOUTH_POUT: LipExpressionHTC = Self(11i32);
    pub const MOUTH_SMILE_RIGHT: LipExpressionHTC = Self(12i32);
    pub const MOUTH_SMILE_LEFT: LipExpressionHTC = Self(13i32);
    pub const MOUTH_SAD_RIGHT: LipExpressionHTC = Self(14i32);
    pub const MOUTH_SAD_LEFT: LipExpressionHTC = Self(15i32);
    pub const CHEEK_PUFF_RIGHT: LipExpressionHTC = Self(16i32);
    pub const CHEEK_PUFF_LEFT: LipExpressionHTC = Self(17i32);
    pub const CHEEK_SUCK: LipExpressionHTC = Self(18i32);
    pub const MOUTH_UPPER_UPRIGHT: LipExpressionHTC = Self(19i32);
    pub const MOUTH_UPPER_UPLEFT: LipExpressionHTC = Self(20i32);
    pub const MOUTH_LOWER_DOWNRIGHT: LipExpressionHTC = Self(21i32);
    pub const MOUTH_LOWER_DOWNLEFT: LipExpressionHTC = Self(22i32);
    pub const MOUTH_UPPER_INSIDE: LipExpressionHTC = Self(23i32);
    pub const MOUTH_LOWER_INSIDE: LipExpressionHTC = Self(24i32);
    pub const MOUTH_LOWER_OVERLAY: LipExpressionHTC = Self(25i32);
    pub const TONGUE_LONGSTEP1: LipExpressionHTC = Self(26i32);
    pub const TONGUE_LEFT: LipExpressionHTC = Self(27i32);
    pub const TONGUE_RIGHT: LipExpressionHTC = Self(28i32);
    pub const TONGUE_UP: LipExpressionHTC = Self(29i32);
    pub const TONGUE_DOWN: LipExpressionHTC = Self(30i32);
    pub const TONGUE_ROLL: LipExpressionHTC = Self(31i32);
    pub const TONGUE_LONGSTEP2: LipExpressionHTC = Self(32i32);
    pub const TONGUE_UPRIGHT_MORPH: LipExpressionHTC = Self(33i32);
    pub const TONGUE_UPLEFT_MORPH: LipExpressionHTC = Self(34i32);
    pub const TONGUE_DOWNRIGHT_MORPH: LipExpressionHTC = Self(35i32);
    pub const TONGUE_DOWNLEFT_MORPH: LipExpressionHTC = Self(36i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LipExpressionHTC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::JAW_RIGHT => Some("JAW_RIGHT"),
            Self::JAW_LEFT => Some("JAW_LEFT"),
            Self::JAW_FORWARD => Some("JAW_FORWARD"),
            Self::JAW_OPEN => Some("JAW_OPEN"),
            Self::MOUTH_APE_SHAPE => Some("MOUTH_APE_SHAPE"),
            Self::MOUTH_UPPER_RIGHT => Some("MOUTH_UPPER_RIGHT"),
            Self::MOUTH_UPPER_LEFT => Some("MOUTH_UPPER_LEFT"),
            Self::MOUTH_LOWER_RIGHT => Some("MOUTH_LOWER_RIGHT"),
            Self::MOUTH_LOWER_LEFT => Some("MOUTH_LOWER_LEFT"),
            Self::MOUTH_UPPER_OVERTURN => Some("MOUTH_UPPER_OVERTURN"),
            Self::MOUTH_LOWER_OVERTURN => Some("MOUTH_LOWER_OVERTURN"),
            Self::MOUTH_POUT => Some("MOUTH_POUT"),
            Self::MOUTH_SMILE_RIGHT => Some("MOUTH_SMILE_RIGHT"),
            Self::MOUTH_SMILE_LEFT => Some("MOUTH_SMILE_LEFT"),
            Self::MOUTH_SAD_RIGHT => Some("MOUTH_SAD_RIGHT"),
            Self::MOUTH_SAD_LEFT => Some("MOUTH_SAD_LEFT"),
            Self::CHEEK_PUFF_RIGHT => Some("CHEEK_PUFF_RIGHT"),
            Self::CHEEK_PUFF_LEFT => Some("CHEEK_PUFF_LEFT"),
            Self::CHEEK_SUCK => Some("CHEEK_SUCK"),
            Self::MOUTH_UPPER_UPRIGHT => Some("MOUTH_UPPER_UPRIGHT"),
            Self::MOUTH_UPPER_UPLEFT => Some("MOUTH_UPPER_UPLEFT"),
            Self::MOUTH_LOWER_DOWNRIGHT => Some("MOUTH_LOWER_DOWNRIGHT"),
            Self::MOUTH_LOWER_DOWNLEFT => Some("MOUTH_LOWER_DOWNLEFT"),
            Self::MOUTH_UPPER_INSIDE => Some("MOUTH_UPPER_INSIDE"),
            Self::MOUTH_LOWER_INSIDE => Some("MOUTH_LOWER_INSIDE"),
            Self::MOUTH_LOWER_OVERLAY => Some("MOUTH_LOWER_OVERLAY"),
            Self::TONGUE_LONGSTEP1 => Some("TONGUE_LONGSTEP1"),
            Self::TONGUE_LEFT => Some("TONGUE_LEFT"),
            Self::TONGUE_RIGHT => Some("TONGUE_RIGHT"),
            Self::TONGUE_UP => Some("TONGUE_UP"),
            Self::TONGUE_DOWN => Some("TONGUE_DOWN"),
            Self::TONGUE_ROLL => Some("TONGUE_ROLL"),
            Self::TONGUE_LONGSTEP2 => Some("TONGUE_LONGSTEP2"),
            Self::TONGUE_UPRIGHT_MORPH => Some("TONGUE_UPRIGHT_MORPH"),
            Self::TONGUE_UPLEFT_MORPH => Some("TONGUE_UPLEFT_MORPH"),
            Self::TONGUE_DOWNRIGHT_MORPH => Some("TONGUE_DOWNRIGHT_MORPH"),
            Self::TONGUE_DOWNLEFT_MORPH => Some("TONGUE_DOWNLEFT_MORPH"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrInstanceCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstanceCreateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InstanceCreateFlags(u64);
impl InstanceCreateFlags {}
bitmask!(InstanceCreateFlags);
#[doc = "See [XrSessionCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionCreateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SessionCreateFlags(u64);
impl SessionCreateFlags {}
bitmask!(SessionCreateFlags);
#[doc = "See [XrSwapchainCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainCreateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainCreateFlags(u64);
impl SwapchainCreateFlags {
    #[doc = "Content will be protected from CPU access"]
    pub const PROTECTED_CONTENT: SwapchainCreateFlags = Self(1 << 0u64);
    #[doc = "Only one image will be acquired from this swapchain over its lifetime"]
    pub const STATIC_IMAGE: SwapchainCreateFlags = Self(1 << 1u64);
}
bitmask!(SwapchainCreateFlags);
#[doc = "See [XrSwapchainUsageFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainUsageFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainUsageFlags(u64);
impl SwapchainUsageFlags {
    #[doc = "Specifies that the image may: be a color rendering target."]
    pub const COLOR_ATTACHMENT: SwapchainUsageFlags = Self(1 << 0u64);
    #[doc = "Specifies that the image may: be a depth/stencil rendering target."]
    pub const DEPTH_STENCIL_ATTACHMENT: SwapchainUsageFlags = Self(1 << 1u64);
    #[doc = "Specifies that the image may: be accessed out of order and that access may: be via atomic operations."]
    pub const UNORDERED_ACCESS: SwapchainUsageFlags = Self(1 << 2u64);
    #[doc = "Specifies that the image may: be used as the source of a transfer operation."]
    pub const TRANSFER_SRC: SwapchainUsageFlags = Self(1 << 3u64);
    #[doc = "Specifies that the image may: be used as the destination of a transfer operation."]
    pub const TRANSFER_DST: SwapchainUsageFlags = Self(1 << 4u64);
    #[doc = "Specifies that the image may: be sampled by a shader."]
    pub const SAMPLED: SwapchainUsageFlags = Self(1 << 5u64);
    #[doc = "Specifies that the image may: be reinterpreted as another image format."]
    pub const MUTABLE_FORMAT: SwapchainUsageFlags = Self(1 << 6u64);
    #[doc = "Specifies that the image may: be used as a input attachment."]
    pub const INPUT_ATTACHMENT: SwapchainUsageFlags = Self(1 << 7u64);
}
bitmask!(SwapchainUsageFlags);
#[doc = "See [XrViewStateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewStateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ViewStateFlags(u64);
impl ViewStateFlags {
    #[doc = "Indicates validity of all XrView orientations"]
    pub const ORIENTATION_VALID: ViewStateFlags = Self(1 << 0u64);
    #[doc = "Indicates validity of all XrView positions"]
    pub const POSITION_VALID: ViewStateFlags = Self(1 << 1u64);
    #[doc = "Indicates whether all XrView orientations are actively tracked"]
    pub const ORIENTATION_TRACKED: ViewStateFlags = Self(1 << 2u64);
    #[doc = "Indicates whether all XrView positions are actively tracked"]
    pub const POSITION_TRACKED: ViewStateFlags = Self(1 << 3u64);
}
bitmask!(ViewStateFlags);
#[doc = "See [XrCompositionLayerFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerFlags(u64);
impl CompositionLayerFlags {
    #[doc = "Enables chromatic aberration correction when not done by default."]
    pub const CORRECT_CHROMATIC_ABERRATION: CompositionLayerFlags = Self(1 << 0u64);
    #[doc = "Enables the layer texture alpha channel."]
    pub const BLEND_TEXTURE_SOURCE_ALPHA: CompositionLayerFlags = Self(1 << 1u64);
    #[doc = "Indicates the texture color channels have not been premultiplied by the texture alpha channel."]
    pub const UNPREMULTIPLIED_ALPHA: CompositionLayerFlags = Self(1 << 2u64);
}
bitmask!(CompositionLayerFlags);
#[doc = "See [XrSpaceLocationFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceLocationFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceLocationFlags(u64);
impl SpaceLocationFlags {
    #[doc = "Indicates that the orientation member contains valid data"]
    pub const ORIENTATION_VALID: SpaceLocationFlags = Self(1 << 0u64);
    #[doc = "Indicates that the position member contains valid data"]
    pub const POSITION_VALID: SpaceLocationFlags = Self(1 << 1u64);
    #[doc = "Indicates whether pose member contains an actively tracked orientation"]
    pub const ORIENTATION_TRACKED: SpaceLocationFlags = Self(1 << 2u64);
    #[doc = "Indicates whether pose member contains an actively tracked position"]
    pub const POSITION_TRACKED: SpaceLocationFlags = Self(1 << 3u64);
}
bitmask!(SpaceLocationFlags);
#[doc = "See [XrSpaceVelocityFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceVelocityFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceVelocityFlags(u64);
impl SpaceVelocityFlags {
    #[doc = "Indicates that the linearVelocity member contains valid data"]
    pub const LINEAR_VALID: SpaceVelocityFlags = Self(1 << 0u64);
    #[doc = "Indicates that the angularVelocity member contains valid data"]
    pub const ANGULAR_VALID: SpaceVelocityFlags = Self(1 << 1u64);
}
bitmask!(SpaceVelocityFlags);
#[doc = "See [XrInputSourceLocalizedNameFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInputSourceLocalizedNameFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InputSourceLocalizedNameFlags(u64);
impl InputSourceLocalizedNameFlags {
    #[doc = "Asks for the part of the string which indicates the top level user path the source represents"]
    pub const USER_PATH: InputSourceLocalizedNameFlags = Self(1 << 0u64);
    #[doc = "Asks for the part of the string which represents the interaction profile of the source"]
    pub const INTERACTION_PROFILE: InputSourceLocalizedNameFlags = Self(1 << 1u64);
    #[doc = "Asks for the part of the string which represents the component on the device which needs to be interacted with"]
    pub const COMPONENT: InputSourceLocalizedNameFlags = Self(1 << 2u64);
}
bitmask!(InputSourceLocalizedNameFlags);
#[doc = "See [XrVulkanInstanceCreateFlagsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanInstanceCreateFlagsKHR)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VulkanInstanceCreateFlagsKHR(u64);
impl VulkanInstanceCreateFlagsKHR {}
bitmask!(VulkanInstanceCreateFlagsKHR);
#[doc = "See [XrVulkanDeviceCreateFlagsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanDeviceCreateFlagsKHR)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VulkanDeviceCreateFlagsKHR(u64);
impl VulkanDeviceCreateFlagsKHR {}
bitmask!(VulkanDeviceCreateFlagsKHR);
#[doc = "See [XrDebugUtilsMessageSeverityFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessageSeverityFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageSeverityFlagsEXT(u64);
impl DebugUtilsMessageSeverityFlagsEXT {
    #[doc = "Most verbose output severity, typically used for debugging."]
    pub const VERBOSE: DebugUtilsMessageSeverityFlagsEXT = Self(1 << 0u64);
    #[doc = "General info message"]
    pub const INFO: DebugUtilsMessageSeverityFlagsEXT = Self(1 << 4u64);
    #[doc = "Indicates the item may be the cause of issues."]
    pub const WARNING: DebugUtilsMessageSeverityFlagsEXT = Self(1 << 8u64);
    #[doc = "Indicates that the item is definitely related to erroneous behavior."]
    pub const ERROR: DebugUtilsMessageSeverityFlagsEXT = Self(1 << 12u64);
}
bitmask!(DebugUtilsMessageSeverityFlagsEXT);
#[doc = "See [XrDebugUtilsMessageTypeFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessageTypeFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageTypeFlagsEXT(u64);
impl DebugUtilsMessageTypeFlagsEXT {
    #[doc = "Indicates this is a general message"]
    pub const GENERAL: DebugUtilsMessageTypeFlagsEXT = Self(1 << 0u64);
    #[doc = "Indicates the message is related to a validation message"]
    pub const VALIDATION: DebugUtilsMessageTypeFlagsEXT = Self(1 << 1u64);
    #[doc = "Indicates the message is related to a potential performance situation"]
    pub const PERFORMANCE: DebugUtilsMessageTypeFlagsEXT = Self(1 << 2u64);
    #[doc = "Indicates the message is related to a non-conformant runtime result"]
    pub const CONFORMANCE: DebugUtilsMessageTypeFlagsEXT = Self(1 << 3u64);
}
bitmask!(DebugUtilsMessageTypeFlagsEXT);
#[doc = "See [XrOverlayMainSessionFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOverlayMainSessionFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlayMainSessionFlagsEXTX(u64);
impl OverlayMainSessionFlagsEXTX {
    #[doc = "Indicates the main session enabled `XR_KHR_composition_layer_depth`"]
    pub const ENABLED_COMPOSITION_LAYER_INFO_DEPTH: OverlayMainSessionFlagsEXTX = Self(1 << 0u64);
}
bitmask!(OverlayMainSessionFlagsEXTX);
#[doc = "See [XrOverlaySessionCreateFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOverlaySessionCreateFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlaySessionCreateFlagsEXTX(u64);
impl OverlaySessionCreateFlagsEXTX {}
bitmask!(OverlaySessionCreateFlagsEXTX);
#[doc = "See [XrAndroidSurfaceSwapchainFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrAndroidSurfaceSwapchainFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AndroidSurfaceSwapchainFlagsFB(u64);
impl AndroidSurfaceSwapchainFlagsFB {
    #[doc = "Create the underlying BufferQueue in synchronous mode"]
    pub const SYNCHRONOUS: AndroidSurfaceSwapchainFlagsFB = Self(1 << 0u64);
    #[doc = "Acquire most recent buffer whose presentation timestamp is not greater than display time of final composited frame"]
    pub const USE_TIMESTAMPS: AndroidSurfaceSwapchainFlagsFB = Self(1 << 1u64);
}
bitmask!(AndroidSurfaceSwapchainFlagsFB);
#[doc = "See [XrCompositionLayerImageLayoutFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerImageLayoutFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerImageLayoutFlagsFB(u64);
impl CompositionLayerImageLayoutFlagsFB {
    #[doc = "The coordinate origin of the swapchain image must be considered to be flipped vertically."]
    pub const VERTICAL_FLIP: CompositionLayerImageLayoutFlagsFB = Self(1 << 0u64);
}
bitmask!(CompositionLayerImageLayoutFlagsFB);
#[doc = "See [XrCompositionLayerSecureContentFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerSecureContentFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerSecureContentFlagsFB(u64);
impl CompositionLayerSecureContentFlagsFB {
    #[doc = "Indicates the layer will only be visible inside the HMD, and not visible to external sources"]
    pub const EXCLUDE_LAYER: CompositionLayerSecureContentFlagsFB = Self(1 << 0u64);
    #[doc = "Indicates the layer will be displayed inside the HMD, but replaced by proxy content when written to external sources"]
    pub const REPLACE_LAYER: CompositionLayerSecureContentFlagsFB = Self(1 << 1u64);
}
bitmask!(CompositionLayerSecureContentFlagsFB);
#[doc = "See [XrSwapchainCreateFoveationFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainCreateFoveationFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainCreateFoveationFlagsFB(u64);
impl SwapchainCreateFoveationFlagsFB {
    #[doc = "Explicitly create the swapchain with scaled bin foveation support. The application must ensure that the swapchain is using the OpenGL graphics API and that the QCOM_texture_foveated extension is supported and enabled."]
    pub const SCALED_BIN: SwapchainCreateFoveationFlagsFB = Self(1 << 0u64);
    #[doc = "Explicitly create the swapchain with fragment density map foveation support. The application must ensure that the swapchain is using the Vulkan graphics API and that the VK_EXT_fragment_density_map extension is supported and enabled."]
    pub const FRAGMENT_DENSITY_MAP: SwapchainCreateFoveationFlagsFB = Self(1 << 1u64);
}
bitmask!(SwapchainCreateFoveationFlagsFB);
#[doc = "See [XrSwapchainStateFoveationFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateFoveationFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainStateFoveationFlagsFB(u64);
impl SwapchainStateFoveationFlagsFB {}
bitmask!(SwapchainStateFoveationFlagsFB);
#[doc = "See [XrTriangleMeshFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrTriangleMeshFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TriangleMeshFlagsFB(u64);
impl TriangleMeshFlagsFB {
    #[doc = "The triangle mesh is mutable (can be modified after it is created)."]
    pub const MUTABLE: TriangleMeshFlagsFB = Self(1 << 0u64);
}
bitmask!(TriangleMeshFlagsFB);
#[doc = "See [XrPassthroughFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughFlagsFB(u64);
impl PassthroughFlagsFB {
    #[doc = "The object (passthrough, layer) is running at creation."]
    pub const IS_RUNNING_AT_CREATION: PassthroughFlagsFB = Self(1 << 0u64);
}
bitmask!(PassthroughFlagsFB);
#[doc = "See [XrPassthroughStateChangedFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughStateChangedFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughStateChangedFlagsFB(u64);
impl PassthroughStateChangedFlagsFB {
    #[doc = "Passthrough system requires reinitialization."]
    pub const REINIT_REQUIRED: PassthroughStateChangedFlagsFB = Self(1 << 0u64);
    #[doc = "Non-recoverable error has occurred. A device reboot or a firmware update may be required."]
    pub const NON_RECOVERABLE_ERROR: PassthroughStateChangedFlagsFB = Self(1 << 1u64);
    #[doc = "A recoverable error has occurred. The runtime will attempt to recover, but some functionality may be temporarily unavailable."]
    pub const RECOVERABLE_ERROR: PassthroughStateChangedFlagsFB = Self(1 << 2u64);
    #[doc = "The runtime has recovered from a previous error and is functioning normally."]
    pub const RESTORED_ERROR: PassthroughStateChangedFlagsFB = Self(1 << 3u64);
}
bitmask!(PassthroughStateChangedFlagsFB);
#[doc = "See [XrHandTrackingAimFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackingAimFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HandTrackingAimFlagsFB(u64);
impl HandTrackingAimFlagsFB {
    #[doc = "Aiming data is computed from additional sources beyond the hand data in the base structure"]
    pub const COMPUTED: HandTrackingAimFlagsFB = Self(1 << 0u64);
    #[doc = "Aiming data is valid"]
    pub const VALID: HandTrackingAimFlagsFB = Self(1 << 1u64);
    #[doc = "Index finger pinch discrete signal"]
    pub const INDEX_PINCHING: HandTrackingAimFlagsFB = Self(1 << 2u64);
    #[doc = "Middle finger pinch discrete signal"]
    pub const MIDDLE_PINCHING: HandTrackingAimFlagsFB = Self(1 << 3u64);
    #[doc = "Ring finger pinch discrete signal"]
    pub const RING_PINCHING: HandTrackingAimFlagsFB = Self(1 << 4u64);
    #[doc = "Little finger pinch discrete signal"]
    pub const LITTLE_PINCHING: HandTrackingAimFlagsFB = Self(1 << 5u64);
    #[doc = "System gesture is active"]
    pub const SYSTEM_GESTURE: HandTrackingAimFlagsFB = Self(1 << 6u64);
    #[doc = "Hand is currently marked as dominant for the system"]
    pub const DOMINANT_HAND: HandTrackingAimFlagsFB = Self(1 << 7u64);
    #[doc = "System menu gesture is active"]
    pub const MENU_PRESSED: HandTrackingAimFlagsFB = Self(1 << 8u64);
}
bitmask!(HandTrackingAimFlagsFB);
#[doc = "See [XrKeyboardTrackingFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrKeyboardTrackingFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyboardTrackingFlagsFB(u64);
impl KeyboardTrackingFlagsFB {
    #[doc = "indicates that the system has a physically tracked keyboard to report.  If not set then no other bits should be considered to be valid or meaningful.  If set either XR_KEYBOARD_TRACKING_LOCAL_BIT_FB or XR_KEYBOARD_TRACKING_REMOTE_BIT_FB must also be set."]
    pub const EXISTS: KeyboardTrackingFlagsFB = Self(1 << 0u64);
    #[doc = "indicates that the physically tracked keyboard is intended to be used in a local pairing with the system.  Mutally exclusive with XR_KEYBOARD_TRACKING_REMOTE_BIT_FB."]
    pub const LOCAL: KeyboardTrackingFlagsFB = Self(1 << 1u64);
    #[doc = "indicates that the physically tracked keyboard is intended to be used while paired to a separate remote computing device. Mutally exclusive with XR_KEYBOARD_TRACKING_LOCAL_BIT_FB."]
    pub const REMOTE: KeyboardTrackingFlagsFB = Self(1 << 2u64);
    #[doc = "indicates that the physically tracked keyboard is actively connected to the headset and capable of sending key data"]
    pub const CONNECTED: KeyboardTrackingFlagsFB = Self(1 << 3u64);
}
bitmask!(KeyboardTrackingFlagsFB);
#[doc = "See [XrKeyboardTrackingQueryFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrKeyboardTrackingQueryFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyboardTrackingQueryFlagsFB(u64);
impl KeyboardTrackingQueryFlagsFB {
    #[doc = "indicates the query is for the physically tracked keyboard that is intended to be used in a local pairing with the System. Mutally exclusive with XR_KEYBOARD_TRACKING_QUERY_REMOTE_BIT_FB."]
    pub const LOCAL: KeyboardTrackingQueryFlagsFB = Self(1 << 1u64);
    #[doc = "indicates the query is for the physically tracked keyboard that may be connected to a separate remote computing device. Mutally exclusive with XR_KEYBOARD_TRACKING_QUERY_LOCAL_BIT_FB."]
    pub const REMOTE: KeyboardTrackingQueryFlagsFB = Self(1 << 2u64);
}
bitmask!(KeyboardTrackingQueryFlagsFB);
#[doc = "See [XrCompositionLayerSpaceWarpInfoFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerSpaceWarpInfoFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerSpaceWarpInfoFlagsFB(u64);
impl CompositionLayerSpaceWarpInfoFlagsFB {}
bitmask!(CompositionLayerSpaceWarpInfoFlagsFB);
#[doc = "See [XrRenderModelFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRenderModelFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RenderModelFlagsFB(u64);
impl RenderModelFlagsFB {}
bitmask!(RenderModelFlagsFB);
#[doc = "See [XrDigitalLensControlFlagsALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDigitalLensControlFlagsALMALENCE)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DigitalLensControlFlagsALMALENCE(u64);
impl DigitalLensControlFlagsALMALENCE {
    #[doc = "disables Digital Lens processing of render textures"]
    pub const PROCESSING_DISABLE: DigitalLensControlFlagsALMALENCE = Self(1 << 0u64);
}
bitmask!(DigitalLensControlFlagsALMALENCE);
#[doc = "See [XrInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstance)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instance(u64);
handle!(Instance);
#[doc = "See [XrSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSession)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Session(u64);
handle!(Session);
#[doc = "See [XrActionSet](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionSet)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ActionSet(u64);
handle!(ActionSet);
#[doc = "See [XrAction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrAction)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Action(u64);
handle!(Action);
#[doc = "See [XrSwapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchain)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Swapchain(u64);
handle!(Swapchain);
#[doc = "See [XrSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpace)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Space(u64);
handle!(Space);
#[doc = "See [XrDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessengerEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessengerEXT(u64);
handle!(DebugUtilsMessengerEXT);
#[doc = "See [XrSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpatialAnchorMSFT(u64);
handle!(SpatialAnchorMSFT);
#[doc = "See [XrHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackerEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct HandTrackerEXT(u64);
handle!(HandTrackerEXT);
#[doc = "See [XrFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveationProfileFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FoveationProfileFB(u64);
handle!(FoveationProfileFB);
#[doc = "See [XrTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrTriangleMeshFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TriangleMeshFB(u64);
handle!(TriangleMeshFB);
#[doc = "See [XrPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughFB(u64);
handle!(PassthroughFB);
#[doc = "See [XrPassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughLayerFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughLayerFB(u64);
handle!(PassthroughLayerFB);
#[doc = "See [XrGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGeometryInstanceFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GeometryInstanceFB(u64);
handle!(GeometryInstanceFB);
#[doc = "See [XrSceneObserverMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneObserverMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SceneObserverMSFT(u64);
handle!(SceneObserverMSFT);
#[doc = "See [XrSceneMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSceneMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SceneMSFT(u64);
handle!(SceneMSFT);
#[doc = "See [XrSpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorStoreConnectionMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpatialAnchorStoreConnectionMSFT(u64);
handle!(SpatialAnchorStoreConnectionMSFT);
#[doc = "See [XrFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFacialTrackerHTC)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FacialTrackerHTC(u64);
handle!(FacialTrackerHTC);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector2f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector2f)"]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector3f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector3f)"]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector4f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector4f)"]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrColor4f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrColor4f)"]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrQuaternionf](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrQuaternionf)"]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrPosef](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPosef)"]
pub struct Posef {
    pub orientation: Quaternionf,
    pub position: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrOffset2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOffset2Df)"]
pub struct Offset2Df {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrExtent2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrExtent2Df)"]
pub struct Extent2Df {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrRect2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRect2Df)"]
pub struct Rect2Df {
    pub offset: Offset2Df,
    pub extent: Extent2Df,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrOffset2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOffset2Di)"]
pub struct Offset2Di {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrExtent2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrExtent2Di)"]
pub struct Extent2Di {
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrRect2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRect2Di)"]
pub struct Rect2Di {
    pub offset: Offset2Di,
    pub extent: Extent2Di,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBaseInStructure](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBaseInStructure)"]
pub struct BaseInStructure {
    pub ty: StructureType,
    pub next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBaseOutStructure](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBaseOutStructure)"]
pub struct BaseOutStructure {
    pub ty: StructureType,
    pub next: *mut BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrApiLayerProperties)"]
pub struct ApiLayerProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub layer_name: [c_char; MAX_API_LAYER_NAME_SIZE],
    pub spec_version: Version,
    pub layer_version: u32,
    pub description: [c_char; MAX_API_LAYER_DESCRIPTION_SIZE],
}
impl ApiLayerProperties {
    pub const TYPE: StructureType = StructureType::API_LAYER_PROPERTIES;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrExtensionProperties)"]
pub struct ExtensionProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub extension_version: u32,
}
impl ExtensionProperties {
    pub const TYPE: StructureType = StructureType::EXTENSION_PROPERTIES;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrApplicationInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrApplicationInfo)"]
pub struct ApplicationInfo {
    pub application_name: [c_char; MAX_APPLICATION_NAME_SIZE],
    pub application_version: u32,
    pub engine_name: [c_char; MAX_ENGINE_NAME_SIZE],
    pub engine_version: u32,
    pub api_version: Version,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInstanceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstanceCreateInfo)"]
pub struct InstanceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: InstanceCreateFlags,
    pub application_info: ApplicationInfo,
    pub enabled_api_layer_count: u32,
    pub enabled_api_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const c_char,
}
impl InstanceCreateInfo {
    pub const TYPE: StructureType = StructureType::INSTANCE_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstanceProperties)"]
pub struct InstanceProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub runtime_version: Version,
    pub runtime_name: [c_char; MAX_RUNTIME_NAME_SIZE],
}
impl InstanceProperties {
    pub const TYPE: StructureType = StructureType::INSTANCE_PROPERTIES;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemGetInfo)"]
pub struct SystemGetInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub form_factor: FormFactor,
}
impl SystemGetInfo {
    pub const TYPE: StructureType = StructureType::SYSTEM_GET_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemProperties)"]
pub struct SystemProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub system_id: SystemId,
    pub vendor_id: u32,
    pub system_name: [c_char; MAX_SYSTEM_NAME_SIZE],
    pub graphics_properties: SystemGraphicsProperties,
    pub tracking_properties: SystemTrackingProperties,
}
impl SystemProperties {
    pub const TYPE: StructureType = StructureType::SYSTEM_PROPERTIES;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSystemGraphicsProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemGraphicsProperties)"]
pub struct SystemGraphicsProperties {
    pub max_swapchain_image_height: u32,
    pub max_swapchain_image_width: u32,
    pub max_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSystemTrackingProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemTrackingProperties)"]
pub struct SystemTrackingProperties {
    pub orientation_tracking: Bool32,
    pub position_tracking: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLWin32KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingOpenGLWin32KHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
#[cfg(windows)]
pub struct GraphicsBindingOpenGLWin32KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub h_dc: HDC,
    pub h_glrc: HGLRC,
}
#[cfg(windows)]
impl GraphicsBindingOpenGLWin32KHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_OPENGL_WIN32_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLXlibKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingOpenGLXlibKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
pub struct GraphicsBindingOpenGLXlibKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub x_display: *mut Display,
    pub visualid: u32,
    pub glx_fb_config: GLXFBConfig,
    pub glx_drawable: GLXDrawable,
    pub glx_context: GLXContext,
}
impl GraphicsBindingOpenGLXlibKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_OPENGL_XLIB_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLXcbKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingOpenGLXcbKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
pub struct GraphicsBindingOpenGLXcbKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub connection: *mut xcb_connection_t,
    pub screen_number: u32,
    pub fbconfigid: xcb_glx_fbconfig_t,
    pub visualid: xcb_visualid_t,
    pub glx_drawable: xcb_glx_drawable_t,
    pub glx_context: xcb_glx_context_t,
}
impl GraphicsBindingOpenGLXcbKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_OPENGL_XCB_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLWaylandKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingOpenGLWaylandKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
pub struct GraphicsBindingOpenGLWaylandKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display: *mut wl_display,
}
impl GraphicsBindingOpenGLWaylandKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_OPENGL_WAYLAND_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D11_enable)"]
#[cfg(windows)]
pub struct GraphicsBindingD3D11KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub device: *mut ID3D11Device,
}
#[cfg(windows)]
impl GraphicsBindingD3D11KHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_D3D11_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D12_enable)"]
#[cfg(windows)]
pub struct GraphicsBindingD3D12KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub device: *mut ID3D12Device,
    pub queue: *mut ID3D12CommandQueue,
}
#[cfg(windows)]
impl GraphicsBindingD3D12KHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_D3D12_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLESAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingOpenGLESAndroidKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_es_enable)"]
#[cfg(target_os = "android")]
pub struct GraphicsBindingOpenGLESAndroidKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
}
#[cfg(target_os = "android")]
impl GraphicsBindingOpenGLESAndroidKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
pub struct GraphicsBindingVulkanKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub instance: VkInstance,
    pub physical_device: VkPhysicalDevice,
    pub device: VkDevice,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
impl GraphicsBindingVulkanKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_VULKAN_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionCreateInfo)"]
pub struct SessionCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: SessionCreateFlags,
    pub system_id: SystemId,
}
impl SessionCreateInfo {
    pub const TYPE: StructureType = StructureType::SESSION_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionBeginInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionBeginInfo)"]
pub struct SessionBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub primary_view_configuration_type: ViewConfigurationType,
}
impl SessionBeginInfo {
    pub const TYPE: StructureType = StructureType::SESSION_BEGIN_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainCreateInfo)"]
pub struct SwapchainCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: SwapchainCreateFlags,
    pub usage_flags: SwapchainUsageFlags,
    pub format: i64,
    pub sample_count: u32,
    pub width: u32,
    pub height: u32,
    pub face_count: u32,
    pub array_size: u32,
    pub mip_count: u32,
}
impl SwapchainCreateInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageBaseHeader)"]
pub struct SwapchainImageBaseHeader {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageOpenGLKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageOpenGLKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
pub struct SwapchainImageOpenGLKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: u32,
}
impl SwapchainImageOpenGLKHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_OPENGL_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageOpenGLESKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageOpenGLESKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_es_enable)"]
pub struct SwapchainImageOpenGLESKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: u32,
}
impl SwapchainImageOpenGLESKHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_OPENGL_ES_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
pub struct SwapchainImageVulkanKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: VkImage,
}
impl SwapchainImageVulkanKHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_VULKAN_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D11_enable)"]
#[cfg(windows)]
pub struct SwapchainImageD3D11KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture: *mut ID3D11Texture2D,
}
#[cfg(windows)]
impl SwapchainImageD3D11KHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_D3D11_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D12_enable)"]
#[cfg(windows)]
pub struct SwapchainImageD3D12KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture: *mut ID3D12Resource,
}
#[cfg(windows)]
impl SwapchainImageD3D12KHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_D3D12_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageAcquireInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageAcquireInfo)"]
pub struct SwapchainImageAcquireInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageAcquireInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_ACQUIRE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageWaitInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageWaitInfo)"]
pub struct SwapchainImageWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub timeout: Duration,
}
impl SwapchainImageWaitInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_WAIT_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageReleaseInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageReleaseInfo)"]
pub struct SwapchainImageReleaseInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageReleaseInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_RELEASE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrReferenceSpaceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrReferenceSpaceCreateInfo)"]
pub struct ReferenceSpaceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub reference_space_type: ReferenceSpaceType,
    pub pose_in_reference_space: Posef,
}
impl ReferenceSpaceCreateInfo {
    pub const TYPE: StructureType = StructureType::REFERENCE_SPACE_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionSpaceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionSpaceCreateInfo)"]
pub struct ActionSpaceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action: Action,
    pub subaction_path: Path,
    pub pose_in_action_space: Posef,
}
impl ActionSpaceCreateInfo {
    pub const TYPE: StructureType = StructureType::ACTION_SPACE_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceLocation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceLocation)"]
pub struct SpaceLocation {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
}
impl SpaceLocation {
    pub const TYPE: StructureType = StructureType::SPACE_LOCATION;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceVelocity](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceVelocity)"]
pub struct SpaceVelocity {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}
impl SpaceVelocity {
    pub const TYPE: StructureType = StructureType::SPACE_VELOCITY;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrFovf](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFovf)"]
pub struct Fovf {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrView](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrView)"]
pub struct View {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub pose: Posef,
    pub fov: Fovf,
}
impl View {
    pub const TYPE: StructureType = StructureType::VIEW;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewLocateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewLocateInfo)"]
pub struct ViewLocateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub display_time: Time,
    pub space: Space,
}
impl ViewLocateInfo {
    pub const TYPE: StructureType = StructureType::VIEW_LOCATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewState](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewState)"]
pub struct ViewState {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_state_flags: ViewStateFlags,
}
impl ViewState {
    pub const TYPE: StructureType = StructureType::VIEW_STATE;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewConfigurationView](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationView)"]
pub struct ViewConfigurationView {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub recommended_image_rect_width: u32,
    pub max_image_rect_width: u32,
    pub recommended_image_rect_height: u32,
    pub max_image_rect_height: u32,
    pub recommended_swapchain_sample_count: u32,
    pub max_swapchain_sample_count: u32,
}
impl ViewConfigurationView {
    pub const TYPE: StructureType = StructureType::VIEW_CONFIGURATION_VIEW;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainSubImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainSubImage)"]
pub struct SwapchainSubImage {
    pub swapchain: Swapchain,
    pub image_rect: Rect2Di,
    pub image_array_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerBaseHeader)"]
pub struct CompositionLayerBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerProjectionView](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerProjectionView)"]
pub struct CompositionLayerProjectionView {
    pub ty: StructureType,
    pub next: *const c_void,
    pub pose: Posef,
    pub fov: Fovf,
    pub sub_image: SwapchainSubImage,
}
impl CompositionLayerProjectionView {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_PROJECTION_VIEW;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerProjection](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerProjection)"]
pub struct CompositionLayerProjection {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub view_count: u32,
    pub views: *const CompositionLayerProjectionView,
}
impl CompositionLayerProjection {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_PROJECTION;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerQuad](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerQuad)"]
pub struct CompositionLayerQuad {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub size: Extent2Df,
}
impl CompositionLayerQuad {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_QUAD;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerCylinderKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerCylinderKHR) - defined by [XR_KHR_composition_layer_cylinder](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_cylinder)"]
pub struct CompositionLayerCylinderKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub radius: f32,
    pub central_angle: f32,
    pub aspect_ratio: f32,
}
impl CompositionLayerCylinderKHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_CYLINDER_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerCubeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerCubeKHR) - defined by [XR_KHR_composition_layer_cube](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_cube)"]
pub struct CompositionLayerCubeKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub swapchain: Swapchain,
    pub image_array_index: u32,
    pub orientation: Quaternionf,
}
impl CompositionLayerCubeKHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_CUBE_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerEquirectKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerEquirectKHR) - defined by [XR_KHR_composition_layer_equirect](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_equirect)"]
pub struct CompositionLayerEquirectKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub radius: f32,
    pub scale: Vector2f,
    pub bias: Vector2f,
}
impl CompositionLayerEquirectKHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_EQUIRECT_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerDepthInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerDepthInfoKHR) - defined by [XR_KHR_composition_layer_depth](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_depth)"]
pub struct CompositionLayerDepthInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub sub_image: SwapchainSubImage,
    pub min_depth: f32,
    pub max_depth: f32,
    pub near_z: f32,
    pub far_z: f32,
}
impl CompositionLayerDepthInfoKHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_DEPTH_INFO_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameBeginInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameBeginInfo)"]
pub struct FrameBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameBeginInfo {
    pub const TYPE: StructureType = StructureType::FRAME_BEGIN_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameEndInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameEndInfo)"]
pub struct FrameEndInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display_time: Time,
    pub environment_blend_mode: EnvironmentBlendMode,
    pub layer_count: u32,
    pub layers: *const *const CompositionLayerBaseHeader,
}
impl FrameEndInfo {
    pub const TYPE: StructureType = StructureType::FRAME_END_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameWaitInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameWaitInfo)"]
pub struct FrameWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameWaitInfo {
    pub const TYPE: StructureType = StructureType::FRAME_WAIT_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameState](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameState)"]
pub struct FrameState {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub predicted_display_time: Time,
    pub predicted_display_period: Duration,
    pub should_render: Bool32,
}
impl FrameState {
    pub const TYPE: StructureType = StructureType::FRAME_STATE;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHapticBaseHeader)"]
pub struct HapticBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticVibration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHapticVibration)"]
pub struct HapticVibration {
    pub ty: StructureType,
    pub next: *const c_void,
    pub duration: Duration,
    pub frequency: f32,
    pub amplitude: f32,
}
impl HapticVibration {
    pub const TYPE: StructureType = StructureType::HAPTIC_VIBRATION;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataBaseHeader)"]
pub struct EventDataBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataBuffer](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataBuffer)"]
pub struct EventDataBuffer {
    pub ty: StructureType,
    pub next: *const c_void,
    pub varying: [u8; 4000usize],
}
impl EventDataBuffer {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_BUFFER;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataEventsLost](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataEventsLost)"]
pub struct EventDataEventsLost {
    pub ty: StructureType,
    pub next: *const c_void,
    pub lost_event_count: u32,
}
impl EventDataEventsLost {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_EVENTS_LOST;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataInstanceLossPending](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataInstanceLossPending)"]
pub struct EventDataInstanceLossPending {
    pub ty: StructureType,
    pub next: *const c_void,
    pub loss_time: Time,
}
impl EventDataInstanceLossPending {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_INSTANCE_LOSS_PENDING;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSessionStateChanged](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataSessionStateChanged)"]
pub struct EventDataSessionStateChanged {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub state: SessionState,
    pub time: Time,
}
impl EventDataSessionStateChanged {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SESSION_STATE_CHANGED;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataReferenceSpaceChangePending](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataReferenceSpaceChangePending)"]
pub struct EventDataReferenceSpaceChangePending {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub reference_space_type: ReferenceSpaceType,
    pub change_time: Time,
    pub pose_valid: Bool32,
    pub pose_in_previous_space: Posef,
}
impl EventDataReferenceSpaceChangePending {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataPerfSettingsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataPerfSettingsEXT) - defined by [XR_EXT_performance_settings](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_performance_settings)"]
pub struct EventDataPerfSettingsEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub domain: PerfSettingsDomainEXT,
    pub sub_domain: PerfSettingsSubDomainEXT,
    pub from_level: PerfSettingsNotificationLevelEXT,
    pub to_level: PerfSettingsNotificationLevelEXT,
}
impl EventDataPerfSettingsEXT {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_PERF_SETTINGS_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVisibilityMaskChangedKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataVisibilityMaskChangedKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_visibility_mask)"]
pub struct EventDataVisibilityMaskChangedKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub view_configuration_type: ViewConfigurationType,
    pub view_index: u32,
}
impl EventDataVisibilityMaskChangedKHR {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewConfigurationProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationProperties)"]
pub struct ViewConfigurationProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub fov_mutable: Bool32,
}
impl ViewConfigurationProperties {
    pub const TYPE: StructureType = StructureType::VIEW_CONFIGURATION_PROPERTIES;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionStateBoolean](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionStateBoolean)"]
pub struct ActionStateBoolean {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: Bool32,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
impl ActionStateBoolean {
    pub const TYPE: StructureType = StructureType::ACTION_STATE_BOOLEAN;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionStateFloat](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionStateFloat)"]
pub struct ActionStateFloat {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: f32,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
impl ActionStateFloat {
    pub const TYPE: StructureType = StructureType::ACTION_STATE_FLOAT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionStateVector2f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionStateVector2f)"]
pub struct ActionStateVector2f {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: Vector2f,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
impl ActionStateVector2f {
    pub const TYPE: StructureType = StructureType::ACTION_STATE_VECTOR2F;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionStatePose](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionStatePose)"]
pub struct ActionStatePose {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
}
impl ActionStatePose {
    pub const TYPE: StructureType = StructureType::ACTION_STATE_POSE;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionStateGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionStateGetInfo)"]
pub struct ActionStateGetInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action: Action,
    pub subaction_path: Path,
}
impl ActionStateGetInfo {
    pub const TYPE: StructureType = StructureType::ACTION_STATE_GET_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticActionInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHapticActionInfo)"]
pub struct HapticActionInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action: Action,
    pub subaction_path: Path,
}
impl HapticActionInfo {
    pub const TYPE: StructureType = StructureType::HAPTIC_ACTION_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionSetCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionSetCreateInfo)"]
pub struct ActionSetCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_set_name: [c_char; MAX_ACTION_SET_NAME_SIZE],
    pub localized_action_set_name: [c_char; MAX_LOCALIZED_ACTION_SET_NAME_SIZE],
    pub priority: u32,
}
impl ActionSetCreateInfo {
    pub const TYPE: StructureType = StructureType::ACTION_SET_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionSuggestedBinding](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionSuggestedBinding)"]
pub struct ActionSuggestedBinding {
    pub action: Action,
    pub binding: Path,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInteractionProfileSuggestedBinding](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInteractionProfileSuggestedBinding)"]
pub struct InteractionProfileSuggestedBinding {
    pub ty: StructureType,
    pub next: *const c_void,
    pub interaction_profile: Path,
    pub count_suggested_bindings: u32,
    pub suggested_bindings: *const ActionSuggestedBinding,
}
impl InteractionProfileSuggestedBinding {
    pub const TYPE: StructureType = StructureType::INTERACTION_PROFILE_SUGGESTED_BINDING;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActiveActionSet](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActiveActionSet)"]
pub struct ActiveActionSet {
    pub action_set: ActionSet,
    pub subaction_path: Path,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionActionSetsAttachInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionActionSetsAttachInfo)"]
pub struct SessionActionSetsAttachInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub count_action_sets: u32,
    pub action_sets: *const ActionSet,
}
impl SessionActionSetsAttachInfo {
    pub const TYPE: StructureType = StructureType::SESSION_ACTION_SETS_ATTACH_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionsSyncInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionsSyncInfo)"]
pub struct ActionsSyncInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub count_active_action_sets: u32,
    pub active_action_sets: *const ActiveActionSet,
}
impl ActionsSyncInfo {
    pub const TYPE: StructureType = StructureType::ACTIONS_SYNC_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBoundSourcesForActionEnumerateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBoundSourcesForActionEnumerateInfo)"]
pub struct BoundSourcesForActionEnumerateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action: Action,
}
impl BoundSourcesForActionEnumerateInfo {
    pub const TYPE: StructureType = StructureType::BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInputSourceLocalizedNameGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInputSourceLocalizedNameGetInfo)"]
pub struct InputSourceLocalizedNameGetInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub source_path: Path,
    pub which_components: InputSourceLocalizedNameFlags,
}
impl InputSourceLocalizedNameGetInfo {
    pub const TYPE: StructureType = StructureType::INPUT_SOURCE_LOCALIZED_NAME_GET_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataInteractionProfileChanged](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataInteractionProfileChanged)"]
pub struct EventDataInteractionProfileChanged {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
}
impl EventDataInteractionProfileChanged {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_INTERACTION_PROFILE_CHANGED;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInteractionProfileState](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInteractionProfileState)"]
pub struct InteractionProfileState {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub interaction_profile: Path,
}
impl InteractionProfileState {
    pub const TYPE: StructureType = StructureType::INTERACTION_PROFILE_STATE;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActionCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionCreateInfo)"]
pub struct ActionCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_name: [c_char; MAX_ACTION_NAME_SIZE],
    pub action_type: ActionType,
    pub count_subaction_paths: u32,
    pub subaction_paths: *const Path,
    pub localized_action_name: [c_char; MAX_LOCALIZED_ACTION_NAME_SIZE],
}
impl ActionCreateInfo {
    pub const TYPE: StructureType = StructureType::ACTION_CREATE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInstanceCreateInfoAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInstanceCreateInfoAndroidKHR) - defined by [XR_KHR_android_create_instance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_android_create_instance)"]
#[cfg(target_os = "android")]
pub struct InstanceCreateInfoAndroidKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub application_vm: *mut c_void,
    pub application_activity: *mut c_void,
}
#[cfg(target_os = "android")]
impl InstanceCreateInfoAndroidKHR {
    pub const TYPE: StructureType = StructureType::INSTANCE_CREATE_INFO_ANDROID_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVulkanSwapchainFormatListCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanSwapchainFormatListCreateInfoKHR) - defined by [XR_KHR_vulkan_swapchain_format_list](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_swapchain_format_list)"]
pub struct VulkanSwapchainFormatListCreateInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_format_count: u32,
    pub view_formats: *const VkFormat,
}
impl VulkanSwapchainFormatListCreateInfoKHR {
    pub const TYPE: StructureType = StructureType::VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsObjectNameInfoEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
pub struct DebugUtilsObjectNameInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub object_name: *const c_char,
}
impl DebugUtilsObjectNameInfoEXT {
    pub const TYPE: StructureType = StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDebugUtilsLabelEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsLabelEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
pub struct DebugUtilsLabelEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub label_name: *const c_char,
}
impl DebugUtilsLabelEXT {
    pub const TYPE: StructureType = StructureType::DEBUG_UTILS_LABEL_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDebugUtilsMessengerCallbackDataEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessengerCallbackDataEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub message_id: *const c_char,
    pub function_name: *const c_char,
    pub message: *const c_char,
    pub object_count: u32,
    pub objects: *mut DebugUtilsObjectNameInfoEXT,
    pub session_label_count: u32,
    pub session_labels: *mut DebugUtilsLabelEXT,
}
impl DebugUtilsMessengerCallbackDataEXT {
    pub const TYPE: StructureType = StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT;
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessengerCreateInfoEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub message_severities: DebugUtilsMessageSeverityFlagsEXT,
    pub message_types: DebugUtilsMessageTypeFlagsEXT,
    pub user_callback: Option<pfn::DebugUtilsMessengerCallbackEXT>,
    pub user_data: *mut c_void,
}
impl DebugUtilsMessengerCreateInfoEXT {
    pub const TYPE: StructureType = StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVisibilityMaskKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVisibilityMaskKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_visibility_mask)"]
pub struct VisibilityMaskKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut Vector2f,
    pub index_capacity_input: u32,
    pub index_count_output: u32,
    pub indices: *mut u32,
}
impl VisibilityMaskKHR {
    pub const TYPE: StructureType = StructureType::VISIBILITY_MASK_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsRequirementsOpenGLKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsRequirementsOpenGLKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
pub struct GraphicsRequirementsOpenGLKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: Version,
    pub max_api_version_supported: Version,
}
impl GraphicsRequirementsOpenGLKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_OPENGL_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsRequirementsOpenGLESKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsRequirementsOpenGLESKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_es_enable)"]
pub struct GraphicsRequirementsOpenGLESKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: Version,
    pub max_api_version_supported: Version,
}
impl GraphicsRequirementsOpenGLESKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsRequirementsVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsRequirementsVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
pub struct GraphicsRequirementsVulkanKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: Version,
    pub max_api_version_supported: Version,
}
impl GraphicsRequirementsVulkanKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_VULKAN_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrGraphicsRequirementsD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsRequirementsD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D11_enable)"]
#[cfg(windows)]
pub struct GraphicsRequirementsD3D11KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub adapter_luid: LUID,
    pub min_feature_level: D3D_FEATURE_LEVEL,
}
#[cfg(windows)]
impl GraphicsRequirementsD3D11KHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_D3D11_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrGraphicsRequirementsD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsRequirementsD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D12_enable)"]
#[cfg(windows)]
pub struct GraphicsRequirementsD3D12KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub adapter_luid: LUID,
    pub min_feature_level: D3D_FEATURE_LEVEL,
}
#[cfg(windows)]
impl GraphicsRequirementsD3D12KHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_D3D12_KHR;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrVulkanInstanceCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanInstanceCreateInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
pub struct VulkanInstanceCreateInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub system_id: SystemId,
    pub create_flags: VulkanInstanceCreateFlagsKHR,
    pub pfn_get_instance_proc_addr: Option<VkGetInstanceProcAddr>,
    pub vulkan_create_info: *const VkInstanceCreateInfo,
    pub vulkan_allocator: *const VkAllocationCallbacks,
}
impl VulkanInstanceCreateInfoKHR {
    pub const TYPE: StructureType = StructureType::VULKAN_INSTANCE_CREATE_INFO_KHR;
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrVulkanDeviceCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanDeviceCreateInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
pub struct VulkanDeviceCreateInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub system_id: SystemId,
    pub create_flags: VulkanDeviceCreateFlagsKHR,
    pub pfn_get_instance_proc_addr: Option<VkGetInstanceProcAddr>,
    pub vulkan_physical_device: VkPhysicalDevice,
    pub vulkan_create_info: *const VkDeviceCreateInfo,
    pub vulkan_allocator: *const VkAllocationCallbacks,
}
impl VulkanDeviceCreateInfoKHR {
    pub const TYPE: StructureType = StructureType::VULKAN_DEVICE_CREATE_INFO_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVulkanGraphicsDeviceGetInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVulkanGraphicsDeviceGetInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
pub struct VulkanGraphicsDeviceGetInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub system_id: SystemId,
    pub vulkan_instance: VkInstance,
}
impl VulkanGraphicsDeviceGetInfoKHR {
    pub const TYPE: StructureType = StructureType::VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionCreateInfoOverlayEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSessionCreateInfoOverlayEXTX) - defined by [XR_EXTX_overlay](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXTX_overlay)"]
pub struct SessionCreateInfoOverlayEXTX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: OverlaySessionCreateFlagsEXTX,
    pub session_layers_placement: u32,
}
impl SessionCreateInfoOverlayEXTX {
    pub const TYPE: StructureType = StructureType::SESSION_CREATE_INFO_OVERLAY_EXTX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataMainSessionVisibilityChangedEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataMainSessionVisibilityChangedEXTX) - defined by [XR_EXTX_overlay](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXTX_overlay)"]
pub struct EventDataMainSessionVisibilityChangedEXTX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub visible: Bool32,
    pub flags: OverlayMainSessionFlagsEXTX,
}
impl EventDataMainSessionVisibilityChangedEXTX {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataDisplayRefreshRateChangedFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataDisplayRefreshRateChangedFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate)"]
pub struct EventDataDisplayRefreshRateChangedFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub from_display_refresh_rate: f32,
    pub to_display_refresh_rate: f32,
}
impl EventDataDisplayRefreshRateChangedFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewConfigurationDepthRangeEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationDepthRangeEXT) - defined by [XR_EXT_view_configuration_depth_range](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_view_configuration_depth_range)"]
pub struct ViewConfigurationDepthRangeEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub recommended_near_z: f32,
    pub min_near_z: f32,
    pub recommended_far_z: f32,
    pub max_far_z: f32,
}
impl ViewConfigurationDepthRangeEXT {
    pub const TYPE: StructureType = StructureType::VIEW_CONFIGURATION_DEPTH_RANGE_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewConfigurationViewFovEPIC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationViewFovEPIC) - defined by [XR_EPIC_view_configuration_fov](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EPIC_view_configuration_fov)"]
pub struct ViewConfigurationViewFovEPIC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub recommended_fov: Fovf,
    pub max_mutable_fov: Fovf,
}
impl ViewConfigurationViewFovEPIC {
    pub const TYPE: StructureType = StructureType::VIEW_CONFIGURATION_VIEW_FOV_EPIC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInteractionProfileAnalogThresholdVALVE](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInteractionProfileAnalogThresholdVALVE) - defined by [XR_VALVE_analog_threshold](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VALVE_analog_threshold)"]
pub struct InteractionProfileAnalogThresholdVALVE {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action: Action,
    pub binding: Path,
    pub on_threshold: f32,
    pub off_threshold: f32,
    pub on_haptic: *const HapticBaseHeader,
    pub off_haptic: *const HapticBaseHeader,
}
impl InteractionProfileAnalogThresholdVALVE {
    pub const TYPE: StructureType = StructureType::INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBindingModificationsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBindingModificationsKHR) - defined by [XR_KHR_binding_modification](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_binding_modification)"]
pub struct BindingModificationsKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub binding_modification_count: u32,
    pub binding_modifications: *const *const BindingModificationBaseHeaderKHR,
}
impl BindingModificationsKHR {
    pub const TYPE: StructureType = StructureType::BINDING_MODIFICATIONS_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBindingModificationBaseHeaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBindingModificationBaseHeaderKHR) - defined by [XR_KHR_binding_modification](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_binding_modification)"]
pub struct BindingModificationBaseHeaderKHR {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemEyeGazeInteractionPropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemEyeGazeInteractionPropertiesEXT) - defined by [XR_EXT_eye_gaze_interaction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_eye_gaze_interaction)"]
pub struct SystemEyeGazeInteractionPropertiesEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_eye_gaze_interaction: Bool32,
}
impl SystemEyeGazeInteractionPropertiesEXT {
    pub const TYPE: StructureType = StructureType::SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEyeGazeSampleTimeEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEyeGazeSampleTimeEXT) - defined by [XR_EXT_eye_gaze_interaction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_eye_gaze_interaction)"]
pub struct EyeGazeSampleTimeEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub time: Time,
}
impl EyeGazeSampleTimeEXT {
    pub const TYPE: StructureType = StructureType::EYE_GAZE_SAMPLE_TIME_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorCreateInfoMSFT)"]
pub struct SpatialAnchorCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub pose: Posef,
    pub time: Time,
}
impl SpatialAnchorCreateInfoMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_ANCHOR_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorSpaceCreateInfoMSFT)"]
pub struct SpatialAnchorSpaceCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub anchor: SpatialAnchorMSFT,
    pub pose_in_anchor_space: Posef,
}
impl SpatialAnchorSpaceCreateInfoMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerImageLayoutFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerImageLayoutFB) - defined by [XR_FB_composition_layer_image_layout](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_composition_layer_image_layout)"]
pub struct CompositionLayerImageLayoutFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub flags: CompositionLayerImageLayoutFlagsFB,
}
impl CompositionLayerImageLayoutFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_IMAGE_LAYOUT_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerAlphaBlendFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerAlphaBlendFB) - defined by [XR_FB_composition_layer_alpha_blend](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_composition_layer_alpha_blend)"]
pub struct CompositionLayerAlphaBlendFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub src_factor_color: BlendFactorFB,
    pub dst_factor_color: BlendFactorFB,
    pub src_factor_alpha: BlendFactorFB,
    pub dst_factor_alpha: BlendFactorFB,
}
impl CompositionLayerAlphaBlendFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_ALPHA_BLEND_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrGraphicsBindingEGLMNDX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGraphicsBindingEGLMNDX) - defined by [XR_MNDX_egl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MNDX_egl_enable)"]
pub struct GraphicsBindingEGLMNDX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub get_proc_address: PFNEGLGETPROCADDRESSPROC,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
}
impl GraphicsBindingEGLMNDX {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_EGL_MNDX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialGraphNodeSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialGraphNodeSpaceCreateInfoMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
pub struct SpatialGraphNodeSpaceCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub node_type: SpatialGraphNodeTypeMSFT,
    pub node_id: [u8; 16usize],
    pub pose: Posef,
}
impl SpatialGraphNodeSpaceCreateInfoMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemHandTrackingPropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemHandTrackingPropertiesEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct SystemHandTrackingPropertiesEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_hand_tracking: Bool32,
}
impl SystemHandTrackingPropertiesEXT {
    pub const TYPE: StructureType = StructureType::SYSTEM_HAND_TRACKING_PROPERTIES_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackerCreateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackerCreateInfoEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandTrackerCreateInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub hand: HandEXT,
    pub hand_joint_set: HandJointSetEXT,
}
impl HandTrackerCreateInfoEXT {
    pub const TYPE: StructureType = StructureType::HAND_TRACKER_CREATE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandJointsLocateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointsLocateInfoEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointsLocateInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
}
impl HandJointsLocateInfoEXT {
    pub const TYPE: StructureType = StructureType::HAND_JOINTS_LOCATE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrHandJointLocationEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointLocationEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointLocationEXT {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
    pub radius: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrHandJointVelocityEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointVelocityEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointVelocityEXT {
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandJointLocationsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointLocationsEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointLocationsEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
    pub joint_count: u32,
    pub joint_locations: *mut HandJointLocationEXT,
}
impl HandJointLocationsEXT {
    pub const TYPE: StructureType = StructureType::HAND_JOINT_LOCATIONS_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandJointVelocitiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointVelocitiesEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointVelocitiesEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub joint_count: u32,
    pub joint_velocities: *mut HandJointVelocityEXT,
}
impl HandJointVelocitiesEXT {
    pub const TYPE: StructureType = StructureType::HAND_JOINT_VELOCITIES_EXT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandJointsMotionRangeInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandJointsMotionRangeInfoEXT) - defined by [XR_EXT_hand_joints_motion_range](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_joints_motion_range)"]
pub struct HandJointsMotionRangeInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub hand_joints_motion_range: HandJointsMotionRangeEXT,
}
impl HandJointsMotionRangeInfoEXT {
    pub const TYPE: StructureType = StructureType::HAND_JOINTS_MOTION_RANGE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshSpaceCreateInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshSpaceCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub hand_pose_type: HandPoseTypeMSFT,
    pub pose_in_hand_mesh_space: Posef,
}
impl HandMeshSpaceCreateInfoMSFT {
    pub const TYPE: StructureType = StructureType::HAND_MESH_SPACE_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshUpdateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshUpdateInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshUpdateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub time: Time,
    pub hand_pose_type: HandPoseTypeMSFT,
}
impl HandMeshUpdateInfoMSFT {
    pub const TYPE: StructureType = StructureType::HAND_MESH_UPDATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
    pub index_buffer_changed: Bool32,
    pub vertex_buffer_changed: Bool32,
    pub index_buffer: HandMeshIndexBufferMSFT,
    pub vertex_buffer: HandMeshVertexBufferMSFT,
}
impl HandMeshMSFT {
    pub const TYPE: StructureType = StructureType::HAND_MESH_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshIndexBufferMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshIndexBufferMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshIndexBufferMSFT {
    pub index_buffer_key: u32,
    pub index_capacity_input: u32,
    pub index_count_output: u32,
    pub indices: *mut u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshVertexBufferMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshVertexBufferMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshVertexBufferMSFT {
    pub vertex_update_time: Time,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut HandMeshVertexMSFT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrHandMeshVertexMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandMeshVertexMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshVertexMSFT {
    pub position: Vector3f,
    pub normal: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemHandTrackingMeshPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemHandTrackingMeshPropertiesMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct SystemHandTrackingMeshPropertiesMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_hand_tracking_mesh: Bool32,
    pub max_hand_mesh_index_count: u32,
    pub max_hand_mesh_vertex_count: u32,
}
impl SystemHandTrackingMeshPropertiesMSFT {
    pub const TYPE: StructureType = StructureType::SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandPoseTypeInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandPoseTypeInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandPoseTypeInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub hand_pose_type: HandPoseTypeMSFT,
}
impl HandPoseTypeInfoMSFT {
    pub const TYPE: StructureType = StructureType::HAND_POSE_TYPE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationSessionBeginInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationSessionBeginInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationSessionBeginInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_count: u32,
    pub enabled_view_configuration_types: *const ViewConfigurationType,
}
impl SecondaryViewConfigurationSessionBeginInfoMSFT {
    pub const TYPE: StructureType =
        StructureType::SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationStateMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationStateMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub active: Bool32,
}
impl SecondaryViewConfigurationStateMSFT {
    pub const TYPE: StructureType = StructureType::SECONDARY_VIEW_CONFIGURATION_STATE_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationFrameStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationFrameStateMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationFrameStateMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_configuration_count: u32,
    pub view_configuration_states: *mut SecondaryViewConfigurationStateMSFT,
}
impl SecondaryViewConfigurationFrameStateMSFT {
    pub const TYPE: StructureType = StructureType::SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationFrameEndInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationFrameEndInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationFrameEndInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_count: u32,
    pub view_configuration_layers_info: *const SecondaryViewConfigurationLayerInfoMSFT,
}
impl SecondaryViewConfigurationFrameEndInfoMSFT {
    pub const TYPE: StructureType = StructureType::SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationLayerInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationLayerInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationLayerInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub environment_blend_mode: EnvironmentBlendMode,
    pub layer_count: u32,
    pub layers: *const *const CompositionLayerBaseHeader,
}
impl SecondaryViewConfigurationLayerInfoMSFT {
    pub const TYPE: StructureType = StructureType::SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSecondaryViewConfigurationSwapchainCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSecondaryViewConfigurationSwapchainCreateInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
pub struct SecondaryViewConfigurationSwapchainCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_type: ViewConfigurationType,
}
impl SecondaryViewConfigurationSwapchainCreateInfoMSFT {
    pub const TYPE: StructureType =
        StructureType::SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHolographicWindowAttachmentMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHolographicWindowAttachmentMSFT) - defined by [XR_MSFT_holographic_window_attachment](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_holographic_window_attachment)"]
#[cfg(windows)]
pub struct HolographicWindowAttachmentMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub holographic_space: *mut IUnknown,
    pub core_window: *mut IUnknown,
}
#[cfg(windows)]
impl HolographicWindowAttachmentMSFT {
    pub const TYPE: StructureType = StructureType::HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrAndroidSurfaceSwapchainCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrAndroidSurfaceSwapchainCreateInfoFB) - defined by [XR_FB_android_surface_swapchain_create](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_android_surface_swapchain_create)"]
#[cfg(target_os = "android")]
pub struct AndroidSurfaceSwapchainCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: AndroidSurfaceSwapchainFlagsFB,
}
#[cfg(target_os = "android")]
impl AndroidSurfaceSwapchainCreateInfoFB {
    pub const TYPE: StructureType = StructureType::ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateBaseHeaderFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateBaseHeaderFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state)"]
pub struct SwapchainStateBaseHeaderFB {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateAndroidSurfaceDimensionsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateAndroidSurfaceDimensionsFB) - defined by [XR_FB_swapchain_update_state_android_surface](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state_android_surface)"]
#[cfg(target_os = "android")]
pub struct SwapchainStateAndroidSurfaceDimensionsFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub width: u32,
    pub height: u32,
}
#[cfg(target_os = "android")]
impl SwapchainStateAndroidSurfaceDimensionsFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateSamplerOpenGLESFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateSamplerOpenGLESFB) - defined by [XR_FB_swapchain_update_state_opengl_es](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state_opengl_es)"]
pub struct SwapchainStateSamplerOpenGLESFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_filter: EGLenum,
    pub mag_filter: EGLenum,
    pub wrap_mode_s: EGLenum,
    pub wrap_mode_t: EGLenum,
    pub swizzle_red: EGLenum,
    pub swizzle_green: EGLenum,
    pub swizzle_blue: EGLenum,
    pub swizzle_alpha: EGLenum,
    pub max_anisotropy: f32,
    pub border_color: Color4f,
}
impl SwapchainStateSamplerOpenGLESFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateSamplerVulkanFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateSamplerVulkanFB) - defined by [XR_FB_swapchain_update_state_vulkan](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state_vulkan)"]
pub struct SwapchainStateSamplerVulkanFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_filter: VkFilter,
    pub mag_filter: VkFilter,
    pub mipmap_mode: VkSamplerMipmapMode,
    pub wrap_mode_s: VkSamplerAddressMode,
    pub wrap_mode_t: VkSamplerAddressMode,
    pub swizzle_red: VkComponentSwizzle,
    pub swizzle_green: VkComponentSwizzle,
    pub swizzle_blue: VkComponentSwizzle,
    pub swizzle_alpha: VkComponentSwizzle,
    pub max_anisotropy: f32,
    pub border_color: Color4f,
}
impl SwapchainStateSamplerVulkanFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_STATE_SAMPLER_VULKAN_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerSecureContentFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerSecureContentFB) - defined by [XR_FB_composition_layer_secure_content](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_composition_layer_secure_content)"]
pub struct CompositionLayerSecureContentFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: CompositionLayerSecureContentFlagsFB,
}
impl CompositionLayerSecureContentFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_SECURE_CONTENT_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLoaderInitInfoBaseHeaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrLoaderInitInfoBaseHeaderKHR) - defined by [XR_KHR_loader_init](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_loader_init)"]
pub struct LoaderInitInfoBaseHeaderKHR {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLoaderInitInfoAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrLoaderInitInfoAndroidKHR) - defined by [XR_KHR_loader_init_android](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_loader_init_android)"]
#[cfg(target_os = "android")]
pub struct LoaderInitInfoAndroidKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub application_vm: *mut c_void,
    pub application_context: *mut c_void,
}
#[cfg(target_os = "android")]
impl LoaderInitInfoAndroidKHR {
    pub const TYPE: StructureType = StructureType::LOADER_INIT_INFO_ANDROID_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerEquirect2KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerEquirect2KHR) - defined by [XR_KHR_composition_layer_equirect2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_equirect2)"]
pub struct CompositionLayerEquirect2KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub radius: f32,
    pub central_horizontal_angle: f32,
    pub upper_vertical_angle: f32,
    pub lower_vertical_angle: f32,
}
impl CompositionLayerEquirect2KHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_EQUIRECT2_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerColorScaleBiasKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerColorScaleBiasKHR) - defined by [XR_KHR_composition_layer_color_scale_bias](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_composition_layer_color_scale_bias)"]
pub struct CompositionLayerColorScaleBiasKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub color_scale: Color4f,
    pub color_bias: Color4f,
}
impl CompositionLayerColorScaleBiasKHR {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrControllerModelKeyStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrControllerModelKeyStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
pub struct ControllerModelKeyStateMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub model_key: ControllerModelKeyMSFT,
}
impl ControllerModelKeyStateMSFT {
    pub const TYPE: StructureType = StructureType::CONTROLLER_MODEL_KEY_STATE_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrControllerModelNodePropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrControllerModelNodePropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
pub struct ControllerModelNodePropertiesMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub parent_node_name: [c_char; MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT],
    pub node_name: [c_char; MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT],
}
impl ControllerModelNodePropertiesMSFT {
    pub const TYPE: StructureType = StructureType::CONTROLLER_MODEL_NODE_PROPERTIES_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrControllerModelPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrControllerModelPropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
pub struct ControllerModelPropertiesMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub node_capacity_input: u32,
    pub node_count_output: u32,
    pub node_properties: *mut ControllerModelNodePropertiesMSFT,
}
impl ControllerModelPropertiesMSFT {
    pub const TYPE: StructureType = StructureType::CONTROLLER_MODEL_PROPERTIES_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrControllerModelNodeStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrControllerModelNodeStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
pub struct ControllerModelNodeStateMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub node_pose: Posef,
}
impl ControllerModelNodeStateMSFT {
    pub const TYPE: StructureType = StructureType::CONTROLLER_MODEL_NODE_STATE_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrControllerModelStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrControllerModelStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
pub struct ControllerModelStateMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub node_capacity_input: u32,
    pub node_count_output: u32,
    pub node_states: *mut ControllerModelNodeStateMSFT,
}
impl ControllerModelStateMSFT {
    pub const TYPE: StructureType = StructureType::CONTROLLER_MODEL_STATE_MSFT;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemColorSpacePropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemColorSpacePropertiesFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_color_space)"]
pub struct SystemColorSpacePropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub color_space: ColorSpaceFB,
}
impl SystemColorSpacePropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_COLOR_SPACE_PROPERTIES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationProfileCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveationProfileCreateInfoFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation)"]
pub struct FoveationProfileCreateInfoFB {
    pub ty: StructureType,
    pub next: *mut c_void,
}
impl FoveationProfileCreateInfoFB {
    pub const TYPE: StructureType = StructureType::FOVEATION_PROFILE_CREATE_INFO_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainCreateInfoFoveationFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainCreateInfoFoveationFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation)"]
pub struct SwapchainCreateInfoFoveationFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub flags: SwapchainCreateFoveationFlagsFB,
}
impl SwapchainCreateInfoFoveationFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_CREATE_INFO_FOVEATION_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateFoveationFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainStateFoveationFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation)"]
pub struct SwapchainStateFoveationFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub flags: SwapchainStateFoveationFlagsFB,
    pub profile: FoveationProfileFB,
}
impl SwapchainStateFoveationFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_STATE_FOVEATION_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageFoveationVulkanFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageFoveationVulkanFB) - defined by [XR_FB_foveation_vulkan](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation_vulkan)"]
pub struct SwapchainImageFoveationVulkanFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: VkImage,
    pub width: u32,
    pub height: u32,
}
impl SwapchainImageFoveationVulkanFB {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationLevelProfileCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveationLevelProfileCreateInfoFB) - defined by [XR_FB_foveation_configuration](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation_configuration)"]
pub struct FoveationLevelProfileCreateInfoFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub level: FoveationLevelFB,
    pub vertical_offset: f32,
    pub dynamic: FoveationDynamicFB,
}
impl FoveationLevelProfileCreateInfoFB {
    pub const TYPE: StructureType = StructureType::FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector4sFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector4sFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
pub struct Vector4sFB {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackingMeshFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
pub struct HandTrackingMeshFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub joint_capacity_input: u32,
    pub joint_count_output: u32,
    pub joint_bind_poses: *mut Posef,
    pub joint_radii: *mut f32,
    pub joint_parents: *mut HandJointEXT,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertex_positions: *mut Vector3f,
    pub vertex_normals: *mut Vector3f,
    pub vertex_u_vs: *mut Vector2f,
    pub vertex_blend_indices: *mut Vector4sFB,
    pub vertex_blend_weights: *mut Vector4f,
    pub index_capacity_input: u32,
    pub index_count_output: u32,
    pub indices: *mut i16,
}
impl HandTrackingMeshFB {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_MESH_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingScaleFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackingScaleFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
pub struct HandTrackingScaleFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub sensor_output: f32,
    pub current_output: f32,
    pub override_hand_scale: Bool32,
    pub override_value_input: f32,
}
impl HandTrackingScaleFB {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_SCALE_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingAimStateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackingAimStateFB) - defined by [XR_FB_hand_tracking_aim](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_aim)"]
pub struct HandTrackingAimStateFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub status: HandTrackingAimFlagsFB,
    pub aim_pose: Posef,
    pub pinch_strength_index: f32,
    pub pinch_strength_middle: f32,
    pub pinch_strength_ring: f32,
    pub pinch_strength_little: f32,
}
impl HandTrackingAimStateFB {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_AIM_STATE_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandCapsuleFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandCapsuleFB) - defined by [XR_FB_hand_tracking_capsules](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_capsules)"]
pub struct HandCapsuleFB {
    pub points: [Vector3f; HAND_TRACKING_CAPSULE_POINT_COUNT_FB],
    pub radius: f32,
    pub joint: HandJointEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingCapsulesStateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHandTrackingCapsulesStateFB) - defined by [XR_FB_hand_tracking_capsules](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_capsules)"]
pub struct HandTrackingCapsulesStateFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub capsules: [HandCapsuleFB; HAND_TRACKING_CAPSULE_COUNT_FB],
}
impl HandTrackingCapsulesStateFB {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_CAPSULES_STATE_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRenderModelPathInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRenderModelPathInfoFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
pub struct RenderModelPathInfoFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub path: Path,
}
impl RenderModelPathInfoFB {
    pub const TYPE: StructureType = StructureType::RENDER_MODEL_PATH_INFO_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
pub struct RenderModelPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub vendor_id: u32,
    pub model_name: [c_char; MAX_RENDER_MODEL_NAME_SIZE_FB],
    pub model_key: RenderModelKeyFB,
    pub model_version: u32,
    pub flags: RenderModelFlagsFB,
}
impl RenderModelPropertiesFB {
    pub const TYPE: StructureType = StructureType::RENDER_MODEL_PROPERTIES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRenderModelBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRenderModelBufferFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
pub struct RenderModelBufferFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub buffer_capacity_input: u32,
    pub buffer_count_output: u32,
    pub buffer: *mut u8,
}
impl RenderModelBufferFB {
    pub const TYPE: StructureType = StructureType::RENDER_MODEL_BUFFER_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRenderModelLoadInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRenderModelLoadInfoFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
pub struct RenderModelLoadInfoFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub model_key: RenderModelKeyFB,
}
impl RenderModelLoadInfoFB {
    pub const TYPE: StructureType = StructureType::RENDER_MODEL_LOAD_INFO_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
pub struct SystemRenderModelPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_render_model_loading: Bool32,
}
impl SystemRenderModelPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_RENDER_MODEL_PROPERTIES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemKeyboardTrackingPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemKeyboardTrackingPropertiesFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
pub struct SystemKeyboardTrackingPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_keyboard_tracking: Bool32,
}
impl SystemKeyboardTrackingPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrKeyboardTrackingDescriptionFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrKeyboardTrackingDescriptionFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
pub struct KeyboardTrackingDescriptionFB {
    pub tracked_keyboard_id: u64,
    pub size: Vector3f,
    pub flags: KeyboardTrackingFlagsFB,
    pub name: [c_char; MAX_KEYBOARD_TRACKING_NAME_SIZE_FB],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrKeyboardSpaceCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrKeyboardSpaceCreateInfoFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
pub struct KeyboardSpaceCreateInfoFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub tracked_keyboard_id: u64,
}
impl KeyboardSpaceCreateInfoFB {
    pub const TYPE: StructureType = StructureType::KEYBOARD_SPACE_CREATE_INFO_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrKeyboardTrackingQueryFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrKeyboardTrackingQueryFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
pub struct KeyboardTrackingQueryFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub flags: KeyboardTrackingQueryFlagsFB,
}
impl KeyboardTrackingQueryFB {
    pub const TYPE: StructureType = StructureType::KEYBOARD_TRACKING_QUERY_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerDepthTestVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerDepthTestVARJO) - defined by [XR_VARJO_composition_layer_depth_test](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_composition_layer_depth_test)"]
pub struct CompositionLayerDepthTestVARJO {
    pub ty: StructureType,
    pub next: *const c_void,
    pub depth_test_range_near_z: f32,
    pub depth_test_range_far_z: f32,
}
impl CompositionLayerDepthTestVARJO {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_DEPTH_TEST_VARJO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViewLocateFoveatedRenderingVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewLocateFoveatedRenderingVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_foveated_rendering)"]
pub struct ViewLocateFoveatedRenderingVARJO {
    pub ty: StructureType,
    pub next: *const c_void,
    pub foveated_rendering_active: Bool32,
}
impl ViewLocateFoveatedRenderingVARJO {
    pub const TYPE: StructureType = StructureType::VIEW_LOCATE_FOVEATED_RENDERING_VARJO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveatedViewConfigurationViewVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFoveatedViewConfigurationViewVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_foveated_rendering)"]
pub struct FoveatedViewConfigurationViewVARJO {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub foveated_rendering_active: Bool32,
}
impl FoveatedViewConfigurationViewVARJO {
    pub const TYPE: StructureType = StructureType::FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemFoveatedRenderingPropertiesVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemFoveatedRenderingPropertiesVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_foveated_rendering)"]
pub struct SystemFoveatedRenderingPropertiesVARJO {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_foveated_rendering: Bool32,
}
impl SystemFoveatedRenderingPropertiesVARJO {
    pub const TYPE: StructureType = StructureType::SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerReprojectionInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerReprojectionInfoMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
pub struct CompositionLayerReprojectionInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub reprojection_mode: ReprojectionModeMSFT,
}
impl CompositionLayerReprojectionInfoMSFT {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_REPROJECTION_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerReprojectionPlaneOverrideMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerReprojectionPlaneOverrideMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
pub struct CompositionLayerReprojectionPlaneOverrideMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub position: Vector3f,
    pub normal: Vector3f,
    pub velocity: Vector3f,
}
impl CompositionLayerReprojectionPlaneOverrideMSFT {
    pub const TYPE: StructureType =
        StructureType::COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrTriangleMeshCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrTriangleMeshCreateInfoFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
pub struct TriangleMeshCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: TriangleMeshFlagsFB,
    pub winding_order: WindingOrderFB,
    pub vertex_count: u32,
    pub vertex_buffer: *const Vector3f,
    pub triangle_count: u32,
    pub index_buffer: *const u32,
}
impl TriangleMeshCreateInfoFB {
    pub const TYPE: StructureType = StructureType::TRIANGLE_MESH_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemPassthroughPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemPassthroughPropertiesFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct SystemPassthroughPropertiesFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub supports_passthrough: Bool32,
}
impl SystemPassthroughPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_PASSTHROUGH_PROPERTIES_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: PassthroughFlagsFB,
}
impl PassthroughCreateInfoFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughLayerCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughLayerCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughLayerCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub passthrough: PassthroughFB,
    pub flags: PassthroughFlagsFB,
    pub purpose: PassthroughLayerPurposeFB,
}
impl PassthroughLayerCreateInfoFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_LAYER_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerPassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct CompositionLayerPassthroughFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: CompositionLayerFlags,
    pub space: Space,
    pub layer_handle: PassthroughLayerFB,
}
impl CompositionLayerPassthroughFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_PASSTHROUGH_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGeometryInstanceCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGeometryInstanceCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct GeometryInstanceCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer: PassthroughLayerFB,
    pub mesh: TriangleMeshFB,
    pub base_space: Space,
    pub pose: Posef,
    pub scale: Vector3f,
}
impl GeometryInstanceCreateInfoFB {
    pub const TYPE: StructureType = StructureType::GEOMETRY_INSTANCE_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGeometryInstanceTransformFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrGeometryInstanceTransformFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct GeometryInstanceTransformFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
    pub pose: Posef,
    pub scale: Vector3f,
}
impl GeometryInstanceTransformFB {
    pub const TYPE: StructureType = StructureType::GEOMETRY_INSTANCE_TRANSFORM_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughStyleFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughStyleFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughStyleFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub texture_opacity_factor: f32,
    pub edge_color: Color4f,
}
impl PassthroughStyleFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_STYLE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorMapMonoToRgbaFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughColorMapMonoToRgbaFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughColorMapMonoToRgbaFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub texture_color_map: [Color4f; PASSTHROUGH_COLOR_MAP_MONO_SIZE_FB],
}
impl PassthroughColorMapMonoToRgbaFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorMapMonoToMonoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughColorMapMonoToMonoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughColorMapMonoToMonoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub texture_color_map: [u8; PASSTHROUGH_COLOR_MAP_MONO_SIZE_FB],
}
impl PassthroughColorMapMonoToMonoFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataPassthroughStateChangedFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataPassthroughStateChangedFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
pub struct EventDataPassthroughStateChangedFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: PassthroughStateChangedFlagsFB,
}
impl EventDataPassthroughStateChangedFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughKeyboardHandsIntensityFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughKeyboardHandsIntensityFB) - defined by [XR_FB_passthrough_keyboard_hands](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough_keyboard_hands)"]
pub struct PassthroughKeyboardHandsIntensityFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub left_hand_intensity: f32,
    pub right_hand_intensity: f32,
}
impl PassthroughKeyboardHandsIntensityFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorPersistenceNameMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorPersistenceNameMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
pub struct SpatialAnchorPersistenceNameMSFT {
    pub name: [c_char; MAX_SPATIAL_ANCHOR_NAME_SIZE_MSFT],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorPersistenceInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorPersistenceInfoMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
pub struct SpatialAnchorPersistenceInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub spatial_anchor_persistence_name: SpatialAnchorPersistenceNameMSFT,
    pub spatial_anchor: SpatialAnchorMSFT,
}
impl SpatialAnchorPersistenceInfoMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorFromPersistedAnchorCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpatialAnchorFromPersistedAnchorCreateInfoMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
pub struct SpatialAnchorFromPersistedAnchorCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
    pub spatial_anchor_persistence_name: SpatialAnchorPersistenceNameMSFT,
}
impl SpatialAnchorFromPersistedAnchorCreateInfoMSFT {
    pub const TYPE: StructureType =
        StructureType::SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFacialTrackerCreateInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFacialTrackerCreateInfoHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
pub struct FacialTrackerCreateInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub facial_tracking_type: FacialTrackingTypeHTC,
}
impl FacialTrackerCreateInfoHTC {
    pub const TYPE: StructureType = StructureType::FACIAL_TRACKER_CREATE_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemFacialTrackingPropertiesHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemFacialTrackingPropertiesHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
pub struct SystemFacialTrackingPropertiesHTC {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub support_eye_facial_tracking: Bool32,
    pub support_lip_facial_tracking: Bool32,
}
impl SystemFacialTrackingPropertiesHTC {
    pub const TYPE: StructureType = StructureType::SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFacialExpressionsHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFacialExpressionsHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
pub struct FacialExpressionsHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub is_active: Bool32,
    pub sample_time: Time,
    pub expression_count: u32,
    pub expression_weightings: *mut f32,
}
impl FacialExpressionsHTC {
    pub const TYPE: StructureType = StructureType::FACIAL_EXPRESSIONS_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrViveTrackerPathsHTCX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViveTrackerPathsHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
pub struct ViveTrackerPathsHTCX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub persistent_path: Path,
    pub role_path: Path,
}
impl ViveTrackerPathsHTCX {
    pub const TYPE: StructureType = StructureType::VIVE_TRACKER_PATHS_HTCX;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataViveTrackerConnectedHTCX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataViveTrackerConnectedHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
pub struct EventDataViveTrackerConnectedHTCX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub paths: *mut ViveTrackerPathsHTCX,
}
impl EventDataViveTrackerConnectedHTCX {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerSpaceWarpInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerSpaceWarpInfoFB) - defined by [XR_FB_space_warp](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_space_warp)"]
pub struct CompositionLayerSpaceWarpInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerSpaceWarpInfoFlagsFB,
    pub motion_vector_sub_image: SwapchainSubImage,
    pub app_space_delta_pose: Posef,
    pub depth_sub_image: SwapchainSubImage,
    pub min_depth: f32,
    pub max_depth: f32,
    pub near_z: f32,
    pub far_z: f32,
}
impl CompositionLayerSpaceWarpInfoFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_SPACE_WARP_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemSpaceWarpPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemSpaceWarpPropertiesFB) - defined by [XR_FB_space_warp](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_space_warp)"]
pub struct SystemSpaceWarpPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub recommended_motion_vector_image_rect_width: u32,
    pub recommended_motion_vector_image_rect_height: u32,
}
impl SystemSpaceWarpPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_SPACE_WARP_PROPERTIES_FB;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemMarkerTrackingPropertiesVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemMarkerTrackingPropertiesVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
pub struct SystemMarkerTrackingPropertiesVARJO {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_marker_tracking: Bool32,
}
impl SystemMarkerTrackingPropertiesVARJO {
    pub const TYPE: StructureType = StructureType::SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO;
    #[doc = r" Construct a partially-initialized value suitable for passing to OpenXR"]
    #[inline]
    pub fn out(next: *mut BaseOutStructure) -> MaybeUninit<Self> {
        let mut x = MaybeUninit::<Self>::uninit();
        unsafe {
            (x.as_mut_ptr() as *mut BaseOutStructure).write(BaseOutStructure {
                ty: Self::TYPE,
                next,
            });
        }
        x
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataMarkerTrackingUpdateVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataMarkerTrackingUpdateVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
pub struct EventDataMarkerTrackingUpdateVARJO {
    pub ty: StructureType,
    pub next: *const c_void,
    pub marker_id: u64,
    pub is_active: Bool32,
    pub is_predicted: Bool32,
    pub time: Time,
}
impl EventDataMarkerTrackingUpdateVARJO {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerSpaceCreateInfoVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrMarkerSpaceCreateInfoVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
pub struct MarkerSpaceCreateInfoVARJO {
    pub ty: StructureType,
    pub next: *const c_void,
    pub marker_id: u64,
    pub pose_in_marker_space: Posef,
}
impl MarkerSpaceCreateInfoVARJO {
    pub const TYPE: StructureType = StructureType::MARKER_SPACE_CREATE_INFO_VARJO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrUuidEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrUuidEXT) - defined by [XR_EXT_uuid](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_uuid)"]
pub struct UuidEXT {
    pub data: [u8; UUID_SIZE_EXT],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDigitalLensControlALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDigitalLensControlALMALENCE)"]
pub struct DigitalLensControlALMALENCE {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: DigitalLensControlFlagsALMALENCE,
}
impl DigitalLensControlALMALENCE {
    pub const TYPE: StructureType = StructureType::DIGITAL_LENS_CONTROL_ALMALENCE;
}
#[doc = r" Function pointer prototypes"]
pub mod pfn {
    use super::*;
    pub type VoidFunction = unsafe extern "system" fn();
    pub type DebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
        DebugUtilsMessageSeverityFlagsEXT,
        DebugUtilsMessageTypeFlagsEXT,
        *const DebugUtilsMessengerCallbackDataEXT,
        *mut c_void,
    ) -> Bool32;
    #[doc = "See [xrGetInstanceProcAddr](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProcAddr)"]
    pub type GetInstanceProcAddr = unsafe extern "system" fn(
        instance: Instance,
        name: *const c_char,
        function: *mut Option<pfn::VoidFunction>,
    ) -> Result;
    #[doc = "See [xrEnumerateApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties)"]
    pub type EnumerateApiLayerProperties = unsafe extern "system" fn(
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ApiLayerProperties,
    ) -> Result;
    #[doc = "See [xrEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties)"]
    pub type EnumerateInstanceExtensionProperties = unsafe extern "system" fn(
        layer_name: *const c_char,
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result;
    #[doc = "See [xrCreateInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance)"]
    pub type CreateInstance = unsafe extern "system" fn(
        create_info: *const InstanceCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    #[doc = "See [xrDestroyInstance](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance)"]
    pub type DestroyInstance = unsafe extern "system" fn(instance: Instance) -> Result;
    #[doc = "See [xrResultToString](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrResultToString)"]
    pub type ResultToString =
        unsafe extern "system" fn(instance: Instance, value: Result, buffer: *mut c_char) -> Result;
    #[doc = "See [xrStructureTypeToString](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStructureTypeToString)"]
    pub type StructureTypeToString = unsafe extern "system" fn(
        instance: Instance,
        value: StructureType,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties)"]
    pub type GetInstanceProperties = unsafe extern "system" fn(
        instance: Instance,
        instance_properties: *mut InstanceProperties,
    ) -> Result;
    #[doc = "See [xrGetSystem](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem)"]
    pub type GetSystem = unsafe extern "system" fn(
        instance: Instance,
        get_info: *const SystemGetInfo,
        system_id: *mut SystemId,
    ) -> Result;
    #[doc = "See [xrGetSystemProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties)"]
    pub type GetSystemProperties = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        properties: *mut SystemProperties,
    ) -> Result;
    #[doc = "See [xrCreateSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession)"]
    pub type CreateSession = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const SessionCreateInfo,
        session: *mut Session,
    ) -> Result;
    #[doc = "See [xrDestroySession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession)"]
    pub type DestroySession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrDestroySpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpace)"]
    pub type DestroySpace = unsafe extern "system" fn(space: Space) -> Result;
    #[doc = "See [xrEnumerateSwapchainFormats](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainFormats)"]
    pub type EnumerateSwapchainFormats = unsafe extern "system" fn(
        session: Session,
        format_capacity_input: u32,
        format_count_output: *mut u32,
        formats: *mut i64,
    ) -> Result;
    #[doc = "See [xrCreateSwapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSwapchain)"]
    pub type CreateSwapchain = unsafe extern "system" fn(
        session: Session,
        create_info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
    ) -> Result;
    #[doc = "See [xrDestroySwapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySwapchain)"]
    pub type DestroySwapchain = unsafe extern "system" fn(swapchain: Swapchain) -> Result;
    #[doc = "See [xrEnumerateSwapchainImages](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainImages)"]
    pub type EnumerateSwapchainImages = unsafe extern "system" fn(
        swapchain: Swapchain,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    ) -> Result;
    #[doc = "See [xrAcquireSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAcquireSwapchainImage)"]
    pub type AcquireSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        acquire_info: *const SwapchainImageAcquireInfo,
        index: *mut u32,
    ) -> Result;
    #[doc = "See [xrWaitSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitSwapchainImage)"]
    pub type WaitSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        wait_info: *const SwapchainImageWaitInfo,
    ) -> Result;
    #[doc = "See [xrReleaseSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrReleaseSwapchainImage)"]
    pub type ReleaseSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        release_info: *const SwapchainImageReleaseInfo,
    ) -> Result;
    #[doc = "See [xrBeginSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginSession)"]
    pub type BeginSession =
        unsafe extern "system" fn(session: Session, begin_info: *const SessionBeginInfo) -> Result;
    #[doc = "See [xrEndSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndSession)"]
    pub type EndSession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrRequestExitSession](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestExitSession)"]
    pub type RequestExitSession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrEnumerateReferenceSpaces](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateReferenceSpaces)"]
    pub type EnumerateReferenceSpaces = unsafe extern "system" fn(
        session: Session,
        space_capacity_input: u32,
        space_count_output: *mut u32,
        spaces: *mut ReferenceSpaceType,
    ) -> Result;
    #[doc = "See [xrCreateReferenceSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateReferenceSpace)"]
    pub type CreateReferenceSpace = unsafe extern "system" fn(
        session: Session,
        create_info: *const ReferenceSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrCreateActionSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSpace)"]
    pub type CreateActionSpace = unsafe extern "system" fn(
        session: Session,
        create_info: *const ActionSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrLocateSpace](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateSpace)"]
    pub type LocateSpace = unsafe extern "system" fn(
        space: Space,
        base_space: Space,
        time: Time,
        location: *mut SpaceLocation,
    ) -> Result;
    #[doc = "See [xrEnumerateViewConfigurations](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurations)"]
    pub type EnumerateViewConfigurations = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type_capacity_input: u32,
        view_configuration_type_count_output: *mut u32,
        view_configuration_types: *mut ViewConfigurationType,
    ) -> Result;
    #[doc = "See [xrEnumerateEnvironmentBlendModes](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateEnvironmentBlendModes)"]
    pub type EnumerateEnvironmentBlendModes = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        environment_blend_mode_capacity_input: u32,
        environment_blend_mode_count_output: *mut u32,
        environment_blend_modes: *mut EnvironmentBlendMode,
    ) -> Result;
    #[doc = "See [xrGetViewConfigurationProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetViewConfigurationProperties)"]
    pub type GetViewConfigurationProperties = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        configuration_properties: *mut ViewConfigurationProperties,
    ) -> Result;
    #[doc = "See [xrEnumerateViewConfigurationViews](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurationViews)"]
    pub type EnumerateViewConfigurationViews = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut ViewConfigurationView,
    ) -> Result;
    #[doc = "See [xrBeginFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginFrame)"]
    pub type BeginFrame = unsafe extern "system" fn(
        session: Session,
        frame_begin_info: *const FrameBeginInfo,
    ) -> Result;
    #[doc = "See [xrLocateViews](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateViews)"]
    pub type LocateViews = unsafe extern "system" fn(
        session: Session,
        view_locate_info: *const ViewLocateInfo,
        view_state: *mut ViewState,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut View,
    ) -> Result;
    #[doc = "See [xrEndFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndFrame)"]
    pub type EndFrame =
        unsafe extern "system" fn(session: Session, frame_end_info: *const FrameEndInfo) -> Result;
    #[doc = "See [xrWaitFrame](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitFrame)"]
    pub type WaitFrame = unsafe extern "system" fn(
        session: Session,
        frame_wait_info: *const FrameWaitInfo,
        frame_state: *mut FrameState,
    ) -> Result;
    #[doc = "See [xrApplyHapticFeedback](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrApplyHapticFeedback)"]
    pub type ApplyHapticFeedback = unsafe extern "system" fn(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
        haptic_feedback: *const HapticBaseHeader,
    ) -> Result;
    #[doc = "See [xrStopHapticFeedback](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStopHapticFeedback)"]
    pub type StopHapticFeedback = unsafe extern "system" fn(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
    ) -> Result;
    #[doc = "See [xrPollEvent](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPollEvent)"]
    pub type PollEvent =
        unsafe extern "system" fn(instance: Instance, event_data: *mut EventDataBuffer) -> Result;
    #[doc = "See [xrStringToPath](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStringToPath)"]
    pub type StringToPath = unsafe extern "system" fn(
        instance: Instance,
        path_string: *const c_char,
        path: *mut Path,
    ) -> Result;
    #[doc = "See [xrPathToString](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPathToString)"]
    pub type PathToString = unsafe extern "system" fn(
        instance: Instance,
        path: Path,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetReferenceSpaceBoundsRect](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetReferenceSpaceBoundsRect)"]
    pub type GetReferenceSpaceBoundsRect = unsafe extern "system" fn(
        session: Session,
        reference_space_type: ReferenceSpaceType,
        bounds: *mut Extent2Df,
    ) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "See [xrSetAndroidApplicationThreadKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetAndroidApplicationThreadKHR) - defined by [XR_KHR_android_thread_settings](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_android_thread_settings)"]
    pub type SetAndroidApplicationThreadKHR = unsafe extern "system" fn(
        session: Session,
        thread_type: AndroidThreadTypeKHR,
        thread_id: u32,
    ) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "See [xrCreateSwapchainAndroidSurfaceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSwapchainAndroidSurfaceKHR) - defined by [XR_KHR_android_surface_swapchain](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_android_surface_swapchain)"]
    pub type CreateSwapchainAndroidSurfaceKHR = unsafe extern "system" fn(
        session: Session,
        info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
        surface: *mut jobject,
    ) -> Result;
    #[doc = "See [xrGetActionStateBoolean](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateBoolean)"]
    pub type GetActionStateBoolean = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateBoolean,
    ) -> Result;
    #[doc = "See [xrGetActionStateFloat](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateFloat)"]
    pub type GetActionStateFloat = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateFloat,
    ) -> Result;
    #[doc = "See [xrGetActionStateVector2f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateVector2f)"]
    pub type GetActionStateVector2f = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateVector2f,
    ) -> Result;
    #[doc = "See [xrGetActionStatePose](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStatePose)"]
    pub type GetActionStatePose = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStatePose,
    ) -> Result;
    #[doc = "See [xrCreateActionSet](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSet)"]
    pub type CreateActionSet = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const ActionSetCreateInfo,
        action_set: *mut ActionSet,
    ) -> Result;
    #[doc = "See [xrDestroyActionSet](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyActionSet)"]
    pub type DestroyActionSet = unsafe extern "system" fn(action_set: ActionSet) -> Result;
    #[doc = "See [xrCreateAction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateAction)"]
    pub type CreateAction = unsafe extern "system" fn(
        action_set: ActionSet,
        create_info: *const ActionCreateInfo,
        action: *mut Action,
    ) -> Result;
    #[doc = "See [xrDestroyAction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyAction)"]
    pub type DestroyAction = unsafe extern "system" fn(action: Action) -> Result;
    #[doc = "See [xrSuggestInteractionProfileBindings](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSuggestInteractionProfileBindings)"]
    pub type SuggestInteractionProfileBindings = unsafe extern "system" fn(
        instance: Instance,
        suggested_bindings: *const InteractionProfileSuggestedBinding,
    ) -> Result;
    #[doc = "See [xrAttachSessionActionSets](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAttachSessionActionSets)"]
    pub type AttachSessionActionSets = unsafe extern "system" fn(
        session: Session,
        attach_info: *const SessionActionSetsAttachInfo,
    ) -> Result;
    #[doc = "See [xrGetCurrentInteractionProfile](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetCurrentInteractionProfile)"]
    pub type GetCurrentInteractionProfile = unsafe extern "system" fn(
        session: Session,
        top_level_user_path: Path,
        interaction_profile: *mut InteractionProfileState,
    ) -> Result;
    #[doc = "See [xrSyncActions](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSyncActions)"]
    pub type SyncActions =
        unsafe extern "system" fn(session: Session, sync_info: *const ActionsSyncInfo) -> Result;
    #[doc = "See [xrEnumerateBoundSourcesForAction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateBoundSourcesForAction)"]
    pub type EnumerateBoundSourcesForAction = unsafe extern "system" fn(
        session: Session,
        enumerate_info: *const BoundSourcesForActionEnumerateInfo,
        source_capacity_input: u32,
        source_count_output: *mut u32,
        sources: *mut Path,
    ) -> Result;
    #[doc = "See [xrGetInputSourceLocalizedName](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInputSourceLocalizedName)"]
    pub type GetInputSourceLocalizedName = unsafe extern "system" fn(
        session: Session,
        get_info: *const InputSourceLocalizedNameGetInfo,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanInstanceExtensionsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanInstanceExtensionsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanInstanceExtensionsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanDeviceExtensionsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanDeviceExtensionsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanDeviceExtensionsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsDeviceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsDeviceKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsDeviceKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        vk_instance: VkInstance,
        vk_physical_device: *mut VkPhysicalDevice,
    ) -> Result;
    #[doc = "See [xrGetOpenGLGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetOpenGLGraphicsRequirementsKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_enable)"]
    pub type GetOpenGLGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLKHR,
    ) -> Result;
    #[doc = "See [xrGetOpenGLESGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetOpenGLESGraphicsRequirementsKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_opengl_es_enable)"]
    pub type GetOpenGLESGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLESKHR,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsRequirementsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsVulkanKHR,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrGetD3D11GraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetD3D11GraphicsRequirementsKHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D11_enable)"]
    pub type GetD3D11GraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D11KHR,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrGetD3D12GraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetD3D12GraphicsRequirementsKHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_D3D12_enable)"]
    pub type GetD3D12GraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D12KHR,
    ) -> Result;
    #[doc = "See [xrPerfSettingsSetPerformanceLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPerfSettingsSetPerformanceLevelEXT) - defined by [XR_EXT_performance_settings](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_performance_settings)"]
    pub type PerfSettingsSetPerformanceLevelEXT = unsafe extern "system" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        level: PerfSettingsLevelEXT,
    ) -> Result;
    #[doc = "See [xrThermalGetTemperatureTrendEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrThermalGetTemperatureTrendEXT) - defined by [XR_EXT_thermal_query](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_thermal_query)"]
    pub type ThermalGetTemperatureTrendEXT = unsafe extern "system" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        notification_level: *mut PerfSettingsNotificationLevelEXT,
        temp_headroom: *mut f32,
        temp_slope: *mut f32,
    ) -> Result;
    #[doc = "See [xrSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetDebugUtilsObjectNameEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
        instance: Instance,
        name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> Result;
    #[doc = "See [xrCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateDebugUtilsMessengerEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type CreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const DebugUtilsMessengerCreateInfoEXT,
        messenger: *mut DebugUtilsMessengerEXT,
    ) -> Result;
    #[doc = "See [xrDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyDebugUtilsMessengerEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type DestroyDebugUtilsMessengerEXT =
        unsafe extern "system" fn(messenger: DebugUtilsMessengerEXT) -> Result;
    #[doc = "See [xrSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSubmitDebugUtilsMessageEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) -> Result;
    #[doc = "See [xrSessionBeginDebugUtilsLabelRegionEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSessionBeginDebugUtilsLabelRegionEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionBeginDebugUtilsLabelRegionEXT = unsafe extern "system" fn(
        session: Session,
        label_info: *const DebugUtilsLabelEXT,
    ) -> Result;
    #[doc = "See [xrSessionEndDebugUtilsLabelRegionEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSessionEndDebugUtilsLabelRegionEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionEndDebugUtilsLabelRegionEXT =
        unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrSessionInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSessionInsertDebugUtilsLabelEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
        session: Session,
        label_info: *const DebugUtilsLabelEXT,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrConvertTimeToWin32PerformanceCounterKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrConvertTimeToWin32PerformanceCounterKHR) - defined by [XR_KHR_win32_convert_performance_counter_time](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_win32_convert_performance_counter_time)"]
    pub type ConvertTimeToWin32PerformanceCounterKHR = unsafe extern "system" fn(
        instance: Instance,
        time: Time,
        performance_counter: *mut LARGE_INTEGER,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrConvertWin32PerformanceCounterToTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrConvertWin32PerformanceCounterToTimeKHR) - defined by [XR_KHR_win32_convert_performance_counter_time](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_win32_convert_performance_counter_time)"]
    pub type ConvertWin32PerformanceCounterToTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        performance_counter: *const LARGE_INTEGER,
        time: *mut Time,
    ) -> Result;
    #[doc = "See [xrCreateVulkanInstanceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateVulkanInstanceKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type CreateVulkanInstanceKHR = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const VulkanInstanceCreateInfoKHR,
        vulkan_instance: *mut VkInstance,
        vulkan_result: *mut VkResult,
    ) -> Result;
    #[doc = "See [xrCreateVulkanDeviceKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateVulkanDeviceKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type CreateVulkanDeviceKHR = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const VulkanDeviceCreateInfoKHR,
        vulkan_device: *mut VkDevice,
        vulkan_result: *mut VkResult,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsDevice2KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsDevice2KHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type GetVulkanGraphicsDevice2KHR = unsafe extern "system" fn(
        instance: Instance,
        get_info: *const VulkanGraphicsDeviceGetInfoKHR,
        vulkan_physical_device: *mut VkPhysicalDevice,
    ) -> Result;
    #[doc = "See [xrConvertTimeToTimespecTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrConvertTimeToTimespecTimeKHR) - defined by [XR_KHR_convert_timespec_time](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_convert_timespec_time)"]
    pub type ConvertTimeToTimespecTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        time: Time,
        timespec_time: *mut timespec,
    ) -> Result;
    #[doc = "See [xrConvertTimespecTimeToTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrConvertTimespecTimeToTimeKHR) - defined by [XR_KHR_convert_timespec_time](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_convert_timespec_time)"]
    pub type ConvertTimespecTimeToTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        timespec_time: *const timespec,
        time: *mut Time,
    ) -> Result;
    #[doc = "See [xrGetVisibilityMaskKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVisibilityMaskKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_visibility_mask)"]
    pub type GetVisibilityMaskKHR = unsafe extern "system" fn(
        session: Session,
        view_configuration_type: ViewConfigurationType,
        view_index: u32,
        visibility_mask_type: VisibilityMaskTypeKHR,
        visibility_mask: *mut VisibilityMaskKHR,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type CreateSpatialAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialAnchorCreateInfoMSFT,
        anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialAnchorSpaceMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type CreateSpatialAnchorSpaceMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialAnchorSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrDestroySpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type DestroySpatialAnchorMSFT =
        unsafe extern "system" fn(anchor: SpatialAnchorMSFT) -> Result;
    #[doc = "See [xrSetInputDeviceActiveEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetInputDeviceActiveEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceActiveEXT = unsafe extern "system" fn(
        session: Session,
        interaction_profile: Path,
        top_level_path: Path,
        is_active: Bool32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateBoolEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetInputDeviceStateBoolEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateBoolEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: Bool32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateFloatEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetInputDeviceStateFloatEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateFloatEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: f32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateVector2fEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetInputDeviceStateVector2fEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateVector2fEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: Vector2f,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceLocationEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetInputDeviceLocationEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceLocationEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        space: Space,
        pose: Posef,
    ) -> Result;
    #[doc = "See [xrInitializeLoaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrInitializeLoaderKHR) - defined by [XR_KHR_loader_init](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_loader_init)"]
    pub type InitializeLoaderKHR =
        unsafe extern "system" fn(loader_init_info: *const LoaderInitInfoBaseHeaderKHR) -> Result;
    #[doc = "See [xrCreateSpatialGraphNodeSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialGraphNodeSpaceMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
    pub type CreateSpatialGraphNodeSpaceMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialGraphNodeSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrCreateHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateHandTrackerEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type CreateHandTrackerEXT = unsafe extern "system" fn(
        session: Session,
        create_info: *const HandTrackerCreateInfoEXT,
        hand_tracker: *mut HandTrackerEXT,
    ) -> Result;
    #[doc = "See [xrDestroyHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyHandTrackerEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type DestroyHandTrackerEXT =
        unsafe extern "system" fn(hand_tracker: HandTrackerEXT) -> Result;
    #[doc = "See [xrLocateHandJointsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateHandJointsEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type LocateHandJointsEXT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        locate_info: *const HandJointsLocateInfoEXT,
        locations: *mut HandJointLocationsEXT,
    ) -> Result;
    #[doc = "See [xrCreateHandMeshSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateHandMeshSpaceMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
    pub type CreateHandMeshSpaceMSFT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        create_info: *const HandMeshSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrUpdateHandMeshMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrUpdateHandMeshMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
    pub type UpdateHandMeshMSFT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        update_info: *const HandMeshUpdateInfoMSFT,
        hand_mesh: *mut HandMeshMSFT,
    ) -> Result;
    #[doc = "See [xrGetControllerModelKeyMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetControllerModelKeyMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelKeyMSFT = unsafe extern "system" fn(
        session: Session,
        top_level_user_path: Path,
        controller_model_key_state: *mut ControllerModelKeyStateMSFT,
    ) -> Result;
    #[doc = "See [xrLoadControllerModelMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLoadControllerModelMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type LoadControllerModelMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    #[doc = "See [xrGetControllerModelPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetControllerModelPropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelPropertiesMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        properties: *mut ControllerModelPropertiesMSFT,
    ) -> Result;
    #[doc = "See [xrGetControllerModelStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetControllerModelStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelStateMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        state: *mut ControllerModelStateMSFT,
    ) -> Result;
    #[doc = "See [xrEnumerateDisplayRefreshRatesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateDisplayRefreshRatesFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type EnumerateDisplayRefreshRatesFB = unsafe extern "system" fn(
        session: Session,
        display_refresh_rate_capacity_input: u32,
        display_refresh_rate_count_output: *mut u32,
        display_refresh_rates: *mut f32,
    ) -> Result;
    #[doc = "See [xrGetDisplayRefreshRateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetDisplayRefreshRateFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type GetDisplayRefreshRateFB =
        unsafe extern "system" fn(session: Session, display_refresh_rate: *mut f32) -> Result;
    #[doc = "See [xrRequestDisplayRefreshRateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestDisplayRefreshRateFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type RequestDisplayRefreshRateFB =
        unsafe extern "system" fn(session: Session, display_refresh_rate: f32) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrCreateSpatialAnchorFromPerceptionAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialAnchorFromPerceptionAnchorMSFT) - defined by [XR_MSFT_perception_anchor_interop](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_perception_anchor_interop)"]
    pub type CreateSpatialAnchorFromPerceptionAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        perception_anchor: *mut IUnknown,
        anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrTryGetPerceptionAnchorFromSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTryGetPerceptionAnchorFromSpatialAnchorMSFT) - defined by [XR_MSFT_perception_anchor_interop](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_perception_anchor_interop)"]
    pub type TryGetPerceptionAnchorFromSpatialAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        anchor: SpatialAnchorMSFT,
        perception_anchor: *mut *mut IUnknown,
    ) -> Result;
    #[doc = "See [xrUpdateSwapchainFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrUpdateSwapchainFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state)"]
    pub type UpdateSwapchainFB = unsafe extern "system" fn(
        swapchain: Swapchain,
        state: *const SwapchainStateBaseHeaderFB,
    ) -> Result;
    #[doc = "See [xrGetSwapchainStateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSwapchainStateFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_swapchain_update_state)"]
    pub type GetSwapchainStateFB = unsafe extern "system" fn(
        swapchain: Swapchain,
        state: *mut SwapchainStateBaseHeaderFB,
    ) -> Result;
    #[doc = "See [xrEnumerateColorSpacesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateColorSpacesFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_color_space)"]
    pub type EnumerateColorSpacesFB = unsafe extern "system" fn(
        session: Session,
        color_space_capacity_input: u32,
        color_space_count_output: *mut u32,
        color_spaces: *mut ColorSpaceFB,
    ) -> Result;
    #[doc = "See [xrSetColorSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetColorSpaceFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_color_space)"]
    pub type SetColorSpaceFB =
        unsafe extern "system" fn(session: Session, colorspace: ColorSpaceFB) -> Result;
    #[doc = "See [xrCreateFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateFoveationProfileFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation)"]
    pub type CreateFoveationProfileFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const FoveationProfileCreateInfoFB,
        profile: *mut FoveationProfileFB,
    ) -> Result;
    #[doc = "See [xrDestroyFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyFoveationProfileFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_foveation)"]
    pub type DestroyFoveationProfileFB =
        unsafe extern "system" fn(profile: FoveationProfileFB) -> Result;
    #[doc = "See [xrGetHandMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetHandMeshFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
    pub type GetHandMeshFB = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        mesh: *mut HandTrackingMeshFB,
    ) -> Result;
    #[doc = "See [xrEnumerateRenderModelPathsFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateRenderModelPathsFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
    pub type EnumerateRenderModelPathsFB = unsafe extern "system" fn(
        session: Session,
        path_capacity_input: u32,
        path_count_output: *mut u32,
        paths: *mut RenderModelPathInfoFB,
    ) -> Result;
    #[doc = "See [xrGetRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
    pub type GetRenderModelPropertiesFB = unsafe extern "system" fn(
        session: Session,
        path: Path,
        properties: *mut RenderModelPropertiesFB,
    ) -> Result;
    #[doc = "See [xrLoadRenderModelFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLoadRenderModelFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_render_model)"]
    pub type LoadRenderModelFB = unsafe extern "system" fn(
        session: Session,
        info: *const RenderModelLoadInfoFB,
        buffer: *mut RenderModelBufferFB,
    ) -> Result;
    #[doc = "See [xrQuerySystemTrackedKeyboardFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrQuerySystemTrackedKeyboardFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
    pub type QuerySystemTrackedKeyboardFB = unsafe extern "system" fn(
        session: Session,
        query_info: *const KeyboardTrackingQueryFB,
        keyboard: *mut KeyboardTrackingDescriptionFB,
    ) -> Result;
    #[doc = "See [xrCreateKeyboardSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateKeyboardSpaceFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_keyboard_tracking)"]
    pub type CreateKeyboardSpaceFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const KeyboardSpaceCreateInfoFB,
        keyboard_space: *mut Space,
    ) -> Result;
    #[doc = "See [xrSetEnvironmentDepthEstimationVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetEnvironmentDepthEstimationVARJO) - defined by [XR_VARJO_environment_depth_estimation](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_environment_depth_estimation)"]
    pub type SetEnvironmentDepthEstimationVARJO =
        unsafe extern "system" fn(session: Session, enabled: Bool32) -> Result;
    #[doc = "See [xrEnumerateReprojectionModesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateReprojectionModesMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
    pub type EnumerateReprojectionModesMSFT = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        mode_capacity_input: u32,
        mode_count_output: *mut u32,
        modes: *mut ReprojectionModeMSFT,
    ) -> Result;
    #[doc = "See [xrGetAudioOutputDeviceGuidOculus](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetAudioOutputDeviceGuidOculus) - defined by [XR_OCULUS_audio_device_guid](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_OCULUS_audio_device_guid)"]
    pub type GetAudioOutputDeviceGuidOculus =
        unsafe extern "system" fn(instance: Instance, buffer: *mut wchar_t) -> Result;
    #[doc = "See [xrGetAudioInputDeviceGuidOculus](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetAudioInputDeviceGuidOculus) - defined by [XR_OCULUS_audio_device_guid](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_OCULUS_audio_device_guid)"]
    pub type GetAudioInputDeviceGuidOculus =
        unsafe extern "system" fn(instance: Instance, buffer: *mut wchar_t) -> Result;
    #[doc = "See [xrCreateTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateTriangleMeshFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type CreateTriangleMeshFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const TriangleMeshCreateInfoFB,
        out_triangle_mesh: *mut TriangleMeshFB,
    ) -> Result;
    #[doc = "See [xrDestroyTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyTriangleMeshFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type DestroyTriangleMeshFB = unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrTriangleMeshGetVertexBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshGetVertexBufferFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshGetVertexBufferFB = unsafe extern "system" fn(
        mesh: TriangleMeshFB,
        out_vertex_buffer: *mut *mut Vector3f,
    ) -> Result;
    #[doc = "See [xrTriangleMeshGetIndexBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshGetIndexBufferFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshGetIndexBufferFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB, out_index_buffer: *mut *mut u32) -> Result;
    #[doc = "See [xrTriangleMeshBeginUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshBeginUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshBeginUpdateFB = unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrTriangleMeshEndUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshEndUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshEndUpdateFB = unsafe extern "system" fn(
        mesh: TriangleMeshFB,
        vertex_count: u32,
        triangle_count: u32,
    ) -> Result;
    #[doc = "See [xrTriangleMeshBeginVertexBufferUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshBeginVertexBufferUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshBeginVertexBufferUpdateFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB, out_vertex_count: *mut u32) -> Result;
    #[doc = "See [xrTriangleMeshEndVertexBufferUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrTriangleMeshEndVertexBufferUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshEndVertexBufferUpdateFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrCreatePassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreatePassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreatePassthroughFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const PassthroughCreateInfoFB,
        out_passthrough: *mut PassthroughFB,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyPassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyPassthroughFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrPassthroughStartFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughStartFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughStartFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrPassthroughPauseFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughPauseFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughPauseFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrCreatePassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreatePassthroughLayerFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreatePassthroughLayerFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const PassthroughLayerCreateInfoFB,
        out_layer: *mut PassthroughLayerFB,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyPassthroughLayerFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyPassthroughLayerFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerPauseFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughLayerPauseFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerPauseFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerResumeFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughLayerResumeFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerResumeFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerSetStyleFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughLayerSetStyleFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerSetStyleFB = unsafe extern "system" fn(
        layer: PassthroughLayerFB,
        style: *const PassthroughStyleFB,
    ) -> Result;
    #[doc = "See [xrCreateGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateGeometryInstanceFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreateGeometryInstanceFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const GeometryInstanceCreateInfoFB,
        out_geometry_instance: *mut GeometryInstanceFB,
    ) -> Result;
    #[doc = "See [xrDestroyGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyGeometryInstanceFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyGeometryInstanceFB =
        unsafe extern "system" fn(instance: GeometryInstanceFB) -> Result;
    #[doc = "See [xrGeometryInstanceSetTransformFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGeometryInstanceSetTransformFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough)"]
    pub type GeometryInstanceSetTransformFB = unsafe extern "system" fn(
        instance: GeometryInstanceFB,
        transformation: *const GeometryInstanceTransformFB,
    ) -> Result;
    #[doc = "See [xrPassthroughLayerSetKeyboardHandsIntensityFB](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPassthroughLayerSetKeyboardHandsIntensityFB) - defined by [XR_FB_passthrough_keyboard_hands](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough_keyboard_hands)"]
    pub type PassthroughLayerSetKeyboardHandsIntensityFB = unsafe extern "system" fn(
        layer: PassthroughLayerFB,
        intensity: *const PassthroughKeyboardHandsIntensityFB,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialAnchorStoreConnectionMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type CreateSpatialAnchorStoreConnectionMSFT = unsafe extern "system" fn(
        session: Session,
        spatial_anchor_store: *mut SpatialAnchorStoreConnectionMSFT,
    ) -> Result;
    #[doc = "See [xrDestroySpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpatialAnchorStoreConnectionMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type DestroySpatialAnchorStoreConnectionMSFT =
        unsafe extern "system" fn(spatial_anchor_store: SpatialAnchorStoreConnectionMSFT) -> Result;
    #[doc = "See [xrPersistSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPersistSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type PersistSpatialAnchorMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_persistence_info: *const SpatialAnchorPersistenceInfoMSFT,
    ) -> Result;
    #[doc = "See [xrEnumeratePersistedSpatialAnchorNamesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumeratePersistedSpatialAnchorNamesMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type EnumeratePersistedSpatialAnchorNamesMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_names_capacity_input: u32,
        spatial_anchor_names_count_output: *mut u32,
        persisted_anchor_names: *mut SpatialAnchorPersistenceNameMSFT,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorFromPersistedNameMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSpatialAnchorFromPersistedNameMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type CreateSpatialAnchorFromPersistedNameMSFT = unsafe extern "system" fn(
        session: Session,
        spatial_anchor_create_info: *const SpatialAnchorFromPersistedAnchorCreateInfoMSFT,
        spatial_anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[doc = "See [xrUnpersistSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrUnpersistSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type UnpersistSpatialAnchorMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_persistence_name: *const SpatialAnchorPersistenceNameMSFT,
    ) -> Result;
    #[doc = "See [xrClearSpatialAnchorStoreMSFT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrClearSpatialAnchorStoreMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type ClearSpatialAnchorStoreMSFT =
        unsafe extern "system" fn(spatial_anchor_store: SpatialAnchorStoreConnectionMSFT) -> Result;
    #[doc = "See [xrCreateFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateFacialTrackerHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type CreateFacialTrackerHTC = unsafe extern "system" fn(
        session: Session,
        create_info: *const FacialTrackerCreateInfoHTC,
        facial_tracker: *mut FacialTrackerHTC,
    ) -> Result;
    #[doc = "See [xrDestroyFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyFacialTrackerHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type DestroyFacialTrackerHTC =
        unsafe extern "system" fn(facial_tracker: FacialTrackerHTC) -> Result;
    #[doc = "See [xrGetFacialExpressionsHTC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetFacialExpressionsHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type GetFacialExpressionsHTC = unsafe extern "system" fn(
        facial_tracker: FacialTrackerHTC,
        facial_expressions: *mut FacialExpressionsHTC,
    ) -> Result;
    #[doc = "See [xrEnumerateViveTrackerPathsHTCX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViveTrackerPathsHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
    pub type EnumerateViveTrackerPathsHTCX = unsafe extern "system" fn(
        instance: Instance,
        path_capacity_input: u32,
        path_count_output: *mut u32,
        paths: *mut ViveTrackerPathsHTCX,
    ) -> Result;
    #[doc = "See [xrSetMarkerTrackingVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetMarkerTrackingVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingVARJO =
        unsafe extern "system" fn(session: Session, enabled: Bool32) -> Result;
    #[doc = "See [xrSetMarkerTrackingTimeoutVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetMarkerTrackingTimeoutVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingTimeoutVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, timeout: Duration) -> Result;
    #[doc = "See [xrSetMarkerTrackingPredictionVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetMarkerTrackingPredictionVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingPredictionVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, enabled: Bool32) -> Result;
    #[doc = "See [xrGetMarkerSizeVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetMarkerSizeVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type GetMarkerSizeVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, size: *mut Extent2Df) -> Result;
    #[doc = "See [xrCreateMarkerSpaceVARJO](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateMarkerSpaceVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type CreateMarkerSpaceVARJO = unsafe extern "system" fn(
        session: Session,
        create_info: *const MarkerSpaceCreateInfoVARJO,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrSetDigitalLensControlALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSetDigitalLensControlALMALENCE) - defined by [XR_ALMALENCE_digital_lens_control](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_ALMALENCE_digital_lens_control)"]
    pub type SetDigitalLensControlALMALENCE = unsafe extern "system" fn(
        session: Session,
        digital_lens_control: *const DigitalLensControlALMALENCE,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsRequirements2KHR](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetVulkanGraphicsRequirements2KHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsRequirements2KHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsVulkanKHR,
    ) -> Result;
}
pub const ALMALENCE_digital_lens_control_SPEC_VERSION: u32 = 1u32;
pub const ALMALENCE_DIGITAL_LENS_CONTROL_EXTENSION_NAME: &[u8] =
    b"XR_ALMALENCE_digital_lens_control\0";
pub const EPIC_view_configuration_fov_SPEC_VERSION: u32 = 2u32;
pub const EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME: &[u8] = b"XR_EPIC_view_configuration_fov\0";
pub const EXT_performance_settings_SPEC_VERSION: u32 = 3u32;
pub const EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME: &[u8] = b"XR_EXT_performance_settings\0";
pub const EXT_thermal_query_SPEC_VERSION: u32 = 2u32;
pub const EXT_THERMAL_QUERY_EXTENSION_NAME: &[u8] = b"XR_EXT_thermal_query\0";
pub const EXT_debug_utils_SPEC_VERSION: u32 = 4u32;
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &[u8] = b"XR_EXT_debug_utils\0";
pub const EXT_eye_gaze_interaction_SPEC_VERSION: u32 = 1u32;
pub const EXT_EYE_GAZE_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_EXT_eye_gaze_interaction\0";
pub const EXT_view_configuration_depth_range_SPEC_VERSION: u32 = 1u32;
pub const EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME: &[u8] =
    b"XR_EXT_view_configuration_depth_range\0";
pub const EXT_conformance_automation_SPEC_VERSION: u32 = 3u32;
pub const EXT_CONFORMANCE_AUTOMATION_EXTENSION_NAME: &[u8] = b"XR_EXT_conformance_automation\0";
pub const EXT_hand_tracking_SPEC_VERSION: u32 = 4u32;
pub const EXT_HAND_TRACKING_EXTENSION_NAME: &[u8] = b"XR_EXT_hand_tracking\0";
#[cfg(windows)]
pub const EXT_win32_appcontainer_compatible_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const EXT_WIN32_APPCONTAINER_COMPATIBLE_EXTENSION_NAME: &[u8] =
    b"XR_EXT_win32_appcontainer_compatible\0";
pub const EXT_hand_joints_motion_range_SPEC_VERSION: u32 = 1u32;
pub const EXT_HAND_JOINTS_MOTION_RANGE_EXTENSION_NAME: &[u8] = b"XR_EXT_hand_joints_motion_range\0";
pub const EXT_samsung_odyssey_controller_SPEC_VERSION: u32 = 1u32;
pub const EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME: &[u8] =
    b"XR_EXT_samsung_odyssey_controller\0";
pub const EXT_hp_mixed_reality_controller_SPEC_VERSION: u32 = 1u32;
pub const EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME: &[u8] =
    b"XR_EXT_hp_mixed_reality_controller\0";
pub const EXT_uuid_SPEC_VERSION: u32 = 1u32;
pub const EXT_UUID_EXTENSION_NAME: &[u8] = b"XR_EXT_uuid\0";
pub const EXTX_overlay_SPEC_VERSION: u32 = 5u32;
pub const EXTX_OVERLAY_EXTENSION_NAME: &[u8] = b"XR_EXTX_overlay\0";
pub const FB_composition_layer_image_layout_SPEC_VERSION: u32 = 1u32;
pub const FB_COMPOSITION_LAYER_IMAGE_LAYOUT_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_image_layout\0";
pub const FB_composition_layer_alpha_blend_SPEC_VERSION: u32 = 2u32;
pub const FB_COMPOSITION_LAYER_ALPHA_BLEND_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_alpha_blend\0";
#[cfg(target_os = "android")]
pub const FB_android_surface_swapchain_create_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const FB_ANDROID_SURFACE_SWAPCHAIN_CREATE_EXTENSION_NAME: &[u8] =
    b"XR_FB_android_surface_swapchain_create\0";
pub const FB_swapchain_update_state_SPEC_VERSION: u32 = 3u32;
pub const FB_SWAPCHAIN_UPDATE_STATE_EXTENSION_NAME: &[u8] = b"XR_FB_swapchain_update_state\0";
pub const FB_composition_layer_secure_content_SPEC_VERSION: u32 = 1u32;
pub const FB_COMPOSITION_LAYER_SECURE_CONTENT_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_secure_content\0";
pub const FB_display_refresh_rate_SPEC_VERSION: u32 = 1u32;
pub const FB_DISPLAY_REFRESH_RATE_EXTENSION_NAME: &[u8] = b"XR_FB_display_refresh_rate\0";
pub const FB_color_space_SPEC_VERSION: u32 = 2u32;
pub const FB_COLOR_SPACE_EXTENSION_NAME: &[u8] = b"XR_FB_color_space\0";
pub const FB_hand_tracking_mesh_SPEC_VERSION: u32 = 1u32;
pub const FB_HAND_TRACKING_MESH_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_mesh\0";
pub const FB_hand_tracking_aim_SPEC_VERSION: u32 = 1u32;
pub const FB_HAND_TRACKING_AIM_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_aim\0";
pub const FB_hand_tracking_capsules_SPEC_VERSION: u32 = 2u32;
pub const FB_HAND_TRACKING_CAPSULES_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_capsules\0";
pub const FB_foveation_SPEC_VERSION: u32 = 1u32;
pub const FB_FOVEATION_EXTENSION_NAME: &[u8] = b"XR_FB_foveation\0";
pub const FB_foveation_configuration_SPEC_VERSION: u32 = 1u32;
pub const FB_FOVEATION_CONFIGURATION_EXTENSION_NAME: &[u8] = b"XR_FB_foveation_configuration\0";
pub const FB_keyboard_tracking_SPEC_VERSION: u32 = 1u32;
pub const FB_KEYBOARD_TRACKING_EXTENSION_NAME: &[u8] = b"XR_FB_keyboard_tracking\0";
pub const FB_triangle_mesh_SPEC_VERSION: u32 = 1u32;
pub const FB_TRIANGLE_MESH_EXTENSION_NAME: &[u8] = b"XR_FB_triangle_mesh\0";
pub const FB_passthrough_SPEC_VERSION: u32 = 1u32;
pub const FB_PASSTHROUGH_EXTENSION_NAME: &[u8] = b"XR_FB_passthrough\0";
pub const FB_render_model_SPEC_VERSION: u32 = 1u32;
pub const FB_RENDER_MODEL_EXTENSION_NAME: &[u8] = b"XR_FB_render_model\0";
pub const FB_foveation_vulkan_SPEC_VERSION: u32 = 1u32;
pub const FB_FOVEATION_VULKAN_EXTENSION_NAME: &[u8] = b"XR_FB_foveation_vulkan\0";
#[cfg(target_os = "android")]
pub const FB_swapchain_update_state_android_surface_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const FB_SWAPCHAIN_UPDATE_STATE_ANDROID_SURFACE_EXTENSION_NAME: &[u8] =
    b"XR_FB_swapchain_update_state_android_surface\0";
pub const FB_swapchain_update_state_opengl_es_SPEC_VERSION: u32 = 1u32;
pub const FB_SWAPCHAIN_UPDATE_STATE_OPENGL_ES_EXTENSION_NAME: &[u8] =
    b"XR_FB_swapchain_update_state_opengl_es\0";
pub const FB_swapchain_update_state_vulkan_SPEC_VERSION: u32 = 1u32;
pub const FB_SWAPCHAIN_UPDATE_STATE_VULKAN_EXTENSION_NAME: &[u8] =
    b"XR_FB_swapchain_update_state_vulkan\0";
pub const FB_space_warp_SPEC_VERSION: u32 = 1u32;
pub const FB_SPACE_WARP_EXTENSION_NAME: &[u8] = b"XR_FB_space_warp\0";
pub const FB_passthrough_keyboard_hands_SPEC_VERSION: u32 = 1u32;
pub const FB_PASSTHROUGH_KEYBOARD_HANDS_EXTENSION_NAME: &[u8] =
    b"XR_FB_passthrough_keyboard_hands\0";
pub const HTC_vive_cosmos_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTC_vive_cosmos_controller_interaction\0";
pub const HTC_facial_tracking_SPEC_VERSION: u32 = 1u32;
pub const HTC_FACIAL_TRACKING_EXTENSION_NAME: &[u8] = b"XR_HTC_facial_tracking\0";
pub const HTC_vive_focus3_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTC_VIVE_FOCUS3_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTC_vive_focus3_controller_interaction\0";
pub const HTCX_vive_tracker_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTCX_VIVE_TRACKER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTCX_vive_tracker_interaction\0";
pub const HUAWEI_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HUAWEI_controller_interaction\0";
#[cfg(target_os = "android")]
pub const KHR_android_thread_settings_SPEC_VERSION: u32 = 5u32;
#[cfg(target_os = "android")]
pub const KHR_ANDROID_THREAD_SETTINGS_EXTENSION_NAME: &[u8] = b"XR_KHR_android_thread_settings\0";
#[cfg(target_os = "android")]
pub const KHR_android_surface_swapchain_SPEC_VERSION: u32 = 4u32;
#[cfg(target_os = "android")]
pub const KHR_ANDROID_SURFACE_SWAPCHAIN_EXTENSION_NAME: &[u8] =
    b"XR_KHR_android_surface_swapchain\0";
pub const KHR_composition_layer_cube_SPEC_VERSION: u32 = 8u32;
pub const KHR_COMPOSITION_LAYER_CUBE_EXTENSION_NAME: &[u8] = b"XR_KHR_composition_layer_cube\0";
#[cfg(target_os = "android")]
pub const KHR_android_create_instance_SPEC_VERSION: u32 = 3u32;
#[cfg(target_os = "android")]
pub const KHR_ANDROID_CREATE_INSTANCE_EXTENSION_NAME: &[u8] = b"XR_KHR_android_create_instance\0";
pub const KHR_composition_layer_depth_SPEC_VERSION: u32 = 5u32;
pub const KHR_COMPOSITION_LAYER_DEPTH_EXTENSION_NAME: &[u8] = b"XR_KHR_composition_layer_depth\0";
pub const KHR_vulkan_swapchain_format_list_SPEC_VERSION: u32 = 4u32;
pub const KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME: &[u8] =
    b"XR_KHR_vulkan_swapchain_format_list\0";
pub const KHR_composition_layer_cylinder_SPEC_VERSION: u32 = 4u32;
pub const KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_cylinder\0";
pub const KHR_composition_layer_equirect_SPEC_VERSION: u32 = 3u32;
pub const KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_equirect\0";
pub const KHR_opengl_enable_SPEC_VERSION: u32 = 10u32;
pub const KHR_OPENGL_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_opengl_enable\0";
pub const KHR_opengl_es_enable_SPEC_VERSION: u32 = 8u32;
pub const KHR_OPENGL_ES_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_opengl_es_enable\0";
pub const KHR_vulkan_enable_SPEC_VERSION: u32 = 8u32;
pub const KHR_VULKAN_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_vulkan_enable\0";
#[cfg(windows)]
pub const KHR_D3D11_enable_SPEC_VERSION: u32 = 8u32;
#[cfg(windows)]
pub const KHR_D3D11_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D11_enable\0";
#[cfg(windows)]
pub const KHR_D3D12_enable_SPEC_VERSION: u32 = 8u32;
#[cfg(windows)]
pub const KHR_D3D12_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D12_enable\0";
pub const KHR_visibility_mask_SPEC_VERSION: u32 = 2u32;
pub const KHR_VISIBILITY_MASK_EXTENSION_NAME: &[u8] = b"XR_KHR_visibility_mask\0";
pub const KHR_composition_layer_color_scale_bias_SPEC_VERSION: u32 = 5u32;
pub const KHR_COMPOSITION_LAYER_COLOR_SCALE_BIAS_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_color_scale_bias\0";
#[cfg(windows)]
pub const KHR_win32_convert_performance_counter_time_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME: &[u8] =
    b"XR_KHR_win32_convert_performance_counter_time\0";
pub const KHR_convert_timespec_time_SPEC_VERSION: u32 = 1u32;
pub const KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME: &[u8] = b"XR_KHR_convert_timespec_time\0";
pub const KHR_loader_init_SPEC_VERSION: u32 = 1u32;
pub const KHR_LOADER_INIT_EXTENSION_NAME: &[u8] = b"XR_KHR_loader_init\0";
#[cfg(target_os = "android")]
pub const KHR_loader_init_android_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const KHR_LOADER_INIT_ANDROID_EXTENSION_NAME: &[u8] = b"XR_KHR_loader_init_android\0";
pub const KHR_vulkan_enable2_SPEC_VERSION: u32 = 2u32;
pub const KHR_VULKAN_ENABLE2_EXTENSION_NAME: &[u8] = b"XR_KHR_vulkan_enable2\0";
pub const KHR_composition_layer_equirect2_SPEC_VERSION: u32 = 1u32;
pub const KHR_COMPOSITION_LAYER_EQUIRECT2_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_equirect2\0";
pub const KHR_binding_modification_SPEC_VERSION: u32 = 1u32;
pub const KHR_BINDING_MODIFICATION_EXTENSION_NAME: &[u8] = b"XR_KHR_binding_modification\0";
pub const KHR_swapchain_usage_input_attachment_bit_SPEC_VERSION: u32 = 3u32;
pub const KHR_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME: &[u8] =
    b"XR_KHR_swapchain_usage_input_attachment_bit\0";
pub const MND_headless_SPEC_VERSION: u32 = 2u32;
pub const MND_HEADLESS_EXTENSION_NAME: &[u8] = b"XR_MND_headless\0";
pub const MND_swapchain_usage_input_attachment_bit_SPEC_VERSION: u32 = 2u32;
pub const MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME: &[u8] =
    b"XR_MND_swapchain_usage_input_attachment_bit\0";
pub const MNDX_egl_enable_SPEC_VERSION: u32 = 1u32;
pub const MNDX_EGL_ENABLE_EXTENSION_NAME: &[u8] = b"XR_MNDX_egl_enable\0";
pub const MSFT_unbounded_reference_space_SPEC_VERSION: u32 = 1u32;
pub const MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_unbounded_reference_space\0";
pub const MSFT_spatial_anchor_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SPATIAL_ANCHOR_EXTENSION_NAME: &[u8] = b"XR_MSFT_spatial_anchor\0";
pub const MSFT_spatial_graph_bridge_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SPATIAL_GRAPH_BRIDGE_EXTENSION_NAME: &[u8] = b"XR_MSFT_spatial_graph_bridge\0";
pub const MSFT_hand_interaction_SPEC_VERSION: u32 = 1u32;
pub const MSFT_HAND_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_MSFT_hand_interaction\0";
pub const MSFT_hand_tracking_mesh_SPEC_VERSION: u32 = 4u32;
pub const MSFT_HAND_TRACKING_MESH_EXTENSION_NAME: &[u8] = b"XR_MSFT_hand_tracking_mesh\0";
pub const MSFT_secondary_view_configuration_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SECONDARY_VIEW_CONFIGURATION_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_secondary_view_configuration\0";
pub const MSFT_first_person_observer_SPEC_VERSION: u32 = 1u32;
pub const MSFT_FIRST_PERSON_OBSERVER_EXTENSION_NAME: &[u8] = b"XR_MSFT_first_person_observer\0";
pub const MSFT_controller_model_SPEC_VERSION: u32 = 2u32;
pub const MSFT_CONTROLLER_MODEL_EXTENSION_NAME: &[u8] = b"XR_MSFT_controller_model\0";
#[cfg(windows)]
pub const MSFT_perception_anchor_interop_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const MSFT_PERCEPTION_ANCHOR_INTEROP_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_perception_anchor_interop\0";
#[cfg(windows)]
pub const MSFT_holographic_window_attachment_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const MSFT_HOLOGRAPHIC_WINDOW_ATTACHMENT_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_holographic_window_attachment\0";
pub const MSFT_composition_layer_reprojection_SPEC_VERSION: u32 = 1u32;
pub const MSFT_COMPOSITION_LAYER_REPROJECTION_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_composition_layer_reprojection\0";
pub const MSFT_scene_understanding_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SCENE_UNDERSTANDING_EXTENSION_NAME: &[u8] = b"XR_MSFT_scene_understanding\0";
pub const MSFT_scene_understanding_serialization_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SCENE_UNDERSTANDING_SERIALIZATION_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_scene_understanding_serialization\0";
pub const MSFT_spatial_anchor_persistence_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SPATIAL_ANCHOR_PERSISTENCE_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_spatial_anchor_persistence\0";
#[cfg(target_os = "android")]
pub const OCULUS_android_session_state_enable_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME: &[u8] =
    b"XR_OCULUS_android_session_state_enable\0";
pub const OCULUS_audio_device_guid_SPEC_VERSION: u32 = 1u32;
pub const OCULUS_AUDIO_DEVICE_GUID_EXTENSION_NAME: &[u8] = b"XR_OCULUS_audio_device_guid\0";
pub const VALVE_analog_threshold_SPEC_VERSION: u32 = 2u32;
pub const VALVE_ANALOG_THRESHOLD_EXTENSION_NAME: &[u8] = b"XR_VALVE_analog_threshold\0";
pub const VARJO_quad_views_SPEC_VERSION: u32 = 1u32;
pub const VARJO_QUAD_VIEWS_EXTENSION_NAME: &[u8] = b"XR_VARJO_quad_views\0";
pub const VARJO_foveated_rendering_SPEC_VERSION: u32 = 2u32;
pub const VARJO_FOVEATED_RENDERING_EXTENSION_NAME: &[u8] = b"XR_VARJO_foveated_rendering\0";
pub const VARJO_composition_layer_depth_test_SPEC_VERSION: u32 = 2u32;
pub const VARJO_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME: &[u8] =
    b"XR_VARJO_composition_layer_depth_test\0";
pub const VARJO_environment_depth_estimation_SPEC_VERSION: u32 = 1u32;
pub const VARJO_ENVIRONMENT_DEPTH_ESTIMATION_EXTENSION_NAME: &[u8] =
    b"XR_VARJO_environment_depth_estimation\0";
pub const VARJO_marker_tracking_SPEC_VERSION: u32 = 1u32;
pub const VARJO_MARKER_TRACKING_EXTENSION_NAME: &[u8] = b"XR_VARJO_marker_tracking\0";
#[cfg(feature = "linked")]
extern "system" {
    #[link_name = "xrGetInstanceProcAddr"]
    pub fn get_instance_proc_addr(
        instance: Instance,
        name: *const c_char,
        function: *mut Option<pfn::VoidFunction>,
    ) -> Result;
    #[link_name = "xrEnumerateApiLayerProperties"]
    pub fn enumerate_api_layer_properties(
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ApiLayerProperties,
    ) -> Result;
    #[link_name = "xrEnumerateInstanceExtensionProperties"]
    pub fn enumerate_instance_extension_properties(
        layer_name: *const c_char,
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result;
    #[link_name = "xrCreateInstance"]
    pub fn create_instance(
        create_info: *const InstanceCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    #[link_name = "xrDestroyInstance"]
    pub fn destroy_instance(instance: Instance) -> Result;
    #[link_name = "xrResultToString"]
    pub fn result_to_string(instance: Instance, value: Result, buffer: *mut c_char) -> Result;
    #[link_name = "xrStructureTypeToString"]
    pub fn structure_type_to_string(
        instance: Instance,
        value: StructureType,
        buffer: *mut c_char,
    ) -> Result;
    #[link_name = "xrGetInstanceProperties"]
    pub fn get_instance_properties(
        instance: Instance,
        instance_properties: *mut InstanceProperties,
    ) -> Result;
    #[link_name = "xrGetSystem"]
    pub fn get_system(
        instance: Instance,
        get_info: *const SystemGetInfo,
        system_id: *mut SystemId,
    ) -> Result;
    #[link_name = "xrGetSystemProperties"]
    pub fn get_system_properties(
        instance: Instance,
        system_id: SystemId,
        properties: *mut SystemProperties,
    ) -> Result;
    #[link_name = "xrCreateSession"]
    pub fn create_session(
        instance: Instance,
        create_info: *const SessionCreateInfo,
        session: *mut Session,
    ) -> Result;
    #[link_name = "xrDestroySession"]
    pub fn destroy_session(session: Session) -> Result;
    #[link_name = "xrDestroySpace"]
    pub fn destroy_space(space: Space) -> Result;
    #[link_name = "xrEnumerateSwapchainFormats"]
    pub fn enumerate_swapchain_formats(
        session: Session,
        format_capacity_input: u32,
        format_count_output: *mut u32,
        formats: *mut i64,
    ) -> Result;
    #[link_name = "xrCreateSwapchain"]
    pub fn create_swapchain(
        session: Session,
        create_info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
    ) -> Result;
    #[link_name = "xrDestroySwapchain"]
    pub fn destroy_swapchain(swapchain: Swapchain) -> Result;
    #[link_name = "xrEnumerateSwapchainImages"]
    pub fn enumerate_swapchain_images(
        swapchain: Swapchain,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    ) -> Result;
    #[link_name = "xrAcquireSwapchainImage"]
    pub fn acquire_swapchain_image(
        swapchain: Swapchain,
        acquire_info: *const SwapchainImageAcquireInfo,
        index: *mut u32,
    ) -> Result;
    #[link_name = "xrWaitSwapchainImage"]
    pub fn wait_swapchain_image(
        swapchain: Swapchain,
        wait_info: *const SwapchainImageWaitInfo,
    ) -> Result;
    #[link_name = "xrReleaseSwapchainImage"]
    pub fn release_swapchain_image(
        swapchain: Swapchain,
        release_info: *const SwapchainImageReleaseInfo,
    ) -> Result;
    #[link_name = "xrBeginSession"]
    pub fn begin_session(session: Session, begin_info: *const SessionBeginInfo) -> Result;
    #[link_name = "xrEndSession"]
    pub fn end_session(session: Session) -> Result;
    #[link_name = "xrRequestExitSession"]
    pub fn request_exit_session(session: Session) -> Result;
    #[link_name = "xrEnumerateReferenceSpaces"]
    pub fn enumerate_reference_spaces(
        session: Session,
        space_capacity_input: u32,
        space_count_output: *mut u32,
        spaces: *mut ReferenceSpaceType,
    ) -> Result;
    #[link_name = "xrCreateReferenceSpace"]
    pub fn create_reference_space(
        session: Session,
        create_info: *const ReferenceSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[link_name = "xrCreateActionSpace"]
    pub fn create_action_space(
        session: Session,
        create_info: *const ActionSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[link_name = "xrLocateSpace"]
    pub fn locate_space(
        space: Space,
        base_space: Space,
        time: Time,
        location: *mut SpaceLocation,
    ) -> Result;
    #[link_name = "xrEnumerateViewConfigurations"]
    pub fn enumerate_view_configurations(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type_capacity_input: u32,
        view_configuration_type_count_output: *mut u32,
        view_configuration_types: *mut ViewConfigurationType,
    ) -> Result;
    #[link_name = "xrEnumerateEnvironmentBlendModes"]
    pub fn enumerate_environment_blend_modes(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        environment_blend_mode_capacity_input: u32,
        environment_blend_mode_count_output: *mut u32,
        environment_blend_modes: *mut EnvironmentBlendMode,
    ) -> Result;
    #[link_name = "xrGetViewConfigurationProperties"]
    pub fn get_view_configuration_properties(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        configuration_properties: *mut ViewConfigurationProperties,
    ) -> Result;
    #[link_name = "xrEnumerateViewConfigurationViews"]
    pub fn enumerate_view_configuration_views(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut ViewConfigurationView,
    ) -> Result;
    #[link_name = "xrBeginFrame"]
    pub fn begin_frame(session: Session, frame_begin_info: *const FrameBeginInfo) -> Result;
    #[link_name = "xrLocateViews"]
    pub fn locate_views(
        session: Session,
        view_locate_info: *const ViewLocateInfo,
        view_state: *mut ViewState,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut View,
    ) -> Result;
    #[link_name = "xrEndFrame"]
    pub fn end_frame(session: Session, frame_end_info: *const FrameEndInfo) -> Result;
    #[link_name = "xrWaitFrame"]
    pub fn wait_frame(
        session: Session,
        frame_wait_info: *const FrameWaitInfo,
        frame_state: *mut FrameState,
    ) -> Result;
    #[link_name = "xrApplyHapticFeedback"]
    pub fn apply_haptic_feedback(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
        haptic_feedback: *const HapticBaseHeader,
    ) -> Result;
    #[link_name = "xrStopHapticFeedback"]
    pub fn stop_haptic_feedback(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
    ) -> Result;
    #[link_name = "xrPollEvent"]
    pub fn poll_event(instance: Instance, event_data: *mut EventDataBuffer) -> Result;
    #[link_name = "xrStringToPath"]
    pub fn string_to_path(
        instance: Instance,
        path_string: *const c_char,
        path: *mut Path,
    ) -> Result;
    #[link_name = "xrPathToString"]
    pub fn path_to_string(
        instance: Instance,
        path: Path,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[link_name = "xrGetReferenceSpaceBoundsRect"]
    pub fn get_reference_space_bounds_rect(
        session: Session,
        reference_space_type: ReferenceSpaceType,
        bounds: *mut Extent2Df,
    ) -> Result;
    #[link_name = "xrGetActionStateBoolean"]
    pub fn get_action_state_boolean(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateBoolean,
    ) -> Result;
    #[link_name = "xrGetActionStateFloat"]
    pub fn get_action_state_float(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateFloat,
    ) -> Result;
    #[link_name = "xrGetActionStateVector2f"]
    pub fn get_action_state_vector2f(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateVector2f,
    ) -> Result;
    #[link_name = "xrGetActionStatePose"]
    pub fn get_action_state_pose(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStatePose,
    ) -> Result;
    #[link_name = "xrCreateActionSet"]
    pub fn create_action_set(
        instance: Instance,
        create_info: *const ActionSetCreateInfo,
        action_set: *mut ActionSet,
    ) -> Result;
    #[link_name = "xrDestroyActionSet"]
    pub fn destroy_action_set(action_set: ActionSet) -> Result;
    #[link_name = "xrCreateAction"]
    pub fn create_action(
        action_set: ActionSet,
        create_info: *const ActionCreateInfo,
        action: *mut Action,
    ) -> Result;
    #[link_name = "xrDestroyAction"]
    pub fn destroy_action(action: Action) -> Result;
    #[link_name = "xrSuggestInteractionProfileBindings"]
    pub fn suggest_interaction_profile_bindings(
        instance: Instance,
        suggested_bindings: *const InteractionProfileSuggestedBinding,
    ) -> Result;
    #[link_name = "xrAttachSessionActionSets"]
    pub fn attach_session_action_sets(
        session: Session,
        attach_info: *const SessionActionSetsAttachInfo,
    ) -> Result;
    #[link_name = "xrGetCurrentInteractionProfile"]
    pub fn get_current_interaction_profile(
        session: Session,
        top_level_user_path: Path,
        interaction_profile: *mut InteractionProfileState,
    ) -> Result;
    #[link_name = "xrSyncActions"]
    pub fn sync_actions(session: Session, sync_info: *const ActionsSyncInfo) -> Result;
    #[link_name = "xrEnumerateBoundSourcesForAction"]
    pub fn enumerate_bound_sources_for_action(
        session: Session,
        enumerate_info: *const BoundSourcesForActionEnumerateInfo,
        source_capacity_input: u32,
        source_count_output: *mut u32,
        sources: *mut Path,
    ) -> Result;
    #[link_name = "xrGetInputSourceLocalizedName"]
    pub fn get_input_source_localized_name(
        session: Session,
        get_info: *const InputSourceLocalizedNameGetInfo,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
}

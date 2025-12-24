#![doc = r" Automatically generated code; do not edit!"]
#![allow(
    non_upper_case_globals,
    clippy::unreadable_literal,
    clippy::identity_op,
    unused,
    clippy::derive_partial_eq_without_eq
)]
use crate::platform::*;
use crate::support::*;
use crate::*;
use libc::{timespec, wchar_t};
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};
pub const CURRENT_API_VERSION: Version = Version::new(1u16, 1u16, 40u32);
pub const EXTENSION_ENUM_BASE: usize = 1000000000usize;
pub const EXTENSION_ENUM_STRIDE: usize = 1000usize;
pub const NULL_PATH: usize = 0usize;
pub const NULL_SYSTEM_ID: usize = 0usize;
pub const NO_DURATION: usize = 0usize;
pub const FREQUENCY_UNSPECIFIED: usize = 0usize;
pub const MIN_COMPOSITION_LAYERS_SUPPORTED: usize = 16usize;
pub const CURRENT_LOADER_API_LAYER_VERSION: usize = 1usize;
pub const CURRENT_LOADER_RUNTIME_VERSION: usize = 1usize;
pub const LOADER_INFO_STRUCT_VERSION: usize = 1usize;
pub const API_LAYER_INFO_STRUCT_VERSION: usize = 1usize;
pub const RUNTIME_INFO_STRUCT_VERSION: usize = 1usize;
pub const API_LAYER_NEXT_INFO_STRUCT_VERSION: usize = 1usize;
pub const API_LAYER_CREATE_INFO_STRUCT_VERSION: usize = 1usize;
pub const API_LAYER_MAX_SETTINGS_PATH_SIZE: usize = 512usize;
pub const HAND_JOINT_COUNT_EXT: usize = 26usize;
pub const NULL_CONTROLLER_MODEL_KEY_MSFT: usize = 0usize;
pub const NULL_RENDER_MODEL_KEY_FB: usize = 0usize;
pub const FACIAL_EXPRESSION_EYE_COUNT_HTC: usize = 14usize;
pub const FACIAL_EXPRESSION_LIP_COUNT_HTC: usize = 37usize;
pub const HAND_FOREARM_JOINT_COUNT_ULTRALEAP: usize = 27usize;
pub const MAX_HAPTIC_PCM_BUFFER_SIZE_FB: usize = 4000usize;
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
pub const UUID_SIZE: usize = 16usize;
pub const GUID_SIZE_MSFT: usize = 16usize;
pub const MAX_CONTROLLER_MODEL_NODE_NAME_SIZE_MSFT: usize = 64usize;
pub const HAND_TRACKING_CAPSULE_POINT_COUNT_FB: usize = 2usize;
pub const HAND_TRACKING_CAPSULE_COUNT_FB: usize = 19usize;
pub const MAX_KEYBOARD_TRACKING_NAME_SIZE_FB: usize = 128usize;
pub const PASSTHROUGH_COLOR_MAP_MONO_SIZE_FB: usize = 256usize;
pub const MAX_RENDER_MODEL_NAME_SIZE_FB: usize = 64usize;
pub const MAX_LOCALIZATION_MAP_NAME_LENGTH_ML: usize = 64usize;
pub const MAX_SPATIAL_ANCHOR_NAME_SIZE_MSFT: usize = 256usize;
pub const MAX_AUDIO_DEVICE_STR_SIZE_OCULUS: usize = 128usize;
pub const FOVEATION_CENTER_SIZE_META: usize = 2usize;
pub const MAX_VIRTUAL_KEYBOARD_COMMIT_TEXT_SIZE_META: usize = 3992usize;
pub const MAX_EXTERNAL_CAMERA_NAME_SIZE_OCULUS: usize = 32usize;
pub const UUID_SIZE_EXT: usize = 16usize;
pub const MAX_SPATIAL_ANCHOR_NAME_SIZE_HTC: usize = 256usize;
pub const NULL_FUTURE_EXT: usize = 0usize;
#[doc = "See [XrLoaderInterfaceStructs](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLoaderInterfaceStructs)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LoaderInterfaceStructs(i32);
impl LoaderInterfaceStructs {
    pub const NINTIALIZED: LoaderInterfaceStructs = Self(0i32);
    pub const OADER_INFO: LoaderInterfaceStructs = Self(1i32);
    pub const PI_LAYER_REQUEST: LoaderInterfaceStructs = Self(2i32);
    pub const UNTIME_REQUEST: LoaderInterfaceStructs = Self(3i32);
    pub const PI_LAYER_CREATE_INFO: LoaderInterfaceStructs = Self(4i32);
    pub const PI_LAYER_NEXT_INFO: LoaderInterfaceStructs = Self(5i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LoaderInterfaceStructs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NINTIALIZED => Some("NINTIALIZED"),
            Self::OADER_INFO => Some("OADER_INFO"),
            Self::PI_LAYER_REQUEST => Some("PI_LAYER_REQUEST"),
            Self::UNTIME_REQUEST => Some("UNTIME_REQUEST"),
            Self::PI_LAYER_CREATE_INFO => Some("PI_LAYER_CREATE_INFO"),
            Self::PI_LAYER_NEXT_INFO => Some("PI_LAYER_NEXT_INFO"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Structure type enumerant - see [XrStructureType](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrStructureType)"]
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
    pub const SPACES_LOCATE_INFO: StructureType = Self(1000471000i32);
    pub const SPACE_LOCATIONS: StructureType = Self(1000471001i32);
    pub const SPACE_VELOCITIES: StructureType = Self(1000471002i32);
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
    pub const GRAPHICS_BINDING_METAL_KHR: StructureType = Self(1000029000i32);
    pub const SWAPCHAIN_IMAGE_METAL_KHR: StructureType = Self(1000029001i32);
    pub const GRAPHICS_REQUIREMENTS_METAL_KHR: StructureType = Self(1000029002i32);
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
    pub const SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT: StructureType =
        Self(1000049001i32);
    pub const SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT: StructureType =
        Self(1000049002i32);
    pub const SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT: StructureType = Self(1000049003i32);
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
    pub const BODY_TRACKER_CREATE_INFO_FB: StructureType = Self(1000076001i32);
    pub const BODY_JOINTS_LOCATE_INFO_FB: StructureType = Self(1000076002i32);
    pub const SYSTEM_BODY_TRACKING_PROPERTIES_FB: StructureType = Self(1000076004i32);
    pub const BODY_JOINT_LOCATIONS_FB: StructureType = Self(1000076005i32);
    pub const BODY_SKELETON_FB: StructureType = Self(1000076006i32);
    pub const INTERACTION_PROFILE_DPAD_BINDING_EXT: StructureType = Self(1000078000i32);
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
    pub const SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB: StructureType = Self(1000113004i32);
    pub const SPATIAL_ANCHOR_CREATE_INFO_FB: StructureType = Self(1000113003i32);
    pub const SPACE_COMPONENT_STATUS_SET_INFO_FB: StructureType = Self(1000113007i32);
    pub const SPACE_COMPONENT_STATUS_FB: StructureType = Self(1000113001i32);
    pub const EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB: StructureType = Self(1000113005i32);
    pub const EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB: StructureType = Self(1000113006i32);
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
    pub const SYSTEM_PASSTHROUGH_PROPERTIES2_FB: StructureType = Self(1000118006i32);
    pub const PASSTHROUGH_STYLE_FB: StructureType = Self(1000118020i32);
    pub const PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB: StructureType = Self(1000118021i32);
    pub const PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB: StructureType = Self(1000118022i32);
    pub const PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB: StructureType = Self(1000118023i32);
    pub const EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB: StructureType = Self(1000118030i32);
    pub const RENDER_MODEL_PATH_INFO_FB: StructureType = Self(1000119000i32);
    pub const RENDER_MODEL_PROPERTIES_FB: StructureType = Self(1000119001i32);
    pub const RENDER_MODEL_BUFFER_FB: StructureType = Self(1000119002i32);
    pub const RENDER_MODEL_LOAD_INFO_FB: StructureType = Self(1000119003i32);
    pub const SYSTEM_RENDER_MODEL_PROPERTIES_FB: StructureType = Self(1000119004i32);
    pub const RENDER_MODEL_CAPABILITIES_REQUEST_FB: StructureType = Self(1000119005i32);
    pub const BINDING_MODIFICATIONS_KHR: StructureType = Self(1000120000i32);
    pub const VIEW_LOCATE_FOVEATED_RENDERING_VARJO: StructureType = Self(1000121000i32);
    pub const FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO: StructureType = Self(1000121001i32);
    pub const SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO: StructureType = Self(1000121002i32);
    pub const COMPOSITION_LAYER_DEPTH_TEST_VARJO: StructureType = Self(1000122000i32);
    pub const SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO: StructureType = Self(1000124000i32);
    pub const EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO: StructureType = Self(1000124001i32);
    pub const MARKER_SPACE_CREATE_INFO_VARJO: StructureType = Self(1000124002i32);
    pub const FRAME_END_INFO_ML: StructureType = Self(1000135000i32);
    pub const GLOBAL_DIMMER_FRAME_END_INFO_ML: StructureType = Self(1000136000i32);
    pub const COORDINATE_SPACE_CREATE_INFO_ML: StructureType = Self(1000137000i32);
    pub const SYSTEM_MARKER_UNDERSTANDING_PROPERTIES_ML: StructureType = Self(1000138000i32);
    pub const MARKER_DETECTOR_CREATE_INFO_ML: StructureType = Self(1000138001i32);
    pub const MARKER_DETECTOR_ARUCO_INFO_ML: StructureType = Self(1000138002i32);
    pub const MARKER_DETECTOR_SIZE_INFO_ML: StructureType = Self(1000138003i32);
    pub const MARKER_DETECTOR_APRIL_TAG_INFO_ML: StructureType = Self(1000138004i32);
    pub const MARKER_DETECTOR_CUSTOM_PROFILE_INFO_ML: StructureType = Self(1000138005i32);
    pub const MARKER_DETECTOR_SNAPSHOT_INFO_ML: StructureType = Self(1000138006i32);
    pub const MARKER_DETECTOR_STATE_ML: StructureType = Self(1000138007i32);
    pub const MARKER_SPACE_CREATE_INFO_ML: StructureType = Self(1000138008i32);
    pub const LOCALIZATION_MAP_ML: StructureType = Self(1000139000i32);
    pub const EVENT_DATA_LOCALIZATION_CHANGED_ML: StructureType = Self(1000139001i32);
    pub const MAP_LOCALIZATION_REQUEST_INFO_ML: StructureType = Self(1000139002i32);
    pub const LOCALIZATION_MAP_IMPORT_INFO_ML: StructureType = Self(1000139003i32);
    pub const LOCALIZATION_ENABLE_EVENTS_INFO_ML: StructureType = Self(1000139004i32);
    pub const EVENT_DATA_HEADSET_FIT_CHANGED_ML: StructureType = Self(1000472000i32);
    pub const EVENT_DATA_EYE_CALIBRATION_CHANGED_ML: StructureType = Self(1000472001i32);
    pub const USER_CALIBRATION_ENABLE_EVENTS_INFO_ML: StructureType = Self(1000472002i32);
    pub const SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT: StructureType = Self(1000142000i32);
    pub const SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT: StructureType =
        Self(1000142001i32);
    pub const SCENE_MARKERS_MSFT: StructureType = Self(1000147000i32);
    pub const SCENE_MARKER_TYPE_FILTER_MSFT: StructureType = Self(1000147001i32);
    pub const SCENE_MARKER_QR_CODES_MSFT: StructureType = Self(1000147002i32);
    pub const SPACE_QUERY_INFO_FB: StructureType = Self(1000156001i32);
    pub const SPACE_QUERY_RESULTS_FB: StructureType = Self(1000156002i32);
    pub const SPACE_STORAGE_LOCATION_FILTER_INFO_FB: StructureType = Self(1000156003i32);
    pub const SPACE_UUID_FILTER_INFO_FB: StructureType = Self(1000156054i32);
    pub const SPACE_COMPONENT_FILTER_INFO_FB: StructureType = Self(1000156052i32);
    pub const EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB: StructureType = Self(1000156103i32);
    pub const EVENT_DATA_SPACE_QUERY_COMPLETE_FB: StructureType = Self(1000156104i32);
    pub const SPACE_SAVE_INFO_FB: StructureType = Self(1000158000i32);
    pub const SPACE_ERASE_INFO_FB: StructureType = Self(1000158001i32);
    pub const EVENT_DATA_SPACE_SAVE_COMPLETE_FB: StructureType = Self(1000158106i32);
    pub const EVENT_DATA_SPACE_ERASE_COMPLETE_FB: StructureType = Self(1000158107i32);
    pub const SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB: StructureType = Self(1000160000i32);
    pub const SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB: StructureType = Self(1000161000i32);
    pub const SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB: StructureType = Self(1000162000i32);
    pub const SWAPCHAIN_STATE_SAMPLER_VULKAN_FB: StructureType = Self(1000163000i32);
    pub const SPACE_SHARE_INFO_FB: StructureType = Self(1000169001i32);
    pub const EVENT_DATA_SPACE_SHARE_COMPLETE_FB: StructureType = Self(1000169002i32);
    pub const COMPOSITION_LAYER_SPACE_WARP_INFO_FB: StructureType = Self(1000171000i32);
    pub const SYSTEM_SPACE_WARP_PROPERTIES_FB: StructureType = Self(1000171001i32);
    pub const HAPTIC_AMPLITUDE_ENVELOPE_VIBRATION_FB: StructureType = Self(1000173001i32);
    pub const SEMANTIC_LABELS_FB: StructureType = Self(1000175000i32);
    pub const ROOM_LAYOUT_FB: StructureType = Self(1000175001i32);
    pub const BOUNDARY_2D_FB: StructureType = Self(1000175002i32);
    pub const SEMANTIC_LABELS_SUPPORT_INFO_FB: StructureType = Self(1000175010i32);
    pub const DIGITAL_LENS_CONTROL_ALMALENCE: StructureType = Self(1000196000i32);
    pub const EVENT_DATA_SCENE_CAPTURE_COMPLETE_FB: StructureType = Self(1000198001i32);
    pub const SCENE_CAPTURE_REQUEST_INFO_FB: StructureType = Self(1000198050i32);
    pub const SPACE_CONTAINER_FB: StructureType = Self(1000199000i32);
    pub const FOVEATION_EYE_TRACKED_PROFILE_CREATE_INFO_META: StructureType = Self(1000200000i32);
    pub const FOVEATION_EYE_TRACKED_STATE_META: StructureType = Self(1000200001i32);
    pub const SYSTEM_FOVEATION_EYE_TRACKED_PROPERTIES_META: StructureType = Self(1000200002i32);
    pub const SYSTEM_FACE_TRACKING_PROPERTIES_FB: StructureType = Self(1000201004i32);
    pub const FACE_TRACKER_CREATE_INFO_FB: StructureType = Self(1000201005i32);
    pub const FACE_EXPRESSION_INFO_FB: StructureType = Self(1000201002i32);
    pub const FACE_EXPRESSION_WEIGHTS_FB: StructureType = Self(1000201006i32);
    pub const EYE_TRACKER_CREATE_INFO_FB: StructureType = Self(1000202001i32);
    pub const EYE_GAZES_INFO_FB: StructureType = Self(1000202002i32);
    pub const EYE_GAZES_FB: StructureType = Self(1000202003i32);
    pub const SYSTEM_EYE_TRACKING_PROPERTIES_FB: StructureType = Self(1000202004i32);
    pub const PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB: StructureType = Self(1000203002i32);
    pub const COMPOSITION_LAYER_SETTINGS_FB: StructureType = Self(1000204000i32);
    pub const HAPTIC_PCM_VIBRATION_FB: StructureType = Self(1000209001i32);
    pub const DEVICE_PCM_SAMPLE_RATE_STATE_FB: StructureType = Self(1000209002i32);
    pub const DEVICE_PCM_SAMPLE_RATE_GET_INFO_FB: StructureType =
        Self::DEVICE_PCM_SAMPLE_RATE_STATE_FB;
    pub const COMPOSITION_LAYER_DEPTH_TEST_FB: StructureType = Self(1000212000i32);
    pub const LOCAL_DIMMING_FRAME_END_INFO_META: StructureType = Self(1000216000i32);
    pub const PASSTHROUGH_PREFERENCES_META: StructureType = Self(1000217000i32);
    pub const SYSTEM_VIRTUAL_KEYBOARD_PROPERTIES_META: StructureType = Self(1000219001i32);
    pub const VIRTUAL_KEYBOARD_CREATE_INFO_META: StructureType = Self(1000219002i32);
    pub const VIRTUAL_KEYBOARD_SPACE_CREATE_INFO_META: StructureType = Self(1000219003i32);
    pub const VIRTUAL_KEYBOARD_LOCATION_INFO_META: StructureType = Self(1000219004i32);
    pub const VIRTUAL_KEYBOARD_MODEL_VISIBILITY_SET_INFO_META: StructureType = Self(1000219005i32);
    pub const VIRTUAL_KEYBOARD_ANIMATION_STATE_META: StructureType = Self(1000219006i32);
    pub const VIRTUAL_KEYBOARD_MODEL_ANIMATION_STATES_META: StructureType = Self(1000219007i32);
    pub const VIRTUAL_KEYBOARD_TEXTURE_DATA_META: StructureType = Self(1000219009i32);
    pub const VIRTUAL_KEYBOARD_INPUT_INFO_META: StructureType = Self(1000219010i32);
    pub const VIRTUAL_KEYBOARD_TEXT_CONTEXT_CHANGE_INFO_META: StructureType = Self(1000219011i32);
    pub const EVENT_DATA_VIRTUAL_KEYBOARD_COMMIT_TEXT_META: StructureType = Self(1000219014i32);
    pub const EVENT_DATA_VIRTUAL_KEYBOARD_BACKSPACE_META: StructureType = Self(1000219015i32);
    pub const EVENT_DATA_VIRTUAL_KEYBOARD_ENTER_META: StructureType = Self(1000219016i32);
    pub const EVENT_DATA_VIRTUAL_KEYBOARD_SHOWN_META: StructureType = Self(1000219017i32);
    pub const EVENT_DATA_VIRTUAL_KEYBOARD_HIDDEN_META: StructureType = Self(1000219018i32);
    pub const EXTERNAL_CAMERA_OCULUS: StructureType = Self(1000226000i32);
    pub const VULKAN_SWAPCHAIN_CREATE_INFO_META: StructureType = Self(1000227000i32);
    pub const PERFORMANCE_METRICS_STATE_META: StructureType = Self(1000232001i32);
    pub const PERFORMANCE_METRICS_COUNTER_META: StructureType = Self(1000232002i32);
    pub const SPACE_LIST_SAVE_INFO_FB: StructureType = Self(1000238000i32);
    pub const EVENT_DATA_SPACE_LIST_SAVE_COMPLETE_FB: StructureType = Self(1000238001i32);
    pub const SPACE_USER_CREATE_INFO_FB: StructureType = Self(1000241001i32);
    pub const SYSTEM_HEADSET_ID_PROPERTIES_META: StructureType = Self(1000245000i32);
    pub const RECOMMENDED_LAYER_RESOLUTION_META: StructureType = Self(1000254000i32);
    pub const RECOMMENDED_LAYER_RESOLUTION_GET_INFO_META: StructureType = Self(1000254001i32);
    pub const SYSTEM_PASSTHROUGH_COLOR_LUT_PROPERTIES_META: StructureType = Self(1000266000i32);
    pub const PASSTHROUGH_COLOR_LUT_CREATE_INFO_META: StructureType = Self(1000266001i32);
    pub const PASSTHROUGH_COLOR_LUT_UPDATE_INFO_META: StructureType = Self(1000266002i32);
    pub const PASSTHROUGH_COLOR_MAP_LUT_META: StructureType = Self(1000266100i32);
    pub const PASSTHROUGH_COLOR_MAP_INTERPOLATED_LUT_META: StructureType = Self(1000266101i32);
    pub const SPACE_TRIANGLE_MESH_GET_INFO_META: StructureType = Self(1000269001i32);
    pub const SPACE_TRIANGLE_MESH_META: StructureType = Self(1000269002i32);
    pub const SYSTEM_FACE_TRACKING_PROPERTIES2_FB: StructureType = Self(1000287013i32);
    pub const FACE_TRACKER_CREATE_INFO2_FB: StructureType = Self(1000287014i32);
    pub const FACE_EXPRESSION_INFO2_FB: StructureType = Self(1000287015i32);
    pub const FACE_EXPRESSION_WEIGHTS2_FB: StructureType = Self(1000287016i32);
    pub const ENVIRONMENT_DEPTH_PROVIDER_CREATE_INFO_META: StructureType = Self(1000291000i32);
    pub const ENVIRONMENT_DEPTH_SWAPCHAIN_CREATE_INFO_META: StructureType = Self(1000291001i32);
    pub const ENVIRONMENT_DEPTH_SWAPCHAIN_STATE_META: StructureType = Self(1000291002i32);
    pub const ENVIRONMENT_DEPTH_IMAGE_ACQUIRE_INFO_META: StructureType = Self(1000291003i32);
    pub const ENVIRONMENT_DEPTH_IMAGE_VIEW_META: StructureType = Self(1000291004i32);
    pub const ENVIRONMENT_DEPTH_IMAGE_META: StructureType = Self(1000291005i32);
    pub const ENVIRONMENT_DEPTH_HAND_REMOVAL_SET_INFO_META: StructureType = Self(1000291006i32);
    pub const SYSTEM_ENVIRONMENT_DEPTH_PROPERTIES_META: StructureType = Self(1000291007i32);
    pub const PASSTHROUGH_CREATE_INFO_HTC: StructureType = Self(1000317001i32);
    pub const PASSTHROUGH_COLOR_HTC: StructureType = Self(1000317002i32);
    pub const PASSTHROUGH_MESH_TRANSFORM_INFO_HTC: StructureType = Self(1000317003i32);
    pub const COMPOSITION_LAYER_PASSTHROUGH_HTC: StructureType = Self(1000317004i32);
    pub const FOVEATION_APPLY_INFO_HTC: StructureType = Self(1000318000i32);
    pub const FOVEATION_DYNAMIC_MODE_INFO_HTC: StructureType = Self(1000318001i32);
    pub const FOVEATION_CUSTOM_MODE_INFO_HTC: StructureType = Self(1000318002i32);
    pub const SYSTEM_ANCHOR_PROPERTIES_HTC: StructureType = Self(1000319000i32);
    pub const SPATIAL_ANCHOR_CREATE_INFO_HTC: StructureType = Self(1000319001i32);
    pub const ACTIVE_ACTION_SET_PRIORITIES_EXT: StructureType = Self(1000373000i32);
    pub const SYSTEM_FORCE_FEEDBACK_CURL_PROPERTIES_MNDX: StructureType = Self(1000375000i32);
    pub const FORCE_FEEDBACK_CURL_APPLY_LOCATIONS_MNDX: StructureType = Self(1000375001i32);
    pub const HAND_TRACKING_DATA_SOURCE_INFO_EXT: StructureType = Self(1000428000i32);
    pub const HAND_TRACKING_DATA_SOURCE_STATE_EXT: StructureType = Self(1000428001i32);
    pub const PLANE_DETECTOR_CREATE_INFO_EXT: StructureType = Self(1000429001i32);
    pub const PLANE_DETECTOR_BEGIN_INFO_EXT: StructureType = Self(1000429002i32);
    pub const PLANE_DETECTOR_GET_INFO_EXT: StructureType = Self(1000429003i32);
    pub const PLANE_DETECTOR_LOCATIONS_EXT: StructureType = Self(1000429004i32);
    pub const PLANE_DETECTOR_LOCATION_EXT: StructureType = Self(1000429005i32);
    pub const PLANE_DETECTOR_POLYGON_BUFFER_EXT: StructureType = Self(1000429006i32);
    pub const SYSTEM_PLANE_DETECTION_PROPERTIES_EXT: StructureType = Self(1000429007i32);
    pub const FUTURE_CANCEL_INFO_EXT: StructureType = Self(1000469000i32);
    pub const FUTURE_POLL_INFO_EXT: StructureType = Self(1000469001i32);
    pub const FUTURE_COMPLETION_EXT: StructureType = Self(1000469002i32);
    pub const FUTURE_POLL_RESULT_EXT: StructureType = Self(1000469003i32);
    pub const EVENT_DATA_USER_PRESENCE_CHANGED_EXT: StructureType = Self(1000470000i32);
    pub const SYSTEM_USER_PRESENCE_PROPERTIES_EXT: StructureType = Self(1000470001i32);
    pub const SPACES_LOCATE_INFO_KHR: StructureType = Self::SPACES_LOCATE_INFO;
    pub const SPACE_LOCATIONS_KHR: StructureType = Self::SPACE_LOCATIONS;
    pub const SPACE_VELOCITIES_KHR: StructureType = Self::SPACE_VELOCITIES;
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
            Self::SPACES_LOCATE_INFO => Some("SPACES_LOCATE_INFO"),
            Self::SPACE_LOCATIONS => Some("SPACE_LOCATIONS"),
            Self::SPACE_VELOCITIES => Some("SPACE_VELOCITIES"),
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
            Self::GRAPHICS_BINDING_METAL_KHR => Some("GRAPHICS_BINDING_METAL_KHR"),
            Self::SWAPCHAIN_IMAGE_METAL_KHR => Some("SWAPCHAIN_IMAGE_METAL_KHR"),
            Self::GRAPHICS_REQUIREMENTS_METAL_KHR => Some("GRAPHICS_REQUIREMENTS_METAL_KHR"),
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
            Self::SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT => {
                Some("SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT")
            }
            Self::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT => {
                Some("SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT")
            }
            Self::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT => {
                Some("SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT")
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
            Self::BODY_TRACKER_CREATE_INFO_FB => Some("BODY_TRACKER_CREATE_INFO_FB"),
            Self::BODY_JOINTS_LOCATE_INFO_FB => Some("BODY_JOINTS_LOCATE_INFO_FB"),
            Self::SYSTEM_BODY_TRACKING_PROPERTIES_FB => Some("SYSTEM_BODY_TRACKING_PROPERTIES_FB"),
            Self::BODY_JOINT_LOCATIONS_FB => Some("BODY_JOINT_LOCATIONS_FB"),
            Self::BODY_SKELETON_FB => Some("BODY_SKELETON_FB"),
            Self::INTERACTION_PROFILE_DPAD_BINDING_EXT => {
                Some("INTERACTION_PROFILE_DPAD_BINDING_EXT")
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
            Self::SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB => {
                Some("SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB")
            }
            Self::SPATIAL_ANCHOR_CREATE_INFO_FB => Some("SPATIAL_ANCHOR_CREATE_INFO_FB"),
            Self::SPACE_COMPONENT_STATUS_SET_INFO_FB => Some("SPACE_COMPONENT_STATUS_SET_INFO_FB"),
            Self::SPACE_COMPONENT_STATUS_FB => Some("SPACE_COMPONENT_STATUS_FB"),
            Self::EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB => {
                Some("EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB")
            }
            Self::EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB => {
                Some("EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB")
            }
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
            Self::SYSTEM_PASSTHROUGH_PROPERTIES2_FB => Some("SYSTEM_PASSTHROUGH_PROPERTIES2_FB"),
            Self::PASSTHROUGH_STYLE_FB => Some("PASSTHROUGH_STYLE_FB"),
            Self::PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB => {
                Some("PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB")
            }
            Self::PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB => {
                Some("PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB")
            }
            Self::PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB => {
                Some("PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB")
            }
            Self::EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB => {
                Some("EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB")
            }
            Self::RENDER_MODEL_PATH_INFO_FB => Some("RENDER_MODEL_PATH_INFO_FB"),
            Self::RENDER_MODEL_PROPERTIES_FB => Some("RENDER_MODEL_PROPERTIES_FB"),
            Self::RENDER_MODEL_BUFFER_FB => Some("RENDER_MODEL_BUFFER_FB"),
            Self::RENDER_MODEL_LOAD_INFO_FB => Some("RENDER_MODEL_LOAD_INFO_FB"),
            Self::SYSTEM_RENDER_MODEL_PROPERTIES_FB => Some("SYSTEM_RENDER_MODEL_PROPERTIES_FB"),
            Self::RENDER_MODEL_CAPABILITIES_REQUEST_FB => {
                Some("RENDER_MODEL_CAPABILITIES_REQUEST_FB")
            }
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
            Self::FRAME_END_INFO_ML => Some("FRAME_END_INFO_ML"),
            Self::GLOBAL_DIMMER_FRAME_END_INFO_ML => Some("GLOBAL_DIMMER_FRAME_END_INFO_ML"),
            Self::COORDINATE_SPACE_CREATE_INFO_ML => Some("COORDINATE_SPACE_CREATE_INFO_ML"),
            Self::SYSTEM_MARKER_UNDERSTANDING_PROPERTIES_ML => {
                Some("SYSTEM_MARKER_UNDERSTANDING_PROPERTIES_ML")
            }
            Self::MARKER_DETECTOR_CREATE_INFO_ML => Some("MARKER_DETECTOR_CREATE_INFO_ML"),
            Self::MARKER_DETECTOR_ARUCO_INFO_ML => Some("MARKER_DETECTOR_ARUCO_INFO_ML"),
            Self::MARKER_DETECTOR_SIZE_INFO_ML => Some("MARKER_DETECTOR_SIZE_INFO_ML"),
            Self::MARKER_DETECTOR_APRIL_TAG_INFO_ML => Some("MARKER_DETECTOR_APRIL_TAG_INFO_ML"),
            Self::MARKER_DETECTOR_CUSTOM_PROFILE_INFO_ML => {
                Some("MARKER_DETECTOR_CUSTOM_PROFILE_INFO_ML")
            }
            Self::MARKER_DETECTOR_SNAPSHOT_INFO_ML => Some("MARKER_DETECTOR_SNAPSHOT_INFO_ML"),
            Self::MARKER_DETECTOR_STATE_ML => Some("MARKER_DETECTOR_STATE_ML"),
            Self::MARKER_SPACE_CREATE_INFO_ML => Some("MARKER_SPACE_CREATE_INFO_ML"),
            Self::LOCALIZATION_MAP_ML => Some("LOCALIZATION_MAP_ML"),
            Self::EVENT_DATA_LOCALIZATION_CHANGED_ML => Some("EVENT_DATA_LOCALIZATION_CHANGED_ML"),
            Self::MAP_LOCALIZATION_REQUEST_INFO_ML => Some("MAP_LOCALIZATION_REQUEST_INFO_ML"),
            Self::LOCALIZATION_MAP_IMPORT_INFO_ML => Some("LOCALIZATION_MAP_IMPORT_INFO_ML"),
            Self::LOCALIZATION_ENABLE_EVENTS_INFO_ML => Some("LOCALIZATION_ENABLE_EVENTS_INFO_ML"),
            Self::EVENT_DATA_HEADSET_FIT_CHANGED_ML => Some("EVENT_DATA_HEADSET_FIT_CHANGED_ML"),
            Self::EVENT_DATA_EYE_CALIBRATION_CHANGED_ML => {
                Some("EVENT_DATA_EYE_CALIBRATION_CHANGED_ML")
            }
            Self::USER_CALIBRATION_ENABLE_EVENTS_INFO_ML => {
                Some("USER_CALIBRATION_ENABLE_EVENTS_INFO_ML")
            }
            Self::SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT")
            }
            Self::SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT")
            }
            Self::SCENE_MARKERS_MSFT => Some("SCENE_MARKERS_MSFT"),
            Self::SCENE_MARKER_TYPE_FILTER_MSFT => Some("SCENE_MARKER_TYPE_FILTER_MSFT"),
            Self::SCENE_MARKER_QR_CODES_MSFT => Some("SCENE_MARKER_QR_CODES_MSFT"),
            Self::SPACE_QUERY_INFO_FB => Some("SPACE_QUERY_INFO_FB"),
            Self::SPACE_QUERY_RESULTS_FB => Some("SPACE_QUERY_RESULTS_FB"),
            Self::SPACE_STORAGE_LOCATION_FILTER_INFO_FB => {
                Some("SPACE_STORAGE_LOCATION_FILTER_INFO_FB")
            }
            Self::SPACE_UUID_FILTER_INFO_FB => Some("SPACE_UUID_FILTER_INFO_FB"),
            Self::SPACE_COMPONENT_FILTER_INFO_FB => Some("SPACE_COMPONENT_FILTER_INFO_FB"),
            Self::EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB => {
                Some("EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB")
            }
            Self::EVENT_DATA_SPACE_QUERY_COMPLETE_FB => Some("EVENT_DATA_SPACE_QUERY_COMPLETE_FB"),
            Self::SPACE_SAVE_INFO_FB => Some("SPACE_SAVE_INFO_FB"),
            Self::SPACE_ERASE_INFO_FB => Some("SPACE_ERASE_INFO_FB"),
            Self::EVENT_DATA_SPACE_SAVE_COMPLETE_FB => Some("EVENT_DATA_SPACE_SAVE_COMPLETE_FB"),
            Self::EVENT_DATA_SPACE_ERASE_COMPLETE_FB => Some("EVENT_DATA_SPACE_ERASE_COMPLETE_FB"),
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
            Self::SPACE_SHARE_INFO_FB => Some("SPACE_SHARE_INFO_FB"),
            Self::EVENT_DATA_SPACE_SHARE_COMPLETE_FB => Some("EVENT_DATA_SPACE_SHARE_COMPLETE_FB"),
            Self::COMPOSITION_LAYER_SPACE_WARP_INFO_FB => {
                Some("COMPOSITION_LAYER_SPACE_WARP_INFO_FB")
            }
            Self::SYSTEM_SPACE_WARP_PROPERTIES_FB => Some("SYSTEM_SPACE_WARP_PROPERTIES_FB"),
            Self::HAPTIC_AMPLITUDE_ENVELOPE_VIBRATION_FB => {
                Some("HAPTIC_AMPLITUDE_ENVELOPE_VIBRATION_FB")
            }
            Self::SEMANTIC_LABELS_FB => Some("SEMANTIC_LABELS_FB"),
            Self::ROOM_LAYOUT_FB => Some("ROOM_LAYOUT_FB"),
            Self::BOUNDARY_2D_FB => Some("BOUNDARY_2D_FB"),
            Self::SEMANTIC_LABELS_SUPPORT_INFO_FB => Some("SEMANTIC_LABELS_SUPPORT_INFO_FB"),
            Self::DIGITAL_LENS_CONTROL_ALMALENCE => Some("DIGITAL_LENS_CONTROL_ALMALENCE"),
            Self::EVENT_DATA_SCENE_CAPTURE_COMPLETE_FB => {
                Some("EVENT_DATA_SCENE_CAPTURE_COMPLETE_FB")
            }
            Self::SCENE_CAPTURE_REQUEST_INFO_FB => Some("SCENE_CAPTURE_REQUEST_INFO_FB"),
            Self::SPACE_CONTAINER_FB => Some("SPACE_CONTAINER_FB"),
            Self::FOVEATION_EYE_TRACKED_PROFILE_CREATE_INFO_META => {
                Some("FOVEATION_EYE_TRACKED_PROFILE_CREATE_INFO_META")
            }
            Self::FOVEATION_EYE_TRACKED_STATE_META => Some("FOVEATION_EYE_TRACKED_STATE_META"),
            Self::SYSTEM_FOVEATION_EYE_TRACKED_PROPERTIES_META => {
                Some("SYSTEM_FOVEATION_EYE_TRACKED_PROPERTIES_META")
            }
            Self::SYSTEM_FACE_TRACKING_PROPERTIES_FB => Some("SYSTEM_FACE_TRACKING_PROPERTIES_FB"),
            Self::FACE_TRACKER_CREATE_INFO_FB => Some("FACE_TRACKER_CREATE_INFO_FB"),
            Self::FACE_EXPRESSION_INFO_FB => Some("FACE_EXPRESSION_INFO_FB"),
            Self::FACE_EXPRESSION_WEIGHTS_FB => Some("FACE_EXPRESSION_WEIGHTS_FB"),
            Self::EYE_TRACKER_CREATE_INFO_FB => Some("EYE_TRACKER_CREATE_INFO_FB"),
            Self::EYE_GAZES_INFO_FB => Some("EYE_GAZES_INFO_FB"),
            Self::EYE_GAZES_FB => Some("EYE_GAZES_FB"),
            Self::SYSTEM_EYE_TRACKING_PROPERTIES_FB => Some("SYSTEM_EYE_TRACKING_PROPERTIES_FB"),
            Self::PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB => {
                Some("PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB")
            }
            Self::COMPOSITION_LAYER_SETTINGS_FB => Some("COMPOSITION_LAYER_SETTINGS_FB"),
            Self::HAPTIC_PCM_VIBRATION_FB => Some("HAPTIC_PCM_VIBRATION_FB"),
            Self::DEVICE_PCM_SAMPLE_RATE_STATE_FB => Some("DEVICE_PCM_SAMPLE_RATE_STATE_FB"),
            Self::COMPOSITION_LAYER_DEPTH_TEST_FB => Some("COMPOSITION_LAYER_DEPTH_TEST_FB"),
            Self::LOCAL_DIMMING_FRAME_END_INFO_META => Some("LOCAL_DIMMING_FRAME_END_INFO_META"),
            Self::PASSTHROUGH_PREFERENCES_META => Some("PASSTHROUGH_PREFERENCES_META"),
            Self::SYSTEM_VIRTUAL_KEYBOARD_PROPERTIES_META => {
                Some("SYSTEM_VIRTUAL_KEYBOARD_PROPERTIES_META")
            }
            Self::VIRTUAL_KEYBOARD_CREATE_INFO_META => Some("VIRTUAL_KEYBOARD_CREATE_INFO_META"),
            Self::VIRTUAL_KEYBOARD_SPACE_CREATE_INFO_META => {
                Some("VIRTUAL_KEYBOARD_SPACE_CREATE_INFO_META")
            }
            Self::VIRTUAL_KEYBOARD_LOCATION_INFO_META => {
                Some("VIRTUAL_KEYBOARD_LOCATION_INFO_META")
            }
            Self::VIRTUAL_KEYBOARD_MODEL_VISIBILITY_SET_INFO_META => {
                Some("VIRTUAL_KEYBOARD_MODEL_VISIBILITY_SET_INFO_META")
            }
            Self::VIRTUAL_KEYBOARD_ANIMATION_STATE_META => {
                Some("VIRTUAL_KEYBOARD_ANIMATION_STATE_META")
            }
            Self::VIRTUAL_KEYBOARD_MODEL_ANIMATION_STATES_META => {
                Some("VIRTUAL_KEYBOARD_MODEL_ANIMATION_STATES_META")
            }
            Self::VIRTUAL_KEYBOARD_TEXTURE_DATA_META => Some("VIRTUAL_KEYBOARD_TEXTURE_DATA_META"),
            Self::VIRTUAL_KEYBOARD_INPUT_INFO_META => Some("VIRTUAL_KEYBOARD_INPUT_INFO_META"),
            Self::VIRTUAL_KEYBOARD_TEXT_CONTEXT_CHANGE_INFO_META => {
                Some("VIRTUAL_KEYBOARD_TEXT_CONTEXT_CHANGE_INFO_META")
            }
            Self::EVENT_DATA_VIRTUAL_KEYBOARD_COMMIT_TEXT_META => {
                Some("EVENT_DATA_VIRTUAL_KEYBOARD_COMMIT_TEXT_META")
            }
            Self::EVENT_DATA_VIRTUAL_KEYBOARD_BACKSPACE_META => {
                Some("EVENT_DATA_VIRTUAL_KEYBOARD_BACKSPACE_META")
            }
            Self::EVENT_DATA_VIRTUAL_KEYBOARD_ENTER_META => {
                Some("EVENT_DATA_VIRTUAL_KEYBOARD_ENTER_META")
            }
            Self::EVENT_DATA_VIRTUAL_KEYBOARD_SHOWN_META => {
                Some("EVENT_DATA_VIRTUAL_KEYBOARD_SHOWN_META")
            }
            Self::EVENT_DATA_VIRTUAL_KEYBOARD_HIDDEN_META => {
                Some("EVENT_DATA_VIRTUAL_KEYBOARD_HIDDEN_META")
            }
            Self::EXTERNAL_CAMERA_OCULUS => Some("EXTERNAL_CAMERA_OCULUS"),
            Self::VULKAN_SWAPCHAIN_CREATE_INFO_META => Some("VULKAN_SWAPCHAIN_CREATE_INFO_META"),
            Self::PERFORMANCE_METRICS_STATE_META => Some("PERFORMANCE_METRICS_STATE_META"),
            Self::PERFORMANCE_METRICS_COUNTER_META => Some("PERFORMANCE_METRICS_COUNTER_META"),
            Self::SPACE_LIST_SAVE_INFO_FB => Some("SPACE_LIST_SAVE_INFO_FB"),
            Self::EVENT_DATA_SPACE_LIST_SAVE_COMPLETE_FB => {
                Some("EVENT_DATA_SPACE_LIST_SAVE_COMPLETE_FB")
            }
            Self::SPACE_USER_CREATE_INFO_FB => Some("SPACE_USER_CREATE_INFO_FB"),
            Self::SYSTEM_HEADSET_ID_PROPERTIES_META => Some("SYSTEM_HEADSET_ID_PROPERTIES_META"),
            Self::RECOMMENDED_LAYER_RESOLUTION_META => Some("RECOMMENDED_LAYER_RESOLUTION_META"),
            Self::RECOMMENDED_LAYER_RESOLUTION_GET_INFO_META => {
                Some("RECOMMENDED_LAYER_RESOLUTION_GET_INFO_META")
            }
            Self::SYSTEM_PASSTHROUGH_COLOR_LUT_PROPERTIES_META => {
                Some("SYSTEM_PASSTHROUGH_COLOR_LUT_PROPERTIES_META")
            }
            Self::PASSTHROUGH_COLOR_LUT_CREATE_INFO_META => {
                Some("PASSTHROUGH_COLOR_LUT_CREATE_INFO_META")
            }
            Self::PASSTHROUGH_COLOR_LUT_UPDATE_INFO_META => {
                Some("PASSTHROUGH_COLOR_LUT_UPDATE_INFO_META")
            }
            Self::PASSTHROUGH_COLOR_MAP_LUT_META => Some("PASSTHROUGH_COLOR_MAP_LUT_META"),
            Self::PASSTHROUGH_COLOR_MAP_INTERPOLATED_LUT_META => {
                Some("PASSTHROUGH_COLOR_MAP_INTERPOLATED_LUT_META")
            }
            Self::SPACE_TRIANGLE_MESH_GET_INFO_META => Some("SPACE_TRIANGLE_MESH_GET_INFO_META"),
            Self::SPACE_TRIANGLE_MESH_META => Some("SPACE_TRIANGLE_MESH_META"),
            Self::SYSTEM_FACE_TRACKING_PROPERTIES2_FB => {
                Some("SYSTEM_FACE_TRACKING_PROPERTIES2_FB")
            }
            Self::FACE_TRACKER_CREATE_INFO2_FB => Some("FACE_TRACKER_CREATE_INFO2_FB"),
            Self::FACE_EXPRESSION_INFO2_FB => Some("FACE_EXPRESSION_INFO2_FB"),
            Self::FACE_EXPRESSION_WEIGHTS2_FB => Some("FACE_EXPRESSION_WEIGHTS2_FB"),
            Self::ENVIRONMENT_DEPTH_PROVIDER_CREATE_INFO_META => {
                Some("ENVIRONMENT_DEPTH_PROVIDER_CREATE_INFO_META")
            }
            Self::ENVIRONMENT_DEPTH_SWAPCHAIN_CREATE_INFO_META => {
                Some("ENVIRONMENT_DEPTH_SWAPCHAIN_CREATE_INFO_META")
            }
            Self::ENVIRONMENT_DEPTH_SWAPCHAIN_STATE_META => {
                Some("ENVIRONMENT_DEPTH_SWAPCHAIN_STATE_META")
            }
            Self::ENVIRONMENT_DEPTH_IMAGE_ACQUIRE_INFO_META => {
                Some("ENVIRONMENT_DEPTH_IMAGE_ACQUIRE_INFO_META")
            }
            Self::ENVIRONMENT_DEPTH_IMAGE_VIEW_META => Some("ENVIRONMENT_DEPTH_IMAGE_VIEW_META"),
            Self::ENVIRONMENT_DEPTH_IMAGE_META => Some("ENVIRONMENT_DEPTH_IMAGE_META"),
            Self::ENVIRONMENT_DEPTH_HAND_REMOVAL_SET_INFO_META => {
                Some("ENVIRONMENT_DEPTH_HAND_REMOVAL_SET_INFO_META")
            }
            Self::SYSTEM_ENVIRONMENT_DEPTH_PROPERTIES_META => {
                Some("SYSTEM_ENVIRONMENT_DEPTH_PROPERTIES_META")
            }
            Self::PASSTHROUGH_CREATE_INFO_HTC => Some("PASSTHROUGH_CREATE_INFO_HTC"),
            Self::PASSTHROUGH_COLOR_HTC => Some("PASSTHROUGH_COLOR_HTC"),
            Self::PASSTHROUGH_MESH_TRANSFORM_INFO_HTC => {
                Some("PASSTHROUGH_MESH_TRANSFORM_INFO_HTC")
            }
            Self::COMPOSITION_LAYER_PASSTHROUGH_HTC => Some("COMPOSITION_LAYER_PASSTHROUGH_HTC"),
            Self::FOVEATION_APPLY_INFO_HTC => Some("FOVEATION_APPLY_INFO_HTC"),
            Self::FOVEATION_DYNAMIC_MODE_INFO_HTC => Some("FOVEATION_DYNAMIC_MODE_INFO_HTC"),
            Self::FOVEATION_CUSTOM_MODE_INFO_HTC => Some("FOVEATION_CUSTOM_MODE_INFO_HTC"),
            Self::SYSTEM_ANCHOR_PROPERTIES_HTC => Some("SYSTEM_ANCHOR_PROPERTIES_HTC"),
            Self::SPATIAL_ANCHOR_CREATE_INFO_HTC => Some("SPATIAL_ANCHOR_CREATE_INFO_HTC"),
            Self::ACTIVE_ACTION_SET_PRIORITIES_EXT => Some("ACTIVE_ACTION_SET_PRIORITIES_EXT"),
            Self::SYSTEM_FORCE_FEEDBACK_CURL_PROPERTIES_MNDX => {
                Some("SYSTEM_FORCE_FEEDBACK_CURL_PROPERTIES_MNDX")
            }
            Self::FORCE_FEEDBACK_CURL_APPLY_LOCATIONS_MNDX => {
                Some("FORCE_FEEDBACK_CURL_APPLY_LOCATIONS_MNDX")
            }
            Self::HAND_TRACKING_DATA_SOURCE_INFO_EXT => Some("HAND_TRACKING_DATA_SOURCE_INFO_EXT"),
            Self::HAND_TRACKING_DATA_SOURCE_STATE_EXT => {
                Some("HAND_TRACKING_DATA_SOURCE_STATE_EXT")
            }
            Self::PLANE_DETECTOR_CREATE_INFO_EXT => Some("PLANE_DETECTOR_CREATE_INFO_EXT"),
            Self::PLANE_DETECTOR_BEGIN_INFO_EXT => Some("PLANE_DETECTOR_BEGIN_INFO_EXT"),
            Self::PLANE_DETECTOR_GET_INFO_EXT => Some("PLANE_DETECTOR_GET_INFO_EXT"),
            Self::PLANE_DETECTOR_LOCATIONS_EXT => Some("PLANE_DETECTOR_LOCATIONS_EXT"),
            Self::PLANE_DETECTOR_LOCATION_EXT => Some("PLANE_DETECTOR_LOCATION_EXT"),
            Self::PLANE_DETECTOR_POLYGON_BUFFER_EXT => Some("PLANE_DETECTOR_POLYGON_BUFFER_EXT"),
            Self::SYSTEM_PLANE_DETECTION_PROPERTIES_EXT => {
                Some("SYSTEM_PLANE_DETECTION_PROPERTIES_EXT")
            }
            Self::FUTURE_CANCEL_INFO_EXT => Some("FUTURE_CANCEL_INFO_EXT"),
            Self::FUTURE_POLL_INFO_EXT => Some("FUTURE_POLL_INFO_EXT"),
            Self::FUTURE_COMPLETION_EXT => Some("FUTURE_COMPLETION_EXT"),
            Self::FUTURE_POLL_RESULT_EXT => Some("FUTURE_POLL_RESULT_EXT"),
            Self::EVENT_DATA_USER_PRESENCE_CHANGED_EXT => {
                Some("EVENT_DATA_USER_PRESENCE_CHANGED_EXT")
            }
            Self::SYSTEM_USER_PRESENCE_PROPERTIES_EXT => {
                Some("SYSTEM_USER_PRESENCE_PROPERTIES_EXT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Error and return codes - see [XrResult](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrResult)"]
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
    #[doc = "One or more of the extensions being enabled has dependency on extensions that are not enabled."]
    pub const ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED: Result = Self(-1000710001i32);
    #[doc = "Insufficient permissions. This error is included for use by vendor extensions. The precise definition of `XR_ERROR_PERMISSION_INSUFFICIENT` and actions possible by the developer or user to resolve it can vary by platform, extension or function. The developer should refer to the documentation of the function that returned the error code and extension it was defined."]
    pub const ERROR_PERMISSION_INSUFFICIENT: Result = Self(-1000710000i32);
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
    #[doc = "The component type is not supported for this space."]
    pub const ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB: Result = Self(-1000113000i32);
    #[doc = "The required component is not enabled for this space."]
    pub const ERROR_SPACE_COMPONENT_NOT_ENABLED_FB: Result = Self(-1000113001i32);
    #[doc = "A request to set the component's status is currently pending."]
    pub const ERROR_SPACE_COMPONENT_STATUS_PENDING_FB: Result = Self(-1000113002i32);
    #[doc = "The component is already set to the requested value."]
    pub const ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB: Result = Self(-1000113003i32);
    #[doc = "The object state is unexpected for the issued command."]
    pub const ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB: Result = Self(-1000118000i32);
    #[doc = "Trying to create an MR feature when one was already created and only one instance is allowed."]
    pub const ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB: Result = Self(-1000118001i32);
    #[doc = "Requested functionality requires a feature to be created first."]
    pub const ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB: Result = Self(-1000118002i32);
    #[doc = "Requested functionality is not permitted - application is not allowed to perform the requested operation."]
    pub const ERROR_NOT_PERMITTED_PASSTHROUGH_FB: Result = Self(-1000118003i32);
    #[doc = "There were insufficient resources available to perform an operation."]
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
    #[doc = "The com.magicleap.permission.MARKER_TRACKING permission was denied."]
    pub const ERROR_MARKER_DETECTOR_PERMISSION_DENIED_ML: Result = Self(-1000138000i32);
    #[doc = "The specified marker could not be located spatially."]
    pub const ERROR_MARKER_DETECTOR_LOCATE_FAILED_ML: Result = Self(-1000138001i32);
    #[doc = "The marker queried does not contain data of the requested type."]
    pub const ERROR_MARKER_DETECTOR_INVALID_DATA_QUERY_ML: Result = Self(-1000138002i32);
    #[doc = "createInfo contains mutually exclusive parameters, such as setting XR_MARKER_DETECTOR_CORNER_REFINE_METHOD_APRIL_TAG_ML with XR_MARKER_TYPE_ARUCO_ML."]
    pub const ERROR_MARKER_DETECTOR_INVALID_CREATE_INFO_ML: Result = Self(-1000138003i32);
    #[doc = "The marker id passed to the function was invalid."]
    pub const ERROR_MARKER_INVALID_ML: Result = Self(-1000138004i32);
    #[doc = "The localization map being imported is not compatible with current OS or mode."]
    pub const ERROR_LOCALIZATION_MAP_INCOMPATIBLE_ML: Result = Self(-1000139000i32);
    #[doc = "The localization map requested is not available."]
    pub const ERROR_LOCALIZATION_MAP_UNAVAILABLE_ML: Result = Self(-1000139001i32);
    #[doc = "The map localization service failed to fulfill the request, retry later."]
    pub const ERROR_LOCALIZATION_MAP_FAIL_ML: Result = Self(-1000139002i32);
    #[doc = "The com.magicleap.permission.SPACE_IMPORT_EXPORT permission was denied."]
    pub const ERROR_LOCALIZATION_MAP_IMPORT_EXPORT_PERMISSION_DENIED_ML: Result =
        Self(-1000139003i32);
    #[doc = "The com.magicleap.permission.SPACE_MANAGER permission was denied."]
    pub const ERROR_LOCALIZATION_MAP_PERMISSION_DENIED_ML: Result = Self(-1000139004i32);
    #[doc = "The map being imported already exists in the system."]
    pub const ERROR_LOCALIZATION_MAP_ALREADY_EXISTS_ML: Result = Self(-1000139005i32);
    #[doc = "The map localization service cannot export cloud based maps."]
    pub const ERROR_LOCALIZATION_MAP_CANNOT_EXPORT_CLOUD_MAP_ML: Result = Self(-1000139006i32);
    #[doc = "A spatial anchor was not found associated with the spatial anchor name provided"]
    pub const ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT: Result = Self(-1000142001i32);
    #[doc = "The spatial anchor name provided was not valid"]
    pub const ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT: Result = Self(-1000142002i32);
    #[doc = "Marker does not encode a string."]
    pub const SCENE_MARKER_DATA_NOT_STRING_MSFT: Result = Self(1000147000i32);
    #[doc = "Anchor import from cloud or export from device failed."]
    pub const ERROR_SPACE_MAPPING_INSUFFICIENT_FB: Result = Self(-1000169000i32);
    #[doc = "Anchors were downloaded from the cloud but failed to be imported/aligned on the device."]
    pub const ERROR_SPACE_LOCALIZATION_FAILED_FB: Result = Self(-1000169001i32);
    #[doc = "Timeout occurred while waiting for network request to complete."]
    pub const ERROR_SPACE_NETWORK_TIMEOUT_FB: Result = Self(-1000169002i32);
    #[doc = "The network request failed."]
    pub const ERROR_SPACE_NETWORK_REQUEST_FAILED_FB: Result = Self(-1000169003i32);
    #[doc = "Cloud storage is required for this operation but is currently disabled."]
    pub const ERROR_SPACE_CLOUD_STORAGE_DISABLED_FB: Result = Self(-1000169004i32);
    #[doc = "The provided data buffer did not match the required size."]
    pub const ERROR_PASSTHROUGH_COLOR_LUT_BUFFER_SIZE_MISMATCH_META: Result = Self(-1000266000i32);
    #[doc = "Warning: The requested depth image is not yet available."]
    pub const ENVIRONMENT_DEPTH_NOT_AVAILABLE_META: Result = Self(1000291000i32);
    #[doc = "Tracking optimization hint is already set for the domain."]
    pub const ERROR_HINT_ALREADY_SET_QCOM: Result = Self(-1000306000i32);
    #[doc = "The provided space is valid but not an anchor."]
    pub const ERROR_NOT_AN_ANCHOR_HTC: Result = Self(-1000319000i32);
    #[doc = "The space passed to the function was not locatable."]
    pub const ERROR_SPACE_NOT_LOCATABLE_EXT: Result = Self(-1000429000i32);
    #[doc = "The permission for this resource was not granted."]
    pub const ERROR_PLANE_DETECTION_PERMISSION_DENIED_EXT: Result = Self(-1000429001i32);
    #[doc = "Returned by completion function to indicate future is not ready."]
    pub const ERROR_FUTURE_PENDING_EXT: Result = Self(-1000469001i32);
    #[doc = "Returned by completion function to indicate future is not valid."]
    pub const ERROR_FUTURE_INVALID_EXT: Result = Self(-1000469002i32);
    pub const ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED_KHR: Result =
        Self::ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED;
    pub const ERROR_PERMISSION_INSUFFICIENT_KHR: Result = Self::ERROR_PERMISSION_INSUFFICIENT;
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
            Self::ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED => {
                Some("ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED")
            }
            Self::ERROR_PERMISSION_INSUFFICIENT => Some("ERROR_PERMISSION_INSUFFICIENT"),
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
            Self::ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB => {
                Some("ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB")
            }
            Self::ERROR_SPACE_COMPONENT_NOT_ENABLED_FB => {
                Some("ERROR_SPACE_COMPONENT_NOT_ENABLED_FB")
            }
            Self::ERROR_SPACE_COMPONENT_STATUS_PENDING_FB => {
                Some("ERROR_SPACE_COMPONENT_STATUS_PENDING_FB")
            }
            Self::ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB => {
                Some("ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB")
            }
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
            Self::ERROR_MARKER_DETECTOR_PERMISSION_DENIED_ML => {
                Some("ERROR_MARKER_DETECTOR_PERMISSION_DENIED_ML")
            }
            Self::ERROR_MARKER_DETECTOR_LOCATE_FAILED_ML => {
                Some("ERROR_MARKER_DETECTOR_LOCATE_FAILED_ML")
            }
            Self::ERROR_MARKER_DETECTOR_INVALID_DATA_QUERY_ML => {
                Some("ERROR_MARKER_DETECTOR_INVALID_DATA_QUERY_ML")
            }
            Self::ERROR_MARKER_DETECTOR_INVALID_CREATE_INFO_ML => {
                Some("ERROR_MARKER_DETECTOR_INVALID_CREATE_INFO_ML")
            }
            Self::ERROR_MARKER_INVALID_ML => Some("ERROR_MARKER_INVALID_ML"),
            Self::ERROR_LOCALIZATION_MAP_INCOMPATIBLE_ML => {
                Some("ERROR_LOCALIZATION_MAP_INCOMPATIBLE_ML")
            }
            Self::ERROR_LOCALIZATION_MAP_UNAVAILABLE_ML => {
                Some("ERROR_LOCALIZATION_MAP_UNAVAILABLE_ML")
            }
            Self::ERROR_LOCALIZATION_MAP_FAIL_ML => Some("ERROR_LOCALIZATION_MAP_FAIL_ML"),
            Self::ERROR_LOCALIZATION_MAP_IMPORT_EXPORT_PERMISSION_DENIED_ML => {
                Some("ERROR_LOCALIZATION_MAP_IMPORT_EXPORT_PERMISSION_DENIED_ML")
            }
            Self::ERROR_LOCALIZATION_MAP_PERMISSION_DENIED_ML => {
                Some("ERROR_LOCALIZATION_MAP_PERMISSION_DENIED_ML")
            }
            Self::ERROR_LOCALIZATION_MAP_ALREADY_EXISTS_ML => {
                Some("ERROR_LOCALIZATION_MAP_ALREADY_EXISTS_ML")
            }
            Self::ERROR_LOCALIZATION_MAP_CANNOT_EXPORT_CLOUD_MAP_ML => {
                Some("ERROR_LOCALIZATION_MAP_CANNOT_EXPORT_CLOUD_MAP_ML")
            }
            Self::ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => {
                Some("ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT")
            }
            Self::ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => {
                Some("ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT")
            }
            Self::SCENE_MARKER_DATA_NOT_STRING_MSFT => Some("SCENE_MARKER_DATA_NOT_STRING_MSFT"),
            Self::ERROR_SPACE_MAPPING_INSUFFICIENT_FB => {
                Some("ERROR_SPACE_MAPPING_INSUFFICIENT_FB")
            }
            Self::ERROR_SPACE_LOCALIZATION_FAILED_FB => Some("ERROR_SPACE_LOCALIZATION_FAILED_FB"),
            Self::ERROR_SPACE_NETWORK_TIMEOUT_FB => Some("ERROR_SPACE_NETWORK_TIMEOUT_FB"),
            Self::ERROR_SPACE_NETWORK_REQUEST_FAILED_FB => {
                Some("ERROR_SPACE_NETWORK_REQUEST_FAILED_FB")
            }
            Self::ERROR_SPACE_CLOUD_STORAGE_DISABLED_FB => {
                Some("ERROR_SPACE_CLOUD_STORAGE_DISABLED_FB")
            }
            Self::ERROR_PASSTHROUGH_COLOR_LUT_BUFFER_SIZE_MISMATCH_META => {
                Some("ERROR_PASSTHROUGH_COLOR_LUT_BUFFER_SIZE_MISMATCH_META")
            }
            Self::ENVIRONMENT_DEPTH_NOT_AVAILABLE_META => {
                Some("ENVIRONMENT_DEPTH_NOT_AVAILABLE_META")
            }
            Self::ERROR_HINT_ALREADY_SET_QCOM => Some("ERROR_HINT_ALREADY_SET_QCOM"),
            Self::ERROR_NOT_AN_ANCHOR_HTC => Some("ERROR_NOT_AN_ANCHOR_HTC"),
            Self::ERROR_SPACE_NOT_LOCATABLE_EXT => Some("ERROR_SPACE_NOT_LOCATABLE_EXT"),
            Self::ERROR_PLANE_DETECTION_PERMISSION_DENIED_EXT => {
                Some("ERROR_PLANE_DETECTION_PERMISSION_DENIED_EXT")
            }
            Self::ERROR_FUTURE_PENDING_EXT => Some("ERROR_FUTURE_PENDING_EXT"),
            Self::ERROR_FUTURE_INVALID_EXT => Some("ERROR_FUTURE_INVALID_EXT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let reason = match *self {
            Self::SUCCESS => Some("function successfully completed"),
            Self::TIMEOUT_EXPIRED => {
                Some("the specified timeout time occurred before the operation could complete")
            }
            Self::SESSION_LOSS_PENDING => Some("the session will be lost soon"),
            Self::EVENT_UNAVAILABLE => Some("no event was available"),
            Self::SPACE_BOUNDS_UNAVAILABLE => {
                Some("the space's bounds are not known at the moment")
            }
            Self::SESSION_NOT_FOCUSED => Some("the session is not in the focused state"),
            Self::FRAME_DISCARDED => Some("a frame has been discarded from composition"),
            Self::ERROR_VALIDATION_FAILURE => Some("the function usage was invalid in some way"),
            Self::ERROR_RUNTIME_FAILURE => Some(
                "the runtime failed to handle the function in an unexpected way that is not covered by another error result",
            ),
            Self::ERROR_OUT_OF_MEMORY => Some("a memory allocation has failed"),
            Self::ERROR_API_VERSION_UNSUPPORTED => {
                Some("the runtime does not support the requested API version")
            }
            Self::ERROR_INITIALIZATION_FAILED => {
                Some("initialization of object could not be completed")
            }
            Self::ERROR_FUNCTION_UNSUPPORTED => {
                Some("the requested function was not found or is otherwise unsupported")
            }
            Self::ERROR_FEATURE_UNSUPPORTED => Some("the requested feature is not supported"),
            Self::ERROR_EXTENSION_NOT_PRESENT => Some("a requested extension is not supported"),
            Self::ERROR_LIMIT_REACHED => {
                Some("the runtime supports no more of the requested resource")
            }
            Self::ERROR_SIZE_INSUFFICIENT => Some("the supplied size was smaller than required"),
            Self::ERROR_HANDLE_INVALID => Some("a supplied object handle was invalid"),
            Self::ERROR_INSTANCE_LOST => Some(
                "the XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated",
            ),
            Self::ERROR_SESSION_RUNNING => Some("the session is already running"),
            Self::ERROR_SESSION_NOT_RUNNING => Some("the session is not yet running"),
            Self::ERROR_SESSION_LOST => Some(
                "the XrSession was lost. It will need to be destroyed and optionally recreated",
            ),
            Self::ERROR_SYSTEM_INVALID => Some("the provided XrSystemId was invalid"),
            Self::ERROR_PATH_INVALID => Some("the provided XrPath was not valid"),
            Self::ERROR_PATH_COUNT_EXCEEDED => {
                Some("the maximum number of supported semantic paths has been reached")
            }
            Self::ERROR_PATH_FORMAT_INVALID => {
                Some("the semantic path character format is invalid")
            }
            Self::ERROR_PATH_UNSUPPORTED => Some("the semantic path is unsupported"),
            Self::ERROR_LAYER_INVALID => Some("the layer was NULL or otherwise invalid"),
            Self::ERROR_LAYER_LIMIT_EXCEEDED => {
                Some("the number of specified layers is greater than the supported number")
            }
            Self::ERROR_SWAPCHAIN_RECT_INVALID => {
                Some("the image rect was negatively sized or otherwise invalid")
            }
            Self::ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => {
                Some("the image format is not supported by the runtime or platform")
            }
            Self::ERROR_ACTION_TYPE_MISMATCH => {
                Some("the API used to retrieve an action's state does not match the action's type")
            }
            Self::ERROR_SESSION_NOT_READY => Some("the session is not in the ready state"),
            Self::ERROR_SESSION_NOT_STOPPING => Some("the session is not in the stopping state"),
            Self::ERROR_TIME_INVALID => {
                Some("the provided XrTime was zero, negative, or out of range")
            }
            Self::ERROR_REFERENCE_SPACE_UNSUPPORTED => {
                Some("the specified reference space is not supported by the runtime or system")
            }
            Self::ERROR_FILE_ACCESS_ERROR => Some("the file could not be accessed"),
            Self::ERROR_FILE_CONTENTS_INVALID => Some("the file's contents were invalid"),
            Self::ERROR_FORM_FACTOR_UNSUPPORTED => Some(
                "the specified form factor is not supported by the current runtime or platform",
            ),
            Self::ERROR_FORM_FACTOR_UNAVAILABLE => Some(
                "the specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off",
            ),
            Self::ERROR_API_LAYER_NOT_PRESENT => {
                Some("a requested API layer is not present or could not be loaded")
            }
            Self::ERROR_CALL_ORDER_INVALID => {
                Some("the call was made without having made a previously required call")
            }
            Self::ERROR_GRAPHICS_DEVICE_INVALID => Some(
                "the given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements",
            ),
            Self::ERROR_POSE_INVALID => {
                Some("the supplied pose was invalid with respect to the requirements")
            }
            Self::ERROR_INDEX_OUT_OF_RANGE => {
                Some("the supplied index was outside the range of valid indices")
            }
            Self::ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => Some(
                "the specified view configuration type is not supported by the runtime or platform",
            ),
            Self::ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => Some(
                "the specified environment blend mode is not supported by the runtime or platform",
            ),
            Self::ERROR_NAME_DUPLICATED => {
                Some("the name provided was a duplicate of an already-existing resource")
            }
            Self::ERROR_NAME_INVALID => Some("the name provided was invalid"),
            Self::ERROR_ACTIONSET_NOT_ATTACHED => {
                Some("a referenced action set is not attached to the session")
            }
            Self::ERROR_ACTIONSETS_ALREADY_ATTACHED => {
                Some("the session already has attached action sets")
            }
            Self::ERROR_LOCALIZED_NAME_DUPLICATED => {
                Some("the localized name provided was a duplicate of an already-existing resource")
            }
            Self::ERROR_LOCALIZED_NAME_INVALID => Some("the localized name provided was invalid"),
            Self::ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING => Some(
                "the xrGetGraphicsRequirements* call was not made before calling xrCreateSession",
            ),
            Self::ERROR_RUNTIME_UNAVAILABLE => {
                Some("the loader was unable to find or load a runtime")
            }
            Self::ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED => Some(
                "one or more of the extensions being enabled has dependency on extensions that are not enabled",
            ),
            Self::ERROR_PERMISSION_INSUFFICIENT => Some(
                "insufficient permissions. This error is included for use by vendor extensions. The precise definition of `XR_ERROR_PERMISSION_INSUFFICIENT` and actions possible by the developer or user to resolve it can vary by platform, extension or function. The developer should refer to the documentation of the function that returned the error code and extension it was defined",
            ),
            Self::ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => {
                Some("xrSetAndroidApplicationThreadKHR failed as thread id is invalid")
            }
            Self::ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => Some(
                "xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority",
            ),
            Self::ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => {
                Some("spatial anchor could not be created at that location")
            }
            Self::ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT => {
                Some("the secondary view configuration was not enabled when creating the session")
            }
            Self::ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT => {
                Some("the controller model key is invalid")
            }
            Self::ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT => {
                Some("the reprojection mode is not supported")
            }
            Self::ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT => {
                Some("compute new scene not completed")
            }
            Self::ERROR_SCENE_COMPONENT_ID_INVALID_MSFT => Some("scene component id invalid"),
            Self::ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT => Some("scene component type mismatch"),
            Self::ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT => Some("scene mesh buffer id invalid"),
            Self::ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT => {
                Some("scene compute feature incompatible")
            }
            Self::ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT => {
                Some("scene compute consistency mismatch")
            }
            Self::ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB => {
                Some("the display refresh rate is not supported by the platform")
            }
            Self::ERROR_COLOR_SPACE_UNSUPPORTED_FB => {
                Some("the color space is not supported by the runtime")
            }
            Self::ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB => {
                Some("the component type is not supported for this space")
            }
            Self::ERROR_SPACE_COMPONENT_NOT_ENABLED_FB => {
                Some("the required component is not enabled for this space")
            }
            Self::ERROR_SPACE_COMPONENT_STATUS_PENDING_FB => {
                Some("a request to set the component's status is currently pending")
            }
            Self::ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB => {
                Some("the component is already set to the requested value")
            }
            Self::ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB => {
                Some("the object state is unexpected for the issued command")
            }
            Self::ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB => Some(
                "trying to create an MR feature when one was already created and only one instance is allowed",
            ),
            Self::ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB => {
                Some("requested functionality requires a feature to be created first")
            }
            Self::ERROR_NOT_PERMITTED_PASSTHROUGH_FB => Some(
                "requested functionality is not permitted - application is not allowed to perform the requested operation",
            ),
            Self::ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB => {
                Some("there were insufficient resources available to perform an operation")
            }
            Self::ERROR_UNKNOWN_PASSTHROUGH_FB => {
                Some("unknown Passthrough error (no further details provided)")
            }
            Self::ERROR_RENDER_MODEL_KEY_INVALID_FB => Some("the model key is invalid"),
            Self::RENDER_MODEL_UNAVAILABLE_FB => Some("the model is unavailable"),
            Self::ERROR_MARKER_NOT_TRACKED_VARJO => {
                Some("marker tracking is disabled or the specified marker is not currently tracked")
            }
            Self::ERROR_MARKER_ID_INVALID_VARJO => Some("the specified marker ID is not valid"),
            Self::ERROR_MARKER_DETECTOR_PERMISSION_DENIED_ML => {
                Some("the com.magicleap.permission.MARKER_TRACKING permission was denied")
            }
            Self::ERROR_MARKER_DETECTOR_LOCATE_FAILED_ML => {
                Some("the specified marker could not be located spatially")
            }
            Self::ERROR_MARKER_DETECTOR_INVALID_DATA_QUERY_ML => {
                Some("the marker queried does not contain data of the requested type")
            }
            Self::ERROR_MARKER_DETECTOR_INVALID_CREATE_INFO_ML => Some(
                "createInfo contains mutually exclusive parameters, such as setting XR_MARKER_DETECTOR_CORNER_REFINE_METHOD_APRIL_TAG_ML with XR_MARKER_TYPE_ARUCO_ML",
            ),
            Self::ERROR_MARKER_INVALID_ML => {
                Some("the marker id passed to the function was invalid")
            }
            Self::ERROR_LOCALIZATION_MAP_INCOMPATIBLE_ML => Some(
                "the localization map being imported is not compatible with current OS or mode",
            ),
            Self::ERROR_LOCALIZATION_MAP_UNAVAILABLE_ML => {
                Some("the localization map requested is not available")
            }
            Self::ERROR_LOCALIZATION_MAP_FAIL_ML => {
                Some("the map localization service failed to fulfill the request, retry later")
            }
            Self::ERROR_LOCALIZATION_MAP_IMPORT_EXPORT_PERMISSION_DENIED_ML => {
                Some("the com.magicleap.permission.SPACE_IMPORT_EXPORT permission was denied")
            }
            Self::ERROR_LOCALIZATION_MAP_PERMISSION_DENIED_ML => {
                Some("the com.magicleap.permission.SPACE_MANAGER permission was denied")
            }
            Self::ERROR_LOCALIZATION_MAP_ALREADY_EXISTS_ML => {
                Some("the map being imported already exists in the system")
            }
            Self::ERROR_LOCALIZATION_MAP_CANNOT_EXPORT_CLOUD_MAP_ML => {
                Some("the map localization service cannot export cloud based maps")
            }
            Self::ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => Some(
                "a spatial anchor was not found associated with the spatial anchor name provided",
            ),
            Self::ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => {
                Some("the spatial anchor name provided was not valid")
            }
            Self::SCENE_MARKER_DATA_NOT_STRING_MSFT => Some("marker does not encode a string"),
            Self::ERROR_SPACE_MAPPING_INSUFFICIENT_FB => {
                Some("anchor import from cloud or export from device failed")
            }
            Self::ERROR_SPACE_LOCALIZATION_FAILED_FB => Some(
                "anchors were downloaded from the cloud but failed to be imported/aligned on the device",
            ),
            Self::ERROR_SPACE_NETWORK_TIMEOUT_FB => {
                Some("timeout occurred while waiting for network request to complete")
            }
            Self::ERROR_SPACE_NETWORK_REQUEST_FAILED_FB => Some("the network request failed"),
            Self::ERROR_SPACE_CLOUD_STORAGE_DISABLED_FB => {
                Some("cloud storage is required for this operation but is currently disabled")
            }
            Self::ERROR_PASSTHROUGH_COLOR_LUT_BUFFER_SIZE_MISMATCH_META => {
                Some("the provided data buffer did not match the required size")
            }
            Self::ENVIRONMENT_DEPTH_NOT_AVAILABLE_META => {
                Some("warning: The requested depth image is not yet available")
            }
            Self::ERROR_HINT_ALREADY_SET_QCOM => {
                Some("tracking optimization hint is already set for the domain")
            }
            Self::ERROR_NOT_AN_ANCHOR_HTC => Some("the provided space is valid but not an anchor"),
            Self::ERROR_SPACE_NOT_LOCATABLE_EXT => {
                Some("the space passed to the function was not locatable")
            }
            Self::ERROR_PLANE_DETECTION_PERMISSION_DENIED_EXT => {
                Some("the permission for this resource was not granted")
            }
            Self::ERROR_FUTURE_PENDING_EXT => {
                Some("returned by completion function to indicate future is not ready")
            }
            Self::ERROR_FUTURE_INVALID_EXT => {
                Some("returned by completion function to indicate future is not valid")
            }
            Self::ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED_KHR => {
                Some("ERROR_EXTENSION_DEPENDENCY_NOT_ENABLED_KHR")
            }
            Self::ERROR_PERMISSION_INSUFFICIENT_KHR => Some("ERROR_PERMISSION_INSUFFICIENT_KHR"),
            _ => None,
        };
        if let Some(reason) = reason {
            fmt.pad(reason)
        } else {
            write!(fmt, "unknown error (code {})", self.0)
        }
    }
}
impl std::error::Error for Result {}
#[doc = "Enums to track objects of various types - see [XrObjectType](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrObjectType)"]
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
    #[doc = "XrSpatialGraphNodeBindingMSFT"]
    pub const SPATIAL_GRAPH_NODE_BINDING_MSFT: ObjectType = Self(1000049000i32);
    #[doc = "XrHandTrackerEXT"]
    pub const HAND_TRACKER_EXT: ObjectType = Self(1000051000i32);
    #[doc = "XrBodyTrackerFB"]
    pub const BODY_TRACKER_FB: ObjectType = Self(1000076000i32);
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
    #[doc = "XrMarkerDetectorML"]
    pub const MARKER_DETECTOR_ML: ObjectType = Self(1000138000i32);
    #[doc = "XrExportedLocalizationMapML"]
    pub const EXPORTED_LOCALIZATION_MAP_ML: ObjectType = Self(1000139000i32);
    #[doc = "XrSpatialAnchorStoreConnectionMSFT"]
    pub const SPATIAL_ANCHOR_STORE_CONNECTION_MSFT: ObjectType = Self(1000142000i32);
    #[doc = "XrFaceTrackerFB"]
    pub const FACE_TRACKER_FB: ObjectType = Self(1000201000i32);
    #[doc = "XrEyeTrackerFB"]
    pub const EYE_TRACKER_FB: ObjectType = Self(1000202000i32);
    #[doc = "XrVirtualKeyboardMETA"]
    pub const VIRTUAL_KEYBOARD_META: ObjectType = Self(1000219000i32);
    #[doc = "XrSpaceUserFB"]
    pub const SPACE_USER_FB: ObjectType = Self(1000241000i32);
    #[doc = "XrPassthroughColorLutMETA"]
    pub const PASSTHROUGH_COLOR_LUT_META: ObjectType = Self(1000266000i32);
    #[doc = "XrFaceTracker2FB"]
    pub const FACE_TRACKER2_FB: ObjectType = Self(1000287012i32);
    #[doc = "XrEnvironmentDepthProviderMETA"]
    pub const ENVIRONMENT_DEPTH_PROVIDER_META: ObjectType = Self(1000291000i32);
    #[doc = "XrEnvironmentDepthSwapchainMETA"]
    pub const ENVIRONMENT_DEPTH_SWAPCHAIN_META: ObjectType = Self(1000291001i32);
    #[doc = "XrPassthroughHTC"]
    pub const PASSTHROUGH_HTC: ObjectType = Self(1000317000i32);
    #[doc = "XrPlaneDetectorEXT"]
    pub const PLANE_DETECTOR_EXT: ObjectType = Self(1000429000i32);
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
            Self::SPATIAL_GRAPH_NODE_BINDING_MSFT => Some("SPATIAL_GRAPH_NODE_BINDING_MSFT"),
            Self::HAND_TRACKER_EXT => Some("HAND_TRACKER_EXT"),
            Self::BODY_TRACKER_FB => Some("BODY_TRACKER_FB"),
            Self::SCENE_OBSERVER_MSFT => Some("SCENE_OBSERVER_MSFT"),
            Self::SCENE_MSFT => Some("SCENE_MSFT"),
            Self::FACIAL_TRACKER_HTC => Some("FACIAL_TRACKER_HTC"),
            Self::FOVEATION_PROFILE_FB => Some("FOVEATION_PROFILE_FB"),
            Self::TRIANGLE_MESH_FB => Some("TRIANGLE_MESH_FB"),
            Self::PASSTHROUGH_FB => Some("PASSTHROUGH_FB"),
            Self::PASSTHROUGH_LAYER_FB => Some("PASSTHROUGH_LAYER_FB"),
            Self::GEOMETRY_INSTANCE_FB => Some("GEOMETRY_INSTANCE_FB"),
            Self::MARKER_DETECTOR_ML => Some("MARKER_DETECTOR_ML"),
            Self::EXPORTED_LOCALIZATION_MAP_ML => Some("EXPORTED_LOCALIZATION_MAP_ML"),
            Self::SPATIAL_ANCHOR_STORE_CONNECTION_MSFT => {
                Some("SPATIAL_ANCHOR_STORE_CONNECTION_MSFT")
            }
            Self::FACE_TRACKER_FB => Some("FACE_TRACKER_FB"),
            Self::EYE_TRACKER_FB => Some("EYE_TRACKER_FB"),
            Self::VIRTUAL_KEYBOARD_META => Some("VIRTUAL_KEYBOARD_META"),
            Self::SPACE_USER_FB => Some("SPACE_USER_FB"),
            Self::PASSTHROUGH_COLOR_LUT_META => Some("PASSTHROUGH_COLOR_LUT_META"),
            Self::FACE_TRACKER2_FB => Some("FACE_TRACKER2_FB"),
            Self::ENVIRONMENT_DEPTH_PROVIDER_META => Some("ENVIRONMENT_DEPTH_PROVIDER_META"),
            Self::ENVIRONMENT_DEPTH_SWAPCHAIN_META => Some("ENVIRONMENT_DEPTH_SWAPCHAIN_META"),
            Self::PASSTHROUGH_HTC => Some("PASSTHROUGH_HTC"),
            Self::PLANE_DETECTOR_EXT => Some("PLANE_DETECTOR_EXT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Android Thread Types - see [XrAndroidThreadTypeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrAndroidThreadTypeKHR)"]
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
#[doc = "eye visibility selector - see [XrEyeVisibility](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeVisibility)"]
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
#[doc = "See [XrActionType](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionType)"]
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
#[doc = "See [XrReferenceSpaceType](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrReferenceSpaceType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReferenceSpaceType(i32);
impl ReferenceSpaceType {
    pub const VIEW: ReferenceSpaceType = Self(1i32);
    pub const LOCAL: ReferenceSpaceType = Self(2i32);
    pub const STAGE: ReferenceSpaceType = Self(3i32);
    pub const LOCAL_FLOOR: ReferenceSpaceType = Self(1000426000i32);
    pub const UNBOUNDED_MSFT: ReferenceSpaceType = Self(1000038000i32);
    pub const COMBINED_EYE_VARJO: ReferenceSpaceType = Self(1000121000i32);
    pub const LOCALIZATION_MAP_ML: ReferenceSpaceType = Self(1000139000i32);
    pub const LOCAL_FLOOR_EXT: ReferenceSpaceType = Self::LOCAL_FLOOR;
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
            Self::LOCAL_FLOOR => Some("LOCAL_FLOOR"),
            Self::UNBOUNDED_MSFT => Some("UNBOUNDED_MSFT"),
            Self::COMBINED_EYE_VARJO => Some("COMBINED_EYE_VARJO"),
            Self::LOCALIZATION_MAP_ML => Some("LOCALIZATION_MAP_ML"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFormFactor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFormFactor)"]
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
#[doc = "See [XrViewConfigurationType](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewConfigurationType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ViewConfigurationType(i32);
impl ViewConfigurationType {
    pub const PRIMARY_MONO: ViewConfigurationType = Self(1i32);
    pub const PRIMARY_STEREO: ViewConfigurationType = Self(2i32);
    pub const PRIMARY_STEREO_WITH_FOVEATED_INSET: ViewConfigurationType = Self(1000037000i32);
    pub const PRIMARY_QUAD_VARJO: ViewConfigurationType = Self::PRIMARY_STEREO_WITH_FOVEATED_INSET;
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
            Self::PRIMARY_STEREO_WITH_FOVEATED_INSET => Some("PRIMARY_STEREO_WITH_FOVEATED_INSET"),
            Self::SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT => {
                Some("SECONDARY_MONO_FIRST_PERSON_OBSERVER_MSFT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrEnvironmentBlendMode](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentBlendMode)"]
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
#[doc = "See [XrSessionState](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionState)"]
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
#[doc = "See [XrPerfSettingsDomainEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerfSettingsDomainEXT)"]
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
#[doc = "See [XrPerfSettingsSubDomainEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerfSettingsSubDomainEXT)"]
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
#[doc = "See [XrPerfSettingsLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerfSettingsLevelEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsLevelEXT(i32);
impl PerfSettingsLevelEXT {
    #[doc = "Performance settings hint used by the application to indicate that it enters a non-XR section (head-locked / static screen), during which power savings are to be prioritized"]
    pub const POWER_SAVINGS: PerfSettingsLevelEXT = Self(0i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters a low and stable complexity section, during which reducing power is more important than occasional late rendering frames"]
    pub const SUSTAINED_LOW: PerfSettingsLevelEXT = Self(25i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters a high or dynamic complexity section, during which the XR Runtime strives for consistent XR compositing and frame rendering within a thermally sustainable range"]
    pub const SUSTAINED_HIGH: PerfSettingsLevelEXT = Self(50i32);
    #[doc = "Performance settings hint used by the application to indicate that the application enters a section with very high complexity, during which the XR Runtime is allowed to step up beyond the thermally sustainable range"]
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
#[doc = "See [XrPerfSettingsNotificationLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerfSettingsNotificationLevelEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsNotificationLevelEXT(i32);
impl PerfSettingsNotificationLevelEXT {
    #[doc = "Notifies that the sub-domain has reached a level where no further actions other than currently applied are necessary"]
    pub const NORMAL: PerfSettingsNotificationLevelEXT = Self(0i32);
    #[doc = "Notifies that the sub-domain has reached an early warning level where the application should start proactive mitigation actions with the goal to return to the XR_PERF_NOTIF_LEVEL_NORMAL level"]
    pub const WARNING: PerfSettingsNotificationLevelEXT = Self(25i32);
    #[doc = "Notifies that the sub-domain has reached a critical level with significant performance degradation. The application should take drastic mitigation action"]
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
#[doc = "See [XrVisibilityMaskTypeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVisibilityMaskTypeKHR)"]
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
#[doc = "See [XrSpatialGraphNodeTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphNodeTypeMSFT)"]
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
#[doc = "See [XrBlendFactorFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBlendFactorFB)"]
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
#[doc = "See [XrSpaceComponentTypeFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceComponentTypeFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpaceComponentTypeFB(i32);
impl SpaceComponentTypeFB {
    #[doc = "Enables tracking the 6 DOF pose of the XrSpace with xrLocateSpace."]
    pub const LOCATABLE: SpaceComponentTypeFB = Self(0i32);
    #[doc = "Enables persistence operations: save and erase."]
    pub const STORABLE: SpaceComponentTypeFB = Self(1i32);
    #[doc = "Enables sharing of spatial entities."]
    pub const SHARABLE: SpaceComponentTypeFB = Self(2i32);
    #[doc = "Bounded 2D component."]
    pub const BOUNDED_2D: SpaceComponentTypeFB = Self(3i32);
    #[doc = "Bounded 3D component."]
    pub const BOUNDED_3D: SpaceComponentTypeFB = Self(4i32);
    #[doc = "Semantic labels component."]
    pub const SEMANTIC_LABELS: SpaceComponentTypeFB = Self(5i32);
    #[doc = "Room layout component."]
    pub const ROOM_LAYOUT: SpaceComponentTypeFB = Self(6i32);
    #[doc = "Space container component."]
    pub const SPACE_CONTAINER: SpaceComponentTypeFB = Self(7i32);
    pub const TRIANGLE_MESH_M: SpaceComponentTypeFB = Self(1000269000i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SpaceComponentTypeFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOCATABLE => Some("LOCATABLE"),
            Self::STORABLE => Some("STORABLE"),
            Self::SHARABLE => Some("SHARABLE"),
            Self::BOUNDED_2D => Some("BOUNDED_2D"),
            Self::BOUNDED_3D => Some("BOUNDED_3D"),
            Self::SEMANTIC_LABELS => Some("SEMANTIC_LABELS"),
            Self::ROOM_LAYOUT => Some("ROOM_LAYOUT"),
            Self::SPACE_CONTAINER => Some("SPACE_CONTAINER"),
            Self::TRIANGLE_MESH_M => Some("TRIANGLE_MESH_M"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrWindingOrderFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrWindingOrderFB)"]
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
#[doc = "See [XrPassthroughLayerPurposeFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughLayerPurposeFB)"]
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
    #[doc = "Passthrough layer purpose for keyboard hands presence with keyboard masked hand transitions (i.e passthrough hands rendered only when they are over the keyboard)."]
    pub const TRACKED_KEYBOARD_MASKED_HANDS: PassthroughLayerPurposeFB = Self(1000203002i32);
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
            Self::TRACKED_KEYBOARD_MASKED_HANDS => Some("TRACKED_KEYBOARD_MASKED_HANDS"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSpaceQueryActionFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceQueryActionFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpaceQueryActionFB(i32);
impl SpaceQueryActionFB {
    #[doc = "Tells the query to perform a load operation on any XrSpace returned by the query."]
    pub const LOAD: SpaceQueryActionFB = Self(0i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SpaceQueryActionFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOAD => Some("LOAD"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSpaceStorageLocationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceStorageLocationFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpaceStorageLocationFB(i32);
impl SpaceStorageLocationFB {
    #[doc = "Invalid storage location"]
    pub const INVALID: SpaceStorageLocationFB = Self(0i32);
    #[doc = "Local device storage"]
    pub const LOCAL: SpaceStorageLocationFB = Self(1i32);
    #[doc = "Cloud storage"]
    pub const CLOUD: SpaceStorageLocationFB = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SpaceStorageLocationFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::INVALID => Some("INVALID"),
            Self::LOCAL => Some("LOCAL"),
            Self::CLOUD => Some("CLOUD"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSpacePersistenceModeFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpacePersistenceModeFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpacePersistenceModeFB(i32);
impl SpacePersistenceModeFB {
    #[doc = "Invalid storage persistence"]
    pub const INVALID: SpacePersistenceModeFB = Self(0i32);
    #[doc = "Store XrSpace indefinitely, or until erased"]
    pub const INDEFINITE: SpacePersistenceModeFB = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SpacePersistenceModeFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::INVALID => Some("INVALID"),
            Self::INDEFINITE => Some("INDEFINITE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrExternalCameraAttachedToDeviceOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExternalCameraAttachedToDeviceOCULUS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExternalCameraAttachedToDeviceOCULUS(i32);
impl ExternalCameraAttachedToDeviceOCULUS {
    #[doc = "External camera is at a fixed point in LOCAL space"]
    pub const NONE: ExternalCameraAttachedToDeviceOCULUS = Self(0i32);
    #[doc = "External camera is attached to the HMD"]
    pub const HMD: ExternalCameraAttachedToDeviceOCULUS = Self(1i32);
    #[doc = "External camera is attached to a left Touch controller"]
    pub const LTOUCH: ExternalCameraAttachedToDeviceOCULUS = Self(2i32);
    #[doc = "External camera is attached to a right Touch controller"]
    pub const RTOUCH: ExternalCameraAttachedToDeviceOCULUS = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ExternalCameraAttachedToDeviceOCULUS {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::HMD => Some("HMD"),
            Self::LTOUCH => Some("LTOUCH"),
            Self::RTOUCH => Some("RTOUCH"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPassthroughColorLutChannelsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorLutChannelsMETA)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PassthroughColorLutChannelsMETA(i32);
impl PassthroughColorLutChannelsMETA {
    pub const RGB: PassthroughColorLutChannelsMETA = Self(1i32);
    pub const RGBA: PassthroughColorLutChannelsMETA = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PassthroughColorLutChannelsMETA {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::RGB => Some("RGB"),
            Self::RGBA => Some("RGBA"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPerformanceMetricsCounterUnitMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerformanceMetricsCounterUnitMETA)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerformanceMetricsCounterUnitMETA(i32);
impl PerformanceMetricsCounterUnitMETA {
    #[doc = "the performance counter unit is generic (unspecified)."]
    pub const GENERIC: PerformanceMetricsCounterUnitMETA = Self(0i32);
    #[doc = "the performance counter unit is percentage (%)."]
    pub const PERCENTAGE: PerformanceMetricsCounterUnitMETA = Self(1i32);
    #[doc = "the performance counter unit is millisecond."]
    pub const MILLISECONDS: PerformanceMetricsCounterUnitMETA = Self(2i32);
    #[doc = "the performance counter unit is byte."]
    pub const BYTES: PerformanceMetricsCounterUnitMETA = Self(3i32);
    #[doc = "the performance counter unit is hertz (Hz)."]
    pub const HERTZ: PerformanceMetricsCounterUnitMETA = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PerformanceMetricsCounterUnitMETA {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::GENERIC => Some("GENERIC"),
            Self::PERCENTAGE => Some("PERCENTAGE"),
            Self::MILLISECONDS => Some("MILLISECONDS"),
            Self::BYTES => Some("BYTES"),
            Self::HERTZ => Some("HERTZ"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFacialTrackingTypeHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFacialTrackingTypeHTC)"]
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
#[doc = "See [XrEyeExpressionHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeExpressionHTC)"]
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
#[doc = "See [XrLipExpressionHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLipExpressionHTC)"]
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
#[doc = "See [XrPassthroughFormHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughFormHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PassthroughFormHTC(i32);
impl PassthroughFormHTC {
    #[doc = "Presents the passthrough with full of the entire screen."]
    pub const PLANAR: PassthroughFormHTC = Self(0i32);
    #[doc = "Presents the passthrough projecting onto a custom mesh."]
    pub const PROJECTED: PassthroughFormHTC = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PassthroughFormHTC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PLANAR => Some("PLANAR"),
            Self::PROJECTED => Some("PROJECTED"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFoveationModeHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationModeHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FoveationModeHTC(i32);
impl FoveationModeHTC {
    #[doc = "No foveation"]
    pub const DISABLE: FoveationModeHTC = Self(0i32);
    #[doc = "Apply system default setting with fixed clear FOV and periphery quality."]
    pub const FIXED: FoveationModeHTC = Self(1i32);
    #[doc = "Allow system to set foveation dynamically according realtime system metric or other extensions."]
    pub const DYNAMIC: FoveationModeHTC = Self(2i32);
    #[doc = "Allow application to set foveation with desired clear FOV, periphery quality, and focal center offset."]
    pub const CUSTOM: FoveationModeHTC = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FoveationModeHTC {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DISABLE => Some("DISABLE"),
            Self::FIXED => Some("FIXED"),
            Self::DYNAMIC => Some("DYNAMIC"),
            Self::CUSTOM => Some("CUSTOM"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFoveationLevelHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationLevelHTC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FoveationLevelHTC(i32);
impl FoveationLevelHTC {
    #[doc = "No foveation"]
    pub const NONE: FoveationLevelHTC = Self(0i32);
    #[doc = "Light periphery pixel density drop and lower performance gain."]
    pub const LOW: FoveationLevelHTC = Self(1i32);
    #[doc = "Medium periphery pixel density drop and medium performance gain"]
    pub const MEDIUM: FoveationLevelHTC = Self(2i32);
    #[doc = "Heavy periphery pixel density drop and higher performance gain"]
    pub const HIGH: FoveationLevelHTC = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FoveationLevelHTC {
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
#[doc = "See [XrLocalDimmingModeMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalDimmingModeMETA)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalDimmingModeMETA(i32);
impl LocalDimmingModeMETA {
    #[doc = "Local dimming is turned off by default for the current submitted frame. This is the same as not chaining XrLocalDimmingModeMETA."]
    pub const OFF: LocalDimmingModeMETA = Self(0i32);
    #[doc = "Local dimming is turned on for the current submitted frame."]
    pub const ON: LocalDimmingModeMETA = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LocalDimmingModeMETA {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OFF => Some("OFF"),
            Self::ON => Some("ON"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceExpressionFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceExpressionFB(i32);
impl FaceExpressionFB {
    pub const BROW_LOWERER_L: FaceExpressionFB = Self(0i32);
    pub const BROW_LOWERER_R: FaceExpressionFB = Self(1i32);
    pub const CHEEK_PUFF_L: FaceExpressionFB = Self(2i32);
    pub const CHEEK_PUFF_R: FaceExpressionFB = Self(3i32);
    pub const CHEEK_RAISER_L: FaceExpressionFB = Self(4i32);
    pub const CHEEK_RAISER_R: FaceExpressionFB = Self(5i32);
    pub const CHEEK_SUCK_L: FaceExpressionFB = Self(6i32);
    pub const CHEEK_SUCK_R: FaceExpressionFB = Self(7i32);
    pub const CHIN_RAISER_B: FaceExpressionFB = Self(8i32);
    pub const CHIN_RAISER_T: FaceExpressionFB = Self(9i32);
    pub const DIMPLER_L: FaceExpressionFB = Self(10i32);
    pub const DIMPLER_R: FaceExpressionFB = Self(11i32);
    pub const EYES_CLOSED_L: FaceExpressionFB = Self(12i32);
    pub const EYES_CLOSED_R: FaceExpressionFB = Self(13i32);
    pub const EYES_LOOK_DOWN_L: FaceExpressionFB = Self(14i32);
    pub const EYES_LOOK_DOWN_R: FaceExpressionFB = Self(15i32);
    pub const EYES_LOOK_LEFT_L: FaceExpressionFB = Self(16i32);
    pub const EYES_LOOK_LEFT_R: FaceExpressionFB = Self(17i32);
    pub const EYES_LOOK_RIGHT_L: FaceExpressionFB = Self(18i32);
    pub const EYES_LOOK_RIGHT_R: FaceExpressionFB = Self(19i32);
    pub const EYES_LOOK_UP_L: FaceExpressionFB = Self(20i32);
    pub const EYES_LOOK_UP_R: FaceExpressionFB = Self(21i32);
    pub const INNER_BROW_RAISER_L: FaceExpressionFB = Self(22i32);
    pub const INNER_BROW_RAISER_R: FaceExpressionFB = Self(23i32);
    pub const JAW_DROP: FaceExpressionFB = Self(24i32);
    pub const JAW_SIDEWAYS_LEFT: FaceExpressionFB = Self(25i32);
    pub const JAW_SIDEWAYS_RIGHT: FaceExpressionFB = Self(26i32);
    pub const JAW_THRUST: FaceExpressionFB = Self(27i32);
    pub const LID_TIGHTENER_L: FaceExpressionFB = Self(28i32);
    pub const LID_TIGHTENER_R: FaceExpressionFB = Self(29i32);
    pub const LIP_CORNER_DEPRESSOR_L: FaceExpressionFB = Self(30i32);
    pub const LIP_CORNER_DEPRESSOR_R: FaceExpressionFB = Self(31i32);
    pub const LIP_CORNER_PULLER_L: FaceExpressionFB = Self(32i32);
    pub const LIP_CORNER_PULLER_R: FaceExpressionFB = Self(33i32);
    pub const LIP_FUNNELER_LB: FaceExpressionFB = Self(34i32);
    pub const LIP_FUNNELER_LT: FaceExpressionFB = Self(35i32);
    pub const LIP_FUNNELER_RB: FaceExpressionFB = Self(36i32);
    pub const LIP_FUNNELER_RT: FaceExpressionFB = Self(37i32);
    pub const LIP_PRESSOR_L: FaceExpressionFB = Self(38i32);
    pub const LIP_PRESSOR_R: FaceExpressionFB = Self(39i32);
    pub const LIP_PUCKER_L: FaceExpressionFB = Self(40i32);
    pub const LIP_PUCKER_R: FaceExpressionFB = Self(41i32);
    pub const LIP_STRETCHER_L: FaceExpressionFB = Self(42i32);
    pub const LIP_STRETCHER_R: FaceExpressionFB = Self(43i32);
    pub const LIP_SUCK_LB: FaceExpressionFB = Self(44i32);
    pub const LIP_SUCK_LT: FaceExpressionFB = Self(45i32);
    pub const LIP_SUCK_RB: FaceExpressionFB = Self(46i32);
    pub const LIP_SUCK_RT: FaceExpressionFB = Self(47i32);
    pub const LIP_TIGHTENER_L: FaceExpressionFB = Self(48i32);
    pub const LIP_TIGHTENER_R: FaceExpressionFB = Self(49i32);
    pub const LIPS_TOWARD: FaceExpressionFB = Self(50i32);
    pub const LOWER_LIP_DEPRESSOR_L: FaceExpressionFB = Self(51i32);
    pub const LOWER_LIP_DEPRESSOR_R: FaceExpressionFB = Self(52i32);
    pub const MOUTH_LEFT: FaceExpressionFB = Self(53i32);
    pub const MOUTH_RIGHT: FaceExpressionFB = Self(54i32);
    pub const NOSE_WRINKLER_L: FaceExpressionFB = Self(55i32);
    pub const NOSE_WRINKLER_R: FaceExpressionFB = Self(56i32);
    pub const OUTER_BROW_RAISER_L: FaceExpressionFB = Self(57i32);
    pub const OUTER_BROW_RAISER_R: FaceExpressionFB = Self(58i32);
    pub const UPPER_LID_RAISER_L: FaceExpressionFB = Self(59i32);
    pub const UPPER_LID_RAISER_R: FaceExpressionFB = Self(60i32);
    pub const UPPER_LIP_RAISER_L: FaceExpressionFB = Self(61i32);
    pub const UPPER_LIP_RAISER_R: FaceExpressionFB = Self(62i32);
    pub const COUNT: FaceExpressionFB = Self(63i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceExpressionFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::BROW_LOWERER_L => Some("BROW_LOWERER_L"),
            Self::BROW_LOWERER_R => Some("BROW_LOWERER_R"),
            Self::CHEEK_PUFF_L => Some("CHEEK_PUFF_L"),
            Self::CHEEK_PUFF_R => Some("CHEEK_PUFF_R"),
            Self::CHEEK_RAISER_L => Some("CHEEK_RAISER_L"),
            Self::CHEEK_RAISER_R => Some("CHEEK_RAISER_R"),
            Self::CHEEK_SUCK_L => Some("CHEEK_SUCK_L"),
            Self::CHEEK_SUCK_R => Some("CHEEK_SUCK_R"),
            Self::CHIN_RAISER_B => Some("CHIN_RAISER_B"),
            Self::CHIN_RAISER_T => Some("CHIN_RAISER_T"),
            Self::DIMPLER_L => Some("DIMPLER_L"),
            Self::DIMPLER_R => Some("DIMPLER_R"),
            Self::EYES_CLOSED_L => Some("EYES_CLOSED_L"),
            Self::EYES_CLOSED_R => Some("EYES_CLOSED_R"),
            Self::EYES_LOOK_DOWN_L => Some("EYES_LOOK_DOWN_L"),
            Self::EYES_LOOK_DOWN_R => Some("EYES_LOOK_DOWN_R"),
            Self::EYES_LOOK_LEFT_L => Some("EYES_LOOK_LEFT_L"),
            Self::EYES_LOOK_LEFT_R => Some("EYES_LOOK_LEFT_R"),
            Self::EYES_LOOK_RIGHT_L => Some("EYES_LOOK_RIGHT_L"),
            Self::EYES_LOOK_RIGHT_R => Some("EYES_LOOK_RIGHT_R"),
            Self::EYES_LOOK_UP_L => Some("EYES_LOOK_UP_L"),
            Self::EYES_LOOK_UP_R => Some("EYES_LOOK_UP_R"),
            Self::INNER_BROW_RAISER_L => Some("INNER_BROW_RAISER_L"),
            Self::INNER_BROW_RAISER_R => Some("INNER_BROW_RAISER_R"),
            Self::JAW_DROP => Some("JAW_DROP"),
            Self::JAW_SIDEWAYS_LEFT => Some("JAW_SIDEWAYS_LEFT"),
            Self::JAW_SIDEWAYS_RIGHT => Some("JAW_SIDEWAYS_RIGHT"),
            Self::JAW_THRUST => Some("JAW_THRUST"),
            Self::LID_TIGHTENER_L => Some("LID_TIGHTENER_L"),
            Self::LID_TIGHTENER_R => Some("LID_TIGHTENER_R"),
            Self::LIP_CORNER_DEPRESSOR_L => Some("LIP_CORNER_DEPRESSOR_L"),
            Self::LIP_CORNER_DEPRESSOR_R => Some("LIP_CORNER_DEPRESSOR_R"),
            Self::LIP_CORNER_PULLER_L => Some("LIP_CORNER_PULLER_L"),
            Self::LIP_CORNER_PULLER_R => Some("LIP_CORNER_PULLER_R"),
            Self::LIP_FUNNELER_LB => Some("LIP_FUNNELER_LB"),
            Self::LIP_FUNNELER_LT => Some("LIP_FUNNELER_LT"),
            Self::LIP_FUNNELER_RB => Some("LIP_FUNNELER_RB"),
            Self::LIP_FUNNELER_RT => Some("LIP_FUNNELER_RT"),
            Self::LIP_PRESSOR_L => Some("LIP_PRESSOR_L"),
            Self::LIP_PRESSOR_R => Some("LIP_PRESSOR_R"),
            Self::LIP_PUCKER_L => Some("LIP_PUCKER_L"),
            Self::LIP_PUCKER_R => Some("LIP_PUCKER_R"),
            Self::LIP_STRETCHER_L => Some("LIP_STRETCHER_L"),
            Self::LIP_STRETCHER_R => Some("LIP_STRETCHER_R"),
            Self::LIP_SUCK_LB => Some("LIP_SUCK_LB"),
            Self::LIP_SUCK_LT => Some("LIP_SUCK_LT"),
            Self::LIP_SUCK_RB => Some("LIP_SUCK_RB"),
            Self::LIP_SUCK_RT => Some("LIP_SUCK_RT"),
            Self::LIP_TIGHTENER_L => Some("LIP_TIGHTENER_L"),
            Self::LIP_TIGHTENER_R => Some("LIP_TIGHTENER_R"),
            Self::LIPS_TOWARD => Some("LIPS_TOWARD"),
            Self::LOWER_LIP_DEPRESSOR_L => Some("LOWER_LIP_DEPRESSOR_L"),
            Self::LOWER_LIP_DEPRESSOR_R => Some("LOWER_LIP_DEPRESSOR_R"),
            Self::MOUTH_LEFT => Some("MOUTH_LEFT"),
            Self::MOUTH_RIGHT => Some("MOUTH_RIGHT"),
            Self::NOSE_WRINKLER_L => Some("NOSE_WRINKLER_L"),
            Self::NOSE_WRINKLER_R => Some("NOSE_WRINKLER_R"),
            Self::OUTER_BROW_RAISER_L => Some("OUTER_BROW_RAISER_L"),
            Self::OUTER_BROW_RAISER_R => Some("OUTER_BROW_RAISER_R"),
            Self::UPPER_LID_RAISER_L => Some("UPPER_LID_RAISER_L"),
            Self::UPPER_LID_RAISER_R => Some("UPPER_LID_RAISER_R"),
            Self::UPPER_LIP_RAISER_L => Some("UPPER_LIP_RAISER_L"),
            Self::UPPER_LIP_RAISER_R => Some("UPPER_LIP_RAISER_R"),
            Self::COUNT => Some("COUNT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceExpressionSetFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionSetFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceExpressionSetFB(i32);
impl FaceExpressionSetFB {
    #[doc = "indicates that the created XrFaceTrackerFB tracks the set of blend shapes described by XrFaceExpressionFB enum, i.e. the xrGetFaceExpressionWeightsFB function returns an array of blend shapes with the count of XR_FACE_EXPRESSION_COUNT_FB and can: be indexed using XrFaceExpressionFB."]
    pub const DEFAULT: FaceExpressionSetFB = Self(0i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceExpressionSetFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceConfidenceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceConfidenceFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceConfidenceFB(i32);
impl FaceConfidenceFB {
    pub const LOWER_FACE: FaceConfidenceFB = Self(0i32);
    pub const UPPER_FACE: FaceConfidenceFB = Self(1i32);
    pub const COUNT: FaceConfidenceFB = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceConfidenceFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOWER_FACE => Some("LOWER_FACE"),
            Self::UPPER_FACE => Some("UPPER_FACE"),
            Self::COUNT => Some("COUNT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceExpression2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpression2FB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceExpression2FB(i32);
impl FaceExpression2FB {
    pub const BROW_LOWERER_L: FaceExpression2FB = Self(0i32);
    pub const BROW_LOWERER_R: FaceExpression2FB = Self(1i32);
    pub const CHEEK_PUFF_L: FaceExpression2FB = Self(2i32);
    pub const CHEEK_PUFF_R: FaceExpression2FB = Self(3i32);
    pub const CHEEK_RAISER_L: FaceExpression2FB = Self(4i32);
    pub const CHEEK_RAISER_R: FaceExpression2FB = Self(5i32);
    pub const CHEEK_SUCK_L: FaceExpression2FB = Self(6i32);
    pub const CHEEK_SUCK_R: FaceExpression2FB = Self(7i32);
    pub const CHIN_RAISER_B: FaceExpression2FB = Self(8i32);
    pub const CHIN_RAISER_T: FaceExpression2FB = Self(9i32);
    pub const DIMPLER_L: FaceExpression2FB = Self(10i32);
    pub const DIMPLER_R: FaceExpression2FB = Self(11i32);
    pub const EYES_CLOSED_L: FaceExpression2FB = Self(12i32);
    pub const EYES_CLOSED_R: FaceExpression2FB = Self(13i32);
    pub const EYES_LOOK_DOWN_L: FaceExpression2FB = Self(14i32);
    pub const EYES_LOOK_DOWN_R: FaceExpression2FB = Self(15i32);
    pub const EYES_LOOK_LEFT_L: FaceExpression2FB = Self(16i32);
    pub const EYES_LOOK_LEFT_R: FaceExpression2FB = Self(17i32);
    pub const EYES_LOOK_RIGHT_L: FaceExpression2FB = Self(18i32);
    pub const EYES_LOOK_RIGHT_R: FaceExpression2FB = Self(19i32);
    pub const EYES_LOOK_UP_L: FaceExpression2FB = Self(20i32);
    pub const EYES_LOOK_UP_R: FaceExpression2FB = Self(21i32);
    pub const INNER_BROW_RAISER_L: FaceExpression2FB = Self(22i32);
    pub const INNER_BROW_RAISER_R: FaceExpression2FB = Self(23i32);
    pub const JAW_DROP: FaceExpression2FB = Self(24i32);
    pub const JAW_SIDEWAYS_LEFT: FaceExpression2FB = Self(25i32);
    pub const JAW_SIDEWAYS_RIGHT: FaceExpression2FB = Self(26i32);
    pub const JAW_THRUST: FaceExpression2FB = Self(27i32);
    pub const LID_TIGHTENER_L: FaceExpression2FB = Self(28i32);
    pub const LID_TIGHTENER_R: FaceExpression2FB = Self(29i32);
    pub const LIP_CORNER_DEPRESSOR_L: FaceExpression2FB = Self(30i32);
    pub const LIP_CORNER_DEPRESSOR_R: FaceExpression2FB = Self(31i32);
    pub const LIP_CORNER_PULLER_L: FaceExpression2FB = Self(32i32);
    pub const LIP_CORNER_PULLER_R: FaceExpression2FB = Self(33i32);
    pub const LIP_FUNNELER_LB: FaceExpression2FB = Self(34i32);
    pub const LIP_FUNNELER_LT: FaceExpression2FB = Self(35i32);
    pub const LIP_FUNNELER_RB: FaceExpression2FB = Self(36i32);
    pub const LIP_FUNNELER_RT: FaceExpression2FB = Self(37i32);
    pub const LIP_PRESSOR_L: FaceExpression2FB = Self(38i32);
    pub const LIP_PRESSOR_R: FaceExpression2FB = Self(39i32);
    pub const LIP_PUCKER_L: FaceExpression2FB = Self(40i32);
    pub const LIP_PUCKER_R: FaceExpression2FB = Self(41i32);
    pub const LIP_STRETCHER_L: FaceExpression2FB = Self(42i32);
    pub const LIP_STRETCHER_R: FaceExpression2FB = Self(43i32);
    pub const LIP_SUCK_LB: FaceExpression2FB = Self(44i32);
    pub const LIP_SUCK_LT: FaceExpression2FB = Self(45i32);
    pub const LIP_SUCK_RB: FaceExpression2FB = Self(46i32);
    pub const LIP_SUCK_RT: FaceExpression2FB = Self(47i32);
    pub const LIP_TIGHTENER_L: FaceExpression2FB = Self(48i32);
    pub const LIP_TIGHTENER_R: FaceExpression2FB = Self(49i32);
    pub const LIPS_TOWARD: FaceExpression2FB = Self(50i32);
    pub const LOWER_LIP_DEPRESSOR_L: FaceExpression2FB = Self(51i32);
    pub const LOWER_LIP_DEPRESSOR_R: FaceExpression2FB = Self(52i32);
    pub const MOUTH_LEFT: FaceExpression2FB = Self(53i32);
    pub const MOUTH_RIGHT: FaceExpression2FB = Self(54i32);
    pub const NOSE_WRINKLER_L: FaceExpression2FB = Self(55i32);
    pub const NOSE_WRINKLER_R: FaceExpression2FB = Self(56i32);
    pub const OUTER_BROW_RAISER_L: FaceExpression2FB = Self(57i32);
    pub const OUTER_BROW_RAISER_R: FaceExpression2FB = Self(58i32);
    pub const UPPER_LID_RAISER_L: FaceExpression2FB = Self(59i32);
    pub const UPPER_LID_RAISER_R: FaceExpression2FB = Self(60i32);
    pub const UPPER_LIP_RAISER_L: FaceExpression2FB = Self(61i32);
    pub const UPPER_LIP_RAISER_R: FaceExpression2FB = Self(62i32);
    pub const TONGUE_TIP_INTERDENTAL: FaceExpression2FB = Self(63i32);
    pub const TONGUE_TIP_ALVEOLAR: FaceExpression2FB = Self(64i32);
    pub const TONGUE_FRONT_DORSAL_PALATE: FaceExpression2FB = Self(65i32);
    pub const TONGUE_MID_DORSAL_PALATE: FaceExpression2FB = Self(66i32);
    pub const TONGUE_BACK_DORSAL_VELAR: FaceExpression2FB = Self(67i32);
    pub const TONGUE_OUT: FaceExpression2FB = Self(68i32);
    pub const TONGUE_RETREAT: FaceExpression2FB = Self(69i32);
    pub const COUNT: FaceExpression2FB = Self(70i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceExpression2FB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::BROW_LOWERER_L => Some("BROW_LOWERER_L"),
            Self::BROW_LOWERER_R => Some("BROW_LOWERER_R"),
            Self::CHEEK_PUFF_L => Some("CHEEK_PUFF_L"),
            Self::CHEEK_PUFF_R => Some("CHEEK_PUFF_R"),
            Self::CHEEK_RAISER_L => Some("CHEEK_RAISER_L"),
            Self::CHEEK_RAISER_R => Some("CHEEK_RAISER_R"),
            Self::CHEEK_SUCK_L => Some("CHEEK_SUCK_L"),
            Self::CHEEK_SUCK_R => Some("CHEEK_SUCK_R"),
            Self::CHIN_RAISER_B => Some("CHIN_RAISER_B"),
            Self::CHIN_RAISER_T => Some("CHIN_RAISER_T"),
            Self::DIMPLER_L => Some("DIMPLER_L"),
            Self::DIMPLER_R => Some("DIMPLER_R"),
            Self::EYES_CLOSED_L => Some("EYES_CLOSED_L"),
            Self::EYES_CLOSED_R => Some("EYES_CLOSED_R"),
            Self::EYES_LOOK_DOWN_L => Some("EYES_LOOK_DOWN_L"),
            Self::EYES_LOOK_DOWN_R => Some("EYES_LOOK_DOWN_R"),
            Self::EYES_LOOK_LEFT_L => Some("EYES_LOOK_LEFT_L"),
            Self::EYES_LOOK_LEFT_R => Some("EYES_LOOK_LEFT_R"),
            Self::EYES_LOOK_RIGHT_L => Some("EYES_LOOK_RIGHT_L"),
            Self::EYES_LOOK_RIGHT_R => Some("EYES_LOOK_RIGHT_R"),
            Self::EYES_LOOK_UP_L => Some("EYES_LOOK_UP_L"),
            Self::EYES_LOOK_UP_R => Some("EYES_LOOK_UP_R"),
            Self::INNER_BROW_RAISER_L => Some("INNER_BROW_RAISER_L"),
            Self::INNER_BROW_RAISER_R => Some("INNER_BROW_RAISER_R"),
            Self::JAW_DROP => Some("JAW_DROP"),
            Self::JAW_SIDEWAYS_LEFT => Some("JAW_SIDEWAYS_LEFT"),
            Self::JAW_SIDEWAYS_RIGHT => Some("JAW_SIDEWAYS_RIGHT"),
            Self::JAW_THRUST => Some("JAW_THRUST"),
            Self::LID_TIGHTENER_L => Some("LID_TIGHTENER_L"),
            Self::LID_TIGHTENER_R => Some("LID_TIGHTENER_R"),
            Self::LIP_CORNER_DEPRESSOR_L => Some("LIP_CORNER_DEPRESSOR_L"),
            Self::LIP_CORNER_DEPRESSOR_R => Some("LIP_CORNER_DEPRESSOR_R"),
            Self::LIP_CORNER_PULLER_L => Some("LIP_CORNER_PULLER_L"),
            Self::LIP_CORNER_PULLER_R => Some("LIP_CORNER_PULLER_R"),
            Self::LIP_FUNNELER_LB => Some("LIP_FUNNELER_LB"),
            Self::LIP_FUNNELER_LT => Some("LIP_FUNNELER_LT"),
            Self::LIP_FUNNELER_RB => Some("LIP_FUNNELER_RB"),
            Self::LIP_FUNNELER_RT => Some("LIP_FUNNELER_RT"),
            Self::LIP_PRESSOR_L => Some("LIP_PRESSOR_L"),
            Self::LIP_PRESSOR_R => Some("LIP_PRESSOR_R"),
            Self::LIP_PUCKER_L => Some("LIP_PUCKER_L"),
            Self::LIP_PUCKER_R => Some("LIP_PUCKER_R"),
            Self::LIP_STRETCHER_L => Some("LIP_STRETCHER_L"),
            Self::LIP_STRETCHER_R => Some("LIP_STRETCHER_R"),
            Self::LIP_SUCK_LB => Some("LIP_SUCK_LB"),
            Self::LIP_SUCK_LT => Some("LIP_SUCK_LT"),
            Self::LIP_SUCK_RB => Some("LIP_SUCK_RB"),
            Self::LIP_SUCK_RT => Some("LIP_SUCK_RT"),
            Self::LIP_TIGHTENER_L => Some("LIP_TIGHTENER_L"),
            Self::LIP_TIGHTENER_R => Some("LIP_TIGHTENER_R"),
            Self::LIPS_TOWARD => Some("LIPS_TOWARD"),
            Self::LOWER_LIP_DEPRESSOR_L => Some("LOWER_LIP_DEPRESSOR_L"),
            Self::LOWER_LIP_DEPRESSOR_R => Some("LOWER_LIP_DEPRESSOR_R"),
            Self::MOUTH_LEFT => Some("MOUTH_LEFT"),
            Self::MOUTH_RIGHT => Some("MOUTH_RIGHT"),
            Self::NOSE_WRINKLER_L => Some("NOSE_WRINKLER_L"),
            Self::NOSE_WRINKLER_R => Some("NOSE_WRINKLER_R"),
            Self::OUTER_BROW_RAISER_L => Some("OUTER_BROW_RAISER_L"),
            Self::OUTER_BROW_RAISER_R => Some("OUTER_BROW_RAISER_R"),
            Self::UPPER_LID_RAISER_L => Some("UPPER_LID_RAISER_L"),
            Self::UPPER_LID_RAISER_R => Some("UPPER_LID_RAISER_R"),
            Self::UPPER_LIP_RAISER_L => Some("UPPER_LIP_RAISER_L"),
            Self::UPPER_LIP_RAISER_R => Some("UPPER_LIP_RAISER_R"),
            Self::TONGUE_TIP_INTERDENTAL => Some("TONGUE_TIP_INTERDENTAL"),
            Self::TONGUE_TIP_ALVEOLAR => Some("TONGUE_TIP_ALVEOLAR"),
            Self::TONGUE_FRONT_DORSAL_PALATE => Some("TONGUE_FRONT_DORSAL_PALATE"),
            Self::TONGUE_MID_DORSAL_PALATE => Some("TONGUE_MID_DORSAL_PALATE"),
            Self::TONGUE_BACK_DORSAL_VELAR => Some("TONGUE_BACK_DORSAL_VELAR"),
            Self::TONGUE_OUT => Some("TONGUE_OUT"),
            Self::TONGUE_RETREAT => Some("TONGUE_RETREAT"),
            Self::COUNT => Some("COUNT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceExpressionSet2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionSet2FB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceExpressionSet2FB(i32);
impl FaceExpressionSet2FB {
    #[doc = "indicates that the created XrFaceTracker2FB tracks the set of blend shapes described by XrFaceExpression2FB enum, i.e. the xrGetFaceExpressionWeights2FB function returns an array of blend shapes with the count of XR_FACE_EXPRESSION2_COUNT_FB and can: be indexed using XrFaceExpression2FB."]
    pub const DEFAULT: FaceExpressionSet2FB = Self(0i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceExpressionSet2FB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceTrackingDataSource2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceTrackingDataSource2FB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceTrackingDataSource2FB(i32);
impl FaceTrackingDataSource2FB {
    #[doc = "face tracking uses visual data to estimate expressions. Face tracking may use audio to further improve the quality of face tracking."]
    pub const VISUAL: FaceTrackingDataSource2FB = Self(0i32);
    #[doc = "face tracking uses audio data to estimate expressions."]
    pub const AUDIO: FaceTrackingDataSource2FB = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceTrackingDataSource2FB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::VISUAL => Some("VISUAL"),
            Self::AUDIO => Some("AUDIO"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFaceConfidence2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceConfidence2FB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FaceConfidence2FB(i32);
impl FaceConfidence2FB {
    pub const LOWER_FACE: FaceConfidence2FB = Self(0i32);
    pub const UPPER_FACE: FaceConfidence2FB = Self(1i32);
    pub const COUNT: FaceConfidence2FB = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FaceConfidence2FB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOWER_FACE => Some("LOWER_FACE"),
            Self::UPPER_FACE => Some("UPPER_FACE"),
            Self::COUNT => Some("COUNT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrBodyJointFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyJointFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodyJointFB(i32);
impl BodyJointFB {
    pub const ROOT: BodyJointFB = Self(0i32);
    pub const HIPS: BodyJointFB = Self(1i32);
    pub const SPINE_LOWER: BodyJointFB = Self(2i32);
    pub const SPINE_MIDDLE: BodyJointFB = Self(3i32);
    pub const SPINE_UPPER: BodyJointFB = Self(4i32);
    pub const CHEST: BodyJointFB = Self(5i32);
    pub const NECK: BodyJointFB = Self(6i32);
    pub const HEAD: BodyJointFB = Self(7i32);
    pub const LEFT_SHOULDER: BodyJointFB = Self(8i32);
    pub const LEFT_SCAPULA: BodyJointFB = Self(9i32);
    pub const LEFT_ARM_UPPER: BodyJointFB = Self(10i32);
    pub const LEFT_ARM_LOWER: BodyJointFB = Self(11i32);
    pub const LEFT_HAND_WRIST_TWIST: BodyJointFB = Self(12i32);
    pub const RIGHT_SHOULDER: BodyJointFB = Self(13i32);
    pub const RIGHT_SCAPULA: BodyJointFB = Self(14i32);
    pub const RIGHT_ARM_UPPER: BodyJointFB = Self(15i32);
    pub const RIGHT_ARM_LOWER: BodyJointFB = Self(16i32);
    pub const RIGHT_HAND_WRIST_TWIST: BodyJointFB = Self(17i32);
    pub const LEFT_HAND_PALM: BodyJointFB = Self(18i32);
    pub const LEFT_HAND_WRIST: BodyJointFB = Self(19i32);
    pub const LEFT_HAND_THUMB_METACARPAL: BodyJointFB = Self(20i32);
    pub const LEFT_HAND_THUMB_PROXIMAL: BodyJointFB = Self(21i32);
    pub const LEFT_HAND_THUMB_DISTAL: BodyJointFB = Self(22i32);
    pub const LEFT_HAND_THUMB_TIP: BodyJointFB = Self(23i32);
    pub const LEFT_HAND_INDEX_METACARPAL: BodyJointFB = Self(24i32);
    pub const LEFT_HAND_INDEX_PROXIMAL: BodyJointFB = Self(25i32);
    pub const LEFT_HAND_INDEX_INTERMEDIATE: BodyJointFB = Self(26i32);
    pub const LEFT_HAND_INDEX_DISTAL: BodyJointFB = Self(27i32);
    pub const LEFT_HAND_INDEX_TIP: BodyJointFB = Self(28i32);
    pub const LEFT_HAND_MIDDLE_METACARPAL: BodyJointFB = Self(29i32);
    pub const LEFT_HAND_MIDDLE_PROXIMAL: BodyJointFB = Self(30i32);
    pub const LEFT_HAND_MIDDLE_INTERMEDIATE: BodyJointFB = Self(31i32);
    pub const LEFT_HAND_MIDDLE_DISTAL: BodyJointFB = Self(32i32);
    pub const LEFT_HAND_MIDDLE_TIP: BodyJointFB = Self(33i32);
    pub const LEFT_HAND_RING_METACARPAL: BodyJointFB = Self(34i32);
    pub const LEFT_HAND_RING_PROXIMAL: BodyJointFB = Self(35i32);
    pub const LEFT_HAND_RING_INTERMEDIATE: BodyJointFB = Self(36i32);
    pub const LEFT_HAND_RING_DISTAL: BodyJointFB = Self(37i32);
    pub const LEFT_HAND_RING_TIP: BodyJointFB = Self(38i32);
    pub const LEFT_HAND_LITTLE_METACARPAL: BodyJointFB = Self(39i32);
    pub const LEFT_HAND_LITTLE_PROXIMAL: BodyJointFB = Self(40i32);
    pub const LEFT_HAND_LITTLE_INTERMEDIATE: BodyJointFB = Self(41i32);
    pub const LEFT_HAND_LITTLE_DISTAL: BodyJointFB = Self(42i32);
    pub const LEFT_HAND_LITTLE_TIP: BodyJointFB = Self(43i32);
    pub const RIGHT_HAND_PALM: BodyJointFB = Self(44i32);
    pub const RIGHT_HAND_WRIST: BodyJointFB = Self(45i32);
    pub const RIGHT_HAND_THUMB_METACARPAL: BodyJointFB = Self(46i32);
    pub const RIGHT_HAND_THUMB_PROXIMAL: BodyJointFB = Self(47i32);
    pub const RIGHT_HAND_THUMB_DISTAL: BodyJointFB = Self(48i32);
    pub const RIGHT_HAND_THUMB_TIP: BodyJointFB = Self(49i32);
    pub const RIGHT_HAND_INDEX_METACARPAL: BodyJointFB = Self(50i32);
    pub const RIGHT_HAND_INDEX_PROXIMAL: BodyJointFB = Self(51i32);
    pub const RIGHT_HAND_INDEX_INTERMEDIATE: BodyJointFB = Self(52i32);
    pub const RIGHT_HAND_INDEX_DISTAL: BodyJointFB = Self(53i32);
    pub const RIGHT_HAND_INDEX_TIP: BodyJointFB = Self(54i32);
    pub const RIGHT_HAND_MIDDLE_METACARPAL: BodyJointFB = Self(55i32);
    pub const RIGHT_HAND_MIDDLE_PROXIMAL: BodyJointFB = Self(56i32);
    pub const RIGHT_HAND_MIDDLE_INTERMEDIATE: BodyJointFB = Self(57i32);
    pub const RIGHT_HAND_MIDDLE_DISTAL: BodyJointFB = Self(58i32);
    pub const RIGHT_HAND_MIDDLE_TIP: BodyJointFB = Self(59i32);
    pub const RIGHT_HAND_RING_METACARPAL: BodyJointFB = Self(60i32);
    pub const RIGHT_HAND_RING_PROXIMAL: BodyJointFB = Self(61i32);
    pub const RIGHT_HAND_RING_INTERMEDIATE: BodyJointFB = Self(62i32);
    pub const RIGHT_HAND_RING_DISTAL: BodyJointFB = Self(63i32);
    pub const RIGHT_HAND_RING_TIP: BodyJointFB = Self(64i32);
    pub const RIGHT_HAND_LITTLE_METACARPAL: BodyJointFB = Self(65i32);
    pub const RIGHT_HAND_LITTLE_PROXIMAL: BodyJointFB = Self(66i32);
    pub const RIGHT_HAND_LITTLE_INTERMEDIATE: BodyJointFB = Self(67i32);
    pub const RIGHT_HAND_LITTLE_DISTAL: BodyJointFB = Self(68i32);
    pub const RIGHT_HAND_LITTLE_TIP: BodyJointFB = Self(69i32);
    pub const COUNT: BodyJointFB = Self(70i32);
    pub const NONE: BodyJointFB = Self(-1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for BodyJointFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ROOT => Some("ROOT"),
            Self::HIPS => Some("HIPS"),
            Self::SPINE_LOWER => Some("SPINE_LOWER"),
            Self::SPINE_MIDDLE => Some("SPINE_MIDDLE"),
            Self::SPINE_UPPER => Some("SPINE_UPPER"),
            Self::CHEST => Some("CHEST"),
            Self::NECK => Some("NECK"),
            Self::HEAD => Some("HEAD"),
            Self::LEFT_SHOULDER => Some("LEFT_SHOULDER"),
            Self::LEFT_SCAPULA => Some("LEFT_SCAPULA"),
            Self::LEFT_ARM_UPPER => Some("LEFT_ARM_UPPER"),
            Self::LEFT_ARM_LOWER => Some("LEFT_ARM_LOWER"),
            Self::LEFT_HAND_WRIST_TWIST => Some("LEFT_HAND_WRIST_TWIST"),
            Self::RIGHT_SHOULDER => Some("RIGHT_SHOULDER"),
            Self::RIGHT_SCAPULA => Some("RIGHT_SCAPULA"),
            Self::RIGHT_ARM_UPPER => Some("RIGHT_ARM_UPPER"),
            Self::RIGHT_ARM_LOWER => Some("RIGHT_ARM_LOWER"),
            Self::RIGHT_HAND_WRIST_TWIST => Some("RIGHT_HAND_WRIST_TWIST"),
            Self::LEFT_HAND_PALM => Some("LEFT_HAND_PALM"),
            Self::LEFT_HAND_WRIST => Some("LEFT_HAND_WRIST"),
            Self::LEFT_HAND_THUMB_METACARPAL => Some("LEFT_HAND_THUMB_METACARPAL"),
            Self::LEFT_HAND_THUMB_PROXIMAL => Some("LEFT_HAND_THUMB_PROXIMAL"),
            Self::LEFT_HAND_THUMB_DISTAL => Some("LEFT_HAND_THUMB_DISTAL"),
            Self::LEFT_HAND_THUMB_TIP => Some("LEFT_HAND_THUMB_TIP"),
            Self::LEFT_HAND_INDEX_METACARPAL => Some("LEFT_HAND_INDEX_METACARPAL"),
            Self::LEFT_HAND_INDEX_PROXIMAL => Some("LEFT_HAND_INDEX_PROXIMAL"),
            Self::LEFT_HAND_INDEX_INTERMEDIATE => Some("LEFT_HAND_INDEX_INTERMEDIATE"),
            Self::LEFT_HAND_INDEX_DISTAL => Some("LEFT_HAND_INDEX_DISTAL"),
            Self::LEFT_HAND_INDEX_TIP => Some("LEFT_HAND_INDEX_TIP"),
            Self::LEFT_HAND_MIDDLE_METACARPAL => Some("LEFT_HAND_MIDDLE_METACARPAL"),
            Self::LEFT_HAND_MIDDLE_PROXIMAL => Some("LEFT_HAND_MIDDLE_PROXIMAL"),
            Self::LEFT_HAND_MIDDLE_INTERMEDIATE => Some("LEFT_HAND_MIDDLE_INTERMEDIATE"),
            Self::LEFT_HAND_MIDDLE_DISTAL => Some("LEFT_HAND_MIDDLE_DISTAL"),
            Self::LEFT_HAND_MIDDLE_TIP => Some("LEFT_HAND_MIDDLE_TIP"),
            Self::LEFT_HAND_RING_METACARPAL => Some("LEFT_HAND_RING_METACARPAL"),
            Self::LEFT_HAND_RING_PROXIMAL => Some("LEFT_HAND_RING_PROXIMAL"),
            Self::LEFT_HAND_RING_INTERMEDIATE => Some("LEFT_HAND_RING_INTERMEDIATE"),
            Self::LEFT_HAND_RING_DISTAL => Some("LEFT_HAND_RING_DISTAL"),
            Self::LEFT_HAND_RING_TIP => Some("LEFT_HAND_RING_TIP"),
            Self::LEFT_HAND_LITTLE_METACARPAL => Some("LEFT_HAND_LITTLE_METACARPAL"),
            Self::LEFT_HAND_LITTLE_PROXIMAL => Some("LEFT_HAND_LITTLE_PROXIMAL"),
            Self::LEFT_HAND_LITTLE_INTERMEDIATE => Some("LEFT_HAND_LITTLE_INTERMEDIATE"),
            Self::LEFT_HAND_LITTLE_DISTAL => Some("LEFT_HAND_LITTLE_DISTAL"),
            Self::LEFT_HAND_LITTLE_TIP => Some("LEFT_HAND_LITTLE_TIP"),
            Self::RIGHT_HAND_PALM => Some("RIGHT_HAND_PALM"),
            Self::RIGHT_HAND_WRIST => Some("RIGHT_HAND_WRIST"),
            Self::RIGHT_HAND_THUMB_METACARPAL => Some("RIGHT_HAND_THUMB_METACARPAL"),
            Self::RIGHT_HAND_THUMB_PROXIMAL => Some("RIGHT_HAND_THUMB_PROXIMAL"),
            Self::RIGHT_HAND_THUMB_DISTAL => Some("RIGHT_HAND_THUMB_DISTAL"),
            Self::RIGHT_HAND_THUMB_TIP => Some("RIGHT_HAND_THUMB_TIP"),
            Self::RIGHT_HAND_INDEX_METACARPAL => Some("RIGHT_HAND_INDEX_METACARPAL"),
            Self::RIGHT_HAND_INDEX_PROXIMAL => Some("RIGHT_HAND_INDEX_PROXIMAL"),
            Self::RIGHT_HAND_INDEX_INTERMEDIATE => Some("RIGHT_HAND_INDEX_INTERMEDIATE"),
            Self::RIGHT_HAND_INDEX_DISTAL => Some("RIGHT_HAND_INDEX_DISTAL"),
            Self::RIGHT_HAND_INDEX_TIP => Some("RIGHT_HAND_INDEX_TIP"),
            Self::RIGHT_HAND_MIDDLE_METACARPAL => Some("RIGHT_HAND_MIDDLE_METACARPAL"),
            Self::RIGHT_HAND_MIDDLE_PROXIMAL => Some("RIGHT_HAND_MIDDLE_PROXIMAL"),
            Self::RIGHT_HAND_MIDDLE_INTERMEDIATE => Some("RIGHT_HAND_MIDDLE_INTERMEDIATE"),
            Self::RIGHT_HAND_MIDDLE_DISTAL => Some("RIGHT_HAND_MIDDLE_DISTAL"),
            Self::RIGHT_HAND_MIDDLE_TIP => Some("RIGHT_HAND_MIDDLE_TIP"),
            Self::RIGHT_HAND_RING_METACARPAL => Some("RIGHT_HAND_RING_METACARPAL"),
            Self::RIGHT_HAND_RING_PROXIMAL => Some("RIGHT_HAND_RING_PROXIMAL"),
            Self::RIGHT_HAND_RING_INTERMEDIATE => Some("RIGHT_HAND_RING_INTERMEDIATE"),
            Self::RIGHT_HAND_RING_DISTAL => Some("RIGHT_HAND_RING_DISTAL"),
            Self::RIGHT_HAND_RING_TIP => Some("RIGHT_HAND_RING_TIP"),
            Self::RIGHT_HAND_LITTLE_METACARPAL => Some("RIGHT_HAND_LITTLE_METACARPAL"),
            Self::RIGHT_HAND_LITTLE_PROXIMAL => Some("RIGHT_HAND_LITTLE_PROXIMAL"),
            Self::RIGHT_HAND_LITTLE_INTERMEDIATE => Some("RIGHT_HAND_LITTLE_INTERMEDIATE"),
            Self::RIGHT_HAND_LITTLE_DISTAL => Some("RIGHT_HAND_LITTLE_DISTAL"),
            Self::RIGHT_HAND_LITTLE_TIP => Some("RIGHT_HAND_LITTLE_TIP"),
            Self::COUNT => Some("COUNT"),
            Self::NONE => Some("NONE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Describes the set of body joints to track when creating an XrBodyTrackerFB. - see [XrBodyJointSetFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyJointSetFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodyJointSetFB(i32);
impl BodyJointSetFB {
    #[doc = "Indicates that the created XrBodyTrackerFB tracks the set of body joints described by XrBodyJointFB enum, i.e. the xrLocateBodyJointsFB function returns an array of joint locations with the count of XR_BODY_JOINT_COUNT_FB and can be indexed using XrBodyJointFB."]
    pub const DEFAULT: BodyJointSetFB = Self(0i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for BodyJointSetFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrEyePositionFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyePositionFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EyePositionFB(i32);
impl EyePositionFB {
    #[doc = "Specifies the position of the left eye."]
    pub const LEFT: EyePositionFB = Self(0i32);
    #[doc = "Specifies the position of the right eye."]
    pub const RIGHT: EyePositionFB = Self(1i32);
    pub const COUNT: EyePositionFB = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for EyePositionFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LEFT => Some("LEFT"),
            Self::RIGHT => Some("RIGHT"),
            Self::COUNT => Some("COUNT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrTrackingOptimizationSettingsDomainQCOM](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrTrackingOptimizationSettingsDomainQCOM)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrackingOptimizationSettingsDomainQCOM(i32);
impl TrackingOptimizationSettingsDomainQCOM {
    #[doc = "Setting applies to all QCOM tracking extensions."]
    pub const ALL: TrackingOptimizationSettingsDomainQCOM = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for TrackingOptimizationSettingsDomainQCOM {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ALL => Some("ALL"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrTrackingOptimizationSettingsHintQCOM](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrTrackingOptimizationSettingsHintQCOM)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrackingOptimizationSettingsHintQCOM(i32);
impl TrackingOptimizationSettingsHintQCOM {
    #[doc = "Used by the application to indicate that it does not have a preference to optimize for. The run-time is understood to choose a balanced approach."]
    pub const NONE: TrackingOptimizationSettingsHintQCOM = Self(0i32);
    #[doc = "Used by the application to indicate that it prefers tracking to be optimized for long range, possibly at the expense of competing interests."]
    pub const LONG_RANGE_PRIORIZATION: TrackingOptimizationSettingsHintQCOM = Self(1i32);
    #[doc = "Used by the application to indicate that it prefers tracking to be optimized for close range, possibly at the expense of competing interests."]
    pub const CLOSE_RANGE_PRIORIZATION: TrackingOptimizationSettingsHintQCOM = Self(2i32);
    #[doc = "Used by the application to indicate that it prefers tracking to be optimized for low power consumption, possibly at the expense of competing interests."]
    pub const LOW_POWER_PRIORIZATION: TrackingOptimizationSettingsHintQCOM = Self(3i32);
    #[doc = "Used by the application to indicate that it prefers tracking to be optimized for increased tracking performance, possibly at the cost of increased power consumption."]
    pub const HIGH_POWER_PRIORIZATION: TrackingOptimizationSettingsHintQCOM = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for TrackingOptimizationSettingsHintQCOM {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::LONG_RANGE_PRIORIZATION => Some("LONG_RANGE_PRIORIZATION"),
            Self::CLOSE_RANGE_PRIORIZATION => Some("CLOSE_RANGE_PRIORIZATION"),
            Self::LOW_POWER_PRIORIZATION => Some("LOW_POWER_PRIORIZATION"),
            Self::HIGH_POWER_PRIORIZATION => Some("HIGH_POWER_PRIORIZATION"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrForceFeedbackCurlLocationMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrForceFeedbackCurlLocationMNDX)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceFeedbackCurlLocationMNDX(i32);
impl ForceFeedbackCurlLocationMNDX {
    #[doc = "force feedback for thumb curl"]
    pub const THUMB_CURL: ForceFeedbackCurlLocationMNDX = Self(0i32);
    #[doc = "force feedback for index finger curl"]
    pub const INDEX_CURL: ForceFeedbackCurlLocationMNDX = Self(1i32);
    #[doc = "force feedback for middle finger curl"]
    pub const MIDDLE_CURL: ForceFeedbackCurlLocationMNDX = Self(2i32);
    #[doc = "force feedback for ring finger curl"]
    pub const RING_CURL: ForceFeedbackCurlLocationMNDX = Self(3i32);
    #[doc = "force feedback for little finger curl"]
    pub const LITTLE_CURL: ForceFeedbackCurlLocationMNDX = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for ForceFeedbackCurlLocationMNDX {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::THUMB_CURL => Some("THUMB_CURL"),
            Self::INDEX_CURL => Some("INDEX_CURL"),
            Self::MIDDLE_CURL => Some("MIDDLE_CURL"),
            Self::RING_CURL => Some("RING_CURL"),
            Self::LITTLE_CURL => Some("LITTLE_CURL"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPlaneDetectionStateEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectionStateEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlaneDetectionStateEXT(i32);
impl PlaneDetectionStateEXT {
    pub const NONE: PlaneDetectionStateEXT = Self(0i32);
    pub const PENDING: PlaneDetectionStateEXT = Self(1i32);
    pub const DONE: PlaneDetectionStateEXT = Self(2i32);
    pub const ERROR: PlaneDetectionStateEXT = Self(3i32);
    pub const FATAL: PlaneDetectionStateEXT = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PlaneDetectionStateEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::PENDING => Some("PENDING"),
            Self::DONE => Some("DONE"),
            Self::ERROR => Some("ERROR"),
            Self::FATAL => Some("FATAL"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPlaneDetectorOrientationEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorOrientationEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlaneDetectorOrientationEXT(i32);
impl PlaneDetectorOrientationEXT {
    #[doc = "The detected plane is horizontal and faces upward (e.g. floor)."]
    pub const HORIZONTAL_UPWARD: PlaneDetectorOrientationEXT = Self(0i32);
    #[doc = "The detected plane is horizontal and faces downward (e.g. ceiling)."]
    pub const HORIZONTAL_DOWNWARD: PlaneDetectorOrientationEXT = Self(1i32);
    #[doc = "The detected plane is vertical (e.g. wall)."]
    pub const VERTICAL: PlaneDetectorOrientationEXT = Self(2i32);
    #[doc = "The detected plane has an arbitrary, non-vertical and non-horizontal orientation."]
    pub const ARBITRARY: PlaneDetectorOrientationEXT = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PlaneDetectorOrientationEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::HORIZONTAL_UPWARD => Some("HORIZONTAL_UPWARD"),
            Self::HORIZONTAL_DOWNWARD => Some("HORIZONTAL_DOWNWARD"),
            Self::VERTICAL => Some("VERTICAL"),
            Self::ARBITRARY => Some("ARBITRARY"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrPlaneDetectorSemanticTypeEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorSemanticTypeEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PlaneDetectorSemanticTypeEXT(i32);
impl PlaneDetectorSemanticTypeEXT {
    #[doc = "The runtime was unable to classify this plane."]
    pub const UNDEFINED: PlaneDetectorSemanticTypeEXT = Self(0i32);
    #[doc = "The detected plane is a ceiling."]
    pub const CEILING: PlaneDetectorSemanticTypeEXT = Self(1i32);
    #[doc = "The detected plane is a floor."]
    pub const FLOOR: PlaneDetectorSemanticTypeEXT = Self(2i32);
    #[doc = "The detected plane is a wall."]
    pub const WALL: PlaneDetectorSemanticTypeEXT = Self(3i32);
    #[doc = "The detected plane is a platform, like a table."]
    pub const PLATFORM: PlaneDetectorSemanticTypeEXT = Self(4i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for PlaneDetectorSemanticTypeEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNDEFINED => Some("UNDEFINED"),
            Self::CEILING => Some("CEILING"),
            Self::FLOOR => Some("FLOOR"),
            Self::WALL => Some("WALL"),
            Self::PLATFORM => Some("PLATFORM"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrVirtualKeyboardLocationTypeMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardLocationTypeMETA)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VirtualKeyboardLocationTypeMETA(i32);
impl VirtualKeyboardLocationTypeMETA {
    #[doc = "Indicates that the application will provide the position and scale of the keyboard."]
    pub const CUSTOM: VirtualKeyboardLocationTypeMETA = Self(0i32);
    #[doc = "Indicates that the runtime will set the position and scale for far field keyboard."]
    pub const FAR: VirtualKeyboardLocationTypeMETA = Self(1i32);
    #[doc = "Indicates that the runtime will set the position and scale for direct interaction keyboard."]
    pub const DIRECT: VirtualKeyboardLocationTypeMETA = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for VirtualKeyboardLocationTypeMETA {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::CUSTOM => Some("CUSTOM"),
            Self::FAR => Some("FAR"),
            Self::DIRECT => Some("DIRECT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrVirtualKeyboardInputSourceMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardInputSourceMETA)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VirtualKeyboardInputSourceMETA(i32);
impl VirtualKeyboardInputSourceMETA {
    #[doc = "Left controller ray."]
    pub const CONTROLLER_RAY_LEFT: VirtualKeyboardInputSourceMETA = Self(1i32);
    #[doc = "Right controller ray."]
    pub const CONTROLLER_RAY_RIGHT: VirtualKeyboardInputSourceMETA = Self(2i32);
    #[doc = "Left hand ray."]
    pub const HAND_RAY_LEFT: VirtualKeyboardInputSourceMETA = Self(3i32);
    #[doc = "Right hand ray."]
    pub const HAND_RAY_RIGHT: VirtualKeyboardInputSourceMETA = Self(4i32);
    #[doc = "Left controller direct touch."]
    pub const CONTROLLER_DIRECT_LEFT: VirtualKeyboardInputSourceMETA = Self(5i32);
    #[doc = "Right controller direct touch."]
    pub const CONTROLLER_DIRECT_RIGHT: VirtualKeyboardInputSourceMETA = Self(6i32);
    #[doc = "Left hand direct touch."]
    pub const HAND_DIRECT_INDEX_TIP_LEFT: VirtualKeyboardInputSourceMETA = Self(7i32);
    #[doc = "Right hand direct touch."]
    pub const HAND_DIRECT_INDEX_TIP_RIGHT: VirtualKeyboardInputSourceMETA = Self(8i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for VirtualKeyboardInputSourceMETA {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::CONTROLLER_RAY_LEFT => Some("CONTROLLER_RAY_LEFT"),
            Self::CONTROLLER_RAY_RIGHT => Some("CONTROLLER_RAY_RIGHT"),
            Self::HAND_RAY_LEFT => Some("HAND_RAY_LEFT"),
            Self::HAND_RAY_RIGHT => Some("HAND_RAY_RIGHT"),
            Self::CONTROLLER_DIRECT_LEFT => Some("CONTROLLER_DIRECT_LEFT"),
            Self::CONTROLLER_DIRECT_RIGHT => Some("CONTROLLER_DIRECT_RIGHT"),
            Self::HAND_DIRECT_INDEX_TIP_LEFT => Some("HAND_DIRECT_INDEX_TIP_LEFT"),
            Self::HAND_DIRECT_INDEX_TIP_RIGHT => Some("HAND_DIRECT_INDEX_TIP_RIGHT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHeadsetFitStatusML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHeadsetFitStatusML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HeadsetFitStatusML(i32);
impl HeadsetFitStatusML {
    #[doc = "Headset fit status not available for unknown reason."]
    pub const UNKNOWN: HeadsetFitStatusML = Self(0i32);
    #[doc = "Headset not worn."]
    pub const NOT_WORN: HeadsetFitStatusML = Self(1i32);
    #[doc = "Good fit."]
    pub const GOOD_FIT: HeadsetFitStatusML = Self(2i32);
    #[doc = "Bad fit."]
    pub const BAD_FIT: HeadsetFitStatusML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HeadsetFitStatusML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::NOT_WORN => Some("NOT_WORN"),
            Self::GOOD_FIT => Some("GOOD_FIT"),
            Self::BAD_FIT => Some("BAD_FIT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrEyeCalibrationStatusML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeCalibrationStatusML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EyeCalibrationStatusML(i32);
impl EyeCalibrationStatusML {
    #[doc = "Eye calibration status not available for unknown reason."]
    pub const UNKNOWN: EyeCalibrationStatusML = Self(0i32);
    #[doc = "User has not performed the eye calibration step. Use system provided app to perform eye calibration."]
    pub const NONE: EyeCalibrationStatusML = Self(1i32);
    #[doc = "Eye calibration is of lower accuracy."]
    pub const COARSE: EyeCalibrationStatusML = Self(2i32);
    #[doc = "Eye calibration is of higher accuracy."]
    pub const FINE: EyeCalibrationStatusML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for EyeCalibrationStatusML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::NONE => Some("NONE"),
            Self::COARSE => Some("COARSE"),
            Self::FINE => Some("FINE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrLocalizationMapStateML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapStateML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalizationMapStateML(i32);
impl LocalizationMapStateML {
    #[doc = "The system is not localized into a map. Features like Spatial Anchors relying on localization will not work."]
    pub const NOT_LOCALIZED: LocalizationMapStateML = Self(0i32);
    #[doc = "The system is localized into a map."]
    pub const LOCALIZED: LocalizationMapStateML = Self(1i32);
    #[doc = "The system is localizing into a map."]
    pub const LOCALIZATION_PENDING: LocalizationMapStateML = Self(2i32);
    #[doc = "Initial localization failed, the system will retry localization."]
    pub const LOCALIZATION_SLEEPING_BEFORE_RETRY: LocalizationMapStateML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LocalizationMapStateML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NOT_LOCALIZED => Some("NOT_LOCALIZED"),
            Self::LOCALIZED => Some("LOCALIZED"),
            Self::LOCALIZATION_PENDING => Some("LOCALIZATION_PENDING"),
            Self::LOCALIZATION_SLEEPING_BEFORE_RETRY => Some("LOCALIZATION_SLEEPING_BEFORE_RETRY"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrLocalizationMapTypeML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapTypeML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalizationMapTypeML(i32);
impl LocalizationMapTypeML {
    #[doc = "The system is localized into an On-Device map, published anchors are not shared between different devices."]
    pub const ON_DEVICE: LocalizationMapTypeML = Self(0i32);
    #[doc = "The system is localized into a Cloud Map, anchors are shared per cloud account settings."]
    pub const CLOUD: LocalizationMapTypeML = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LocalizationMapTypeML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ON_DEVICE => Some("ON_DEVICE"),
            Self::CLOUD => Some("CLOUD"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrLocalizationMapConfidenceML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapConfidenceML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LocalizationMapConfidenceML(i32);
impl LocalizationMapConfidenceML {
    #[doc = "The localization map has poor confidence, systems relying on the localization map are likely to have poor performance."]
    pub const POOR: LocalizationMapConfidenceML = Self(0i32);
    #[doc = "The confidence is fair, current environmental conditions may adversely affect localization."]
    pub const FAIR: LocalizationMapConfidenceML = Self(1i32);
    #[doc = "The confidence is high, persistent content should be stable."]
    pub const GOOD: LocalizationMapConfidenceML = Self(2i32);
    #[doc = "This is a very high-confidence localization, persistent content will be very stable."]
    pub const EXCELLENT: LocalizationMapConfidenceML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for LocalizationMapConfidenceML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::POOR => Some("POOR"),
            Self::FAIR => Some("FAIR"),
            Self::GOOD => Some("GOOD"),
            Self::EXCELLENT => Some("EXCELLENT"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorProfileML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorProfileML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorProfileML(i32);
impl MarkerDetectorProfileML {
    #[doc = "Tracker profile that covers standard use cases. If this does not suite the needs of the application try the other profiles listed below."]
    pub const DEFAULT: MarkerDetectorProfileML = Self(0i32);
    #[doc = "Optimized for speed. Use this profile to reduce the compute load and increase detection/tracker speed. This can result in low accuracy poses."]
    pub const SPEED: MarkerDetectorProfileML = Self(1i32);
    #[doc = "Optimized for accuracy. Use this profile to optimize for accurate marker poses. This can cause increased load on the compute."]
    pub const ACCURACY: MarkerDetectorProfileML = Self(2i32);
    #[doc = "Optimized for small targets. Use this profile to optimize for markers that are small or for larger markers that need to be detected from afar."]
    pub const SMALL_TARGETS: MarkerDetectorProfileML = Self(3i32);
    #[doc = "Optimized for FoV. Use this profile to be able to detect markers across a larger FoV. The marker tracker system will attempt to use multiple cameras to detect the markers."]
    pub const LARGE_FOV: MarkerDetectorProfileML = Self(4i32);
    #[doc = "Custom Tracker Profile. The application can define a custom tracker profile. See XrMarkerDetectorCustomProfileInfoML for more details."]
    pub const CUSTOM: MarkerDetectorProfileML = Self(5i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorProfileML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            Self::SPEED => Some("SPEED"),
            Self::ACCURACY => Some("ACCURACY"),
            Self::SMALL_TARGETS => Some("SMALL_TARGETS"),
            Self::LARGE_FOV => Some("LARGE_FOV"),
            Self::CUSTOM => Some("CUSTOM"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerTypeML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerTypeML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerTypeML(i32);
impl MarkerTypeML {
    #[doc = "Aruco Marker detection and localization. The marker id of the Aruco marker is available via xrGetMarkerNumberML."]
    pub const ARUCO: MarkerTypeML = Self(0i32);
    #[doc = "AprilTag detection and localization. The marker id of the AprilTags is available via xrGetMarkerNumberML."]
    pub const APRIL_TAG: MarkerTypeML = Self(1i32);
    #[doc = "QR code detection and localization. The contents of the QR code is available via xrGetMarkerStringML."]
    pub const QR: MarkerTypeML = Self(2i32);
    #[doc = "EAN-13, detection only, not locatable. The contents of the barcode is available via xrGetMarkerStringML."]
    pub const EAN_13: MarkerTypeML = Self(3i32);
    #[doc = "UPC-A, detection only, not locatable. The contents of the barcode is available via xrGetMarkerStringML."]
    pub const UPC_A: MarkerTypeML = Self(4i32);
    #[doc = "Code 128, detection only, not locatable. The contents of the barcode is available via xrGetMarkerStringML."]
    pub const CODE_128: MarkerTypeML = Self(5i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerTypeML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ARUCO => Some("ARUCO"),
            Self::APRIL_TAG => Some("APRIL_TAG"),
            Self::QR => Some("QR"),
            Self::EAN_13 => Some("EAN_13"),
            Self::UPC_A => Some("UPC_A"),
            Self::CODE_128 => Some("CODE_128"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerArucoDictML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerArucoDictML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerArucoDictML(i32);
impl MarkerArucoDictML {
    #[doc = "4 by 4 pixel Aruco marker dictionary with 50 IDs."]
    pub const DICT_4X4_50: MarkerArucoDictML = Self(0i32);
    #[doc = "4 by 4 pixel Aruco marker dictionary with 100 IDs."]
    pub const DICT_4X4_100: MarkerArucoDictML = Self(1i32);
    #[doc = "4 by 4 pixel Aruco marker dictionary with 250 IDs."]
    pub const DICT_4X4_250: MarkerArucoDictML = Self(2i32);
    #[doc = "4 by 4 pixel Aruco marker dictionary with 1000 IDs."]
    pub const DICT_4X4_1000: MarkerArucoDictML = Self(3i32);
    #[doc = "5 by 5 pixel Aruco marker dictionary with 50 IDs."]
    pub const DICT_5X5_50: MarkerArucoDictML = Self(4i32);
    #[doc = "5 by 5 pixel Aruco marker dictionary with 100 IDs."]
    pub const DICT_5X5_100: MarkerArucoDictML = Self(5i32);
    #[doc = "5 by 5 pixel Aruco marker dictionary with 250 IDs."]
    pub const DICT_5X5_250: MarkerArucoDictML = Self(6i32);
    #[doc = "5 by 5 pixel Aruco marker dictionary with 1000 IDs."]
    pub const DICT_5X5_1000: MarkerArucoDictML = Self(7i32);
    #[doc = "6 by 6 pixel Aruco marker dictionary with 50 IDs."]
    pub const DICT_6X6_50: MarkerArucoDictML = Self(8i32);
    #[doc = "6 by 6 pixel Aruco marker dictionary with 100 IDs."]
    pub const DICT_6X6_100: MarkerArucoDictML = Self(9i32);
    #[doc = "6 by 6 pixel Aruco marker dictionary with 250 IDs."]
    pub const DICT_6X6_250: MarkerArucoDictML = Self(10i32);
    #[doc = "6 by 6 pixel Aruco marker dictionary with 1000 IDs."]
    pub const DICT_6X6_1000: MarkerArucoDictML = Self(11i32);
    #[doc = "7 by 7 pixel Aruco marker dictionary with 50 IDs."]
    pub const DICT_7X7_50: MarkerArucoDictML = Self(12i32);
    #[doc = "7 by 7 pixel Aruco marker dictionary with 100 IDs."]
    pub const DICT_7X7_100: MarkerArucoDictML = Self(13i32);
    #[doc = "7 by 7 pixel Aruco marker dictionary with 250 IDs."]
    pub const DICT_7X7_250: MarkerArucoDictML = Self(14i32);
    #[doc = "7 by 7 pixel Aruco marker dictionary with 1000 IDs."]
    pub const DICT_7X7_1000: MarkerArucoDictML = Self(15i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerArucoDictML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DICT_4X4_50 => Some("DICT_4X4_50"),
            Self::DICT_4X4_100 => Some("DICT_4X4_100"),
            Self::DICT_4X4_250 => Some("DICT_4X4_250"),
            Self::DICT_4X4_1000 => Some("DICT_4X4_1000"),
            Self::DICT_5X5_50 => Some("DICT_5X5_50"),
            Self::DICT_5X5_100 => Some("DICT_5X5_100"),
            Self::DICT_5X5_250 => Some("DICT_5X5_250"),
            Self::DICT_5X5_1000 => Some("DICT_5X5_1000"),
            Self::DICT_6X6_50 => Some("DICT_6X6_50"),
            Self::DICT_6X6_100 => Some("DICT_6X6_100"),
            Self::DICT_6X6_250 => Some("DICT_6X6_250"),
            Self::DICT_6X6_1000 => Some("DICT_6X6_1000"),
            Self::DICT_7X7_50 => Some("DICT_7X7_50"),
            Self::DICT_7X7_100 => Some("DICT_7X7_100"),
            Self::DICT_7X7_250 => Some("DICT_7X7_250"),
            Self::DICT_7X7_1000 => Some("DICT_7X7_1000"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerAprilTagDictML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerAprilTagDictML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerAprilTagDictML(i32);
impl MarkerAprilTagDictML {
    #[doc = "4 by 4 bits, minimum Hamming distance between any two codes = 5, 30 codes."]
    pub const DICT_16H5: MarkerAprilTagDictML = Self(0i32);
    #[doc = "5 by 5 bits, minimum Hamming distance between any two codes = 9, 35 codes."]
    pub const DICT_25H9: MarkerAprilTagDictML = Self(1i32);
    #[doc = "6 by 6 bits, minimum Hamming distance between any two codes = 10, 2320 codes."]
    pub const DICT_36H10: MarkerAprilTagDictML = Self(2i32);
    #[doc = "6 by 6 bits, minimum Hamming distance between any two codes = 11, 587 codes."]
    pub const DICT_36H11: MarkerAprilTagDictML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerAprilTagDictML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DICT_16H5 => Some("DICT_16H5"),
            Self::DICT_25H9 => Some("DICT_25H9"),
            Self::DICT_36H10 => Some("DICT_36H10"),
            Self::DICT_36H11 => Some("DICT_36H11"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorFpsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorFpsML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorFpsML(i32);
impl MarkerDetectorFpsML {
    #[doc = "Low FPS."]
    pub const LOW: MarkerDetectorFpsML = Self(0i32);
    #[doc = "Medium FPS."]
    pub const MEDIUM: MarkerDetectorFpsML = Self(1i32);
    #[doc = "High FPS."]
    pub const HIGH: MarkerDetectorFpsML = Self(2i32);
    #[doc = "Max possible FPS."]
    pub const MAX: MarkerDetectorFpsML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorFpsML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOW => Some("LOW"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::HIGH => Some("HIGH"),
            Self::MAX => Some("MAX"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorResolutionML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorResolutionML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorResolutionML(i32);
impl MarkerDetectorResolutionML {
    #[doc = "Low Resolution."]
    pub const LOW: MarkerDetectorResolutionML = Self(0i32);
    #[doc = "Medium Resolution."]
    pub const MEDIUM: MarkerDetectorResolutionML = Self(1i32);
    #[doc = "High Resolution."]
    pub const HIGH: MarkerDetectorResolutionML = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorResolutionML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOW => Some("LOW"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::HIGH => Some("HIGH"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorCameraML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorCameraML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorCameraML(i32);
impl MarkerDetectorCameraML {
    #[doc = "Single RGB camera."]
    pub const RGB_CAMERA: MarkerDetectorCameraML = Self(0i32);
    #[doc = "One or more world cameras."]
    pub const WORLD_CAMERAS: MarkerDetectorCameraML = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorCameraML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::RGB_CAMERA => Some("RGB_CAMERA"),
            Self::WORLD_CAMERAS => Some("WORLD_CAMERAS"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorCornerRefineMethodML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorCornerRefineMethodML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorCornerRefineMethodML(i32);
impl MarkerDetectorCornerRefineMethodML {
    #[doc = "No refinement. Inaccurate corners."]
    pub const NONE: MarkerDetectorCornerRefineMethodML = Self(0i32);
    #[doc = "Subpixel refinement. Corners have subpixel coordinates. High detection rate, very fast, reasonable accuracy."]
    pub const SUBPIX: MarkerDetectorCornerRefineMethodML = Self(1i32);
    #[doc = "Contour refinement. High detection rate, fast, reasonable accuracy."]
    pub const CONTOUR: MarkerDetectorCornerRefineMethodML = Self(2i32);
    #[doc = "AprilTag refinement. Reasonable detection rate, slowest, but very accurate. Only valid with AprilTags."]
    pub const APRIL_TAG: MarkerDetectorCornerRefineMethodML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorCornerRefineMethodML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::SUBPIX => Some("SUBPIX"),
            Self::CONTOUR => Some("CONTOUR"),
            Self::APRIL_TAG => Some("APRIL_TAG"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorFullAnalysisIntervalML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorFullAnalysisIntervalML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorFullAnalysisIntervalML(i32);
impl MarkerDetectorFullAnalysisIntervalML {
    #[doc = "Detector analyzes every frame fully."]
    pub const MAX: MarkerDetectorFullAnalysisIntervalML = Self(0i32);
    #[doc = "Detector analyzes frame fully very often."]
    pub const FAST: MarkerDetectorFullAnalysisIntervalML = Self(1i32);
    #[doc = "Detector analyzes frame fully a few times per second."]
    pub const MEDIUM: MarkerDetectorFullAnalysisIntervalML = Self(2i32);
    #[doc = "Detector analyzes frame fully about every second."]
    pub const SLOW: MarkerDetectorFullAnalysisIntervalML = Self(3i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorFullAnalysisIntervalML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::MAX => Some("MAX"),
            Self::FAST => Some("FAST"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::SLOW => Some("SLOW"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMarkerDetectorStatusML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorStatusML)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MarkerDetectorStatusML(i32);
impl MarkerDetectorStatusML {
    #[doc = "The marker detector is working on a new snapshot."]
    pub const PENDING: MarkerDetectorStatusML = Self(0i32);
    #[doc = "The marker detector is ready to be inspected."]
    pub const READY: MarkerDetectorStatusML = Self(1i32);
    #[doc = "The marker detector has encountered a fatal error."]
    pub const ERROR: MarkerDetectorStatusML = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for MarkerDetectorStatusML {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PENDING => Some("PENDING"),
            Self::READY => Some("READY"),
            Self::ERROR => Some("ERROR"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrFutureStateEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFutureStateEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FutureStateEXT(i32);
impl FutureStateEXT {
    pub const PENDING: FutureStateEXT = Self(1i32);
    pub const READY: FutureStateEXT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for FutureStateEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PENDING => Some("PENDING"),
            Self::READY => Some("READY"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandEXT)"]
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
#[doc = "See [XrHandJointEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointEXT)"]
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
#[doc = "See [XrHandJointSetEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointSetEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandJointSetEXT(i32);
impl HandJointSetEXT {
    pub const DEFAULT: HandJointSetEXT = Self(0i32);
    pub const HAND_WITH_FOREARM_ULTRA: HandJointSetEXT = Self(1000149000i32);
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
            Self::HAND_WITH_FOREARM_ULTRA => Some("HAND_WITH_FOREARM_ULTRA"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandJointsMotionRangeEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointsMotionRangeEXT)"]
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
#[doc = "See [XrHandTrackingDataSourceEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingDataSourceEXT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandTrackingDataSourceEXT(i32);
impl HandTrackingDataSourceEXT {
    #[doc = "This data source value indicates individual fingers and joints are tracked from unobstructed data source such as optical hand tracking, data gloves, or motion capture devices."]
    pub const UNOBSTRUCTED: HandTrackingDataSourceEXT = Self(1i32);
    #[doc = "This data source value indicates hand joints are inferred based on motion controller state."]
    pub const CONTROLLER: HandTrackingDataSourceEXT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandTrackingDataSourceEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNOBSTRUCTED => Some("UNOBSTRUCTED"),
            Self::CONTROLLER => Some("CONTROLLER"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrHandPoseTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandPoseTypeMSFT)"]
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
#[doc = "See [XrSceneObjectTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneObjectTypeMSFT)"]
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
#[doc = "See [XrScenePlaneAlignmentTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrScenePlaneAlignmentTypeMSFT)"]
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
#[doc = "See [XrSceneComputeStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneComputeStateMSFT)"]
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
#[doc = "See [XrSceneComputeFeatureMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneComputeFeatureMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneComputeFeatureMSFT(i32);
impl SceneComputeFeatureMSFT {
    pub const PLANE: SceneComputeFeatureMSFT = Self(1i32);
    pub const PLANE_MESH: SceneComputeFeatureMSFT = Self(2i32);
    pub const VISUAL_MESH: SceneComputeFeatureMSFT = Self(3i32);
    pub const COLLIDER_MESH: SceneComputeFeatureMSFT = Self(4i32);
    pub const SERIALIZE_SCENE: SceneComputeFeatureMSFT = Self(1000098000i32);
    pub const MARKER: SceneComputeFeatureMSFT = Self(1000147000i32);
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
            Self::MARKER => Some("MARKER"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneComputeConsistencyMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneComputeConsistencyMSFT)"]
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
#[doc = "See [XrSceneComponentTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneComponentTypeMSFT)"]
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
    pub const MARKER: SceneComponentTypeMSFT = Self(1000147000i32);
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
            Self::MARKER => Some("MARKER"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrMeshComputeLodMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMeshComputeLodMSFT)"]
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
#[doc = "See [XrSceneMarkerTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneMarkerTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneMarkerTypeMSFT(i32);
impl SceneMarkerTypeMSFT {
    pub const QR_CODE: SceneMarkerTypeMSFT = Self(1i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneMarkerTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::QR_CODE => Some("QR_CODE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrSceneMarkerQRCodeSymbolTypeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneMarkerQRCodeSymbolTypeMSFT)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SceneMarkerQRCodeSymbolTypeMSFT(i32);
impl SceneMarkerQRCodeSymbolTypeMSFT {
    pub const QR_CODE: SceneMarkerQRCodeSymbolTypeMSFT = Self(1i32);
    pub const MICRO_QR_CODE: SceneMarkerQRCodeSymbolTypeMSFT = Self(2i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for SceneMarkerQRCodeSymbolTypeMSFT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::QR_CODE => Some("QR_CODE"),
            Self::MICRO_QR_CODE => Some("MICRO_QR_CODE"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrColorSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrColorSpaceFB)"]
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
#[doc = "See [XrFoveationLevelFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationLevelFB)"]
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
#[doc = "See [XrFoveationDynamicFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationDynamicFB)"]
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
#[doc = "See [XrReprojectionModeMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrReprojectionModeMSFT)"]
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
#[doc = "See [XrHandForearmJointULTRALEAP](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandForearmJointULTRALEAP)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HandForearmJointULTRALEAP(i32);
impl HandForearmJointULTRALEAP {
    pub const PALM: HandForearmJointULTRALEAP = Self(0i32);
    pub const WRIST: HandForearmJointULTRALEAP = Self(1i32);
    pub const THUMB_METACARPAL: HandForearmJointULTRALEAP = Self(2i32);
    pub const THUMB_PROXIMAL: HandForearmJointULTRALEAP = Self(3i32);
    pub const THUMB_DISTAL: HandForearmJointULTRALEAP = Self(4i32);
    pub const THUMB_TIP: HandForearmJointULTRALEAP = Self(5i32);
    pub const INDEX_METACARPAL: HandForearmJointULTRALEAP = Self(6i32);
    pub const INDEX_PROXIMAL: HandForearmJointULTRALEAP = Self(7i32);
    pub const INDEX_INTERMEDIATE: HandForearmJointULTRALEAP = Self(8i32);
    pub const INDEX_DISTAL: HandForearmJointULTRALEAP = Self(9i32);
    pub const INDEX_TIP: HandForearmJointULTRALEAP = Self(10i32);
    pub const MIDDLE_METACARPAL: HandForearmJointULTRALEAP = Self(11i32);
    pub const MIDDLE_PROXIMAL: HandForearmJointULTRALEAP = Self(12i32);
    pub const MIDDLE_INTERMEDIATE: HandForearmJointULTRALEAP = Self(13i32);
    pub const MIDDLE_DISTAL: HandForearmJointULTRALEAP = Self(14i32);
    pub const MIDDLE_TIP: HandForearmJointULTRALEAP = Self(15i32);
    pub const RING_METACARPAL: HandForearmJointULTRALEAP = Self(16i32);
    pub const RING_PROXIMAL: HandForearmJointULTRALEAP = Self(17i32);
    pub const RING_INTERMEDIATE: HandForearmJointULTRALEAP = Self(18i32);
    pub const RING_DISTAL: HandForearmJointULTRALEAP = Self(19i32);
    pub const RING_TIP: HandForearmJointULTRALEAP = Self(20i32);
    pub const LITTLE_METACARPAL: HandForearmJointULTRALEAP = Self(21i32);
    pub const LITTLE_PROXIMAL: HandForearmJointULTRALEAP = Self(22i32);
    pub const LITTLE_INTERMEDIATE: HandForearmJointULTRALEAP = Self(23i32);
    pub const LITTLE_DISTAL: HandForearmJointULTRALEAP = Self(24i32);
    pub const LITTLE_TIP: HandForearmJointULTRALEAP = Self(25i32);
    pub const ELBOW: HandForearmJointULTRALEAP = Self(26i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for HandForearmJointULTRALEAP {
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
            Self::ELBOW => Some("ELBOW"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrCompareOpFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompareOpFB)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompareOpFB(i32);
impl CompareOpFB {
    #[doc = "Comparison is never true."]
    pub const NEVER: CompareOpFB = Self(0i32);
    #[doc = "Comparison is true if source less than is destination."]
    pub const LESS: CompareOpFB = Self(1i32);
    #[doc = "Comparison is true if source is equal to destination."]
    pub const EQUAL: CompareOpFB = Self(2i32);
    #[doc = "Comparison is true if source is less than or equal to destination."]
    pub const LESS_OR_EQUAL: CompareOpFB = Self(3i32);
    #[doc = "Comparison is true if source is greater than destination."]
    pub const GREATER: CompareOpFB = Self(4i32);
    #[doc = "Comparison is true if source is not equal to destination."]
    pub const NOT_EQUAL: CompareOpFB = Self(5i32);
    #[doc = "Comparison is true if source is greater than or equal to destination."]
    pub const GREATER_OR_EQUAL: CompareOpFB = Self(6i32);
    #[doc = "Comparison is always true."]
    pub const ALWAYS: CompareOpFB = Self(7i32);
    pub fn from_raw(x: i32) -> Self {
        Self(x)
    }
    pub fn into_raw(self) -> i32 {
        self.0
    }
}
impl fmt::Debug for CompareOpFB {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NEVER => Some("NEVER"),
            Self::LESS => Some("LESS"),
            Self::EQUAL => Some("EQUAL"),
            Self::LESS_OR_EQUAL => Some("LESS_OR_EQUAL"),
            Self::GREATER => Some("GREATER"),
            Self::NOT_EQUAL => Some("NOT_EQUAL"),
            Self::GREATER_OR_EQUAL => Some("GREATER_OR_EQUAL"),
            Self::ALWAYS => Some("ALWAYS"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "See [XrInstanceCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInstanceCreateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InstanceCreateFlags(u64);
impl InstanceCreateFlags {}
bitmask!(InstanceCreateFlags);
#[doc = "See [XrSessionCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionCreateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SessionCreateFlags(u64);
impl SessionCreateFlags {}
bitmask!(SessionCreateFlags);
#[doc = "See [XrSwapchainCreateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainCreateFlagBits)"]
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
#[doc = "See [XrSwapchainUsageFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainUsageFlagBits)"]
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
#[doc = "See [XrViewStateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewStateFlagBits)"]
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
#[doc = "See [XrCompositionLayerFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerFlags(u64);
impl CompositionLayerFlags {
    #[doc = "Enables chromatic aberration correction when not done by default. This flag has no effect on any known conformant runtime, and is officially deprecated in OpenXR 1.1."]
    pub const CORRECT_CHROMATIC_ABERRATION: CompositionLayerFlags = Self(1 << 0u64);
    #[doc = "Enables the layer texture alpha channel."]
    pub const BLEND_TEXTURE_SOURCE_ALPHA: CompositionLayerFlags = Self(1 << 1u64);
    #[doc = "Indicates the texture color channels have not been premultiplied by the texture alpha channel."]
    pub const UNPREMULTIPLIED_ALPHA: CompositionLayerFlags = Self(1 << 2u64);
    #[doc = "Indicates that the texture alpha channel stores transparency instead of opacity, and is to be inverted before layer blending."]
    pub const INVERTED_ALPHA: CompositionLayerFlags = Self(1 << 3u64);
}
bitmask!(CompositionLayerFlags);
#[doc = "See [XrSpaceLocationFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceLocationFlagBits)"]
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
#[doc = "See [XrSpaceVelocityFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceVelocityFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceVelocityFlags(u64);
impl SpaceVelocityFlags {
    #[doc = "Indicates that the linearVelocity member contains valid data. Applications must: not read the linearVelocity field if this flag is unset."]
    pub const LINEAR_VALID: SpaceVelocityFlags = Self(1 << 0u64);
    #[doc = "Indicates that the angularVelocity member contains valid data. Applications must: not read the angularVelocity field if this flag is unset."]
    pub const ANGULAR_VALID: SpaceVelocityFlags = Self(1 << 1u64);
}
bitmask!(SpaceVelocityFlags);
#[doc = "See [XrInputSourceLocalizedNameFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInputSourceLocalizedNameFlagBits)"]
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
#[doc = "See [XrVulkanInstanceCreateFlagsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanInstanceCreateFlagsKHR)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VulkanInstanceCreateFlagsKHR(u64);
impl VulkanInstanceCreateFlagsKHR {}
bitmask!(VulkanInstanceCreateFlagsKHR);
#[doc = "See [XrVulkanDeviceCreateFlagsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanDeviceCreateFlagsKHR)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VulkanDeviceCreateFlagsKHR(u64);
impl VulkanDeviceCreateFlagsKHR {}
bitmask!(VulkanDeviceCreateFlagsKHR);
#[doc = "See [XrDebugUtilsMessageSeverityFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsMessageSeverityFlagsEXT)"]
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
#[doc = "See [XrDebugUtilsMessageTypeFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsMessageTypeFlagsEXT)"]
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
#[doc = "See [XrOverlayMainSessionFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrOverlayMainSessionFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlayMainSessionFlagsEXTX(u64);
impl OverlayMainSessionFlagsEXTX {
    #[doc = "Indicates the main session enabled `XR_KHR_composition_layer_depth`"]
    pub const ENABLED_COMPOSITION_LAYER_INFO_DEPTH: OverlayMainSessionFlagsEXTX = Self(1 << 0u64);
}
bitmask!(OverlayMainSessionFlagsEXTX);
#[doc = "See [XrOverlaySessionCreateFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrOverlaySessionCreateFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlaySessionCreateFlagsEXTX(u64);
impl OverlaySessionCreateFlagsEXTX {}
bitmask!(OverlaySessionCreateFlagsEXTX);
#[doc = "See [XrAndroidSurfaceSwapchainFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrAndroidSurfaceSwapchainFlagsFB)"]
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
#[doc = "See [XrCompositionLayerImageLayoutFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerImageLayoutFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerImageLayoutFlagsFB(u64);
impl CompositionLayerImageLayoutFlagsFB {
    #[doc = "The coordinate origin of the swapchain image must be considered to be flipped vertically."]
    pub const VERTICAL_FLIP: CompositionLayerImageLayoutFlagsFB = Self(1 << 0u64);
}
bitmask!(CompositionLayerImageLayoutFlagsFB);
#[doc = "See [XrCompositionLayerSecureContentFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSecureContentFlagsFB)"]
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
#[doc = "See [XrSwapchainCreateFoveationFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainCreateFoveationFlagsFB)"]
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
#[doc = "See [XrSwapchainStateFoveationFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateFoveationFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainStateFoveationFlagsFB(u64);
impl SwapchainStateFoveationFlagsFB {}
bitmask!(SwapchainStateFoveationFlagsFB);
#[doc = "See [XrFoveationEyeTrackedProfileCreateFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationEyeTrackedProfileCreateFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FoveationEyeTrackedProfileCreateFlagsMETA(u64);
impl FoveationEyeTrackedProfileCreateFlagsMETA {}
bitmask!(FoveationEyeTrackedProfileCreateFlagsMETA);
#[doc = "See [XrFoveationEyeTrackedStateFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationEyeTrackedStateFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FoveationEyeTrackedStateFlagsMETA(u64);
impl FoveationEyeTrackedStateFlagsMETA {
    #[doc = "Indicates whether or not foveation data is valid. This can happen if the eye tracker is obscured, the camera has dirt, or eye lid is closed, etc."]
    pub const VALID: FoveationEyeTrackedStateFlagsMETA = Self(1 << 0u64);
}
bitmask!(FoveationEyeTrackedStateFlagsMETA);
#[doc = "See [XrTriangleMeshFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrTriangleMeshFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TriangleMeshFlagsFB(u64);
impl TriangleMeshFlagsFB {
    #[doc = "The triangle mesh is mutable (can be modified after it is created)."]
    pub const MUTABLE: TriangleMeshFlagsFB = Self(1 << 0u64);
}
bitmask!(TriangleMeshFlagsFB);
#[doc = "See [XrPassthroughFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughFlagsFB(u64);
impl PassthroughFlagsFB {
    #[doc = "The object (passthrough, layer) is running at creation."]
    pub const IS_RUNNING_AT_CREATION: PassthroughFlagsFB = Self(1 << 0u64);
    #[doc = "The passthrough system sends depth information to the compositor. Only applicable to layer objects."]
    pub const LAYER_DEPTH: PassthroughFlagsFB = Self(1 << 1u64);
}
bitmask!(PassthroughFlagsFB);
#[doc = "See [XrPassthroughStateChangedFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughStateChangedFlagsFB)"]
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
#[doc = "See [XrPassthroughCapabilityFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughCapabilityFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughCapabilityFlagsFB(u64);
impl PassthroughCapabilityFlagsFB {
    #[doc = "The system supports passthrough."]
    pub const PASSTHROUGH_CAPABILITY: PassthroughCapabilityFlagsFB = Self(1 << 0u64);
    #[doc = "The system can show passthrough with realistic colors. XR_PASSTHROUGH_CAPABILITY_BIT_FB must: be set if XR_PASSTHROUGH_CAPABILITY_COLOR_BIT_FB is set."]
    pub const COLOR: PassthroughCapabilityFlagsFB = Self(1 << 1u64);
    #[doc = "The system supports passthrough layers composited using depth testing. XR_PASSTHROUGH_CAPABILITY_BIT_FB must: be set if XR_PASSTHROUGH_CAPABILITY_LAYER_DEPTH_BIT_FB is set."]
    pub const LAYER_DEPTH: PassthroughCapabilityFlagsFB = Self(1 << 2u64);
}
bitmask!(PassthroughCapabilityFlagsFB);
#[doc = "See [XrSemanticLabelsSupportFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSemanticLabelsSupportFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SemanticLabelsSupportFlagsFB(u64);
impl SemanticLabelsSupportFlagsFB {
    #[doc = "If set, and the runtime reports the extensionVersion as 2 or greater, the runtime may: return multiple semantic labels separated by a comma without spaces. Otherwise, the runtime must: return a single semantic label."]
    pub const MULTIPLE_SEMANTIC_LABELS: SemanticLabelsSupportFlagsFB = Self(1 << 0u64);
    #[doc = "If set, and the runtime reports the extensionVersion as 3 or greater, the runtime must: return \"TABLE\" instead of \"DESK\" as a semantic label to the application. Otherwise, the runtime must: return \"DESK\" instead of \"TABLE\" as a semantic label to the application, when applicable."]
    pub const ACCEPT_DESK_TO_TABLE_MIGRATION: SemanticLabelsSupportFlagsFB = Self(1 << 1u64);
    #[doc = "If set, and the runtime reports the extensionVersion as 4 or greater, the runtime may: return \"INVISIBLE_WALL_FACE\" instead of \"WALL_FACE\" as a semantic label to the application in order to represent an invisible wall used to conceptually separate a space (e.g., separate a living space from a kitchen space in an open floor plan house even though there is no real wall between the two spaces) instead of a real wall. Otherwise, the runtime must: return \"WALL_FACE\" as a semantic label to the application in order to represent both an invisible and real wall, when applicable."]
    pub const ACCEPT_INVISIBLE_WALL_FACE: SemanticLabelsSupportFlagsFB = Self(1 << 2u64);
}
bitmask!(SemanticLabelsSupportFlagsFB);
#[doc = "See [XrHandTrackingAimFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingAimFlagsFB)"]
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
#[doc = "See [XrKeyboardTrackingFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrKeyboardTrackingFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyboardTrackingFlagsFB(u64);
impl KeyboardTrackingFlagsFB {
    #[doc = "indicates that the system has a physically tracked keyboard to report.  If not set then no other bits should be considered to be valid or meaningful.  If set either XR_KEYBOARD_TRACKING_LOCAL_BIT_FB or XR_KEYBOARD_TRACKING_REMOTE_BIT_FB must also be set."]
    pub const EXISTS: KeyboardTrackingFlagsFB = Self(1 << 0u64);
    #[doc = "indicates that the physically tracked keyboard is intended to be used in a local pairing with the system.  Mutually exclusive with XR_KEYBOARD_TRACKING_REMOTE_BIT_FB."]
    pub const LOCAL: KeyboardTrackingFlagsFB = Self(1 << 1u64);
    #[doc = "indicates that the physically tracked keyboard is intended to be used while paired to a separate remote computing device. Mutually exclusive with XR_KEYBOARD_TRACKING_LOCAL_BIT_FB."]
    pub const REMOTE: KeyboardTrackingFlagsFB = Self(1 << 2u64);
    #[doc = "indicates that the physically tracked keyboard is actively connected to the headset and capable of sending key data"]
    pub const CONNECTED: KeyboardTrackingFlagsFB = Self(1 << 3u64);
}
bitmask!(KeyboardTrackingFlagsFB);
#[doc = "See [XrKeyboardTrackingQueryFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrKeyboardTrackingQueryFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyboardTrackingQueryFlagsFB(u64);
impl KeyboardTrackingQueryFlagsFB {
    #[doc = "indicates the query is for the physically tracked keyboard that is intended to be used in a local pairing with the System. Mutually exclusive with XR_KEYBOARD_TRACKING_QUERY_REMOTE_BIT_FB."]
    pub const LOCAL: KeyboardTrackingQueryFlagsFB = Self(1 << 1u64);
    #[doc = "indicates the query is for the physically tracked keyboard that may be connected to a separate remote computing device. Mutually exclusive with XR_KEYBOARD_TRACKING_QUERY_LOCAL_BIT_FB."]
    pub const REMOTE: KeyboardTrackingQueryFlagsFB = Self(1 << 2u64);
}
bitmask!(KeyboardTrackingQueryFlagsFB);
#[doc = "See [XrCompositionLayerSpaceWarpInfoFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSpaceWarpInfoFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerSpaceWarpInfoFlagsFB(u64);
impl CompositionLayerSpaceWarpInfoFlagsFB {
    #[doc = "Skip current frame's space warp extrapolation"]
    pub const FRAME_SKIP: CompositionLayerSpaceWarpInfoFlagsFB = Self(1 << 0u64);
}
bitmask!(CompositionLayerSpaceWarpInfoFlagsFB);
#[doc = "See [XrRenderModelFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RenderModelFlagsFB(u64);
impl RenderModelFlagsFB {
    #[doc = "Minimal level of support.  Can only contain a single mesh.  Can only contain a single texture.  Can not contain transparency.  Assumes unlit rendering.  Requires Extension KHR_texturebasisu."]
    pub const SUPPORTS_GLTF_2_0_SUBSET_1: RenderModelFlagsFB = Self(1 << 0u64);
    #[doc = "All of XR_RENDER_MODEL_SUPPORTS_GLTF_2_0_SUBSET_1_BIT_FB support plus: Multiple meshes. Multiple Textures. Texture Transparency."]
    pub const SUPPORTS_GLTF_2_0_SUBSET_2: RenderModelFlagsFB = Self(1 << 1u64);
}
bitmask!(RenderModelFlagsFB);
#[doc = "See [XrDigitalLensControlFlagsALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDigitalLensControlFlagsALMALENCE)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DigitalLensControlFlagsALMALENCE(u64);
impl DigitalLensControlFlagsALMALENCE {
    #[doc = "disables Digital Lens processing of render textures"]
    pub const PROCESSING_DISABLE: DigitalLensControlFlagsALMALENCE = Self(1 << 0u64);
}
bitmask!(DigitalLensControlFlagsALMALENCE);
#[doc = "See [XrCompositionLayerSettingsFlagsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSettingsFlagsFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerSettingsFlagsFB(u64);
impl CompositionLayerSettingsFlagsFB {
    #[doc = "Indicates compositor may: use layer texture supersampling."]
    pub const NORMAL_SUPER_SAMPLING: CompositionLayerSettingsFlagsFB = Self(1 << 0u64);
    #[doc = "Indicates compositor may: use high quality layer texture supersampling."]
    pub const QUALITY_SUPER_SAMPLING: CompositionLayerSettingsFlagsFB = Self(1 << 1u64);
    #[doc = "Indicates compositor may: use layer texture sharpening."]
    pub const NORMAL_SHARPENING: CompositionLayerSettingsFlagsFB = Self(1 << 2u64);
    #[doc = "Indicates compositor may: use high quality layer texture sharpening."]
    pub const QUALITY_SHARPENING: CompositionLayerSettingsFlagsFB = Self(1 << 3u64);
    #[doc = "Indicates compositor may: automatically toggle a texture filtering mechanism to improve visual quality of layer. This must: not be the only bit set. (Added by XR_META_automatic_layer_filter)"]
    pub const AUTO_LAYER_FILTER: CompositionLayerSettingsFlagsFB = Self(1 << 5u64);
}
bitmask!(CompositionLayerSettingsFlagsFB);
#[doc = "See [XrExternalCameraStatusFlagsOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExternalCameraStatusFlagsOCULUS)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ExternalCameraStatusFlagsOCULUS(u64);
impl ExternalCameraStatusFlagsOCULUS {
    #[doc = "External camera is connected"]
    pub const CONNECTED: ExternalCameraStatusFlagsOCULUS = Self(1 << 0u64);
    #[doc = "External camera is undergoing calibration"]
    pub const CALIBRATING: ExternalCameraStatusFlagsOCULUS = Self(1 << 1u64);
    #[doc = "External camera has tried and failed calibration"]
    pub const CALIBRATION_FAILED: ExternalCameraStatusFlagsOCULUS = Self(1 << 2u64);
    #[doc = "External camera has tried and passed calibration"]
    pub const CALIBRATED: ExternalCameraStatusFlagsOCULUS = Self(1 << 3u64);
    #[doc = "External camera is capturing"]
    pub const CAPTURING: ExternalCameraStatusFlagsOCULUS = Self(1 << 4u64);
}
bitmask!(ExternalCameraStatusFlagsOCULUS);
#[doc = "See [XrPerformanceMetricsCounterFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerformanceMetricsCounterFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PerformanceMetricsCounterFlagsMETA(u64);
impl PerformanceMetricsCounterFlagsMETA {
    #[doc = "Indicates any of the values in XrPerformanceMetricsCounterMETA is valid."]
    pub const ANY_VALUE_VALID: PerformanceMetricsCounterFlagsMETA = Self(1 << 0u64);
    #[doc = "Indicates the uintValue in XrPerformanceMetricsCounterMETA is valid."]
    pub const UINT_VALUE_VALID: PerformanceMetricsCounterFlagsMETA = Self(1 << 1u64);
    #[doc = "Indicates the floatValue in XrPerformanceMetricsCounterMETA is valid."]
    pub const FLOAT_VALUE_VALID: PerformanceMetricsCounterFlagsMETA = Self(1 << 2u64);
}
bitmask!(PerformanceMetricsCounterFlagsMETA);
#[doc = "See [XrPassthroughPreferenceFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughPreferenceFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PassthroughPreferenceFlagsMETA(u64);
impl PassthroughPreferenceFlagsMETA {
    #[doc = "Indicates that the runtime recommends apps to default to a mixed reality experience with passthrough (if supported)."]
    pub const DEFAULT_TO_ACTIVE: PassthroughPreferenceFlagsMETA = Self(1 << 0u64);
}
bitmask!(PassthroughPreferenceFlagsMETA);
#[doc = "See [XrFoveationDynamicFlagsHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationDynamicFlagsHTC)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FoveationDynamicFlagsHTC(u64);
impl FoveationDynamicFlagsHTC {
    #[doc = "Allow system to set periphery pixel density dynamically."]
    pub const LEVEL_ENABLED: FoveationDynamicFlagsHTC = Self(1 << 0u64);
    #[doc = "Allow system to set clear FOV degree dynamically."]
    pub const CLEAR_FOV_ENABLED: FoveationDynamicFlagsHTC = Self(1 << 1u64);
    #[doc = "Allow system to set focal center offset dynamically."]
    pub const FOCAL_CENTER_OFFSET_ENABLED: FoveationDynamicFlagsHTC = Self(1 << 2u64);
}
bitmask!(FoveationDynamicFlagsHTC);
#[doc = "See [XrFrameEndInfoFlagsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameEndInfoFlagsML)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FrameEndInfoFlagsML(u64);
impl FrameEndInfoFlagsML {
    #[doc = "Indicates that the content for this frame is protected and should not be recorded or captured outside the graphics system."]
    pub const PROTECTED: FrameEndInfoFlagsML = Self(1 << 0u64);
    #[doc = "Indicates that a soft fade to transparent should be added to the frame in the compositor to blend any hard edges at the FOV limits."]
    pub const VIGNETTE: FrameEndInfoFlagsML = Self(1 << 1u64);
}
bitmask!(FrameEndInfoFlagsML);
#[doc = "See [XrGlobalDimmerFrameEndInfoFlagsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGlobalDimmerFrameEndInfoFlagsML)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GlobalDimmerFrameEndInfoFlagsML(u64);
impl GlobalDimmerFrameEndInfoFlagsML {
    #[doc = "Indicates that the global dimmer should: be enabled and controlled by dimmerValue."]
    pub const ENABLED: GlobalDimmerFrameEndInfoFlagsML = Self(1 << 0u64);
}
bitmask!(GlobalDimmerFrameEndInfoFlagsML);
#[doc = "See [XrPlaneDetectorFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PlaneDetectorFlagsEXT(u64);
impl PlaneDetectorFlagsEXT {
    #[doc = "populate the plane contour information"]
    pub const ENABLE_CONTOUR: PlaneDetectorFlagsEXT = Self(1 << 0u64);
}
bitmask!(PlaneDetectorFlagsEXT);
#[doc = "See [XrPlaneDetectionCapabilityFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectionCapabilityFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PlaneDetectionCapabilityFlagsEXT(u64);
impl PlaneDetectionCapabilityFlagsEXT {
    #[doc = "plane detection is supported"]
    pub const PLANE_DETECTION: PlaneDetectionCapabilityFlagsEXT = Self(1 << 0u64);
    #[doc = "polygon buffers for holes in planes can be generated"]
    pub const PLANE_HOLES: PlaneDetectionCapabilityFlagsEXT = Self(1 << 1u64);
    #[doc = "plane detection supports ceiling semantic classification"]
    pub const SEMANTIC_CEILING: PlaneDetectionCapabilityFlagsEXT = Self(1 << 2u64);
    #[doc = "plane detection supports floor semantic classification"]
    pub const SEMANTIC_FLOOR: PlaneDetectionCapabilityFlagsEXT = Self(1 << 3u64);
    #[doc = "plane detection supports wall semantic classification"]
    pub const SEMANTIC_WALL: PlaneDetectionCapabilityFlagsEXT = Self(1 << 4u64);
    #[doc = "plane detection supports platform semantic classification (for example table tops)"]
    pub const SEMANTIC_PLATFORM: PlaneDetectionCapabilityFlagsEXT = Self(1 << 5u64);
    #[doc = "plane detection supports plane orientation classification. If not supported planes are always classified as ARBITRARY."]
    pub const ORIENTATION: PlaneDetectionCapabilityFlagsEXT = Self(1 << 6u64);
}
bitmask!(PlaneDetectionCapabilityFlagsEXT);
#[doc = "See [XrVirtualKeyboardInputStateFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardInputStateFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct VirtualKeyboardInputStateFlagsMETA(u64);
impl VirtualKeyboardInputStateFlagsMETA {
    #[doc = "If the input source is considered 'pressed' at all. Pinch for hands, Primary button for controllers."]
    pub const PRESSED: VirtualKeyboardInputStateFlagsMETA = Self(1 << 0u64);
}
bitmask!(VirtualKeyboardInputStateFlagsMETA);
#[doc = "See [XrLocalizationMapErrorFlagsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapErrorFlagsML)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LocalizationMapErrorFlagsML(u64);
impl LocalizationMapErrorFlagsML {
    #[doc = "Localization failed for an unknown reason."]
    pub const UNKNOWN: LocalizationMapErrorFlagsML = Self(1 << 0u64);
    #[doc = "Localization failed because the user is outside of the mapped area."]
    pub const OUT_OF_MAPPED_AREA: LocalizationMapErrorFlagsML = Self(1 << 1u64);
    #[doc = "There are not enough features in the environment to successfully localize."]
    pub const LOW_FEATURE_COUNT: LocalizationMapErrorFlagsML = Self(1 << 2u64);
    #[doc = "Localization failed due to excessive motion."]
    pub const EXCESSIVE_MOTION: LocalizationMapErrorFlagsML = Self(1 << 3u64);
    #[doc = "Localization failed because the lighting levels are too low in the environment."]
    pub const LOW_LIGHT: LocalizationMapErrorFlagsML = Self(1 << 4u64);
    #[doc = "A headpose failure caused localization to be unsuccessful."]
    pub const HEADPOSE: LocalizationMapErrorFlagsML = Self(1 << 5u64);
}
bitmask!(LocalizationMapErrorFlagsML);
#[doc = "See [XrEnvironmentDepthProviderCreateFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthProviderCreateFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EnvironmentDepthProviderCreateFlagsMETA(u64);
impl EnvironmentDepthProviderCreateFlagsMETA {}
bitmask!(EnvironmentDepthProviderCreateFlagsMETA);
#[doc = "See [XrEnvironmentDepthSwapchainCreateFlagsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthSwapchainCreateFlagsMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EnvironmentDepthSwapchainCreateFlagsMETA(u64);
impl EnvironmentDepthSwapchainCreateFlagsMETA {}
bitmask!(EnvironmentDepthSwapchainCreateFlagsMETA);
#[doc = "See [XrInstance](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInstance)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Instance(u64);
handle!(Instance);
#[doc = "See [XrSession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSession)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Session(u64);
handle!(Session);
#[doc = "See [XrActionSet](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionSet)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ActionSet(u64);
handle!(ActionSet);
#[doc = "See [XrAction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrAction)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Action(u64);
handle!(Action);
#[doc = "See [XrSwapchain](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchain)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Swapchain(u64);
handle!(Swapchain);
#[doc = "See [XrSpace](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpace)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Space(u64);
handle!(Space);
#[doc = "See [XrDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsMessengerEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DebugUtilsMessengerEXT(u64);
handle!(DebugUtilsMessengerEXT);
#[doc = "See [XrSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpatialAnchorMSFT(u64);
handle!(SpatialAnchorMSFT);
#[doc = "See [XrHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackerEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct HandTrackerEXT(u64);
handle!(HandTrackerEXT);
#[doc = "See [XrFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationProfileFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FoveationProfileFB(u64);
handle!(FoveationProfileFB);
#[doc = "See [XrTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrTriangleMeshFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TriangleMeshFB(u64);
handle!(TriangleMeshFB);
#[doc = "See [XrPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassthroughFB(u64);
handle!(PassthroughFB);
#[doc = "See [XrPassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughLayerFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassthroughLayerFB(u64);
handle!(PassthroughLayerFB);
#[doc = "See [XrGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGeometryInstanceFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GeometryInstanceFB(u64);
handle!(GeometryInstanceFB);
#[doc = "See [XrFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFacialTrackerHTC)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FacialTrackerHTC(u64);
handle!(FacialTrackerHTC);
#[doc = "See [XrPassthroughHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughHTC)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassthroughHTC(u64);
handle!(PassthroughHTC);
#[doc = "See [XrFaceTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceTrackerFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FaceTrackerFB(u64);
handle!(FaceTrackerFB);
#[doc = "See [XrFaceTracker2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceTracker2FB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FaceTracker2FB(u64);
handle!(FaceTracker2FB);
#[doc = "See [XrBodyTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyTrackerFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BodyTrackerFB(u64);
handle!(BodyTrackerFB);
#[doc = "See [XrEyeTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeTrackerFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EyeTrackerFB(u64);
handle!(EyeTrackerFB);
#[doc = "See [XrSpaceUserFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceUserFB)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpaceUserFB(u64);
handle!(SpaceUserFB);
#[doc = "See [XrPassthroughColorLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorLutMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PassthroughColorLutMETA(u64);
handle!(PassthroughColorLutMETA);
#[doc = "See [XrPlaneDetectorEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlaneDetectorEXT(u64);
handle!(PlaneDetectorEXT);
#[doc = "See [XrVirtualKeyboardMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VirtualKeyboardMETA(u64);
handle!(VirtualKeyboardMETA);
#[doc = "See [XrExportedLocalizationMapML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExportedLocalizationMapML)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ExportedLocalizationMapML(u64);
handle!(ExportedLocalizationMapML);
#[doc = "See [XrMarkerDetectorML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorML)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MarkerDetectorML(u64);
handle!(MarkerDetectorML);
#[doc = "See [XrEnvironmentDepthProviderMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthProviderMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentDepthProviderMETA(u64);
handle!(EnvironmentDepthProviderMETA);
#[doc = "See [XrEnvironmentDepthSwapchainMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthSwapchainMETA)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct EnvironmentDepthSwapchainMETA(u64);
handle!(EnvironmentDepthSwapchainMETA);
#[doc = "See [XrSpatialGraphNodeBindingMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphNodeBindingMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpatialGraphNodeBindingMSFT(u64);
handle!(SpatialGraphNodeBindingMSFT);
#[doc = "See [XrSceneObserverMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneObserverMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SceneObserverMSFT(u64);
handle!(SceneObserverMSFT);
#[doc = "See [XrSceneMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SceneMSFT(u64);
handle!(SceneMSFT);
#[doc = "See [XrSpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorStoreConnectionMSFT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpatialAnchorStoreConnectionMSFT(u64);
handle!(SpatialAnchorStoreConnectionMSFT);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector2f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVector2f)"]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector3f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVector3f)"]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrVector4f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVector4f)"]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrColor4f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrColor4f)"]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrQuaternionf](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrQuaternionf)"]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrPosef](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPosef)"]
pub struct Posef {
    pub orientation: Quaternionf,
    pub position: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrOffset2Df](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrOffset2Df)"]
pub struct Offset2Df {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrExtent2Df](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExtent2Df)"]
pub struct Extent2Df {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrRect2Df](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRect2Df)"]
pub struct Rect2Df {
    pub offset: Offset2Df,
    pub extent: Extent2Df,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrOffset2Di](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrOffset2Di)"]
pub struct Offset2Di {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrExtent2Di](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExtent2Di)"]
pub struct Extent2Di {
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrRect2Di](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRect2Di)"]
pub struct Rect2Di {
    pub offset: Offset2Di,
    pub extent: Extent2Di,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = "See [XrNegotiateLoaderInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrNegotiateLoaderInfo)"]
pub struct NegotiateLoaderInfo {
    pub struct_type: LoaderInterfaceStructs,
    pub struct_version: u32,
    pub struct_size: usize,
    pub min_interface_version: u32,
    pub max_interface_version: u32,
    pub min_api_version: Version,
    pub max_api_version: Version,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrNegotiateApiLayerRequest](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrNegotiateApiLayerRequest)"]
pub struct NegotiateApiLayerRequest {
    pub struct_type: LoaderInterfaceStructs,
    pub struct_version: u32,
    pub struct_size: usize,
    pub layer_interface_version: u32,
    pub layer_api_version: Version,
    pub get_instance_proc_addr: Option<pfn::GetInstanceProcAddr>,
    pub create_api_layer_instance: Option<pfn::CreateApiLayerInstance>,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrNegotiateRuntimeRequest](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrNegotiateRuntimeRequest)"]
pub struct NegotiateRuntimeRequest {
    pub struct_type: LoaderInterfaceStructs,
    pub struct_version: u32,
    pub struct_size: usize,
    pub runtime_interface_version: u32,
    pub runtime_api_version: Version,
    pub get_instance_proc_addr: Option<pfn::GetInstanceProcAddr>,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrApiLayerNextInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrApiLayerNextInfo)"]
pub struct ApiLayerNextInfo {
    pub struct_type: LoaderInterfaceStructs,
    pub struct_version: u32,
    pub struct_size: usize,
    pub layer_name: [c_char; MAX_API_LAYER_NAME_SIZE],
    pub next_get_instance_proc_addr: Option<pfn::GetInstanceProcAddr>,
    pub next_create_api_layer_instance: Option<pfn::CreateApiLayerInstance>,
    pub next: *mut ApiLayerNextInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrApiLayerCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrApiLayerCreateInfo)"]
pub struct ApiLayerCreateInfo {
    pub struct_type: LoaderInterfaceStructs,
    pub struct_version: u32,
    pub struct_size: usize,
    pub loader_instance: *mut c_void,
    pub settings_file_location: [c_char; API_LAYER_MAX_SETTINGS_PATH_SIZE],
    pub next_info: *mut ApiLayerNextInfo,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBaseInStructure](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBaseInStructure)"]
pub struct BaseInStructure {
    pub ty: StructureType,
    pub next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBaseOutStructure](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBaseOutStructure)"]
pub struct BaseOutStructure {
    pub ty: StructureType,
    pub next: *mut BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrApiLayerProperties)"]
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
#[doc = "See [XrExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExtensionProperties)"]
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
#[doc = "See [XrApplicationInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrApplicationInfo)"]
pub struct ApplicationInfo {
    pub application_name: [c_char; MAX_APPLICATION_NAME_SIZE],
    pub application_version: u32,
    pub engine_name: [c_char; MAX_ENGINE_NAME_SIZE],
    pub engine_version: u32,
    pub api_version: Version,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInstanceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInstanceCreateInfo)"]
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
#[doc = "See [XrInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInstanceProperties)"]
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
#[doc = "See [XrSystemGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemGetInfo)"]
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
#[doc = "See [XrSystemProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemProperties)"]
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
#[doc = "See [XrSystemGraphicsProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemGraphicsProperties)"]
pub struct SystemGraphicsProperties {
    pub max_swapchain_image_height: u32,
    pub max_swapchain_image_width: u32,
    pub max_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSystemTrackingProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemTrackingProperties)"]
pub struct SystemTrackingProperties {
    pub orientation_tracking: Bool32,
    pub position_tracking: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingOpenGLWin32KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingOpenGLWin32KHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrGraphicsBindingOpenGLXlibKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingOpenGLXlibKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrGraphicsBindingOpenGLXcbKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingOpenGLXcbKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrGraphicsBindingOpenGLWaylandKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingOpenGLWaylandKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrGraphicsBindingD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D11_enable)"]
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
#[doc = "See [XrGraphicsBindingD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D12_enable)"]
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
#[doc = "See [XrGraphicsBindingOpenGLESAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingOpenGLESAndroidKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_es_enable)"]
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
#[doc = "See [XrGraphicsBindingVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
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
pub type GraphicsBindingVulkan2KHR = GraphicsBindingVulkanKHR;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsBindingMetalKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingMetalKHR) - defined by [XR_KHR_metal_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_metal_enable)"]
#[cfg(target_vendor = "apple")]
pub struct GraphicsBindingMetalKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub command_queue: *mut c_void,
}
#[cfg(target_vendor = "apple")]
impl GraphicsBindingMetalKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_METAL_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionCreateInfo)"]
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
#[doc = "See [XrSessionBeginInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionBeginInfo)"]
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
#[doc = "See [XrSwapchainCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainCreateInfo)"]
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
#[doc = "See [XrSwapchainImageBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageBaseHeader)"]
pub struct SwapchainImageBaseHeader {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageOpenGLKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageOpenGLKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrSwapchainImageOpenGLESKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageOpenGLESKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_es_enable)"]
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
#[doc = "See [XrSwapchainImageVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
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
pub type SwapchainImageVulkan2KHR = SwapchainImageVulkanKHR;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D11_enable)"]
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
#[doc = "See [XrSwapchainImageD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D12_enable)"]
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
#[doc = "See [XrSwapchainImageMetalKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageMetalKHR) - defined by [XR_KHR_metal_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_metal_enable)"]
#[cfg(target_vendor = "apple")]
pub struct SwapchainImageMetalKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub texture: *mut c_void,
}
#[cfg(target_vendor = "apple")]
impl SwapchainImageMetalKHR {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_METAL_KHR;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageAcquireInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageAcquireInfo)"]
pub struct SwapchainImageAcquireInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageAcquireInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_ACQUIRE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainImageWaitInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageWaitInfo)"]
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
#[doc = "See [XrSwapchainImageReleaseInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageReleaseInfo)"]
pub struct SwapchainImageReleaseInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageReleaseInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_RELEASE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrReferenceSpaceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrReferenceSpaceCreateInfo)"]
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
#[doc = "See [XrActionSpaceCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionSpaceCreateInfo)"]
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
#[doc = "See [XrSpaceLocation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceLocation)"]
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
#[doc = "See [XrSpaceVelocity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceVelocity)"]
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
#[doc = "See [XrFovf](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFovf)"]
pub struct Fovf {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrView](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrView)"]
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
#[doc = "See [XrViewLocateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewLocateInfo)"]
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
#[doc = "See [XrViewState](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewState)"]
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
#[doc = "See [XrViewConfigurationView](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewConfigurationView)"]
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
#[doc = "See [XrSwapchainSubImage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainSubImage)"]
pub struct SwapchainSubImage {
    pub swapchain: Swapchain,
    pub image_rect: Rect2Di,
    pub image_array_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerBaseHeader)"]
pub struct CompositionLayerBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerProjectionView](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerProjectionView)"]
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
#[doc = "See [XrCompositionLayerProjection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerProjection)"]
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
#[doc = "See [XrCompositionLayerQuad](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerQuad)"]
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
#[doc = "See [XrCompositionLayerCylinderKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerCylinderKHR) - defined by [XR_KHR_composition_layer_cylinder](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_cylinder)"]
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
#[doc = "See [XrCompositionLayerCubeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerCubeKHR) - defined by [XR_KHR_composition_layer_cube](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_cube)"]
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
#[doc = "See [XrCompositionLayerEquirectKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerEquirectKHR) - defined by [XR_KHR_composition_layer_equirect](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_equirect)"]
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
#[doc = "See [XrCompositionLayerDepthInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerDepthInfoKHR) - defined by [XR_KHR_composition_layer_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_depth)"]
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
#[doc = "See [XrFrameBeginInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameBeginInfo)"]
pub struct FrameBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameBeginInfo {
    pub const TYPE: StructureType = StructureType::FRAME_BEGIN_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameEndInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameEndInfo)"]
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
#[doc = "See [XrFrameWaitInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameWaitInfo)"]
pub struct FrameWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameWaitInfo {
    pub const TYPE: StructureType = StructureType::FRAME_WAIT_INFO;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameState](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameState)"]
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
#[doc = "See [XrHapticBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHapticBaseHeader)"]
pub struct HapticBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticVibration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHapticVibration)"]
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
#[doc = "See [XrEventDataBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataBaseHeader)"]
pub struct EventDataBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataBuffer](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataBuffer)"]
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
#[doc = "See [XrEventDataEventsLost](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataEventsLost)"]
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
#[doc = "See [XrEventDataInstanceLossPending](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataInstanceLossPending)"]
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
#[doc = "See [XrEventDataSessionStateChanged](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSessionStateChanged)"]
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
#[doc = "See [XrEventDataReferenceSpaceChangePending](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataReferenceSpaceChangePending)"]
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
#[doc = "See [XrEventDataPerfSettingsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataPerfSettingsEXT) - defined by [XR_EXT_performance_settings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_performance_settings)"]
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
#[doc = "See [XrEventDataVisibilityMaskChangedKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVisibilityMaskChangedKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_visibility_mask)"]
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
#[doc = "See [XrViewConfigurationProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewConfigurationProperties)"]
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
#[doc = "See [XrActionStateBoolean](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionStateBoolean)"]
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
#[doc = "See [XrActionStateFloat](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionStateFloat)"]
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
#[doc = "See [XrActionStateVector2f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionStateVector2f)"]
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
#[doc = "See [XrActionStatePose](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionStatePose)"]
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
#[doc = "See [XrActionStateGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionStateGetInfo)"]
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
#[doc = "See [XrHapticActionInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHapticActionInfo)"]
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
#[doc = "See [XrActionSetCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionSetCreateInfo)"]
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
#[doc = "See [XrActionSuggestedBinding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionSuggestedBinding)"]
pub struct ActionSuggestedBinding {
    pub action: Action,
    pub binding: Path,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInteractionProfileSuggestedBinding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInteractionProfileSuggestedBinding)"]
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
#[doc = "See [XrActiveActionSet](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActiveActionSet)"]
pub struct ActiveActionSet {
    pub action_set: ActionSet,
    pub subaction_path: Path,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionActionSetsAttachInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionActionSetsAttachInfo)"]
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
#[doc = "See [XrActionsSyncInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionsSyncInfo)"]
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
#[doc = "See [XrBoundSourcesForActionEnumerateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBoundSourcesForActionEnumerateInfo)"]
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
#[doc = "See [XrInputSourceLocalizedNameGetInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInputSourceLocalizedNameGetInfo)"]
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
#[doc = "See [XrEventDataInteractionProfileChanged](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataInteractionProfileChanged)"]
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
#[doc = "See [XrInteractionProfileState](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInteractionProfileState)"]
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
#[doc = "See [XrActionCreateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActionCreateInfo)"]
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
#[doc = "See [XrInstanceCreateInfoAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInstanceCreateInfoAndroidKHR) - defined by [XR_KHR_android_create_instance](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_android_create_instance)"]
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
#[doc = "See [XrVulkanSwapchainFormatListCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanSwapchainFormatListCreateInfoKHR) - defined by [XR_KHR_vulkan_swapchain_format_list](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_swapchain_format_list)"]
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
#[doc = "See [XrDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsObjectNameInfoEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
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
#[doc = "See [XrDebugUtilsLabelEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsLabelEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
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
#[doc = "See [XrDebugUtilsMessengerCallbackDataEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsMessengerCallbackDataEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
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
#[doc = "See [XrDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDebugUtilsMessengerCreateInfoEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
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
#[doc = "See [XrVisibilityMaskKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVisibilityMaskKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_visibility_mask)"]
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
#[doc = "See [XrGraphicsRequirementsOpenGLKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsOpenGLKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
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
#[doc = "See [XrGraphicsRequirementsOpenGLESKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsOpenGLESKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_es_enable)"]
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
#[doc = "See [XrGraphicsRequirementsVulkanKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsVulkanKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
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
pub type GraphicsRequirementsVulkan2KHR = GraphicsRequirementsVulkanKHR;
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrGraphicsRequirementsD3D11KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsD3D11KHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D11_enable)"]
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
#[doc = "See [XrGraphicsRequirementsD3D12KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsD3D12KHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D12_enable)"]
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
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrGraphicsRequirementsMetalKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsRequirementsMetalKHR) - defined by [XR_KHR_metal_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_metal_enable)"]
#[cfg(target_vendor = "apple")]
pub struct GraphicsRequirementsMetalKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub metal_device: *mut c_void,
}
#[cfg(target_vendor = "apple")]
impl GraphicsRequirementsMetalKHR {
    pub const TYPE: StructureType = StructureType::GRAPHICS_REQUIREMENTS_METAL_KHR;
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
#[doc = "See [XrVulkanInstanceCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanInstanceCreateInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
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
#[doc = "See [XrVulkanDeviceCreateInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanDeviceCreateInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
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
#[doc = "See [XrVulkanGraphicsDeviceGetInfoKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanGraphicsDeviceGetInfoKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
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
#[doc = "See [XrVulkanSwapchainCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVulkanSwapchainCreateInfoMETA) - defined by [XR_META_vulkan_swapchain_create_info](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_vulkan_swapchain_create_info)"]
pub struct VulkanSwapchainCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub additional_create_flags: VkImageCreateFlags,
    pub additional_usage_flags: VkImageUsageFlags,
}
impl VulkanSwapchainCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::VULKAN_SWAPCHAIN_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSessionCreateInfoOverlayEXTX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSessionCreateInfoOverlayEXTX) - defined by [XR_EXTX_overlay](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXTX_overlay)"]
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
#[doc = "See [XrEventDataMainSessionVisibilityChangedEXTX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataMainSessionVisibilityChangedEXTX) - defined by [XR_EXTX_overlay](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXTX_overlay)"]
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
#[doc = "See [XrEventDataDisplayRefreshRateChangedFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataDisplayRefreshRateChangedFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_display_refresh_rate)"]
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
#[doc = "See [XrViewConfigurationDepthRangeEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewConfigurationDepthRangeEXT) - defined by [XR_EXT_view_configuration_depth_range](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_view_configuration_depth_range)"]
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
#[doc = "See [XrViewConfigurationViewFovEPIC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewConfigurationViewFovEPIC) - defined by [XR_EPIC_view_configuration_fov](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EPIC_view_configuration_fov)"]
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
#[doc = "See [XrInteractionProfileDpadBindingEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInteractionProfileDpadBindingEXT) - defined by [XR_EXT_dpad_binding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_dpad_binding)"]
pub struct InteractionProfileDpadBindingEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub binding: Path,
    pub action_set: ActionSet,
    pub force_threshold: f32,
    pub force_threshold_released: f32,
    pub center_region: f32,
    pub wedge_angle: f32,
    pub is_sticky: Bool32,
    pub on_haptic: *const HapticBaseHeader,
    pub off_haptic: *const HapticBaseHeader,
}
impl InteractionProfileDpadBindingEXT {
    pub const TYPE: StructureType = StructureType::INTERACTION_PROFILE_DPAD_BINDING_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrInteractionProfileAnalogThresholdVALVE](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrInteractionProfileAnalogThresholdVALVE) - defined by [XR_VALVE_analog_threshold](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VALVE_analog_threshold)"]
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
#[doc = "See [XrBindingModificationsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBindingModificationsKHR) - defined by [XR_KHR_binding_modification](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_binding_modification)"]
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
#[doc = "See [XrBindingModificationBaseHeaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBindingModificationBaseHeaderKHR) - defined by [XR_KHR_binding_modification](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_binding_modification)"]
pub struct BindingModificationBaseHeaderKHR {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemEyeGazeInteractionPropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemEyeGazeInteractionPropertiesEXT) - defined by [XR_EXT_eye_gaze_interaction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_eye_gaze_interaction)"]
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
#[doc = "See [XrEyeGazeSampleTimeEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeGazeSampleTimeEXT) - defined by [XR_EXT_eye_gaze_interaction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_eye_gaze_interaction)"]
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
#[doc = "See [XrSpatialAnchorCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorCreateInfoMSFT)"]
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
#[doc = "See [XrSpatialAnchorSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorSpaceCreateInfoMSFT)"]
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
#[doc = "See [XrCompositionLayerImageLayoutFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerImageLayoutFB) - defined by [XR_FB_composition_layer_image_layout](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_composition_layer_image_layout)"]
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
#[doc = "See [XrCompositionLayerAlphaBlendFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerAlphaBlendFB) - defined by [XR_FB_composition_layer_alpha_blend](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_composition_layer_alpha_blend)"]
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
#[doc = "See [XrGraphicsBindingEGLMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGraphicsBindingEGLMNDX) - defined by [XR_MNDX_egl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MNDX_egl_enable)"]
pub struct GraphicsBindingEGLMNDX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub get_proc_address: Option<pfn::EglGetProcAddressMNDX>,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
}
impl GraphicsBindingEGLMNDX {
    pub const TYPE: StructureType = StructureType::GRAPHICS_BINDING_EGL_MNDX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialGraphNodeSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphNodeSpaceCreateInfoMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
pub struct SpatialGraphNodeSpaceCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub node_type: SpatialGraphNodeTypeMSFT,
    pub node_id: [u8; GUID_SIZE_MSFT],
    pub pose: Posef,
}
impl SpatialGraphNodeSpaceCreateInfoMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialGraphStaticNodeBindingCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphStaticNodeBindingCreateInfoMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
pub struct SpatialGraphStaticNodeBindingCreateInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub pose_in_space: Posef,
    pub time: Time,
}
impl SpatialGraphStaticNodeBindingCreateInfoMSFT {
    pub const TYPE: StructureType =
        StructureType::SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialGraphNodeBindingPropertiesGetInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphNodeBindingPropertiesGetInfoMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
pub struct SpatialGraphNodeBindingPropertiesGetInfoMSFT {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SpatialGraphNodeBindingPropertiesGetInfoMSFT {
    pub const TYPE: StructureType =
        StructureType::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialGraphNodeBindingPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialGraphNodeBindingPropertiesMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
pub struct SpatialGraphNodeBindingPropertiesMSFT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub node_id: [u8; GUID_SIZE_MSFT],
    pub pose_in_node_space: Posef,
}
impl SpatialGraphNodeBindingPropertiesMSFT {
    pub const TYPE: StructureType = StructureType::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT;
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
#[doc = "See [XrSystemHandTrackingPropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemHandTrackingPropertiesEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
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
#[doc = "See [XrHandTrackerCreateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackerCreateInfoEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
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
#[doc = "See [XrHandJointsLocateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointsLocateInfoEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
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
#[doc = "See [XrHandJointLocationEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointLocationEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointLocationEXT {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
    pub radius: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrHandJointVelocityEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointVelocityEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
pub struct HandJointVelocityEXT {
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandJointLocationsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointLocationsEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
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
#[doc = "See [XrHandJointVelocitiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointVelocitiesEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
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
#[doc = "See [XrSystemFaceTrackingPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemFaceTrackingPropertiesFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
pub struct SystemFaceTrackingPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_face_tracking: Bool32,
}
impl SystemFaceTrackingPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_FACE_TRACKING_PROPERTIES_FB;
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
#[doc = "See [XrFaceTrackerCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceTrackerCreateInfoFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
pub struct FaceTrackerCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub face_expression_set: FaceExpressionSetFB,
}
impl FaceTrackerCreateInfoFB {
    pub const TYPE: StructureType = StructureType::FACE_TRACKER_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFaceExpressionInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionInfoFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
pub struct FaceExpressionInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub time: Time,
}
impl FaceExpressionInfoFB {
    pub const TYPE: StructureType = StructureType::FACE_EXPRESSION_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrFaceExpressionStatusFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionStatusFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
pub struct FaceExpressionStatusFB {
    pub is_valid: Bool32,
    pub is_eye_following_blendshapes_valid: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFaceExpressionWeightsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionWeightsFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
pub struct FaceExpressionWeightsFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub weight_count: u32,
    pub weights: *mut f32,
    pub confidence_count: u32,
    pub confidences: *mut f32,
    pub status: FaceExpressionStatusFB,
    pub time: Time,
}
impl FaceExpressionWeightsFB {
    pub const TYPE: StructureType = StructureType::FACE_EXPRESSION_WEIGHTS_FB;
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
#[doc = "See [XrSystemFaceTrackingProperties2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemFaceTrackingProperties2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
pub struct SystemFaceTrackingProperties2FB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_visual_face_tracking: Bool32,
    pub supports_audio_face_tracking: Bool32,
}
impl SystemFaceTrackingProperties2FB {
    pub const TYPE: StructureType = StructureType::SYSTEM_FACE_TRACKING_PROPERTIES2_FB;
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
#[doc = "See [XrFaceTrackerCreateInfo2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceTrackerCreateInfo2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
pub struct FaceTrackerCreateInfo2FB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub face_expression_set: FaceExpressionSet2FB,
    pub requested_data_source_count: u32,
    pub requested_data_sources: *mut FaceTrackingDataSource2FB,
}
impl FaceTrackerCreateInfo2FB {
    pub const TYPE: StructureType = StructureType::FACE_TRACKER_CREATE_INFO2_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFaceExpressionInfo2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionInfo2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
pub struct FaceExpressionInfo2FB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub time: Time,
}
impl FaceExpressionInfo2FB {
    pub const TYPE: StructureType = StructureType::FACE_EXPRESSION_INFO2_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFaceExpressionWeights2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFaceExpressionWeights2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
pub struct FaceExpressionWeights2FB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub weight_count: u32,
    pub weights: *mut f32,
    pub confidence_count: u32,
    pub confidences: *mut f32,
    pub is_valid: Bool32,
    pub is_eye_following_blendshapes_valid: Bool32,
    pub data_source: FaceTrackingDataSource2FB,
    pub time: Time,
}
impl FaceExpressionWeights2FB {
    pub const TYPE: StructureType = StructureType::FACE_EXPRESSION_WEIGHTS2_FB;
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
#[doc = "See [XrSystemBodyTrackingPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemBodyTrackingPropertiesFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct SystemBodyTrackingPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_body_tracking: Bool32,
}
impl SystemBodyTrackingPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_BODY_TRACKING_PROPERTIES_FB;
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
#[doc = "See [XrBodyTrackerCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyTrackerCreateInfoFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodyTrackerCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub body_joint_set: BodyJointSetFB,
}
impl BodyTrackerCreateInfoFB {
    pub const TYPE: StructureType = StructureType::BODY_TRACKER_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrBodySkeletonJointFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodySkeletonJointFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodySkeletonJointFB {
    pub joint: i32,
    pub parent_joint: i32,
    pub pose: Posef,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBodySkeletonFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodySkeletonFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodySkeletonFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub joint_count: u32,
    pub joints: *mut BodySkeletonJointFB,
}
impl BodySkeletonFB {
    pub const TYPE: StructureType = StructureType::BODY_SKELETON_FB;
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
#[doc = "See [XrBodyJointsLocateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyJointsLocateInfoFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodyJointsLocateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
}
impl BodyJointsLocateInfoFB {
    pub const TYPE: StructureType = StructureType::BODY_JOINTS_LOCATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrBodyJointLocationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyJointLocationFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodyJointLocationFB {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBodyJointLocationsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBodyJointLocationsFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
pub struct BodyJointLocationsFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
    pub confidence: f32,
    pub joint_count: u32,
    pub joint_locations: *mut BodyJointLocationFB,
    pub skeleton_changed_count: u32,
    pub time: Time,
}
impl BodyJointLocationsFB {
    pub const TYPE: StructureType = StructureType::BODY_JOINT_LOCATIONS_FB;
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
#[doc = "See [XrSystemEyeTrackingPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemEyeTrackingPropertiesFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
pub struct SystemEyeTrackingPropertiesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_eye_tracking: Bool32,
}
impl SystemEyeTrackingPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_EYE_TRACKING_PROPERTIES_FB;
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
#[doc = "See [XrEyeTrackerCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeTrackerCreateInfoFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
pub struct EyeTrackerCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl EyeTrackerCreateInfoFB {
    pub const TYPE: StructureType = StructureType::EYE_TRACKER_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEyeGazesInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeGazesInfoFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
pub struct EyeGazesInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
}
impl EyeGazesInfoFB {
    pub const TYPE: StructureType = StructureType::EYE_GAZES_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrEyeGazeFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeGazeFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
pub struct EyeGazeFB {
    pub is_valid: Bool32,
    pub gaze_pose: Posef,
    pub gaze_confidence: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEyeGazesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEyeGazesFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
pub struct EyeGazesFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub gaze: [EyeGazeFB; EYE_POSITION_COUNT_FB],
    pub time: Time,
}
impl EyeGazesFB {
    pub const TYPE: StructureType = StructureType::EYE_GAZES_FB;
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
#[doc = "See [XrHandJointsMotionRangeInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandJointsMotionRangeInfoEXT) - defined by [XR_EXT_hand_joints_motion_range](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_joints_motion_range)"]
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
#[doc = "See [XrHandTrackingDataSourceInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingDataSourceInfoEXT) - defined by [XR_EXT_hand_tracking_data_source](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking_data_source)"]
pub struct HandTrackingDataSourceInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub requested_data_source_count: u32,
    pub requested_data_sources: *mut HandTrackingDataSourceEXT,
}
impl HandTrackingDataSourceInfoEXT {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_DATA_SOURCE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingDataSourceStateEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingDataSourceStateEXT) - defined by [XR_EXT_hand_tracking_data_source](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking_data_source)"]
pub struct HandTrackingDataSourceStateEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
    pub data_source: HandTrackingDataSourceEXT,
}
impl HandTrackingDataSourceStateEXT {
    pub const TYPE: StructureType = StructureType::HAND_TRACKING_DATA_SOURCE_STATE_EXT;
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
#[doc = "See [XrHandMeshSpaceCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshSpaceCreateInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
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
#[doc = "See [XrHandMeshUpdateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshUpdateInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
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
#[doc = "See [XrHandMeshMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
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
#[doc = "See [XrHandMeshIndexBufferMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshIndexBufferMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshIndexBufferMSFT {
    pub index_buffer_key: u32,
    pub index_capacity_input: u32,
    pub index_count_output: u32,
    pub indices: *mut u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandMeshVertexBufferMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshVertexBufferMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshVertexBufferMSFT {
    pub vertex_update_time: Time,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut HandMeshVertexMSFT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrHandMeshVertexMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandMeshVertexMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
pub struct HandMeshVertexMSFT {
    pub position: Vector3f,
    pub normal: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemHandTrackingMeshPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemHandTrackingMeshPropertiesMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
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
#[doc = "See [XrHandPoseTypeInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandPoseTypeInfoMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
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
#[doc = "See [XrSecondaryViewConfigurationSessionBeginInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationSessionBeginInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrSecondaryViewConfigurationStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationStateMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrSecondaryViewConfigurationFrameStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationFrameStateMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrSecondaryViewConfigurationFrameEndInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationFrameEndInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrSecondaryViewConfigurationLayerInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationLayerInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrSecondaryViewConfigurationSwapchainCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSecondaryViewConfigurationSwapchainCreateInfoMSFT) - defined by [XR_MSFT_secondary_view_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_secondary_view_configuration)"]
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
#[doc = "See [XrHolographicWindowAttachmentMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHolographicWindowAttachmentMSFT) - defined by [XR_MSFT_holographic_window_attachment](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_holographic_window_attachment)"]
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
#[doc = "See [XrAndroidSurfaceSwapchainCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrAndroidSurfaceSwapchainCreateInfoFB) - defined by [XR_FB_android_surface_swapchain_create](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_android_surface_swapchain_create)"]
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
#[doc = "See [XrSwapchainStateBaseHeaderFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateBaseHeaderFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state)"]
pub struct SwapchainStateBaseHeaderFB {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSwapchainStateAndroidSurfaceDimensionsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateAndroidSurfaceDimensionsFB) - defined by [XR_FB_swapchain_update_state_android_surface](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state_android_surface)"]
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
#[doc = "See [XrSwapchainStateSamplerOpenGLESFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateSamplerOpenGLESFB) - defined by [XR_FB_swapchain_update_state_opengl_es](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state_opengl_es)"]
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
#[doc = "See [XrSwapchainStateSamplerVulkanFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateSamplerVulkanFB) - defined by [XR_FB_swapchain_update_state_vulkan](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state_vulkan)"]
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
#[doc = "See [XrCompositionLayerSecureContentFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSecureContentFB) - defined by [XR_FB_composition_layer_secure_content](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_composition_layer_secure_content)"]
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
#[doc = "See [XrLoaderInitInfoBaseHeaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLoaderInitInfoBaseHeaderKHR) - defined by [XR_KHR_loader_init](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_loader_init)"]
pub struct LoaderInitInfoBaseHeaderKHR {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLoaderInitInfoAndroidKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLoaderInitInfoAndroidKHR) - defined by [XR_KHR_loader_init_android](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_loader_init_android)"]
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
#[doc = "See [XrCompositionLayerEquirect2KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerEquirect2KHR) - defined by [XR_KHR_composition_layer_equirect2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_equirect2)"]
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
#[doc = "See [XrCompositionLayerColorScaleBiasKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerColorScaleBiasKHR) - defined by [XR_KHR_composition_layer_color_scale_bias](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_composition_layer_color_scale_bias)"]
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
#[doc = "See [XrControllerModelKeyStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrControllerModelKeyStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
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
#[doc = "See [XrControllerModelNodePropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrControllerModelNodePropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
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
#[doc = "See [XrControllerModelPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrControllerModelPropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
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
#[doc = "See [XrControllerModelNodeStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrControllerModelNodeStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
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
#[doc = "See [XrControllerModelStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrControllerModelStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
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
#[doc = "See [XrSystemColorSpacePropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemColorSpacePropertiesFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_color_space)"]
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
#[doc = "See [XrSystemSpatialEntityPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemSpatialEntityPropertiesFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct SystemSpatialEntityPropertiesFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub supports_spatial_entity: Bool32,
}
impl SystemSpatialEntityPropertiesFB {
    pub const TYPE: StructureType = StructureType::SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorCreateInfoFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct SpatialAnchorCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub pose_in_space: Posef,
    pub time: Time,
}
impl SpatialAnchorCreateInfoFB {
    pub const TYPE: StructureType = StructureType::SPATIAL_ANCHOR_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceComponentStatusSetInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceComponentStatusSetInfoFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct SpaceComponentStatusSetInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub component_type: SpaceComponentTypeFB,
    pub enabled: Bool32,
    pub timeout: Duration,
}
impl SpaceComponentStatusSetInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_COMPONENT_STATUS_SET_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceComponentStatusFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceComponentStatusFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct SpaceComponentStatusFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub enabled: Bool32,
    pub change_pending: Bool32,
}
impl SpaceComponentStatusFB {
    pub const TYPE: StructureType = StructureType::SPACE_COMPONENT_STATUS_FB;
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
#[doc = "See [XrEventDataSpatialAnchorCreateCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpatialAnchorCreateCompleteFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct EventDataSpatialAnchorCreateCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
    pub space: Space,
    pub uuid: UuidEXT,
}
impl EventDataSpatialAnchorCreateCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceSetStatusCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceSetStatusCompleteFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
pub struct EventDataSpaceSetStatusCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
    pub space: Space,
    pub uuid: UuidEXT,
    pub component_type: SpaceComponentTypeFB,
    pub enabled: Bool32,
}
impl EventDataSpaceSetStatusCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationProfileCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationProfileCreateInfoFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation)"]
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
#[doc = "See [XrSwapchainCreateInfoFoveationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainCreateInfoFoveationFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation)"]
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
#[doc = "See [XrSwapchainStateFoveationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainStateFoveationFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation)"]
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
#[doc = "See [XrSwapchainImageFoveationVulkanFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSwapchainImageFoveationVulkanFB) - defined by [XR_FB_foveation_vulkan](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation_vulkan)"]
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
#[doc = "See [XrFoveationLevelProfileCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationLevelProfileCreateInfoFB) - defined by [XR_FB_foveation_configuration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation_configuration)"]
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
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationEyeTrackedProfileCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationEyeTrackedProfileCreateInfoMETA) - defined by [XR_META_foveation_eye_tracked](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_foveation_eye_tracked)"]
pub struct FoveationEyeTrackedProfileCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: FoveationEyeTrackedProfileCreateFlagsMETA,
}
impl FoveationEyeTrackedProfileCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::FOVEATION_EYE_TRACKED_PROFILE_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationEyeTrackedStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationEyeTrackedStateMETA) - defined by [XR_META_foveation_eye_tracked](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_foveation_eye_tracked)"]
pub struct FoveationEyeTrackedStateMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub foveation_center: [Vector2f; FOVEATION_CENTER_SIZE_META],
    pub flags: FoveationEyeTrackedStateFlagsMETA,
}
impl FoveationEyeTrackedStateMETA {
    pub const TYPE: StructureType = StructureType::FOVEATION_EYE_TRACKED_STATE_META;
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
#[doc = "See [XrSystemFoveationEyeTrackedPropertiesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemFoveationEyeTrackedPropertiesMETA) - defined by [XR_META_foveation_eye_tracked](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_foveation_eye_tracked)"]
pub struct SystemFoveationEyeTrackedPropertiesMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_foveation_eye_tracked: Bool32,
}
impl SystemFoveationEyeTrackedPropertiesMETA {
    pub const TYPE: StructureType = StructureType::SYSTEM_FOVEATION_EYE_TRACKED_PROPERTIES_META;
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
#[doc = "See [XrVector4sFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVector4sFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
pub struct Vector4sFB {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingMeshFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
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
#[doc = "See [XrHandTrackingScaleFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingScaleFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
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
#[doc = "See [XrHandTrackingAimStateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingAimStateFB) - defined by [XR_FB_hand_tracking_aim](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_aim)"]
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
#[doc = "See [XrHandCapsuleFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandCapsuleFB) - defined by [XR_FB_hand_tracking_capsules](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_capsules)"]
pub struct HandCapsuleFB {
    pub points: [Vector3f; HAND_TRACKING_CAPSULE_POINT_COUNT_FB],
    pub radius: f32,
    pub joint: HandJointEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHandTrackingCapsulesStateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHandTrackingCapsulesStateFB) - defined by [XR_FB_hand_tracking_capsules](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_capsules)"]
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
#[doc = "See [XrRenderModelPathInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelPathInfoFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
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
#[doc = "See [XrRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
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
#[doc = "See [XrRenderModelCapabilitiesRequestFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelCapabilitiesRequestFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
pub struct RenderModelCapabilitiesRequestFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub flags: RenderModelFlagsFB,
}
impl RenderModelCapabilitiesRequestFB {
    pub const TYPE: StructureType = StructureType::RENDER_MODEL_CAPABILITIES_REQUEST_FB;
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
#[doc = "See [XrRenderModelBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelBufferFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
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
#[doc = "See [XrRenderModelLoadInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRenderModelLoadInfoFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
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
#[doc = "See [XrSystemRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
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
#[doc = "See [XrSpaceQueryInfoBaseHeaderFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceQueryInfoBaseHeaderFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceQueryInfoBaseHeaderFB {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceFilterInfoBaseHeaderFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceFilterInfoBaseHeaderFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceFilterInfoBaseHeaderFB {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceQueryInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceQueryInfoFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceQueryInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub query_action: SpaceQueryActionFB,
    pub max_result_count: u32,
    pub timeout: Duration,
    pub filter: *const SpaceFilterInfoBaseHeaderFB,
    pub exclude_filter: *const SpaceFilterInfoBaseHeaderFB,
}
impl SpaceQueryInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_QUERY_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceStorageLocationFilterInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceStorageLocationFilterInfoFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceStorageLocationFilterInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub location: SpaceStorageLocationFB,
}
impl SpaceStorageLocationFilterInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_STORAGE_LOCATION_FILTER_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceUuidFilterInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceUuidFilterInfoFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceUuidFilterInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub uuid_count: u32,
    pub uuids: *mut UuidEXT,
}
impl SpaceUuidFilterInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_UUID_FILTER_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceComponentFilterInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceComponentFilterInfoFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceComponentFilterInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub component_type: SpaceComponentTypeFB,
}
impl SpaceComponentFilterInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_COMPONENT_FILTER_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceQueryResultFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceQueryResultFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceQueryResultFB {
    pub space: Space,
    pub uuid: UuidEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceQueryResultsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceQueryResultsFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct SpaceQueryResultsFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub result_capacity_input: u32,
    pub result_count_output: u32,
    pub results: *mut SpaceQueryResultFB,
}
impl SpaceQueryResultsFB {
    pub const TYPE: StructureType = StructureType::SPACE_QUERY_RESULTS_FB;
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
#[doc = "See [XrEventDataSpaceQueryResultsAvailableFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceQueryResultsAvailableFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct EventDataSpaceQueryResultsAvailableFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
}
impl EventDataSpaceQueryResultsAvailableFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceQueryCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceQueryCompleteFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
pub struct EventDataSpaceQueryCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
}
impl EventDataSpaceQueryCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_QUERY_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceSaveInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceSaveInfoFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
pub struct SpaceSaveInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub location: SpaceStorageLocationFB,
    pub persistence_mode: SpacePersistenceModeFB,
}
impl SpaceSaveInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_SAVE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceEraseInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceEraseInfoFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
pub struct SpaceEraseInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub location: SpaceStorageLocationFB,
}
impl SpaceEraseInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_ERASE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceSaveCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceSaveCompleteFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
pub struct EventDataSpaceSaveCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
    pub space: Space,
    pub uuid: UuidEXT,
    pub location: SpaceStorageLocationFB,
}
impl EventDataSpaceSaveCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_SAVE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceEraseCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceEraseCompleteFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
pub struct EventDataSpaceEraseCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
    pub space: Space,
    pub uuid: UuidEXT,
    pub location: SpaceStorageLocationFB,
}
impl EventDataSpaceEraseCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_ERASE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceShareInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceShareInfoFB) - defined by [XR_FB_spatial_entity_sharing](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_sharing)"]
pub struct SpaceShareInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space_count: u32,
    pub spaces: *mut Space,
    pub user_count: u32,
    pub users: *mut SpaceUserFB,
}
impl SpaceShareInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_SHARE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceShareCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceShareCompleteFB) - defined by [XR_FB_spatial_entity_sharing](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_sharing)"]
pub struct EventDataSpaceShareCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
}
impl EventDataSpaceShareCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_SHARE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceListSaveInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceListSaveInfoFB) - defined by [XR_FB_spatial_entity_storage_batch](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage_batch)"]
pub struct SpaceListSaveInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space_count: u32,
    pub spaces: *mut Space,
    pub location: SpaceStorageLocationFB,
}
impl SpaceListSaveInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_LIST_SAVE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSpaceListSaveCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSpaceListSaveCompleteFB) - defined by [XR_FB_spatial_entity_storage_batch](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage_batch)"]
pub struct EventDataSpaceListSaveCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
}
impl EventDataSpaceListSaveCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SPACE_LIST_SAVE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceContainerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceContainerFB) - defined by [XR_FB_spatial_entity_container](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_container)"]
pub struct SpaceContainerFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub uuid_capacity_input: u32,
    pub uuid_count_output: u32,
    pub uuids: *mut UuidEXT,
}
impl SpaceContainerFB {
    pub const TYPE: StructureType = StructureType::SPACE_CONTAINER_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceTriangleMeshGetInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceTriangleMeshGetInfoMETA) - defined by [XR_META_spatial_entity_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_spatial_entity_mesh)"]
pub struct SpaceTriangleMeshGetInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SpaceTriangleMeshGetInfoMETA {
    pub const TYPE: StructureType = StructureType::SPACE_TRIANGLE_MESH_GET_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceTriangleMeshMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceTriangleMeshMETA) - defined by [XR_META_spatial_entity_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_spatial_entity_mesh)"]
pub struct SpaceTriangleMeshMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut Vector3f,
    pub index_capacity_input: u32,
    pub index_count_output: u32,
    pub indices: *mut u32,
}
impl SpaceTriangleMeshMETA {
    pub const TYPE: StructureType = StructureType::SPACE_TRIANGLE_MESH_META;
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
#[doc = "See [XrOffset3DfFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrOffset3DfFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct Offset3DfFB {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrRect3DfFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRect3DfFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct Rect3DfFB {
    pub offset: Offset3DfFB,
    pub extent: Extent3DfFB,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSemanticLabelsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSemanticLabelsFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct SemanticLabelsFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub buffer_capacity_input: u32,
    pub buffer_count_output: u32,
    pub buffer: *mut c_char,
}
impl SemanticLabelsFB {
    pub const TYPE: StructureType = StructureType::SEMANTIC_LABELS_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRoomLayoutFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRoomLayoutFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct RoomLayoutFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub floor_uuid: UuidEXT,
    pub ceiling_uuid: UuidEXT,
    pub wall_uuid_capacity_input: u32,
    pub wall_uuid_count_output: u32,
    pub wall_uuids: *mut UuidEXT,
}
impl RoomLayoutFB {
    pub const TYPE: StructureType = StructureType::ROOM_LAYOUT_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrBoundary2DFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBoundary2DFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct Boundary2DFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut Vector2f,
}
impl Boundary2DFB {
    pub const TYPE: StructureType = StructureType::BOUNDARY_2D_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSemanticLabelsSupportInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSemanticLabelsSupportInfoFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
pub struct SemanticLabelsSupportInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: SemanticLabelsSupportFlagsFB,
    pub recognized_labels: *const c_char,
}
impl SemanticLabelsSupportInfoFB {
    pub const TYPE: StructureType = StructureType::SEMANTIC_LABELS_SUPPORT_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSceneCaptureRequestInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSceneCaptureRequestInfoFB) - defined by [XR_FB_scene_capture](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene_capture)"]
pub struct SceneCaptureRequestInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_byte_count: u32,
    pub request: *const c_char,
}
impl SceneCaptureRequestInfoFB {
    pub const TYPE: StructureType = StructureType::SCENE_CAPTURE_REQUEST_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataSceneCaptureCompleteFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataSceneCaptureCompleteFB) - defined by [XR_FB_scene_capture](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene_capture)"]
pub struct EventDataSceneCaptureCompleteFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub request_id: AsyncRequestIdFB,
    pub result: Result,
}
impl EventDataSceneCaptureCompleteFB {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_SCENE_CAPTURE_COMPLETE_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemKeyboardTrackingPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemKeyboardTrackingPropertiesFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
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
#[doc = "See [XrKeyboardTrackingDescriptionFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrKeyboardTrackingDescriptionFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
pub struct KeyboardTrackingDescriptionFB {
    pub tracked_keyboard_id: u64,
    pub size: Vector3f,
    pub flags: KeyboardTrackingFlagsFB,
    pub name: [c_char; MAX_KEYBOARD_TRACKING_NAME_SIZE_FB],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrKeyboardSpaceCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrKeyboardSpaceCreateInfoFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
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
#[doc = "See [XrKeyboardTrackingQueryFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrKeyboardTrackingQueryFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
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
#[doc = "See [XrCompositionLayerDepthTestVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerDepthTestVARJO) - defined by [XR_VARJO_composition_layer_depth_test](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_composition_layer_depth_test)"]
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
#[doc = "See [XrViewLocateFoveatedRenderingVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViewLocateFoveatedRenderingVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_foveated_rendering)"]
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
#[doc = "See [XrFoveatedViewConfigurationViewVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveatedViewConfigurationViewVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_foveated_rendering)"]
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
#[doc = "See [XrSystemFoveatedRenderingPropertiesVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemFoveatedRenderingPropertiesVARJO) - defined by [XR_VARJO_foveated_rendering](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_foveated_rendering)"]
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
#[doc = "See [XrCompositionLayerReprojectionInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerReprojectionInfoMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
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
#[doc = "See [XrCompositionLayerReprojectionPlaneOverrideMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerReprojectionPlaneOverrideMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
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
#[doc = "See [XrTriangleMeshCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrTriangleMeshCreateInfoFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
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
#[doc = "See [XrSystemPassthroughPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemPassthroughPropertiesFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrSystemPassthroughProperties2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemPassthroughProperties2FB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
pub struct SystemPassthroughProperties2FB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub capabilities: PassthroughCapabilityFlagsFB,
}
impl SystemPassthroughProperties2FB {
    pub const TYPE: StructureType = StructureType::SYSTEM_PASSTHROUGH_PROPERTIES2_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughLayerCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughLayerCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrCompositionLayerPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerPassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrGeometryInstanceCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGeometryInstanceCreateInfoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrGeometryInstanceTransformFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGeometryInstanceTransformFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughStyleFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughStyleFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughColorMapMonoToRgbaFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorMapMonoToRgbaFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughColorMapMonoToMonoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorMapMonoToMonoFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughBrightnessContrastSaturationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughBrightnessContrastSaturationFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
pub struct PassthroughBrightnessContrastSaturationFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
}
impl PassthroughBrightnessContrastSaturationFB {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataPassthroughStateChangedFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataPassthroughStateChangedFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
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
#[doc = "See [XrPassthroughKeyboardHandsIntensityFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughKeyboardHandsIntensityFB) - defined by [XR_FB_passthrough_keyboard_hands](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough_keyboard_hands)"]
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
#[doc = "See [XrLocalDimmingFrameEndInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalDimmingFrameEndInfoMETA) - defined by [XR_META_local_dimming](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_local_dimming)"]
pub struct LocalDimmingFrameEndInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub local_dimming_mode: LocalDimmingModeMETA,
}
impl LocalDimmingFrameEndInfoMETA {
    pub const TYPE: StructureType = StructureType::LOCAL_DIMMING_FRAME_END_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorPersistenceNameMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorPersistenceNameMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
pub struct SpatialAnchorPersistenceNameMSFT {
    pub name: [c_char; MAX_SPATIAL_ANCHOR_NAME_SIZE_MSFT],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorPersistenceInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorPersistenceInfoMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
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
#[doc = "See [XrSpatialAnchorFromPersistedAnchorCreateInfoMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorFromPersistedAnchorCreateInfoMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
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
#[doc = "See [XrFacialTrackerCreateInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFacialTrackerCreateInfoHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
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
#[doc = "See [XrSystemFacialTrackingPropertiesHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemFacialTrackingPropertiesHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
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
#[doc = "See [XrFacialExpressionsHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFacialExpressionsHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
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
#[doc = "See [XrPassthroughCreateInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughCreateInfoHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
pub struct PassthroughCreateInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub form: PassthroughFormHTC,
}
impl PassthroughCreateInfoHTC {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_CREATE_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
pub struct PassthroughColorHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub alpha: f32,
}
impl PassthroughColorHTC {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughMeshTransformInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughMeshTransformInfoHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
pub struct PassthroughMeshTransformInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub vertex_count: u32,
    pub vertices: *const Vector3f,
    pub index_count: u32,
    pub indices: *const u32,
    pub base_space: Space,
    pub time: Time,
    pub pose: Posef,
    pub scale: Vector3f,
}
impl PassthroughMeshTransformInfoHTC {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_MESH_TRANSFORM_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerPassthroughHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerPassthroughHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
pub struct CompositionLayerPassthroughHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub passthrough: PassthroughHTC,
    pub color: PassthroughColorHTC,
}
impl CompositionLayerPassthroughHTC {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_PASSTHROUGH_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorCreateInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorCreateInfoHTC) - defined by [XR_HTC_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_anchor)"]
pub struct SpatialAnchorCreateInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub pose_in_space: Posef,
    pub name: SpatialAnchorNameHTC,
}
impl SpatialAnchorCreateInfoHTC {
    pub const TYPE: StructureType = StructureType::SPATIAL_ANCHOR_CREATE_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpatialAnchorNameHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpatialAnchorNameHTC) - defined by [XR_HTC_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_anchor)"]
pub struct SpatialAnchorNameHTC {
    pub name: [c_char; MAX_SPATIAL_ANCHOR_NAME_SIZE_HTC],
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemAnchorPropertiesHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemAnchorPropertiesHTC) - defined by [XR_HTC_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_anchor)"]
pub struct SystemAnchorPropertiesHTC {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_anchor: Bool32,
}
impl SystemAnchorPropertiesHTC {
    pub const TYPE: StructureType = StructureType::SYSTEM_ANCHOR_PROPERTIES_HTC;
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
#[doc = "See [XrViveTrackerPathsHTCX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrViveTrackerPathsHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
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
#[doc = "See [XrEventDataViveTrackerConnectedHTCX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataViveTrackerConnectedHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
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
#[doc = "See [XrCompositionLayerSpaceWarpInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSpaceWarpInfoFB) - defined by [XR_FB_space_warp](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_space_warp)"]
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
#[doc = "See [XrSystemSpaceWarpPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemSpaceWarpPropertiesFB) - defined by [XR_FB_space_warp](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_space_warp)"]
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
#[doc = "See [XrSystemMarkerTrackingPropertiesVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemMarkerTrackingPropertiesVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
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
#[doc = "See [XrEventDataMarkerTrackingUpdateVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataMarkerTrackingUpdateVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
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
#[doc = "See [XrMarkerSpaceCreateInfoVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerSpaceCreateInfoVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
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
#[doc = "See [XrGlobalDimmerFrameEndInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrGlobalDimmerFrameEndInfoML) - defined by [XR_ML_global_dimmer](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_global_dimmer)"]
pub struct GlobalDimmerFrameEndInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub dimmer_value: f32,
    pub flags: GlobalDimmerFrameEndInfoFlagsML,
}
impl GlobalDimmerFrameEndInfoML {
    pub const TYPE: StructureType = StructureType::GLOBAL_DIMMER_FRAME_END_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDigitalLensControlALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDigitalLensControlALMALENCE)"]
pub struct DigitalLensControlALMALENCE {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: DigitalLensControlFlagsALMALENCE,
}
impl DigitalLensControlALMALENCE {
    pub const TYPE: StructureType = StructureType::DIGITAL_LENS_CONTROL_ALMALENCE;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerSettingsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerSettingsFB) - defined by [XR_FB_composition_layer_settings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_composition_layer_settings)"]
pub struct CompositionLayerSettingsFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerSettingsFlagsFB,
}
impl CompositionLayerSettingsFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_SETTINGS_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = "See [XrExternalCameraIntrinsicsOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExternalCameraIntrinsicsOCULUS) - defined by [XR_OCULUS_external_camera](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_external_camera)"]
pub struct ExternalCameraIntrinsicsOCULUS {
    pub last_change_time: Time,
    pub fov: Fovf,
    pub virtual_near_plane_distance: f32,
    pub virtual_far_plane_distance: f32,
    pub image_sensor_pixel_resolution: Extent2Di,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = "See [XrExternalCameraExtrinsicsOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExternalCameraExtrinsicsOCULUS) - defined by [XR_OCULUS_external_camera](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_external_camera)"]
pub struct ExternalCameraExtrinsicsOCULUS {
    pub last_change_time: Time,
    pub camera_status_flags: ExternalCameraStatusFlagsOCULUS,
    pub attached_to_device: ExternalCameraAttachedToDeviceOCULUS,
    pub relative_pose: Posef,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrExternalCameraOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExternalCameraOCULUS) - defined by [XR_OCULUS_external_camera](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_external_camera)"]
pub struct ExternalCameraOCULUS {
    pub ty: StructureType,
    pub next: *const c_void,
    pub name: [c_char; MAX_EXTERNAL_CAMERA_NAME_SIZE_OCULUS],
    pub intrinsics: ExternalCameraIntrinsicsOCULUS,
    pub extrinsics: ExternalCameraExtrinsicsOCULUS,
}
impl ExternalCameraOCULUS {
    pub const TYPE: StructureType = StructureType::EXTERNAL_CAMERA_OCULUS;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPerformanceMetricsStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerformanceMetricsStateMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
pub struct PerformanceMetricsStateMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub enabled: Bool32,
}
impl PerformanceMetricsStateMETA {
    pub const TYPE: StructureType = StructureType::PERFORMANCE_METRICS_STATE_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPerformanceMetricsCounterMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPerformanceMetricsCounterMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
pub struct PerformanceMetricsCounterMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub counter_flags: PerformanceMetricsCounterFlagsMETA,
    pub counter_unit: PerformanceMetricsCounterUnitMETA,
    pub uint_value: u32,
    pub float_value: f32,
}
impl PerformanceMetricsCounterMETA {
    pub const TYPE: StructureType = StructureType::PERFORMANCE_METRICS_COUNTER_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughPreferencesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughPreferencesMETA) - defined by [XR_META_passthrough_preferences](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_preferences)"]
pub struct PassthroughPreferencesMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: PassthroughPreferenceFlagsMETA,
}
impl PassthroughPreferencesMETA {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_PREFERENCES_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemHeadsetIdPropertiesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemHeadsetIdPropertiesMETA) - defined by [XR_META_headset_id](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_headset_id)"]
pub struct SystemHeadsetIdPropertiesMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub id: UuidEXT,
}
impl SystemHeadsetIdPropertiesMETA {
    pub const TYPE: StructureType = StructureType::SYSTEM_HEADSET_ID_PROPERTIES_META;
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
#[doc = "See [XrPassthroughColorLutDataMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorLutDataMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct PassthroughColorLutDataMETA {
    pub buffer_size: u32,
    pub buffer: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorLutCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorLutCreateInfoMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct PassthroughColorLutCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub channels: PassthroughColorLutChannelsMETA,
    pub resolution: u32,
    pub data: PassthroughColorLutDataMETA,
}
impl PassthroughColorLutCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_LUT_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorLutUpdateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorLutUpdateInfoMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct PassthroughColorLutUpdateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub data: PassthroughColorLutDataMETA,
}
impl PassthroughColorLutUpdateInfoMETA {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_LUT_UPDATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorMapLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorMapLutMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct PassthroughColorMapLutMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub color_lut: PassthroughColorLutMETA,
    pub weight: f32,
}
impl PassthroughColorMapLutMETA {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_MAP_LUT_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPassthroughColorMapInterpolatedLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPassthroughColorMapInterpolatedLutMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct PassthroughColorMapInterpolatedLutMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub source_color_lut: PassthroughColorLutMETA,
    pub target_color_lut: PassthroughColorLutMETA,
    pub weight: f32,
}
impl PassthroughColorMapInterpolatedLutMETA {
    pub const TYPE: StructureType = StructureType::PASSTHROUGH_COLOR_MAP_INTERPOLATED_LUT_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemPassthroughColorLutPropertiesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemPassthroughColorLutPropertiesMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
pub struct SystemPassthroughColorLutPropertiesMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub max_color_lut_resolution: u32,
}
impl SystemPassthroughColorLutPropertiesMETA {
    pub const TYPE: StructureType = StructureType::SYSTEM_PASSTHROUGH_COLOR_LUT_PROPERTIES_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationApplyInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationApplyInfoHTC) - defined by [XR_HTC_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_foveation)"]
pub struct FoveationApplyInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub mode: FoveationModeHTC,
    pub sub_image_count: u32,
    pub sub_images: *mut SwapchainSubImage,
}
impl FoveationApplyInfoHTC {
    pub const TYPE: StructureType = StructureType::FOVEATION_APPLY_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = "See [XrFoveationConfigurationHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationConfigurationHTC) - defined by [XR_HTC_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_foveation)"]
pub struct FoveationConfigurationHTC {
    pub level: FoveationLevelHTC,
    pub clear_fov_degree: f32,
    pub focal_center_offset: Vector2f,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationDynamicModeInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationDynamicModeInfoHTC) - defined by [XR_HTC_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_foveation)"]
pub struct FoveationDynamicModeInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub dynamic_flags: FoveationDynamicFlagsHTC,
}
impl FoveationDynamicModeInfoHTC {
    pub const TYPE: StructureType = StructureType::FOVEATION_DYNAMIC_MODE_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFoveationCustomModeInfoHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFoveationCustomModeInfoHTC) - defined by [XR_HTC_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_foveation)"]
pub struct FoveationCustomModeInfoHTC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub config_count: u32,
    pub configs: *const FoveationConfigurationHTC,
}
impl FoveationCustomModeInfoHTC {
    pub const TYPE: StructureType = StructureType::FOVEATION_CUSTOM_MODE_INFO_HTC;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActiveActionSetPrioritiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActiveActionSetPrioritiesEXT) - defined by [XR_EXT_active_action_set_priority](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_active_action_set_priority)"]
pub struct ActiveActionSetPrioritiesEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_set_priority_count: u32,
    pub action_set_priorities: *const ActiveActionSetPriorityEXT,
}
impl ActiveActionSetPrioritiesEXT {
    pub const TYPE: StructureType = StructureType::ACTIVE_ACTION_SET_PRIORITIES_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrActiveActionSetPriorityEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrActiveActionSetPriorityEXT) - defined by [XR_EXT_active_action_set_priority](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_active_action_set_priority)"]
pub struct ActiveActionSetPriorityEXT {
    pub action_set: ActionSet,
    pub priority_override: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCompositionLayerDepthTestFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCompositionLayerDepthTestFB) - defined by [XR_FB_composition_layer_depth_test](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_composition_layer_depth_test)"]
pub struct CompositionLayerDepthTestFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub depth_mask: Bool32,
    pub compare_op: CompareOpFB,
}
impl CompositionLayerDepthTestFB {
    pub const TYPE: StructureType = StructureType::COMPOSITION_LAYER_DEPTH_TEST_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrCoordinateSpaceCreateInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrCoordinateSpaceCreateInfoML)"]
pub struct CoordinateSpaceCreateInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub cfuid: MLCoordinateFrameUID,
    pub pose_in_coordinate_space: Posef,
}
impl CoordinateSpaceCreateInfoML {
    pub const TYPE: StructureType = StructureType::COORDINATE_SPACE_CREATE_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFrameEndInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrameEndInfoML) - defined by [XR_ML_frame_end_info](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_frame_end_info)"]
pub struct FrameEndInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub focus_distance: f32,
    pub flags: FrameEndInfoFlagsML,
}
impl FrameEndInfoML {
    pub const TYPE: StructureType = StructureType::FRAME_END_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticAmplitudeEnvelopeVibrationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHapticAmplitudeEnvelopeVibrationFB) - defined by [XR_FB_haptic_amplitude_envelope](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_haptic_amplitude_envelope)"]
pub struct HapticAmplitudeEnvelopeVibrationFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub duration: Duration,
    pub amplitude_count: u32,
    pub amplitudes: *const f32,
}
impl HapticAmplitudeEnvelopeVibrationFB {
    pub const TYPE: StructureType = StructureType::HAPTIC_AMPLITUDE_ENVELOPE_VIBRATION_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrHapticPcmVibrationFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrHapticPcmVibrationFB) - defined by [XR_FB_haptic_pcm](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_haptic_pcm)"]
pub struct HapticPcmVibrationFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub buffer_size: u32,
    pub buffer: *const f32,
    pub sample_rate: f32,
    pub append: Bool32,
    pub samples_consumed: *mut u32,
}
impl HapticPcmVibrationFB {
    pub const TYPE: StructureType = StructureType::HAPTIC_PCM_VIBRATION_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrDevicePcmSampleRateStateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrDevicePcmSampleRateStateFB) - defined by [XR_FB_haptic_pcm](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_haptic_pcm)"]
pub struct DevicePcmSampleRateStateFB {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub sample_rate: f32,
}
impl DevicePcmSampleRateStateFB {
    pub const TYPE: StructureType = StructureType::DEVICE_PCM_SAMPLE_RATE_STATE_FB;
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
pub type DevicePcmSampleRateGetInfoFB = DevicePcmSampleRateStateFB;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceUserCreateInfoFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceUserCreateInfoFB) - defined by [XR_FB_spatial_entity_user](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_user)"]
pub struct SpaceUserCreateInfoFB {
    pub ty: StructureType,
    pub next: *const c_void,
    pub user_id: SpaceUserIdFB,
}
impl SpaceUserCreateInfoFB {
    pub const TYPE: StructureType = StructureType::SPACE_USER_CREATE_INFO_FB;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemForceFeedbackCurlPropertiesMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemForceFeedbackCurlPropertiesMNDX) - defined by [XR_MNDX_force_feedback_curl](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MNDX_force_feedback_curl)"]
pub struct SystemForceFeedbackCurlPropertiesMNDX {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_force_feedback_curl: Bool32,
}
impl SystemForceFeedbackCurlPropertiesMNDX {
    pub const TYPE: StructureType = StructureType::SYSTEM_FORCE_FEEDBACK_CURL_PROPERTIES_MNDX;
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
#[doc = "See [XrForceFeedbackCurlApplyLocationsMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrForceFeedbackCurlApplyLocationsMNDX) - defined by [XR_MNDX_force_feedback_curl](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MNDX_force_feedback_curl)"]
pub struct ForceFeedbackCurlApplyLocationsMNDX {
    pub ty: StructureType,
    pub next: *const c_void,
    pub location_count: u32,
    pub locations: *mut ForceFeedbackCurlApplyLocationMNDX,
}
impl ForceFeedbackCurlApplyLocationsMNDX {
    pub const TYPE: StructureType = StructureType::FORCE_FEEDBACK_CURL_APPLY_LOCATIONS_MNDX;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = "See [XrForceFeedbackCurlApplyLocationMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrForceFeedbackCurlApplyLocationMNDX) - defined by [XR_MNDX_force_feedback_curl](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MNDX_force_feedback_curl)"]
pub struct ForceFeedbackCurlApplyLocationMNDX {
    pub location: ForceFeedbackCurlLocationMNDX,
    pub value: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemPlaneDetectionPropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemPlaneDetectionPropertiesEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct SystemPlaneDetectionPropertiesEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supported_features: PlaneDetectionCapabilityFlagsEXT,
}
impl SystemPlaneDetectionPropertiesEXT {
    pub const TYPE: StructureType = StructureType::SYSTEM_PLANE_DETECTION_PROPERTIES_EXT;
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
#[doc = "See [XrPlaneDetectorCreateInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorCreateInfoEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorCreateInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub flags: PlaneDetectorFlagsEXT,
}
impl PlaneDetectorCreateInfoEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_CREATE_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPlaneDetectorBeginInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorBeginInfoEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorBeginInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
    pub orientation_count: u32,
    pub orientations: *const PlaneDetectorOrientationEXT,
    pub semantic_type_count: u32,
    pub semantic_types: *const PlaneDetectorSemanticTypeEXT,
    pub max_planes: u32,
    pub min_area: f32,
    pub bounding_box_pose: Posef,
    pub bounding_box_extent: Extent3DfEXT,
}
impl PlaneDetectorBeginInfoEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_BEGIN_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPlaneDetectorGetInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorGetInfoEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorGetInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
}
impl PlaneDetectorGetInfoEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_GET_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrPlaneDetectorLocationsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorLocationsEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorLocationsEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub plane_location_capacity_input: u32,
    pub plane_location_count_output: u32,
    pub plane_locations: *mut PlaneDetectorLocationEXT,
}
impl PlaneDetectorLocationsEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_LOCATIONS_EXT;
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
#[doc = "See [XrPlaneDetectorLocationEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorLocationEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorLocationEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub plane_id: u64,
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
    pub extents: Extent2Df,
    pub orientation: PlaneDetectorOrientationEXT,
    pub semantic_type: PlaneDetectorSemanticTypeEXT,
    pub polygon_buffer_count: u32,
}
impl PlaneDetectorLocationEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_LOCATION_EXT;
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
#[doc = "See [XrPlaneDetectorPolygonBufferEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrPlaneDetectorPolygonBufferEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
pub struct PlaneDetectorPolygonBufferEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub vertex_capacity_input: u32,
    pub vertex_count_output: u32,
    pub vertices: *mut Vector2f,
}
impl PlaneDetectorPolygonBufferEXT {
    pub const TYPE: StructureType = StructureType::PLANE_DETECTOR_POLYGON_BUFFER_EXT;
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
#[doc = "See [XrSystemVirtualKeyboardPropertiesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemVirtualKeyboardPropertiesMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct SystemVirtualKeyboardPropertiesMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_virtual_keyboard: Bool32,
}
impl SystemVirtualKeyboardPropertiesMETA {
    pub const TYPE: StructureType = StructureType::SYSTEM_VIRTUAL_KEYBOARD_PROPERTIES_META;
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
#[doc = "See [XrVirtualKeyboardCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardCreateInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl VirtualKeyboardCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVirtualKeyboardSpaceCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardSpaceCreateInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardSpaceCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub location_type: VirtualKeyboardLocationTypeMETA,
    pub space: Space,
    pub pose_in_space: Posef,
}
impl VirtualKeyboardSpaceCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_SPACE_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVirtualKeyboardLocationInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardLocationInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardLocationInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub location_type: VirtualKeyboardLocationTypeMETA,
    pub space: Space,
    pub pose_in_space: Posef,
    pub scale: f32,
}
impl VirtualKeyboardLocationInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_LOCATION_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVirtualKeyboardModelVisibilitySetInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardModelVisibilitySetInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardModelVisibilitySetInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub visible: Bool32,
}
impl VirtualKeyboardModelVisibilitySetInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_MODEL_VISIBILITY_SET_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVirtualKeyboardAnimationStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardAnimationStateMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardAnimationStateMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub animation_index: i32,
    pub fraction: f32,
}
impl VirtualKeyboardAnimationStateMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_ANIMATION_STATE_META;
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
#[doc = "See [XrVirtualKeyboardModelAnimationStatesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardModelAnimationStatesMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardModelAnimationStatesMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub state_capacity_input: u32,
    pub state_count_output: u32,
    pub states: *mut VirtualKeyboardAnimationStateMETA,
}
impl VirtualKeyboardModelAnimationStatesMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_MODEL_ANIMATION_STATES_META;
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
#[doc = "See [XrVirtualKeyboardTextureDataMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardTextureDataMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardTextureDataMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture_width: u32,
    pub texture_height: u32,
    pub buffer_capacity_input: u32,
    pub buffer_count_output: u32,
    pub buffer: *mut u8,
}
impl VirtualKeyboardTextureDataMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_TEXTURE_DATA_META;
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
#[doc = "See [XrVirtualKeyboardInputInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardInputInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardInputInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub input_source: VirtualKeyboardInputSourceMETA,
    pub input_space: Space,
    pub input_pose_in_space: Posef,
    pub input_state: VirtualKeyboardInputStateFlagsMETA,
}
impl VirtualKeyboardInputInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_INPUT_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrVirtualKeyboardTextContextChangeInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrVirtualKeyboardTextContextChangeInfoMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct VirtualKeyboardTextContextChangeInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub text_context: *const c_char,
}
impl VirtualKeyboardTextContextChangeInfoMETA {
    pub const TYPE: StructureType = StructureType::VIRTUAL_KEYBOARD_TEXT_CONTEXT_CHANGE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVirtualKeyboardCommitTextMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVirtualKeyboardCommitTextMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct EventDataVirtualKeyboardCommitTextMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub keyboard: VirtualKeyboardMETA,
    pub text: [c_char; MAX_VIRTUAL_KEYBOARD_COMMIT_TEXT_SIZE_META],
}
impl EventDataVirtualKeyboardCommitTextMETA {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_COMMIT_TEXT_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVirtualKeyboardBackspaceMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVirtualKeyboardBackspaceMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct EventDataVirtualKeyboardBackspaceMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub keyboard: VirtualKeyboardMETA,
}
impl EventDataVirtualKeyboardBackspaceMETA {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_BACKSPACE_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVirtualKeyboardEnterMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVirtualKeyboardEnterMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct EventDataVirtualKeyboardEnterMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub keyboard: VirtualKeyboardMETA,
}
impl EventDataVirtualKeyboardEnterMETA {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_ENTER_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVirtualKeyboardShownMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVirtualKeyboardShownMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct EventDataVirtualKeyboardShownMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub keyboard: VirtualKeyboardMETA,
}
impl EventDataVirtualKeyboardShownMETA {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_SHOWN_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataVirtualKeyboardHiddenMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataVirtualKeyboardHiddenMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
pub struct EventDataVirtualKeyboardHiddenMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub keyboard: VirtualKeyboardMETA,
}
impl EventDataVirtualKeyboardHiddenMETA {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_VIRTUAL_KEYBOARD_HIDDEN_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrUserCalibrationEnableEventsInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrUserCalibrationEnableEventsInfoML) - defined by [XR_ML_user_calibration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_user_calibration)"]
pub struct UserCalibrationEnableEventsInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub enabled: Bool32,
}
impl UserCalibrationEnableEventsInfoML {
    pub const TYPE: StructureType = StructureType::USER_CALIBRATION_ENABLE_EVENTS_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataHeadsetFitChangedML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataHeadsetFitChangedML) - defined by [XR_ML_user_calibration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_user_calibration)"]
pub struct EventDataHeadsetFitChangedML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub status: HeadsetFitStatusML,
    pub time: Time,
}
impl EventDataHeadsetFitChangedML {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_HEADSET_FIT_CHANGED_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataEyeCalibrationChangedML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataEyeCalibrationChangedML) - defined by [XR_ML_user_calibration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_user_calibration)"]
pub struct EventDataEyeCalibrationChangedML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub status: EyeCalibrationStatusML,
}
impl EventDataEyeCalibrationChangedML {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_EYE_CALIBRATION_CHANGED_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLocalizationMapML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct LocalizationMapML {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub name: [c_char; MAX_LOCALIZATION_MAP_NAME_LENGTH_ML],
    pub map_uuid: UuidEXT,
    pub map_type: LocalizationMapTypeML,
}
impl LocalizationMapML {
    pub const TYPE: StructureType = StructureType::LOCALIZATION_MAP_ML;
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
#[doc = "See [XrLocalizationEnableEventsInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationEnableEventsInfoML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct LocalizationEnableEventsInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub enabled: Bool32,
}
impl LocalizationEnableEventsInfoML {
    pub const TYPE: StructureType = StructureType::LOCALIZATION_ENABLE_EVENTS_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEventDataLocalizationChangedML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataLocalizationChangedML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct EventDataLocalizationChangedML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub state: LocalizationMapStateML,
    pub map: LocalizationMapML,
    pub confidence: LocalizationMapConfidenceML,
    pub error_flags: LocalizationMapErrorFlagsML,
}
impl EventDataLocalizationChangedML {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_LOCALIZATION_CHANGED_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLocalizationMapQueryInfoBaseHeaderML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapQueryInfoBaseHeaderML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct LocalizationMapQueryInfoBaseHeaderML {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMapLocalizationRequestInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMapLocalizationRequestInfoML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct MapLocalizationRequestInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub map_uuid: UuidEXT,
}
impl MapLocalizationRequestInfoML {
    pub const TYPE: StructureType = StructureType::MAP_LOCALIZATION_REQUEST_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrLocalizationMapImportInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrLocalizationMapImportInfoML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
pub struct LocalizationMapImportInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub size: u32,
    pub data: *mut c_char,
}
impl LocalizationMapImportInfoML {
    pub const TYPE: StructureType = StructureType::LOCALIZATION_MAP_IMPORT_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemMarkerUnderstandingPropertiesML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemMarkerUnderstandingPropertiesML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct SystemMarkerUnderstandingPropertiesML {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_marker_understanding: Bool32,
}
impl SystemMarkerUnderstandingPropertiesML {
    pub const TYPE: StructureType = StructureType::SYSTEM_MARKER_UNDERSTANDING_PROPERTIES_ML;
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
#[doc = "See [XrMarkerDetectorCreateInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorCreateInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorCreateInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub profile: MarkerDetectorProfileML,
    pub marker_type: MarkerTypeML,
}
impl MarkerDetectorCreateInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_CREATE_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorArucoInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorArucoInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorArucoInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub aruco_dict: MarkerArucoDictML,
}
impl MarkerDetectorArucoInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_ARUCO_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorSizeInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorSizeInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorSizeInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub marker_length: f32,
}
impl MarkerDetectorSizeInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_SIZE_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorAprilTagInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorAprilTagInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorAprilTagInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub april_tag_dict: MarkerAprilTagDictML,
}
impl MarkerDetectorAprilTagInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_APRIL_TAG_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorCustomProfileInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorCustomProfileInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorCustomProfileInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub fps_hint: MarkerDetectorFpsML,
    pub resolution_hint: MarkerDetectorResolutionML,
    pub camera_hint: MarkerDetectorCameraML,
    pub corner_refine_method: MarkerDetectorCornerRefineMethodML,
    pub use_edge_refinement: Bool32,
    pub full_analysis_interval_hint: MarkerDetectorFullAnalysisIntervalML,
}
impl MarkerDetectorCustomProfileInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_CUSTOM_PROFILE_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorSnapshotInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorSnapshotInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorSnapshotInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl MarkerDetectorSnapshotInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_SNAPSHOT_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrMarkerDetectorStateML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerDetectorStateML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerDetectorStateML {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub state: MarkerDetectorStatusML,
}
impl MarkerDetectorStateML {
    pub const TYPE: StructureType = StructureType::MARKER_DETECTOR_STATE_ML;
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
#[doc = "See [XrMarkerSpaceCreateInfoML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrMarkerSpaceCreateInfoML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
pub struct MarkerSpaceCreateInfoML {
    pub ty: StructureType,
    pub next: *const c_void,
    pub marker_detector: MarkerDetectorML,
    pub marker: MarkerML,
    pub pose_in_marker_space: Posef,
}
impl MarkerSpaceCreateInfoML {
    pub const TYPE: StructureType = StructureType::MARKER_SPACE_CREATE_INFO_ML;
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrColor3f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrColor3f)"]
pub struct Color3f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
pub type Color3fKHR = Color3f;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrExtent3Df](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrExtent3Df)"]
pub struct Extent3Df {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
pub type Extent3DfEXT = Extent3Df;
pub type Extent3DfFB = Extent3Df;
pub type Extent3DfKHR = Extent3Df;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSpheref](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpheref)"]
pub struct Spheref {
    pub center: Posef,
    pub radius: f32,
}
pub type SpherefKHR = Spheref;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrBoxf](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrBoxf)"]
pub struct Boxf {
    pub center: Posef,
    pub extents: Extent3Df,
}
pub type BoxfKHR = Boxf;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrFrustumf](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFrustumf)"]
pub struct Frustumf {
    pub pose: Posef,
    pub fov: Fovf,
    pub near_z: f32,
    pub far_z: f32,
}
pub type FrustumfKHR = Frustumf;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrUuid](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrUuid)"]
pub struct Uuid {
    pub data: [u8; UUID_SIZE],
}
pub type UuidEXT = Uuid;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrRecommendedLayerResolutionMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRecommendedLayerResolutionMETA) - defined by [XR_META_recommended_layer_resolution](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_recommended_layer_resolution)"]
pub struct RecommendedLayerResolutionMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub recommended_image_dimensions: Extent2Di,
    pub is_valid: Bool32,
}
impl RecommendedLayerResolutionMETA {
    pub const TYPE: StructureType = StructureType::RECOMMENDED_LAYER_RESOLUTION_META;
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
#[doc = "See [XrRecommendedLayerResolutionGetInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrRecommendedLayerResolutionGetInfoMETA) - defined by [XR_META_recommended_layer_resolution](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_recommended_layer_resolution)"]
pub struct RecommendedLayerResolutionGetInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer: *const CompositionLayerBaseHeader,
    pub predicted_display_time: Time,
}
impl RecommendedLayerResolutionGetInfoMETA {
    pub const TYPE: StructureType = StructureType::RECOMMENDED_LAYER_RESOLUTION_GET_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemUserPresencePropertiesEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemUserPresencePropertiesEXT) - defined by [XR_EXT_user_presence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_user_presence)"]
pub struct SystemUserPresencePropertiesEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_user_presence: Bool32,
}
impl SystemUserPresencePropertiesEXT {
    pub const TYPE: StructureType = StructureType::SYSTEM_USER_PRESENCE_PROPERTIES_EXT;
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
#[doc = "See [XrEventDataUserPresenceChangedEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEventDataUserPresenceChangedEXT) - defined by [XR_EXT_user_presence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_user_presence)"]
pub struct EventDataUserPresenceChangedEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub is_user_present: Bool32,
}
impl EventDataUserPresenceChangedEXT {
    pub const TYPE: StructureType = StructureType::EVENT_DATA_USER_PRESENCE_CHANGED_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFutureCompletionBaseHeaderEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFutureCompletionBaseHeaderEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
pub struct FutureCompletionBaseHeaderEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub future_result: Result,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFutureCompletionEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFutureCompletionEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
pub struct FutureCompletionEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub future_result: Result,
}
impl FutureCompletionEXT {
    pub const TYPE: StructureType = StructureType::FUTURE_COMPLETION_EXT;
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
#[doc = "See [XrFutureCancelInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFutureCancelInfoEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
pub struct FutureCancelInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub future: FutureEXT,
}
impl FutureCancelInfoEXT {
    pub const TYPE: StructureType = StructureType::FUTURE_CANCEL_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFuturePollInfoEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFuturePollInfoEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
pub struct FuturePollInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub future: FutureEXT,
}
impl FuturePollInfoEXT {
    pub const TYPE: StructureType = StructureType::FUTURE_POLL_INFO_EXT;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrFuturePollResultEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrFuturePollResultEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
pub struct FuturePollResultEXT {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub state: FutureStateEXT,
}
impl FuturePollResultEXT {
    pub const TYPE: StructureType = StructureType::FUTURE_POLL_RESULT_EXT;
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
#[doc = "See [XrEnvironmentDepthProviderCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthProviderCreateInfoMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthProviderCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: EnvironmentDepthProviderCreateFlagsMETA,
}
impl EnvironmentDepthProviderCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_PROVIDER_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEnvironmentDepthSwapchainCreateInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthSwapchainCreateInfoMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthSwapchainCreateInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: EnvironmentDepthSwapchainCreateFlagsMETA,
}
impl EnvironmentDepthSwapchainCreateInfoMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_SWAPCHAIN_CREATE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEnvironmentDepthSwapchainStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthSwapchainStateMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthSwapchainStateMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub width: u32,
    pub height: u32,
}
impl EnvironmentDepthSwapchainStateMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_SWAPCHAIN_STATE_META;
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
#[doc = "See [XrEnvironmentDepthImageAcquireInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthImageAcquireInfoMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthImageAcquireInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub space: Space,
    pub display_time: Time,
}
impl EnvironmentDepthImageAcquireInfoMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_IMAGE_ACQUIRE_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEnvironmentDepthImageViewMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthImageViewMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthImageViewMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub fov: Fovf,
    pub pose: Posef,
}
impl EnvironmentDepthImageViewMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_IMAGE_VIEW_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEnvironmentDepthImageMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthImageMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthImageMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub swapchain_index: u32,
    pub near_z: f32,
    pub far_z: f32,
    pub views: [EnvironmentDepthImageViewMETA; 2usize],
}
impl EnvironmentDepthImageMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_IMAGE_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrEnvironmentDepthHandRemovalSetInfoMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthHandRemovalSetInfoMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct EnvironmentDepthHandRemovalSetInfoMETA {
    pub ty: StructureType,
    pub next: *const c_void,
    pub enabled: Bool32,
}
impl EnvironmentDepthHandRemovalSetInfoMETA {
    pub const TYPE: StructureType = StructureType::ENVIRONMENT_DEPTH_HAND_REMOVAL_SET_INFO_META;
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSystemEnvironmentDepthPropertiesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSystemEnvironmentDepthPropertiesMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
pub struct SystemEnvironmentDepthPropertiesMETA {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub supports_environment_depth: Bool32,
    pub supports_hand_removal: Bool32,
}
impl SystemEnvironmentDepthPropertiesMETA {
    pub const TYPE: StructureType = StructureType::SYSTEM_ENVIRONMENT_DEPTH_PROPERTIES_META;
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
#[doc = "See [XrSpacesLocateInfo](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpacesLocateInfo)"]
pub struct SpacesLocateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub base_space: Space,
    pub time: Time,
    pub space_count: u32,
    pub spaces: *const Space,
}
impl SpacesLocateInfo {
    pub const TYPE: StructureType = StructureType::SPACES_LOCATE_INFO;
}
pub type SpacesLocateInfoKHR = SpacesLocateInfo;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceLocations](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceLocations)"]
pub struct SpaceLocations {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub location_count: u32,
    pub locations: *mut SpaceLocationData,
}
impl SpaceLocations {
    pub const TYPE: StructureType = StructureType::SPACE_LOCATIONS;
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
pub type SpaceLocationsKHR = SpaceLocations;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSpaceLocationData](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceLocationData)"]
pub struct SpaceLocationData {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
}
pub type SpaceLocationDataKHR = SpaceLocationData;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[doc = "See [XrSpaceVelocities](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceVelocities)"]
pub struct SpaceVelocities {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub velocity_count: u32,
    pub velocities: *mut SpaceVelocityData,
}
impl SpaceVelocities {
    pub const TYPE: StructureType = StructureType::SPACE_VELOCITIES;
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
pub type SpaceVelocitiesKHR = SpaceVelocities;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[doc = "See [XrSpaceVelocityData](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrSpaceVelocityData)"]
pub struct SpaceVelocityData {
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}
pub type SpaceVelocityDataKHR = SpaceVelocityData;
#[doc = r" Function pointer prototypes"]
pub mod pfn {
    use super::*;
    pub use crate::platform::EglGetProcAddressMNDX;
    pub type VoidFunction = unsafe extern "system" fn();
    pub type DebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
        DebugUtilsMessageSeverityFlagsEXT,
        DebugUtilsMessageTypeFlagsEXT,
        *const DebugUtilsMessengerCallbackDataEXT,
        *mut c_void,
    ) -> Bool32;
    #[doc = "See [xrNegotiateLoaderRuntimeInterface](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrNegotiateLoaderRuntimeInterface)"]
    pub type NegotiateLoaderRuntimeInterface = unsafe extern "system" fn(
        loader_info: *const NegotiateLoaderInfo,
        runtime_request: *mut NegotiateRuntimeRequest,
    ) -> Result;
    #[doc = "See [xrNegotiateLoaderApiLayerInterface](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrNegotiateLoaderApiLayerInterface)"]
    pub type NegotiateLoaderApiLayerInterface = unsafe extern "system" fn(
        loader_info: *const NegotiateLoaderInfo,
        layer_name: *const c_char,
        api_layer_request: *mut NegotiateApiLayerRequest,
    ) -> Result;
    #[doc = "See [xrCreateApiLayerInstance](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateApiLayerInstance)"]
    pub type CreateApiLayerInstance = unsafe extern "system" fn(
        info: *const InstanceCreateInfo,
        layer_info: *const ApiLayerCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    #[doc = "See [xrGetInstanceProcAddr](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetInstanceProcAddr)"]
    pub type GetInstanceProcAddr = unsafe extern "system" fn(
        instance: Instance,
        name: *const c_char,
        function: *mut Option<pfn::VoidFunction>,
    ) -> Result;
    #[doc = "See [xrEnumerateApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateApiLayerProperties)"]
    pub type EnumerateApiLayerProperties = unsafe extern "system" fn(
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ApiLayerProperties,
    ) -> Result;
    #[doc = "See [xrEnumerateInstanceExtensionProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateInstanceExtensionProperties)"]
    pub type EnumerateInstanceExtensionProperties = unsafe extern "system" fn(
        layer_name: *const c_char,
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result;
    #[doc = "See [xrCreateInstance](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateInstance)"]
    pub type CreateInstance = unsafe extern "system" fn(
        create_info: *const InstanceCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    #[doc = "See [xrDestroyInstance](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyInstance)"]
    pub type DestroyInstance = unsafe extern "system" fn(instance: Instance) -> Result;
    #[doc = "See [xrResultToString](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrResultToString)"]
    pub type ResultToString =
        unsafe extern "system" fn(instance: Instance, value: Result, buffer: *mut c_char) -> Result;
    #[doc = "See [xrStructureTypeToString](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrStructureTypeToString)"]
    pub type StructureTypeToString = unsafe extern "system" fn(
        instance: Instance,
        value: StructureType,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetInstanceProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetInstanceProperties)"]
    pub type GetInstanceProperties = unsafe extern "system" fn(
        instance: Instance,
        instance_properties: *mut InstanceProperties,
    ) -> Result;
    #[doc = "See [xrGetSystem](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSystem)"]
    pub type GetSystem = unsafe extern "system" fn(
        instance: Instance,
        get_info: *const SystemGetInfo,
        system_id: *mut SystemId,
    ) -> Result;
    #[doc = "See [xrGetSystemProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSystemProperties)"]
    pub type GetSystemProperties = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        properties: *mut SystemProperties,
    ) -> Result;
    #[doc = "See [xrCreateSession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSession)"]
    pub type CreateSession = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const SessionCreateInfo,
        session: *mut Session,
    ) -> Result;
    #[doc = "See [xrDestroySession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySession)"]
    pub type DestroySession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrDestroySpace](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySpace)"]
    pub type DestroySpace = unsafe extern "system" fn(space: Space) -> Result;
    #[doc = "See [xrEnumerateSwapchainFormats](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateSwapchainFormats)"]
    pub type EnumerateSwapchainFormats = unsafe extern "system" fn(
        session: Session,
        format_capacity_input: u32,
        format_count_output: *mut u32,
        formats: *mut i64,
    ) -> Result;
    #[doc = "See [xrCreateSwapchain](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSwapchain)"]
    pub type CreateSwapchain = unsafe extern "system" fn(
        session: Session,
        create_info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
    ) -> Result;
    #[doc = "See [xrDestroySwapchain](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySwapchain)"]
    pub type DestroySwapchain = unsafe extern "system" fn(swapchain: Swapchain) -> Result;
    #[doc = "See [xrEnumerateSwapchainImages](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateSwapchainImages)"]
    pub type EnumerateSwapchainImages = unsafe extern "system" fn(
        swapchain: Swapchain,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    ) -> Result;
    #[doc = "See [xrAcquireSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrAcquireSwapchainImage)"]
    pub type AcquireSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        acquire_info: *const SwapchainImageAcquireInfo,
        index: *mut u32,
    ) -> Result;
    #[doc = "See [xrWaitSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrWaitSwapchainImage)"]
    pub type WaitSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        wait_info: *const SwapchainImageWaitInfo,
    ) -> Result;
    #[doc = "See [xrReleaseSwapchainImage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrReleaseSwapchainImage)"]
    pub type ReleaseSwapchainImage = unsafe extern "system" fn(
        swapchain: Swapchain,
        release_info: *const SwapchainImageReleaseInfo,
    ) -> Result;
    #[doc = "See [xrBeginSession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrBeginSession)"]
    pub type BeginSession =
        unsafe extern "system" fn(session: Session, begin_info: *const SessionBeginInfo) -> Result;
    #[doc = "See [xrEndSession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEndSession)"]
    pub type EndSession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrRequestExitSession](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrRequestExitSession)"]
    pub type RequestExitSession = unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrEnumerateReferenceSpaces](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateReferenceSpaces)"]
    pub type EnumerateReferenceSpaces = unsafe extern "system" fn(
        session: Session,
        space_capacity_input: u32,
        space_count_output: *mut u32,
        spaces: *mut ReferenceSpaceType,
    ) -> Result;
    #[doc = "See [xrCreateReferenceSpace](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateReferenceSpace)"]
    pub type CreateReferenceSpace = unsafe extern "system" fn(
        session: Session,
        create_info: *const ReferenceSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrCreateActionSpace](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateActionSpace)"]
    pub type CreateActionSpace = unsafe extern "system" fn(
        session: Session,
        create_info: *const ActionSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrLocateSpace](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateSpace)"]
    pub type LocateSpace = unsafe extern "system" fn(
        space: Space,
        base_space: Space,
        time: Time,
        location: *mut SpaceLocation,
    ) -> Result;
    #[doc = "See [xrEnumerateViewConfigurations](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateViewConfigurations)"]
    pub type EnumerateViewConfigurations = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type_capacity_input: u32,
        view_configuration_type_count_output: *mut u32,
        view_configuration_types: *mut ViewConfigurationType,
    ) -> Result;
    #[doc = "See [xrEnumerateEnvironmentBlendModes](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateEnvironmentBlendModes)"]
    pub type EnumerateEnvironmentBlendModes = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        environment_blend_mode_capacity_input: u32,
        environment_blend_mode_count_output: *mut u32,
        environment_blend_modes: *mut EnvironmentBlendMode,
    ) -> Result;
    #[doc = "See [xrGetViewConfigurationProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetViewConfigurationProperties)"]
    pub type GetViewConfigurationProperties = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        configuration_properties: *mut ViewConfigurationProperties,
    ) -> Result;
    #[doc = "See [xrEnumerateViewConfigurationViews](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateViewConfigurationViews)"]
    pub type EnumerateViewConfigurationViews = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut ViewConfigurationView,
    ) -> Result;
    #[doc = "See [xrBeginFrame](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrBeginFrame)"]
    pub type BeginFrame = unsafe extern "system" fn(
        session: Session,
        frame_begin_info: *const FrameBeginInfo,
    ) -> Result;
    #[doc = "See [xrLocateViews](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateViews)"]
    pub type LocateViews = unsafe extern "system" fn(
        session: Session,
        view_locate_info: *const ViewLocateInfo,
        view_state: *mut ViewState,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut View,
    ) -> Result;
    #[doc = "See [xrEndFrame](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEndFrame)"]
    pub type EndFrame =
        unsafe extern "system" fn(session: Session, frame_end_info: *const FrameEndInfo) -> Result;
    #[doc = "See [xrWaitFrame](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrWaitFrame)"]
    pub type WaitFrame = unsafe extern "system" fn(
        session: Session,
        frame_wait_info: *const FrameWaitInfo,
        frame_state: *mut FrameState,
    ) -> Result;
    #[doc = "See [xrApplyHapticFeedback](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrApplyHapticFeedback)"]
    pub type ApplyHapticFeedback = unsafe extern "system" fn(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
        haptic_feedback: *const HapticBaseHeader,
    ) -> Result;
    #[doc = "See [xrStopHapticFeedback](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrStopHapticFeedback)"]
    pub type StopHapticFeedback = unsafe extern "system" fn(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
    ) -> Result;
    #[doc = "See [xrPollEvent](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPollEvent)"]
    pub type PollEvent =
        unsafe extern "system" fn(instance: Instance, event_data: *mut EventDataBuffer) -> Result;
    #[doc = "See [xrStringToPath](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrStringToPath)"]
    pub type StringToPath = unsafe extern "system" fn(
        instance: Instance,
        path_string: *const c_char,
        path: *mut Path,
    ) -> Result;
    #[doc = "See [xrPathToString](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPathToString)"]
    pub type PathToString = unsafe extern "system" fn(
        instance: Instance,
        path: Path,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetReferenceSpaceBoundsRect](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetReferenceSpaceBoundsRect)"]
    pub type GetReferenceSpaceBoundsRect = unsafe extern "system" fn(
        session: Session,
        reference_space_type: ReferenceSpaceType,
        bounds: *mut Extent2Df,
    ) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "See [xrSetAndroidApplicationThreadKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetAndroidApplicationThreadKHR) - defined by [XR_KHR_android_thread_settings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_android_thread_settings)"]
    pub type SetAndroidApplicationThreadKHR = unsafe extern "system" fn(
        session: Session,
        thread_type: AndroidThreadTypeKHR,
        thread_id: u32,
    ) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "See [xrCreateSwapchainAndroidSurfaceKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSwapchainAndroidSurfaceKHR) - defined by [XR_KHR_android_surface_swapchain](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_android_surface_swapchain)"]
    pub type CreateSwapchainAndroidSurfaceKHR = unsafe extern "system" fn(
        session: Session,
        info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
        surface: *mut jobject,
    ) -> Result;
    #[doc = "See [xrGetActionStateBoolean](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetActionStateBoolean)"]
    pub type GetActionStateBoolean = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateBoolean,
    ) -> Result;
    #[doc = "See [xrGetActionStateFloat](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetActionStateFloat)"]
    pub type GetActionStateFloat = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateFloat,
    ) -> Result;
    #[doc = "See [xrGetActionStateVector2f](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetActionStateVector2f)"]
    pub type GetActionStateVector2f = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStateVector2f,
    ) -> Result;
    #[doc = "See [xrGetActionStatePose](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetActionStatePose)"]
    pub type GetActionStatePose = unsafe extern "system" fn(
        session: Session,
        get_info: *const ActionStateGetInfo,
        state: *mut ActionStatePose,
    ) -> Result;
    #[doc = "See [xrCreateActionSet](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateActionSet)"]
    pub type CreateActionSet = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const ActionSetCreateInfo,
        action_set: *mut ActionSet,
    ) -> Result;
    #[doc = "See [xrDestroyActionSet](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyActionSet)"]
    pub type DestroyActionSet = unsafe extern "system" fn(action_set: ActionSet) -> Result;
    #[doc = "See [xrCreateAction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateAction)"]
    pub type CreateAction = unsafe extern "system" fn(
        action_set: ActionSet,
        create_info: *const ActionCreateInfo,
        action: *mut Action,
    ) -> Result;
    #[doc = "See [xrDestroyAction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyAction)"]
    pub type DestroyAction = unsafe extern "system" fn(action: Action) -> Result;
    #[doc = "See [xrSuggestInteractionProfileBindings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSuggestInteractionProfileBindings)"]
    pub type SuggestInteractionProfileBindings = unsafe extern "system" fn(
        instance: Instance,
        suggested_bindings: *const InteractionProfileSuggestedBinding,
    ) -> Result;
    #[doc = "See [xrAttachSessionActionSets](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrAttachSessionActionSets)"]
    pub type AttachSessionActionSets = unsafe extern "system" fn(
        session: Session,
        attach_info: *const SessionActionSetsAttachInfo,
    ) -> Result;
    #[doc = "See [xrGetCurrentInteractionProfile](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetCurrentInteractionProfile)"]
    pub type GetCurrentInteractionProfile = unsafe extern "system" fn(
        session: Session,
        top_level_user_path: Path,
        interaction_profile: *mut InteractionProfileState,
    ) -> Result;
    #[doc = "See [xrSyncActions](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSyncActions)"]
    pub type SyncActions =
        unsafe extern "system" fn(session: Session, sync_info: *const ActionsSyncInfo) -> Result;
    #[doc = "See [xrEnumerateBoundSourcesForAction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateBoundSourcesForAction)"]
    pub type EnumerateBoundSourcesForAction = unsafe extern "system" fn(
        session: Session,
        enumerate_info: *const BoundSourcesForActionEnumerateInfo,
        source_capacity_input: u32,
        source_count_output: *mut u32,
        sources: *mut Path,
    ) -> Result;
    #[doc = "See [xrGetInputSourceLocalizedName](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetInputSourceLocalizedName)"]
    pub type GetInputSourceLocalizedName = unsafe extern "system" fn(
        session: Session,
        get_info: *const InputSourceLocalizedNameGetInfo,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanInstanceExtensionsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanInstanceExtensionsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanInstanceExtensionsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanDeviceExtensionsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanDeviceExtensionsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanDeviceExtensionsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsDeviceKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanGraphicsDeviceKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsDeviceKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        vk_instance: VkInstance,
        vk_physical_device: *mut VkPhysicalDevice,
    ) -> Result;
    #[doc = "See [xrGetOpenGLGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetOpenGLGraphicsRequirementsKHR) - defined by [XR_KHR_opengl_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_enable)"]
    pub type GetOpenGLGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLKHR,
    ) -> Result;
    #[doc = "See [xrGetOpenGLESGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetOpenGLESGraphicsRequirementsKHR) - defined by [XR_KHR_opengl_es_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_opengl_es_enable)"]
    pub type GetOpenGLESGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLESKHR,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanGraphicsRequirementsKHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsVulkanKHR,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrGetD3D11GraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetD3D11GraphicsRequirementsKHR) - defined by [XR_KHR_D3D11_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D11_enable)"]
    pub type GetD3D11GraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D11KHR,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrGetD3D12GraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetD3D12GraphicsRequirementsKHR) - defined by [XR_KHR_D3D12_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_D3D12_enable)"]
    pub type GetD3D12GraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D12KHR,
    ) -> Result;
    #[cfg(target_vendor = "apple")]
    #[doc = "See [xrGetMetalGraphicsRequirementsKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMetalGraphicsRequirementsKHR) - defined by [XR_KHR_metal_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_metal_enable)"]
    pub type GetMetalGraphicsRequirementsKHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsMetalKHR,
    ) -> Result;
    #[doc = "See [xrPerfSettingsSetPerformanceLevelEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPerfSettingsSetPerformanceLevelEXT) - defined by [XR_EXT_performance_settings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_performance_settings)"]
    pub type PerfSettingsSetPerformanceLevelEXT = unsafe extern "system" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        level: PerfSettingsLevelEXT,
    ) -> Result;
    #[doc = "See [xrThermalGetTemperatureTrendEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrThermalGetTemperatureTrendEXT) - defined by [XR_EXT_thermal_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_thermal_query)"]
    pub type ThermalGetTemperatureTrendEXT = unsafe extern "system" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        notification_level: *mut PerfSettingsNotificationLevelEXT,
        temp_headroom: *mut f32,
        temp_slope: *mut f32,
    ) -> Result;
    #[doc = "See [xrSetDebugUtilsObjectNameEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetDebugUtilsObjectNameEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
        instance: Instance,
        name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> Result;
    #[doc = "See [xrCreateDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateDebugUtilsMessengerEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type CreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const DebugUtilsMessengerCreateInfoEXT,
        messenger: *mut DebugUtilsMessengerEXT,
    ) -> Result;
    #[doc = "See [xrDestroyDebugUtilsMessengerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyDebugUtilsMessengerEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type DestroyDebugUtilsMessengerEXT =
        unsafe extern "system" fn(messenger: DebugUtilsMessengerEXT) -> Result;
    #[doc = "See [xrSubmitDebugUtilsMessageEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSubmitDebugUtilsMessageEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) -> Result;
    #[doc = "See [xrSessionBeginDebugUtilsLabelRegionEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSessionBeginDebugUtilsLabelRegionEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionBeginDebugUtilsLabelRegionEXT = unsafe extern "system" fn(
        session: Session,
        label_info: *const DebugUtilsLabelEXT,
    ) -> Result;
    #[doc = "See [xrSessionEndDebugUtilsLabelRegionEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSessionEndDebugUtilsLabelRegionEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionEndDebugUtilsLabelRegionEXT =
        unsafe extern "system" fn(session: Session) -> Result;
    #[doc = "See [xrSessionInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSessionInsertDebugUtilsLabelEXT) - defined by [XR_EXT_debug_utils](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_debug_utils)"]
    pub type SessionInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
        session: Session,
        label_info: *const DebugUtilsLabelEXT,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrConvertTimeToWin32PerformanceCounterKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrConvertTimeToWin32PerformanceCounterKHR) - defined by [XR_KHR_win32_convert_performance_counter_time](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_win32_convert_performance_counter_time)"]
    pub type ConvertTimeToWin32PerformanceCounterKHR = unsafe extern "system" fn(
        instance: Instance,
        time: Time,
        performance_counter: *mut LARGE_INTEGER,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrConvertWin32PerformanceCounterToTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrConvertWin32PerformanceCounterToTimeKHR) - defined by [XR_KHR_win32_convert_performance_counter_time](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_win32_convert_performance_counter_time)"]
    pub type ConvertWin32PerformanceCounterToTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        performance_counter: *const LARGE_INTEGER,
        time: *mut Time,
    ) -> Result;
    #[doc = "See [xrCreateVulkanInstanceKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateVulkanInstanceKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type CreateVulkanInstanceKHR = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const VulkanInstanceCreateInfoKHR,
        vulkan_instance: *mut VkInstance,
        vulkan_result: *mut VkResult,
    ) -> Result;
    #[doc = "See [xrCreateVulkanDeviceKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateVulkanDeviceKHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type CreateVulkanDeviceKHR = unsafe extern "system" fn(
        instance: Instance,
        create_info: *const VulkanDeviceCreateInfoKHR,
        vulkan_device: *mut VkDevice,
        vulkan_result: *mut VkResult,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsDevice2KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanGraphicsDevice2KHR) - defined by [XR_KHR_vulkan_enable2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable2)"]
    pub type GetVulkanGraphicsDevice2KHR = unsafe extern "system" fn(
        instance: Instance,
        get_info: *const VulkanGraphicsDeviceGetInfoKHR,
        vulkan_physical_device: *mut VkPhysicalDevice,
    ) -> Result;
    #[doc = "See [xrConvertTimeToTimespecTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrConvertTimeToTimespecTimeKHR) - defined by [XR_KHR_convert_timespec_time](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_convert_timespec_time)"]
    pub type ConvertTimeToTimespecTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        time: Time,
        timespec_time: *mut timespec,
    ) -> Result;
    #[doc = "See [xrConvertTimespecTimeToTimeKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrConvertTimespecTimeToTimeKHR) - defined by [XR_KHR_convert_timespec_time](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_convert_timespec_time)"]
    pub type ConvertTimespecTimeToTimeKHR = unsafe extern "system" fn(
        instance: Instance,
        timespec_time: *const timespec,
        time: *mut Time,
    ) -> Result;
    #[doc = "See [xrGetVisibilityMaskKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVisibilityMaskKHR) - defined by [XR_KHR_visibility_mask](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_visibility_mask)"]
    pub type GetVisibilityMaskKHR = unsafe extern "system" fn(
        session: Session,
        view_configuration_type: ViewConfigurationType,
        view_index: u32,
        visibility_mask_type: VisibilityMaskTypeKHR,
        visibility_mask: *mut VisibilityMaskKHR,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type CreateSpatialAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialAnchorCreateInfoMSFT,
        anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorSpaceMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type CreateSpatialAnchorSpaceMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialAnchorSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrDestroySpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor)"]
    pub type DestroySpatialAnchorMSFT =
        unsafe extern "system" fn(anchor: SpatialAnchorMSFT) -> Result;
    #[doc = "See [xrSetInputDeviceActiveEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetInputDeviceActiveEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceActiveEXT = unsafe extern "system" fn(
        session: Session,
        interaction_profile: Path,
        top_level_path: Path,
        is_active: Bool32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateBoolEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetInputDeviceStateBoolEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateBoolEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: Bool32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateFloatEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetInputDeviceStateFloatEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateFloatEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: f32,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceStateVector2fEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetInputDeviceStateVector2fEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceStateVector2fEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        state: Vector2f,
    ) -> Result;
    #[doc = "See [xrSetInputDeviceLocationEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetInputDeviceLocationEXT) - defined by [XR_EXT_conformance_automation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_conformance_automation)"]
    pub type SetInputDeviceLocationEXT = unsafe extern "system" fn(
        session: Session,
        top_level_path: Path,
        input_source_path: Path,
        space: Space,
        pose: Posef,
    ) -> Result;
    #[doc = "See [xrInitializeLoaderKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrInitializeLoaderKHR) - defined by [XR_KHR_loader_init](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_loader_init)"]
    pub type InitializeLoaderKHR =
        unsafe extern "system" fn(loader_init_info: *const LoaderInitInfoBaseHeaderKHR) -> Result;
    #[doc = "See [xrCreateSpatialGraphNodeSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialGraphNodeSpaceMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
    pub type CreateSpatialGraphNodeSpaceMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialGraphNodeSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrTryCreateSpatialGraphStaticNodeBindingMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTryCreateSpatialGraphStaticNodeBindingMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
    pub type TryCreateSpatialGraphStaticNodeBindingMSFT = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialGraphStaticNodeBindingCreateInfoMSFT,
        node_binding: *mut SpatialGraphNodeBindingMSFT,
    ) -> Result;
    #[doc = "See [xrDestroySpatialGraphNodeBindingMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySpatialGraphNodeBindingMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
    pub type DestroySpatialGraphNodeBindingMSFT =
        unsafe extern "system" fn(node_binding: SpatialGraphNodeBindingMSFT) -> Result;
    #[doc = "See [xrGetSpatialGraphNodeBindingPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpatialGraphNodeBindingPropertiesMSFT) - defined by [XR_MSFT_spatial_graph_bridge](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_graph_bridge)"]
    pub type GetSpatialGraphNodeBindingPropertiesMSFT = unsafe extern "system" fn(
        node_binding: SpatialGraphNodeBindingMSFT,
        get_info: *const SpatialGraphNodeBindingPropertiesGetInfoMSFT,
        properties: *mut SpatialGraphNodeBindingPropertiesMSFT,
    ) -> Result;
    #[doc = "See [xrCreateHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateHandTrackerEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type CreateHandTrackerEXT = unsafe extern "system" fn(
        session: Session,
        create_info: *const HandTrackerCreateInfoEXT,
        hand_tracker: *mut HandTrackerEXT,
    ) -> Result;
    #[doc = "See [xrDestroyHandTrackerEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyHandTrackerEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type DestroyHandTrackerEXT =
        unsafe extern "system" fn(hand_tracker: HandTrackerEXT) -> Result;
    #[doc = "See [xrLocateHandJointsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateHandJointsEXT) - defined by [XR_EXT_hand_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_hand_tracking)"]
    pub type LocateHandJointsEXT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        locate_info: *const HandJointsLocateInfoEXT,
        locations: *mut HandJointLocationsEXT,
    ) -> Result;
    #[doc = "See [xrCreateFaceTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateFaceTrackerFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
    pub type CreateFaceTrackerFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const FaceTrackerCreateInfoFB,
        face_tracker: *mut FaceTrackerFB,
    ) -> Result;
    #[doc = "See [xrDestroyFaceTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyFaceTrackerFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
    pub type DestroyFaceTrackerFB =
        unsafe extern "system" fn(face_tracker: FaceTrackerFB) -> Result;
    #[doc = "See [xrGetFaceExpressionWeightsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetFaceExpressionWeightsFB) - defined by [XR_FB_face_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking)"]
    pub type GetFaceExpressionWeightsFB = unsafe extern "system" fn(
        face_tracker: FaceTrackerFB,
        expression_info: *const FaceExpressionInfoFB,
        expression_weights: *mut FaceExpressionWeightsFB,
    ) -> Result;
    #[doc = "See [xrCreateFaceTracker2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateFaceTracker2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
    pub type CreateFaceTracker2FB = unsafe extern "system" fn(
        session: Session,
        create_info: *const FaceTrackerCreateInfo2FB,
        face_tracker: *mut FaceTracker2FB,
    ) -> Result;
    #[doc = "See [xrDestroyFaceTracker2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyFaceTracker2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
    pub type DestroyFaceTracker2FB =
        unsafe extern "system" fn(face_tracker: FaceTracker2FB) -> Result;
    #[doc = "See [xrGetFaceExpressionWeights2FB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetFaceExpressionWeights2FB) - defined by [XR_FB_face_tracking2](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_face_tracking2)"]
    pub type GetFaceExpressionWeights2FB = unsafe extern "system" fn(
        face_tracker: FaceTracker2FB,
        expression_info: *const FaceExpressionInfo2FB,
        expression_weights: *mut FaceExpressionWeights2FB,
    ) -> Result;
    #[doc = "See [xrCreateBodyTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateBodyTrackerFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
    pub type CreateBodyTrackerFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const BodyTrackerCreateInfoFB,
        body_tracker: *mut BodyTrackerFB,
    ) -> Result;
    #[doc = "See [xrDestroyBodyTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyBodyTrackerFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
    pub type DestroyBodyTrackerFB =
        unsafe extern "system" fn(body_tracker: BodyTrackerFB) -> Result;
    #[doc = "See [xrLocateBodyJointsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateBodyJointsFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
    pub type LocateBodyJointsFB = unsafe extern "system" fn(
        body_tracker: BodyTrackerFB,
        locate_info: *const BodyJointsLocateInfoFB,
        locations: *mut BodyJointLocationsFB,
    ) -> Result;
    #[doc = "See [xrGetBodySkeletonFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetBodySkeletonFB) - defined by [XR_FB_body_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_body_tracking)"]
    pub type GetBodySkeletonFB = unsafe extern "system" fn(
        body_tracker: BodyTrackerFB,
        skeleton: *mut BodySkeletonFB,
    ) -> Result;
    #[doc = "See [xrCreateEyeTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateEyeTrackerFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
    pub type CreateEyeTrackerFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const EyeTrackerCreateInfoFB,
        eye_tracker: *mut EyeTrackerFB,
    ) -> Result;
    #[doc = "See [xrDestroyEyeTrackerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyEyeTrackerFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
    pub type DestroyEyeTrackerFB = unsafe extern "system" fn(eye_tracker: EyeTrackerFB) -> Result;
    #[doc = "See [xrGetEyeGazesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetEyeGazesFB) - defined by [XR_FB_eye_tracking_social](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_eye_tracking_social)"]
    pub type GetEyeGazesFB = unsafe extern "system" fn(
        eye_tracker: EyeTrackerFB,
        gaze_info: *const EyeGazesInfoFB,
        eye_gazes: *mut EyeGazesFB,
    ) -> Result;
    #[doc = "See [xrCreateHandMeshSpaceMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateHandMeshSpaceMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
    pub type CreateHandMeshSpaceMSFT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        create_info: *const HandMeshSpaceCreateInfoMSFT,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrUpdateHandMeshMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrUpdateHandMeshMSFT) - defined by [XR_MSFT_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_hand_tracking_mesh)"]
    pub type UpdateHandMeshMSFT = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        update_info: *const HandMeshUpdateInfoMSFT,
        hand_mesh: *mut HandMeshMSFT,
    ) -> Result;
    #[doc = "See [xrGetControllerModelKeyMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetControllerModelKeyMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelKeyMSFT = unsafe extern "system" fn(
        session: Session,
        top_level_user_path: Path,
        controller_model_key_state: *mut ControllerModelKeyStateMSFT,
    ) -> Result;
    #[doc = "See [xrLoadControllerModelMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLoadControllerModelMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type LoadControllerModelMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    #[doc = "See [xrGetControllerModelPropertiesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetControllerModelPropertiesMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelPropertiesMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        properties: *mut ControllerModelPropertiesMSFT,
    ) -> Result;
    #[doc = "See [xrGetControllerModelStateMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetControllerModelStateMSFT) - defined by [XR_MSFT_controller_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_controller_model)"]
    pub type GetControllerModelStateMSFT = unsafe extern "system" fn(
        session: Session,
        model_key: ControllerModelKeyMSFT,
        state: *mut ControllerModelStateMSFT,
    ) -> Result;
    #[doc = "See [xrEnumerateDisplayRefreshRatesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateDisplayRefreshRatesFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type EnumerateDisplayRefreshRatesFB = unsafe extern "system" fn(
        session: Session,
        display_refresh_rate_capacity_input: u32,
        display_refresh_rate_count_output: *mut u32,
        display_refresh_rates: *mut f32,
    ) -> Result;
    #[doc = "See [xrGetDisplayRefreshRateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetDisplayRefreshRateFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type GetDisplayRefreshRateFB =
        unsafe extern "system" fn(session: Session, display_refresh_rate: *mut f32) -> Result;
    #[doc = "See [xrRequestDisplayRefreshRateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrRequestDisplayRefreshRateFB) - defined by [XR_FB_display_refresh_rate](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_display_refresh_rate)"]
    pub type RequestDisplayRefreshRateFB =
        unsafe extern "system" fn(session: Session, display_refresh_rate: f32) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrCreateSpatialAnchorFromPerceptionAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorFromPerceptionAnchorMSFT) - defined by [XR_MSFT_perception_anchor_interop](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_perception_anchor_interop)"]
    pub type CreateSpatialAnchorFromPerceptionAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        perception_anchor: *mut IUnknown,
        anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[cfg(windows)]
    #[doc = "See [xrTryGetPerceptionAnchorFromSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTryGetPerceptionAnchorFromSpatialAnchorMSFT) - defined by [XR_MSFT_perception_anchor_interop](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_perception_anchor_interop)"]
    pub type TryGetPerceptionAnchorFromSpatialAnchorMSFT = unsafe extern "system" fn(
        session: Session,
        anchor: SpatialAnchorMSFT,
        perception_anchor: *mut *mut IUnknown,
    ) -> Result;
    #[doc = "See [xrUpdateSwapchainFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrUpdateSwapchainFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state)"]
    pub type UpdateSwapchainFB = unsafe extern "system" fn(
        swapchain: Swapchain,
        state: *const SwapchainStateBaseHeaderFB,
    ) -> Result;
    #[doc = "See [xrGetSwapchainStateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSwapchainStateFB) - defined by [XR_FB_swapchain_update_state](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_swapchain_update_state)"]
    pub type GetSwapchainStateFB = unsafe extern "system" fn(
        swapchain: Swapchain,
        state: *mut SwapchainStateBaseHeaderFB,
    ) -> Result;
    #[doc = "See [xrEnumerateColorSpacesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateColorSpacesFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_color_space)"]
    pub type EnumerateColorSpacesFB = unsafe extern "system" fn(
        session: Session,
        color_space_capacity_input: u32,
        color_space_count_output: *mut u32,
        color_spaces: *mut ColorSpaceFB,
    ) -> Result;
    #[doc = "See [xrSetColorSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetColorSpaceFB) - defined by [XR_FB_color_space](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_color_space)"]
    pub type SetColorSpaceFB =
        unsafe extern "system" fn(session: Session, color_space: ColorSpaceFB) -> Result;
    #[doc = "See [xrCreateFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateFoveationProfileFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation)"]
    pub type CreateFoveationProfileFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const FoveationProfileCreateInfoFB,
        profile: *mut FoveationProfileFB,
    ) -> Result;
    #[doc = "See [xrDestroyFoveationProfileFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyFoveationProfileFB) - defined by [XR_FB_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_foveation)"]
    pub type DestroyFoveationProfileFB =
        unsafe extern "system" fn(profile: FoveationProfileFB) -> Result;
    #[doc = "See [xrGetFoveationEyeTrackedStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetFoveationEyeTrackedStateMETA) - defined by [XR_META_foveation_eye_tracked](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_foveation_eye_tracked)"]
    pub type GetFoveationEyeTrackedStateMETA = unsafe extern "system" fn(
        session: Session,
        foveation_state: *mut FoveationEyeTrackedStateMETA,
    ) -> Result;
    #[doc = "See [xrGetHandMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetHandMeshFB) - defined by [XR_FB_hand_tracking_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_hand_tracking_mesh)"]
    pub type GetHandMeshFB = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        mesh: *mut HandTrackingMeshFB,
    ) -> Result;
    #[doc = "See [xrEnumerateRenderModelPathsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateRenderModelPathsFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
    pub type EnumerateRenderModelPathsFB = unsafe extern "system" fn(
        session: Session,
        path_capacity_input: u32,
        path_count_output: *mut u32,
        paths: *mut RenderModelPathInfoFB,
    ) -> Result;
    #[doc = "See [xrGetRenderModelPropertiesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetRenderModelPropertiesFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
    pub type GetRenderModelPropertiesFB = unsafe extern "system" fn(
        session: Session,
        path: Path,
        properties: *mut RenderModelPropertiesFB,
    ) -> Result;
    #[doc = "See [xrLoadRenderModelFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLoadRenderModelFB) - defined by [XR_FB_render_model](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_render_model)"]
    pub type LoadRenderModelFB = unsafe extern "system" fn(
        session: Session,
        info: *const RenderModelLoadInfoFB,
        buffer: *mut RenderModelBufferFB,
    ) -> Result;
    #[doc = "See [xrQuerySystemTrackedKeyboardFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrQuerySystemTrackedKeyboardFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
    pub type QuerySystemTrackedKeyboardFB = unsafe extern "system" fn(
        session: Session,
        query_info: *const KeyboardTrackingQueryFB,
        keyboard: *mut KeyboardTrackingDescriptionFB,
    ) -> Result;
    #[doc = "See [xrCreateKeyboardSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateKeyboardSpaceFB) - defined by [XR_FB_keyboard_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_keyboard_tracking)"]
    pub type CreateKeyboardSpaceFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const KeyboardSpaceCreateInfoFB,
        keyboard_space: *mut Space,
    ) -> Result;
    #[doc = "See [xrSetEnvironmentDepthEstimationVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetEnvironmentDepthEstimationVARJO) - defined by [XR_VARJO_environment_depth_estimation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_environment_depth_estimation)"]
    pub type SetEnvironmentDepthEstimationVARJO =
        unsafe extern "system" fn(session: Session, enabled: Bool32) -> Result;
    #[doc = "See [xrEnumerateReprojectionModesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateReprojectionModesMSFT) - defined by [XR_MSFT_composition_layer_reprojection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_composition_layer_reprojection)"]
    pub type EnumerateReprojectionModesMSFT = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        mode_capacity_input: u32,
        mode_count_output: *mut u32,
        modes: *mut ReprojectionModeMSFT,
    ) -> Result;
    #[doc = "See [xrGetAudioOutputDeviceGuidOculus](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetAudioOutputDeviceGuidOculus) - defined by [XR_OCULUS_audio_device_guid](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_audio_device_guid)"]
    pub type GetAudioOutputDeviceGuidOculus =
        unsafe extern "system" fn(instance: Instance, buffer: *mut wchar_t) -> Result;
    #[doc = "See [xrGetAudioInputDeviceGuidOculus](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetAudioInputDeviceGuidOculus) - defined by [XR_OCULUS_audio_device_guid](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_audio_device_guid)"]
    pub type GetAudioInputDeviceGuidOculus =
        unsafe extern "system" fn(instance: Instance, buffer: *mut wchar_t) -> Result;
    #[doc = "See [xrCreateSpatialAnchorFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
    pub type CreateSpatialAnchorFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpatialAnchorCreateInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceUuidFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceUuidFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
    pub type GetSpaceUuidFB = unsafe extern "system" fn(space: Space, uuid: *mut UuidEXT) -> Result;
    #[doc = "See [xrEnumerateSpaceSupportedComponentsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateSpaceSupportedComponentsFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
    pub type EnumerateSpaceSupportedComponentsFB = unsafe extern "system" fn(
        space: Space,
        component_type_capacity_input: u32,
        component_type_count_output: *mut u32,
        component_types: *mut SpaceComponentTypeFB,
    ) -> Result;
    #[doc = "See [xrSetSpaceComponentStatusFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetSpaceComponentStatusFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
    pub type SetSpaceComponentStatusFB = unsafe extern "system" fn(
        space: Space,
        info: *const SpaceComponentStatusSetInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceComponentStatusFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceComponentStatusFB) - defined by [XR_FB_spatial_entity](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity)"]
    pub type GetSpaceComponentStatusFB = unsafe extern "system" fn(
        space: Space,
        component_type: SpaceComponentTypeFB,
        status: *mut SpaceComponentStatusFB,
    ) -> Result;
    #[doc = "See [xrCreateTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateTriangleMeshFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type CreateTriangleMeshFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const TriangleMeshCreateInfoFB,
        out_triangle_mesh: *mut TriangleMeshFB,
    ) -> Result;
    #[doc = "See [xrDestroyTriangleMeshFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyTriangleMeshFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type DestroyTriangleMeshFB = unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrTriangleMeshGetVertexBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshGetVertexBufferFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshGetVertexBufferFB = unsafe extern "system" fn(
        mesh: TriangleMeshFB,
        out_vertex_buffer: *mut *mut Vector3f,
    ) -> Result;
    #[doc = "See [xrTriangleMeshGetIndexBufferFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshGetIndexBufferFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshGetIndexBufferFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB, out_index_buffer: *mut *mut u32) -> Result;
    #[doc = "See [xrTriangleMeshBeginUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshBeginUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshBeginUpdateFB = unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrTriangleMeshEndUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshEndUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshEndUpdateFB = unsafe extern "system" fn(
        mesh: TriangleMeshFB,
        vertex_count: u32,
        triangle_count: u32,
    ) -> Result;
    #[doc = "See [xrTriangleMeshBeginVertexBufferUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshBeginVertexBufferUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshBeginVertexBufferUpdateFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB, out_vertex_count: *mut u32) -> Result;
    #[doc = "See [xrTriangleMeshEndVertexBufferUpdateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrTriangleMeshEndVertexBufferUpdateFB) - defined by [XR_FB_triangle_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_triangle_mesh)"]
    pub type TriangleMeshEndVertexBufferUpdateFB =
        unsafe extern "system" fn(mesh: TriangleMeshFB) -> Result;
    #[doc = "See [xrCreatePassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreatePassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreatePassthroughFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const PassthroughCreateInfoFB,
        out_passthrough: *mut PassthroughFB,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyPassthroughFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyPassthroughFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrPassthroughStartFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughStartFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughStartFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrPassthroughPauseFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughPauseFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughPauseFB = unsafe extern "system" fn(passthrough: PassthroughFB) -> Result;
    #[doc = "See [xrCreatePassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreatePassthroughLayerFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreatePassthroughLayerFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const PassthroughLayerCreateInfoFB,
        out_layer: *mut PassthroughLayerFB,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughLayerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyPassthroughLayerFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyPassthroughLayerFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerPauseFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughLayerPauseFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerPauseFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerResumeFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughLayerResumeFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerResumeFB =
        unsafe extern "system" fn(layer: PassthroughLayerFB) -> Result;
    #[doc = "See [xrPassthroughLayerSetStyleFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughLayerSetStyleFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type PassthroughLayerSetStyleFB = unsafe extern "system" fn(
        layer: PassthroughLayerFB,
        style: *const PassthroughStyleFB,
    ) -> Result;
    #[doc = "See [xrCreateGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateGeometryInstanceFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type CreateGeometryInstanceFB = unsafe extern "system" fn(
        session: Session,
        create_info: *const GeometryInstanceCreateInfoFB,
        out_geometry_instance: *mut GeometryInstanceFB,
    ) -> Result;
    #[doc = "See [xrDestroyGeometryInstanceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyGeometryInstanceFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type DestroyGeometryInstanceFB =
        unsafe extern "system" fn(instance: GeometryInstanceFB) -> Result;
    #[doc = "See [xrGeometryInstanceSetTransformFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGeometryInstanceSetTransformFB) - defined by [XR_FB_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough)"]
    pub type GeometryInstanceSetTransformFB = unsafe extern "system" fn(
        instance: GeometryInstanceFB,
        transformation: *const GeometryInstanceTransformFB,
    ) -> Result;
    #[doc = "See [xrQuerySpacesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrQuerySpacesFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
    pub type QuerySpacesFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceQueryInfoBaseHeaderFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrRetrieveSpaceQueryResultsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrRetrieveSpaceQueryResultsFB) - defined by [XR_FB_spatial_entity_query](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_query)"]
    pub type RetrieveSpaceQueryResultsFB = unsafe extern "system" fn(
        session: Session,
        request_id: AsyncRequestIdFB,
        results: *mut SpaceQueryResultsFB,
    ) -> Result;
    #[doc = "See [xrSaveSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSaveSpaceFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
    pub type SaveSpaceFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceSaveInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrEraseSpaceFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEraseSpaceFB) - defined by [XR_FB_spatial_entity_storage](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage)"]
    pub type EraseSpaceFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceEraseInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrSaveSpaceListFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSaveSpaceListFB) - defined by [XR_FB_spatial_entity_storage_batch](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_storage_batch)"]
    pub type SaveSpaceListFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceListSaveInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrShareSpacesFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrShareSpacesFB) - defined by [XR_FB_spatial_entity_sharing](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_sharing)"]
    pub type ShareSpacesFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceShareInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceContainerFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceContainerFB) - defined by [XR_FB_spatial_entity_container](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_container)"]
    pub type GetSpaceContainerFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        space_container_output: *mut SpaceContainerFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceTriangleMeshMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceTriangleMeshMETA) - defined by [XR_META_spatial_entity_mesh](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_spatial_entity_mesh)"]
    pub type GetSpaceTriangleMeshMETA = unsafe extern "system" fn(
        space: Space,
        get_info: *const SpaceTriangleMeshGetInfoMETA,
        triangle_mesh_output: *mut SpaceTriangleMeshMETA,
    ) -> Result;
    #[doc = "See [xrGetSpaceBoundingBox2DFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceBoundingBox2DFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
    pub type GetSpaceBoundingBox2DFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        bounding_box2_d_output: *mut Rect2Df,
    ) -> Result;
    #[doc = "See [xrGetSpaceBoundingBox3DFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceBoundingBox3DFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
    pub type GetSpaceBoundingBox3DFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        bounding_box3_d_output: *mut Rect3DfFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceSemanticLabelsFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceSemanticLabelsFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
    pub type GetSpaceSemanticLabelsFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        semantic_labels_output: *mut SemanticLabelsFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceBoundary2DFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceBoundary2DFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
    pub type GetSpaceBoundary2DFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        boundary2_d_output: *mut Boundary2DFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceRoomLayoutFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceRoomLayoutFB) - defined by [XR_FB_scene](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene)"]
    pub type GetSpaceRoomLayoutFB = unsafe extern "system" fn(
        session: Session,
        space: Space,
        room_layout_output: *mut RoomLayoutFB,
    ) -> Result;
    #[doc = "See [xrRequestSceneCaptureFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrRequestSceneCaptureFB) - defined by [XR_FB_scene_capture](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_scene_capture)"]
    pub type RequestSceneCaptureFB = unsafe extern "system" fn(
        session: Session,
        info: *const SceneCaptureRequestInfoFB,
        request_id: *mut AsyncRequestIdFB,
    ) -> Result;
    #[doc = "See [xrPassthroughLayerSetKeyboardHandsIntensityFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPassthroughLayerSetKeyboardHandsIntensityFB) - defined by [XR_FB_passthrough_keyboard_hands](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_passthrough_keyboard_hands)"]
    pub type PassthroughLayerSetKeyboardHandsIntensityFB = unsafe extern "system" fn(
        layer: PassthroughLayerFB,
        intensity: *const PassthroughKeyboardHandsIntensityFB,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorStoreConnectionMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type CreateSpatialAnchorStoreConnectionMSFT = unsafe extern "system" fn(
        session: Session,
        spatial_anchor_store: *mut SpatialAnchorStoreConnectionMSFT,
    ) -> Result;
    #[doc = "See [xrDestroySpatialAnchorStoreConnectionMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySpatialAnchorStoreConnectionMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type DestroySpatialAnchorStoreConnectionMSFT =
        unsafe extern "system" fn(spatial_anchor_store: SpatialAnchorStoreConnectionMSFT) -> Result;
    #[doc = "See [xrPersistSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPersistSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type PersistSpatialAnchorMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_persistence_info: *const SpatialAnchorPersistenceInfoMSFT,
    ) -> Result;
    #[doc = "See [xrEnumeratePersistedSpatialAnchorNamesMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumeratePersistedSpatialAnchorNamesMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type EnumeratePersistedSpatialAnchorNamesMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_name_capacity_input: u32,
        spatial_anchor_name_count_output: *mut u32,
        spatial_anchor_names: *mut SpatialAnchorPersistenceNameMSFT,
    ) -> Result;
    #[doc = "See [xrCreateSpatialAnchorFromPersistedNameMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorFromPersistedNameMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type CreateSpatialAnchorFromPersistedNameMSFT = unsafe extern "system" fn(
        session: Session,
        spatial_anchor_create_info: *const SpatialAnchorFromPersistedAnchorCreateInfoMSFT,
        spatial_anchor: *mut SpatialAnchorMSFT,
    ) -> Result;
    #[doc = "See [xrUnpersistSpatialAnchorMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrUnpersistSpatialAnchorMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type UnpersistSpatialAnchorMSFT = unsafe extern "system" fn(
        spatial_anchor_store: SpatialAnchorStoreConnectionMSFT,
        spatial_anchor_persistence_name: *const SpatialAnchorPersistenceNameMSFT,
    ) -> Result;
    #[doc = "See [xrClearSpatialAnchorStoreMSFT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrClearSpatialAnchorStoreMSFT) - defined by [XR_MSFT_spatial_anchor_persistence](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MSFT_spatial_anchor_persistence)"]
    pub type ClearSpatialAnchorStoreMSFT =
        unsafe extern "system" fn(spatial_anchor_store: SpatialAnchorStoreConnectionMSFT) -> Result;
    #[doc = "See [xrCreateFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateFacialTrackerHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type CreateFacialTrackerHTC = unsafe extern "system" fn(
        session: Session,
        create_info: *const FacialTrackerCreateInfoHTC,
        facial_tracker: *mut FacialTrackerHTC,
    ) -> Result;
    #[doc = "See [xrDestroyFacialTrackerHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyFacialTrackerHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type DestroyFacialTrackerHTC =
        unsafe extern "system" fn(facial_tracker: FacialTrackerHTC) -> Result;
    #[doc = "See [xrGetFacialExpressionsHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetFacialExpressionsHTC) - defined by [XR_HTC_facial_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_facial_tracking)"]
    pub type GetFacialExpressionsHTC = unsafe extern "system" fn(
        facial_tracker: FacialTrackerHTC,
        facial_expressions: *mut FacialExpressionsHTC,
    ) -> Result;
    #[doc = "See [xrCreatePassthroughHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreatePassthroughHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
    pub type CreatePassthroughHTC = unsafe extern "system" fn(
        session: Session,
        create_info: *const PassthroughCreateInfoHTC,
        passthrough: *mut PassthroughHTC,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyPassthroughHTC) - defined by [XR_HTC_passthrough](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_passthrough)"]
    pub type DestroyPassthroughHTC =
        unsafe extern "system" fn(passthrough: PassthroughHTC) -> Result;
    #[doc = "See [xrCreateSpatialAnchorHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpatialAnchorHTC) - defined by [XR_HTC_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_anchor)"]
    pub type CreateSpatialAnchorHTC = unsafe extern "system" fn(
        session: Session,
        create_info: *const SpatialAnchorCreateInfoHTC,
        anchor: *mut Space,
    ) -> Result;
    #[doc = "See [xrGetSpatialAnchorNameHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpatialAnchorNameHTC) - defined by [XR_HTC_anchor](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_anchor)"]
    pub type GetSpatialAnchorNameHTC =
        unsafe extern "system" fn(anchor: Space, name: *mut SpatialAnchorNameHTC) -> Result;
    #[doc = "See [xrEnumerateViveTrackerPathsHTCX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateViveTrackerPathsHTCX) - defined by [XR_HTCX_vive_tracker_interaction](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTCX_vive_tracker_interaction)"]
    pub type EnumerateViveTrackerPathsHTCX = unsafe extern "system" fn(
        instance: Instance,
        path_capacity_input: u32,
        path_count_output: *mut u32,
        paths: *mut ViveTrackerPathsHTCX,
    ) -> Result;
    #[doc = "See [xrSetMarkerTrackingVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetMarkerTrackingVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingVARJO =
        unsafe extern "system" fn(session: Session, enabled: Bool32) -> Result;
    #[doc = "See [xrSetMarkerTrackingTimeoutVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetMarkerTrackingTimeoutVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingTimeoutVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, timeout: Duration) -> Result;
    #[doc = "See [xrSetMarkerTrackingPredictionVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetMarkerTrackingPredictionVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type SetMarkerTrackingPredictionVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, enable: Bool32) -> Result;
    #[doc = "See [xrGetMarkerSizeVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerSizeVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type GetMarkerSizeVARJO =
        unsafe extern "system" fn(session: Session, marker_id: u64, size: *mut Extent2Df) -> Result;
    #[doc = "See [xrCreateMarkerSpaceVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateMarkerSpaceVARJO) - defined by [XR_VARJO_marker_tracking](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_marker_tracking)"]
    pub type CreateMarkerSpaceVARJO = unsafe extern "system" fn(
        session: Session,
        create_info: *const MarkerSpaceCreateInfoVARJO,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrSetDigitalLensControlALMALENCE](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetDigitalLensControlALMALENCE) - defined by [XR_ALMALENCE_digital_lens_control](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ALMALENCE_digital_lens_control)"]
    pub type SetDigitalLensControlALMALENCE = unsafe extern "system" fn(
        session: Session,
        digital_lens_control: *const DigitalLensControlALMALENCE,
    ) -> Result;
    #[doc = "See [xrSetViewOffsetVARJO](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetViewOffsetVARJO) - defined by [XR_VARJO_view_offset](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_VARJO_view_offset)"]
    pub type SetViewOffsetVARJO =
        unsafe extern "system" fn(session: Session, offset: f32) -> Result;
    #[doc = "See [xrEnumerateExternalCamerasOCULUS](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateExternalCamerasOCULUS) - defined by [XR_OCULUS_external_camera](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_OCULUS_external_camera)"]
    pub type EnumerateExternalCamerasOCULUS = unsafe extern "system" fn(
        session: Session,
        camera_capacity_input: u32,
        camera_count_output: *mut u32,
        cameras: *mut ExternalCameraOCULUS,
    ) -> Result;
    #[doc = "See [xrCreatePassthroughColorLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreatePassthroughColorLutMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
    pub type CreatePassthroughColorLutMETA = unsafe extern "system" fn(
        passthrough: PassthroughFB,
        create_info: *const PassthroughColorLutCreateInfoMETA,
        color_lut: *mut PassthroughColorLutMETA,
    ) -> Result;
    #[doc = "See [xrDestroyPassthroughColorLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyPassthroughColorLutMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
    pub type DestroyPassthroughColorLutMETA =
        unsafe extern "system" fn(color_lut: PassthroughColorLutMETA) -> Result;
    #[doc = "See [xrUpdatePassthroughColorLutMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrUpdatePassthroughColorLutMETA) - defined by [XR_META_passthrough_color_lut](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_color_lut)"]
    pub type UpdatePassthroughColorLutMETA = unsafe extern "system" fn(
        color_lut: PassthroughColorLutMETA,
        update_info: *const PassthroughColorLutUpdateInfoMETA,
    ) -> Result;
    #[doc = "See [xrEnumeratePerformanceMetricsCounterPathsMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumeratePerformanceMetricsCounterPathsMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
    pub type EnumeratePerformanceMetricsCounterPathsMETA = unsafe extern "system" fn(
        instance: Instance,
        counter_path_capacity_input: u32,
        counter_path_count_output: *mut u32,
        counter_paths: *mut Path,
    ) -> Result;
    #[doc = "See [xrSetPerformanceMetricsStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetPerformanceMetricsStateMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
    pub type SetPerformanceMetricsStateMETA = unsafe extern "system" fn(
        session: Session,
        state: *const PerformanceMetricsStateMETA,
    ) -> Result;
    #[doc = "See [xrGetPerformanceMetricsStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetPerformanceMetricsStateMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
    pub type GetPerformanceMetricsStateMETA = unsafe extern "system" fn(
        session: Session,
        state: *mut PerformanceMetricsStateMETA,
    ) -> Result;
    #[doc = "See [xrQueryPerformanceMetricsCounterMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrQueryPerformanceMetricsCounterMETA) - defined by [XR_META_performance_metrics](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_performance_metrics)"]
    pub type QueryPerformanceMetricsCounterMETA = unsafe extern "system" fn(
        session: Session,
        counter_path: Path,
        counter: *mut PerformanceMetricsCounterMETA,
    ) -> Result;
    #[doc = "See [xrGetPassthroughPreferencesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetPassthroughPreferencesMETA) - defined by [XR_META_passthrough_preferences](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_passthrough_preferences)"]
    pub type GetPassthroughPreferencesMETA = unsafe extern "system" fn(
        session: Session,
        preferences: *mut PassthroughPreferencesMETA,
    ) -> Result;
    #[doc = "See [xrApplyFoveationHTC](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrApplyFoveationHTC) - defined by [XR_HTC_foveation](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_HTC_foveation)"]
    pub type ApplyFoveationHTC = unsafe extern "system" fn(
        session: Session,
        apply_info: *const FoveationApplyInfoHTC,
    ) -> Result;
    #[doc = "See [xrCreateSpaceFromCoordinateFrameUIDML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpaceFromCoordinateFrameUIDML) - defined by [XR_ML_compat](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_compat)"]
    pub type CreateSpaceFromCoordinateFrameUIDML = unsafe extern "system" fn(
        session: Session,
        create_info: *const CoordinateSpaceCreateInfoML,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrGetDeviceSampleRateFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetDeviceSampleRateFB) - defined by [XR_FB_haptic_pcm](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_haptic_pcm)"]
    pub type GetDeviceSampleRateFB = unsafe extern "system" fn(
        session: Session,
        haptic_action_info: *const HapticActionInfo,
        device_sample_rate: *mut DevicePcmSampleRateGetInfoFB,
    ) -> Result;
    #[doc = "See [xrSetTrackingOptimizationSettingsHintQCOM](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetTrackingOptimizationSettingsHintQCOM) - defined by [XR_QCOM_tracking_optimization_settings](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_QCOM_tracking_optimization_settings)"]
    pub type SetTrackingOptimizationSettingsHintQCOM = unsafe extern "system" fn(
        session: Session,
        domain: TrackingOptimizationSettingsDomainQCOM,
        hint: TrackingOptimizationSettingsHintQCOM,
    ) -> Result;
    #[doc = "See [xrCreateSpaceUserFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateSpaceUserFB) - defined by [XR_FB_spatial_entity_user](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_user)"]
    pub type CreateSpaceUserFB = unsafe extern "system" fn(
        session: Session,
        info: *const SpaceUserCreateInfoFB,
        user: *mut SpaceUserFB,
    ) -> Result;
    #[doc = "See [xrGetSpaceUserIdFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetSpaceUserIdFB) - defined by [XR_FB_spatial_entity_user](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_user)"]
    pub type GetSpaceUserIdFB =
        unsafe extern "system" fn(user: SpaceUserFB, user_id: *mut SpaceUserIdFB) -> Result;
    #[doc = "See [xrDestroySpaceUserFB](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroySpaceUserFB) - defined by [XR_FB_spatial_entity_user](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_FB_spatial_entity_user)"]
    pub type DestroySpaceUserFB = unsafe extern "system" fn(user: SpaceUserFB) -> Result;
    #[doc = "See [xrGetRecommendedLayerResolutionMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetRecommendedLayerResolutionMETA) - defined by [XR_META_recommended_layer_resolution](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_recommended_layer_resolution)"]
    pub type GetRecommendedLayerResolutionMETA = unsafe extern "system" fn(
        session: Session,
        info: *const RecommendedLayerResolutionGetInfoMETA,
        resolution: *mut RecommendedLayerResolutionMETA,
    ) -> Result;
    #[doc = "See [xrApplyForceFeedbackCurlMNDX](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrApplyForceFeedbackCurlMNDX) - defined by [XR_MNDX_force_feedback_curl](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MNDX_force_feedback_curl)"]
    pub type ApplyForceFeedbackCurlMNDX = unsafe extern "system" fn(
        hand_tracker: HandTrackerEXT,
        locations: *const ForceFeedbackCurlApplyLocationsMNDX,
    ) -> Result;
    #[doc = "See [xrCreatePlaneDetectorEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreatePlaneDetectorEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type CreatePlaneDetectorEXT = unsafe extern "system" fn(
        session: Session,
        create_info: *const PlaneDetectorCreateInfoEXT,
        plane_detector: *mut PlaneDetectorEXT,
    ) -> Result;
    #[doc = "See [xrDestroyPlaneDetectorEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyPlaneDetectorEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type DestroyPlaneDetectorEXT =
        unsafe extern "system" fn(plane_detector: PlaneDetectorEXT) -> Result;
    #[doc = "See [xrBeginPlaneDetectionEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrBeginPlaneDetectionEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type BeginPlaneDetectionEXT = unsafe extern "system" fn(
        plane_detector: PlaneDetectorEXT,
        begin_info: *const PlaneDetectorBeginInfoEXT,
    ) -> Result;
    #[doc = "See [xrGetPlaneDetectionStateEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetPlaneDetectionStateEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type GetPlaneDetectionStateEXT = unsafe extern "system" fn(
        plane_detector: PlaneDetectorEXT,
        state: *mut PlaneDetectionStateEXT,
    ) -> Result;
    #[doc = "See [xrGetPlaneDetectionsEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetPlaneDetectionsEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type GetPlaneDetectionsEXT = unsafe extern "system" fn(
        plane_detector: PlaneDetectorEXT,
        info: *const PlaneDetectorGetInfoEXT,
        locations: *mut PlaneDetectorLocationsEXT,
    ) -> Result;
    #[doc = "See [xrGetPlanePolygonBufferEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetPlanePolygonBufferEXT) - defined by [XR_EXT_plane_detection](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_plane_detection)"]
    pub type GetPlanePolygonBufferEXT = unsafe extern "system" fn(
        plane_detector: PlaneDetectorEXT,
        plane_id: u64,
        polygon_buffer_index: u32,
        polygon_buffer: *mut PlaneDetectorPolygonBufferEXT,
    ) -> Result;
    #[doc = "See [xrCreateVirtualKeyboardMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateVirtualKeyboardMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type CreateVirtualKeyboardMETA = unsafe extern "system" fn(
        session: Session,
        create_info: *const VirtualKeyboardCreateInfoMETA,
        keyboard: *mut VirtualKeyboardMETA,
    ) -> Result;
    #[doc = "See [xrDestroyVirtualKeyboardMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyVirtualKeyboardMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type DestroyVirtualKeyboardMETA =
        unsafe extern "system" fn(keyboard: VirtualKeyboardMETA) -> Result;
    #[doc = "See [xrCreateVirtualKeyboardSpaceMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateVirtualKeyboardSpaceMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type CreateVirtualKeyboardSpaceMETA = unsafe extern "system" fn(
        session: Session,
        keyboard: VirtualKeyboardMETA,
        create_info: *const VirtualKeyboardSpaceCreateInfoMETA,
        keyboard_space: *mut Space,
    ) -> Result;
    #[doc = "See [xrSuggestVirtualKeyboardLocationMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSuggestVirtualKeyboardLocationMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type SuggestVirtualKeyboardLocationMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        location_info: *const VirtualKeyboardLocationInfoMETA,
    ) -> Result;
    #[doc = "See [xrGetVirtualKeyboardScaleMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVirtualKeyboardScaleMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type GetVirtualKeyboardScaleMETA =
        unsafe extern "system" fn(keyboard: VirtualKeyboardMETA, scale: *mut f32) -> Result;
    #[doc = "See [xrSetVirtualKeyboardModelVisibilityMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetVirtualKeyboardModelVisibilityMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type SetVirtualKeyboardModelVisibilityMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        model_visibility: *const VirtualKeyboardModelVisibilitySetInfoMETA,
    ) -> Result;
    #[doc = "See [xrGetVirtualKeyboardModelAnimationStatesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVirtualKeyboardModelAnimationStatesMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type GetVirtualKeyboardModelAnimationStatesMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        animation_states: *mut VirtualKeyboardModelAnimationStatesMETA,
    ) -> Result;
    #[doc = "See [xrGetVirtualKeyboardDirtyTexturesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVirtualKeyboardDirtyTexturesMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type GetVirtualKeyboardDirtyTexturesMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        texture_id_capacity_input: u32,
        texture_id_count_output: *mut u32,
        texture_ids: *mut u64,
    ) -> Result;
    #[doc = "See [xrGetVirtualKeyboardTextureDataMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVirtualKeyboardTextureDataMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type GetVirtualKeyboardTextureDataMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        texture_id: u64,
        texture_data: *mut VirtualKeyboardTextureDataMETA,
    ) -> Result;
    #[doc = "See [xrSendVirtualKeyboardInputMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSendVirtualKeyboardInputMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type SendVirtualKeyboardInputMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        info: *const VirtualKeyboardInputInfoMETA,
        interactor_root_pose: *mut Posef,
    ) -> Result;
    #[doc = "See [xrChangeVirtualKeyboardTextContextMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrChangeVirtualKeyboardTextContextMETA) - defined by [XR_META_virtual_keyboard](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_virtual_keyboard)"]
    pub type ChangeVirtualKeyboardTextContextMETA = unsafe extern "system" fn(
        keyboard: VirtualKeyboardMETA,
        change_info: *const VirtualKeyboardTextContextChangeInfoMETA,
    ) -> Result;
    #[doc = "See [xrEnableUserCalibrationEventsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnableUserCalibrationEventsML) - defined by [XR_ML_user_calibration](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_user_calibration)"]
    pub type EnableUserCalibrationEventsML = unsafe extern "system" fn(
        instance: Instance,
        enable_info: *const UserCalibrationEnableEventsInfoML,
    ) -> Result;
    #[doc = "See [xrEnableLocalizationEventsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnableLocalizationEventsML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type EnableLocalizationEventsML = unsafe extern "system" fn(
        session: Session,
        info: *const LocalizationEnableEventsInfoML,
    ) -> Result;
    #[doc = "See [xrQueryLocalizationMapsML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrQueryLocalizationMapsML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type QueryLocalizationMapsML = unsafe extern "system" fn(
        session: Session,
        query_info: *const LocalizationMapQueryInfoBaseHeaderML,
        map_capacity_input: u32,
        map_count_output: *mut u32,
        maps: *mut LocalizationMapML,
    ) -> Result;
    #[doc = "See [xrRequestMapLocalizationML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrRequestMapLocalizationML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type RequestMapLocalizationML = unsafe extern "system" fn(
        session: Session,
        request_info: *const MapLocalizationRequestInfoML,
    ) -> Result;
    #[doc = "See [xrImportLocalizationMapML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrImportLocalizationMapML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type ImportLocalizationMapML = unsafe extern "system" fn(
        session: Session,
        import_info: *const LocalizationMapImportInfoML,
        map_uuid: *mut UuidEXT,
    ) -> Result;
    #[doc = "See [xrCreateExportedLocalizationMapML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateExportedLocalizationMapML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type CreateExportedLocalizationMapML = unsafe extern "system" fn(
        session: Session,
        map_uuid: *const UuidEXT,
        map: *mut ExportedLocalizationMapML,
    ) -> Result;
    #[doc = "See [xrDestroyExportedLocalizationMapML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyExportedLocalizationMapML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type DestroyExportedLocalizationMapML =
        unsafe extern "system" fn(map: ExportedLocalizationMapML) -> Result;
    #[doc = "See [xrGetExportedLocalizationMapDataML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetExportedLocalizationMapDataML) - defined by [XR_ML_localization_map](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_localization_map)"]
    pub type GetExportedLocalizationMapDataML = unsafe extern "system" fn(
        map: ExportedLocalizationMapML,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrCreateMarkerDetectorML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateMarkerDetectorML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type CreateMarkerDetectorML = unsafe extern "system" fn(
        session: Session,
        create_info: *const MarkerDetectorCreateInfoML,
        marker_detector: *mut MarkerDetectorML,
    ) -> Result;
    #[doc = "See [xrDestroyMarkerDetectorML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyMarkerDetectorML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type DestroyMarkerDetectorML =
        unsafe extern "system" fn(marker_detector: MarkerDetectorML) -> Result;
    #[doc = "See [xrSnapshotMarkerDetectorML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSnapshotMarkerDetectorML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type SnapshotMarkerDetectorML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        snapshot_info: *mut MarkerDetectorSnapshotInfoML,
    ) -> Result;
    #[doc = "See [xrGetMarkerDetectorStateML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerDetectorStateML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkerDetectorStateML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        state: *mut MarkerDetectorStateML,
    ) -> Result;
    #[doc = "See [xrGetMarkersML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkersML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkersML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        marker_capacity_input: u32,
        marker_count_output: *mut u32,
        markers: *mut MarkerML,
    ) -> Result;
    #[doc = "See [xrGetMarkerReprojectionErrorML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerReprojectionErrorML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkerReprojectionErrorML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        marker: MarkerML,
        reprojection_error_meters: *mut f32,
    ) -> Result;
    #[doc = "See [xrGetMarkerLengthML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerLengthML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkerLengthML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        marker: MarkerML,
        meters: *mut f32,
    ) -> Result;
    #[doc = "See [xrGetMarkerNumberML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerNumberML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkerNumberML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        marker: MarkerML,
        number: *mut u64,
    ) -> Result;
    #[doc = "See [xrGetMarkerStringML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetMarkerStringML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type GetMarkerStringML = unsafe extern "system" fn(
        marker_detector: MarkerDetectorML,
        marker: MarkerML,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut c_char,
    ) -> Result;
    #[doc = "See [xrCreateMarkerSpaceML](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateMarkerSpaceML) - defined by [XR_ML_marker_understanding](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_ML_marker_understanding)"]
    pub type CreateMarkerSpaceML = unsafe extern "system" fn(
        session: Session,
        create_info: *const MarkerSpaceCreateInfoML,
        space: *mut Space,
    ) -> Result;
    #[doc = "See [xrPollFutureEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrPollFutureEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
    pub type PollFutureEXT = unsafe extern "system" fn(
        instance: Instance,
        poll_info: *const FuturePollInfoEXT,
        poll_result: *mut FuturePollResultEXT,
    ) -> Result;
    #[doc = "See [xrCancelFutureEXT](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCancelFutureEXT) - defined by [XR_EXT_future](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_EXT_future)"]
    pub type CancelFutureEXT = unsafe extern "system" fn(
        instance: Instance,
        cancel_info: *const FutureCancelInfoEXT,
    ) -> Result;
    #[doc = "See [xrCreateEnvironmentDepthProviderMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateEnvironmentDepthProviderMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type CreateEnvironmentDepthProviderMETA = unsafe extern "system" fn(
        session: Session,
        create_info: *const EnvironmentDepthProviderCreateInfoMETA,
        environment_depth_provider: *mut EnvironmentDepthProviderMETA,
    ) -> Result;
    #[doc = "See [xrDestroyEnvironmentDepthProviderMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyEnvironmentDepthProviderMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type DestroyEnvironmentDepthProviderMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
    ) -> Result;
    #[doc = "See [xrStartEnvironmentDepthProviderMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrStartEnvironmentDepthProviderMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type StartEnvironmentDepthProviderMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
    ) -> Result;
    #[doc = "See [xrStopEnvironmentDepthProviderMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrStopEnvironmentDepthProviderMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type StopEnvironmentDepthProviderMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
    ) -> Result;
    #[doc = "See [xrCreateEnvironmentDepthSwapchainMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrCreateEnvironmentDepthSwapchainMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type CreateEnvironmentDepthSwapchainMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
        create_info: *const EnvironmentDepthSwapchainCreateInfoMETA,
        swapchain: *mut EnvironmentDepthSwapchainMETA,
    ) -> Result;
    #[doc = "See [xrDestroyEnvironmentDepthSwapchainMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrDestroyEnvironmentDepthSwapchainMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type DestroyEnvironmentDepthSwapchainMETA =
        unsafe extern "system" fn(swapchain: EnvironmentDepthSwapchainMETA) -> Result;
    #[doc = "See [xrEnumerateEnvironmentDepthSwapchainImagesMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateEnvironmentDepthSwapchainImagesMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type EnumerateEnvironmentDepthSwapchainImagesMETA = unsafe extern "system" fn(
        swapchain: EnvironmentDepthSwapchainMETA,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    )
        -> Result;
    #[doc = "See [xrGetEnvironmentDepthSwapchainStateMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetEnvironmentDepthSwapchainStateMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type GetEnvironmentDepthSwapchainStateMETA = unsafe extern "system" fn(
        swapchain: EnvironmentDepthSwapchainMETA,
        state: *mut EnvironmentDepthSwapchainStateMETA,
    ) -> Result;
    #[doc = "See [xrAcquireEnvironmentDepthImageMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrAcquireEnvironmentDepthImageMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type AcquireEnvironmentDepthImageMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
        acquire_info: *const EnvironmentDepthImageAcquireInfoMETA,
        environment_depth_image: *mut EnvironmentDepthImageMETA,
    ) -> Result;
    #[doc = "See [xrSetEnvironmentDepthHandRemovalMETA](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrSetEnvironmentDepthHandRemovalMETA) - defined by [XR_META_environment_depth](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth)"]
    pub type SetEnvironmentDepthHandRemovalMETA = unsafe extern "system" fn(
        environment_depth_provider: EnvironmentDepthProviderMETA,
        set_info: *const EnvironmentDepthHandRemovalSetInfoMETA,
    ) -> Result;
    #[doc = "See [xrLocateSpaces](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateSpaces)"]
    pub type LocateSpaces = unsafe extern "system" fn(
        session: Session,
        locate_info: *const SpacesLocateInfo,
        space_locations: *mut SpaceLocations,
    ) -> Result;
    #[doc = "See [xrGetVulkanGraphicsRequirements2KHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrGetVulkanGraphicsRequirements2KHR) - defined by [XR_KHR_vulkan_enable](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_KHR_vulkan_enable)"]
    pub type GetVulkanGraphicsRequirements2KHR = unsafe extern "system" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsVulkanKHR,
    ) -> Result;
    #[doc = "See [xrLocateSpacesKHR](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#xrLocateSpacesKHR)"]
    pub type LocateSpacesKHR = unsafe extern "system" fn(
        session: Session,
        locate_info: *const SpacesLocateInfo,
        space_locations: *mut SpaceLocations,
    ) -> Result;
}
pub const ALMALENCE_digital_lens_control_SPEC_VERSION: u32 = 1u32;
pub const ALMALENCE_DIGITAL_LENS_CONTROL_EXTENSION_NAME: &[u8] =
    b"XR_ALMALENCE_digital_lens_control\0";
pub const BD_controller_interaction_SPEC_VERSION: u32 = 2u32;
pub const BD_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_BD_controller_interaction\0";
pub const EPIC_view_configuration_fov_SPEC_VERSION: u32 = 2u32;
pub const EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME: &[u8] = b"XR_EPIC_view_configuration_fov\0";
pub const EXT_performance_settings_SPEC_VERSION: u32 = 4u32;
pub const EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME: &[u8] = b"XR_EXT_performance_settings\0";
pub const EXT_thermal_query_SPEC_VERSION: u32 = 2u32;
pub const EXT_THERMAL_QUERY_EXTENSION_NAME: &[u8] = b"XR_EXT_thermal_query\0";
pub const EXT_debug_utils_SPEC_VERSION: u32 = 5u32;
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &[u8] = b"XR_EXT_debug_utils\0";
pub const EXT_eye_gaze_interaction_SPEC_VERSION: u32 = 2u32;
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
pub const EXT_dpad_binding_SPEC_VERSION: u32 = 1u32;
pub const EXT_DPAD_BINDING_EXTENSION_NAME: &[u8] = b"XR_EXT_dpad_binding\0";
pub const EXT_hand_joints_motion_range_SPEC_VERSION: u32 = 1u32;
pub const EXT_HAND_JOINTS_MOTION_RANGE_EXTENSION_NAME: &[u8] = b"XR_EXT_hand_joints_motion_range\0";
pub const EXT_samsung_odyssey_controller_SPEC_VERSION: u32 = 1u32;
pub const EXT_SAMSUNG_ODYSSEY_CONTROLLER_EXTENSION_NAME: &[u8] =
    b"XR_EXT_samsung_odyssey_controller\0";
pub const EXT_hp_mixed_reality_controller_SPEC_VERSION: u32 = 1u32;
pub const EXT_HP_MIXED_REALITY_CONTROLLER_EXTENSION_NAME: &[u8] =
    b"XR_EXT_hp_mixed_reality_controller\0";
pub const EXT_palm_pose_SPEC_VERSION: u32 = 3u32;
pub const EXT_PALM_POSE_EXTENSION_NAME: &[u8] = b"XR_EXT_palm_pose\0";
pub const EXT_uuid_SPEC_VERSION: u32 = 1u32;
pub const EXT_UUID_EXTENSION_NAME: &[u8] = b"XR_EXT_uuid\0";
pub const EXT_hand_interaction_SPEC_VERSION: u32 = 1u32;
pub const EXT_HAND_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_EXT_hand_interaction\0";
pub const EXT_active_action_set_priority_SPEC_VERSION: u32 = 1u32;
pub const EXT_ACTIVE_ACTION_SET_PRIORITY_EXTENSION_NAME: &[u8] =
    b"XR_EXT_active_action_set_priority\0";
pub const EXT_local_floor_SPEC_VERSION: u32 = 1u32;
pub const EXT_LOCAL_FLOOR_EXTENSION_NAME: &[u8] = b"XR_EXT_local_floor\0";
pub const EXT_hand_tracking_data_source_SPEC_VERSION: u32 = 1u32;
pub const EXT_HAND_TRACKING_DATA_SOURCE_EXTENSION_NAME: &[u8] =
    b"XR_EXT_hand_tracking_data_source\0";
pub const EXT_plane_detection_SPEC_VERSION: u32 = 2u32;
pub const EXT_PLANE_DETECTION_EXTENSION_NAME: &[u8] = b"XR_EXT_plane_detection\0";
pub const EXT_future_SPEC_VERSION: u32 = 1u32;
pub const EXT_FUTURE_EXTENSION_NAME: &[u8] = b"XR_EXT_future\0";
pub const EXT_user_presence_SPEC_VERSION: u32 = 1u32;
pub const EXT_USER_PRESENCE_EXTENSION_NAME: &[u8] = b"XR_EXT_user_presence\0";
pub const EXT_composition_layer_inverted_alpha_SPEC_VERSION: u32 = 1u32;
pub const EXT_COMPOSITION_LAYER_INVERTED_ALPHA_EXTENSION_NAME: &[u8] =
    b"XR_EXT_composition_layer_inverted_alpha\0";
pub const FB_composition_layer_image_layout_SPEC_VERSION: u32 = 1u32;
pub const FB_COMPOSITION_LAYER_IMAGE_LAYOUT_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_image_layout\0";
pub const FB_composition_layer_alpha_blend_SPEC_VERSION: u32 = 3u32;
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
pub const FB_body_tracking_SPEC_VERSION: u32 = 1u32;
pub const FB_BODY_TRACKING_EXTENSION_NAME: &[u8] = b"XR_FB_body_tracking\0";
pub const FB_display_refresh_rate_SPEC_VERSION: u32 = 1u32;
pub const FB_DISPLAY_REFRESH_RATE_EXTENSION_NAME: &[u8] = b"XR_FB_display_refresh_rate\0";
pub const FB_color_space_SPEC_VERSION: u32 = 3u32;
pub const FB_COLOR_SPACE_EXTENSION_NAME: &[u8] = b"XR_FB_color_space\0";
pub const FB_hand_tracking_mesh_SPEC_VERSION: u32 = 3u32;
pub const FB_HAND_TRACKING_MESH_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_mesh\0";
pub const FB_hand_tracking_aim_SPEC_VERSION: u32 = 2u32;
pub const FB_HAND_TRACKING_AIM_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_aim\0";
pub const FB_hand_tracking_capsules_SPEC_VERSION: u32 = 3u32;
pub const FB_HAND_TRACKING_CAPSULES_EXTENSION_NAME: &[u8] = b"XR_FB_hand_tracking_capsules\0";
pub const FB_spatial_entity_SPEC_VERSION: u32 = 3u32;
pub const FB_SPATIAL_ENTITY_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity\0";
pub const FB_foveation_SPEC_VERSION: u32 = 1u32;
pub const FB_FOVEATION_EXTENSION_NAME: &[u8] = b"XR_FB_foveation\0";
pub const FB_foveation_configuration_SPEC_VERSION: u32 = 1u32;
pub const FB_FOVEATION_CONFIGURATION_EXTENSION_NAME: &[u8] = b"XR_FB_foveation_configuration\0";
pub const FB_keyboard_tracking_SPEC_VERSION: u32 = 1u32;
pub const FB_KEYBOARD_TRACKING_EXTENSION_NAME: &[u8] = b"XR_FB_keyboard_tracking\0";
pub const FB_triangle_mesh_SPEC_VERSION: u32 = 2u32;
pub const FB_TRIANGLE_MESH_EXTENSION_NAME: &[u8] = b"XR_FB_triangle_mesh\0";
pub const FB_passthrough_SPEC_VERSION: u32 = 4u32;
pub const FB_PASSTHROUGH_EXTENSION_NAME: &[u8] = b"XR_FB_passthrough\0";
pub const FB_render_model_SPEC_VERSION: u32 = 4u32;
pub const FB_RENDER_MODEL_EXTENSION_NAME: &[u8] = b"XR_FB_render_model\0";
pub const FB_spatial_entity_query_SPEC_VERSION: u32 = 1u32;
pub const FB_SPATIAL_ENTITY_QUERY_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity_query\0";
pub const FB_spatial_entity_storage_SPEC_VERSION: u32 = 1u32;
pub const FB_SPATIAL_ENTITY_STORAGE_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity_storage\0";
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
pub const FB_touch_controller_pro_SPEC_VERSION: u32 = 1u32;
pub const FB_TOUCH_CONTROLLER_PRO_EXTENSION_NAME: &[u8] = b"XR_FB_touch_controller_pro\0";
pub const FB_spatial_entity_sharing_SPEC_VERSION: u32 = 1u32;
pub const FB_SPATIAL_ENTITY_SHARING_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity_sharing\0";
pub const FB_space_warp_SPEC_VERSION: u32 = 2u32;
pub const FB_SPACE_WARP_EXTENSION_NAME: &[u8] = b"XR_FB_space_warp\0";
pub const FB_haptic_amplitude_envelope_SPEC_VERSION: u32 = 1u32;
pub const FB_HAPTIC_AMPLITUDE_ENVELOPE_EXTENSION_NAME: &[u8] = b"XR_FB_haptic_amplitude_envelope\0";
pub const FB_scene_SPEC_VERSION: u32 = 4u32;
pub const FB_SCENE_EXTENSION_NAME: &[u8] = b"XR_FB_scene\0";
pub const FB_scene_capture_SPEC_VERSION: u32 = 1u32;
pub const FB_SCENE_CAPTURE_EXTENSION_NAME: &[u8] = b"XR_FB_scene_capture\0";
pub const FB_spatial_entity_container_SPEC_VERSION: u32 = 2u32;
pub const FB_SPATIAL_ENTITY_CONTAINER_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity_container\0";
pub const FB_face_tracking_SPEC_VERSION: u32 = 1u32;
pub const FB_FACE_TRACKING_EXTENSION_NAME: &[u8] = b"XR_FB_face_tracking\0";
pub const FB_eye_tracking_social_SPEC_VERSION: u32 = 1u32;
pub const FB_EYE_TRACKING_SOCIAL_EXTENSION_NAME: &[u8] = b"XR_FB_eye_tracking_social\0";
pub const FB_passthrough_keyboard_hands_SPEC_VERSION: u32 = 2u32;
pub const FB_PASSTHROUGH_KEYBOARD_HANDS_EXTENSION_NAME: &[u8] =
    b"XR_FB_passthrough_keyboard_hands\0";
pub const FB_composition_layer_settings_SPEC_VERSION: u32 = 1u32;
pub const FB_COMPOSITION_LAYER_SETTINGS_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_settings\0";
pub const FB_touch_controller_proximity_SPEC_VERSION: u32 = 1u32;
pub const FB_TOUCH_CONTROLLER_PROXIMITY_EXTENSION_NAME: &[u8] =
    b"XR_FB_touch_controller_proximity\0";
pub const FB_haptic_pcm_SPEC_VERSION: u32 = 1u32;
pub const FB_HAPTIC_PCM_EXTENSION_NAME: &[u8] = b"XR_FB_haptic_pcm\0";
pub const FB_composition_layer_depth_test_SPEC_VERSION: u32 = 1u32;
pub const FB_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME: &[u8] =
    b"XR_FB_composition_layer_depth_test\0";
pub const FB_spatial_entity_storage_batch_SPEC_VERSION: u32 = 1u32;
pub const FB_SPATIAL_ENTITY_STORAGE_BATCH_EXTENSION_NAME: &[u8] =
    b"XR_FB_spatial_entity_storage_batch\0";
pub const FB_spatial_entity_user_SPEC_VERSION: u32 = 1u32;
pub const FB_SPATIAL_ENTITY_USER_EXTENSION_NAME: &[u8] = b"XR_FB_spatial_entity_user\0";
pub const FB_face_tracking2_SPEC_VERSION: u32 = 1u32;
pub const FB_FACE_TRACKING2_EXTENSION_NAME: &[u8] = b"XR_FB_face_tracking2\0";
pub const HTC_vive_cosmos_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTC_VIVE_COSMOS_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTC_vive_cosmos_controller_interaction\0";
pub const HTC_facial_tracking_SPEC_VERSION: u32 = 2u32;
pub const HTC_FACIAL_TRACKING_EXTENSION_NAME: &[u8] = b"XR_HTC_facial_tracking\0";
pub const HTC_vive_focus3_controller_interaction_SPEC_VERSION: u32 = 2u32;
pub const HTC_VIVE_FOCUS3_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTC_vive_focus3_controller_interaction\0";
pub const HTC_hand_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTC_HAND_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_HTC_hand_interaction\0";
pub const HTC_vive_wrist_tracker_interaction_SPEC_VERSION: u32 = 1u32;
pub const HTC_VIVE_WRIST_TRACKER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTC_vive_wrist_tracker_interaction\0";
pub const HTC_passthrough_SPEC_VERSION: u32 = 1u32;
pub const HTC_PASSTHROUGH_EXTENSION_NAME: &[u8] = b"XR_HTC_passthrough\0";
pub const HTC_foveation_SPEC_VERSION: u32 = 1u32;
pub const HTC_FOVEATION_EXTENSION_NAME: &[u8] = b"XR_HTC_foveation\0";
pub const HTC_anchor_SPEC_VERSION: u32 = 1u32;
pub const HTC_ANCHOR_EXTENSION_NAME: &[u8] = b"XR_HTC_anchor\0";
pub const HUAWEI_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const HUAWEI_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HUAWEI_controller_interaction\0";
#[cfg(target_os = "android")]
pub const KHR_android_thread_settings_SPEC_VERSION: u32 = 6u32;
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
pub const KHR_composition_layer_depth_SPEC_VERSION: u32 = 6u32;
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
pub const KHR_D3D11_enable_SPEC_VERSION: u32 = 9u32;
#[cfg(windows)]
pub const KHR_D3D11_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D11_enable\0";
#[cfg(windows)]
pub const KHR_D3D12_enable_SPEC_VERSION: u32 = 9u32;
#[cfg(windows)]
pub const KHR_D3D12_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D12_enable\0";
#[cfg(target_vendor = "apple")]
pub const KHR_metal_enable_SPEC_VERSION: u32 = 1u32;
#[cfg(target_vendor = "apple")]
pub const KHR_METAL_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_metal_enable\0";
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
pub const KHR_loader_init_SPEC_VERSION: u32 = 2u32;
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
pub const KHR_locate_spaces_SPEC_VERSION: u32 = 1u32;
pub const KHR_LOCATE_SPACES_EXTENSION_NAME: &[u8] = b"XR_KHR_locate_spaces\0";
pub const KHR_maintenance1_SPEC_VERSION: u32 = 1u32;
pub const KHR_MAINTENANCE1_EXTENSION_NAME: &[u8] = b"XR_KHR_maintenance1\0";
pub const META_foveation_eye_tracked_SPEC_VERSION: u32 = 1u32;
pub const META_FOVEATION_EYE_TRACKED_EXTENSION_NAME: &[u8] = b"XR_META_foveation_eye_tracked\0";
pub const META_local_dimming_SPEC_VERSION: u32 = 1u32;
pub const META_LOCAL_DIMMING_EXTENSION_NAME: &[u8] = b"XR_META_local_dimming\0";
pub const META_passthrough_preferences_SPEC_VERSION: u32 = 1u32;
pub const META_PASSTHROUGH_PREFERENCES_EXTENSION_NAME: &[u8] = b"XR_META_passthrough_preferences\0";
pub const META_virtual_keyboard_SPEC_VERSION: u32 = 1u32;
pub const META_VIRTUAL_KEYBOARD_EXTENSION_NAME: &[u8] = b"XR_META_virtual_keyboard\0";
pub const META_vulkan_swapchain_create_info_SPEC_VERSION: u32 = 1u32;
pub const META_VULKAN_SWAPCHAIN_CREATE_INFO_EXTENSION_NAME: &[u8] =
    b"XR_META_vulkan_swapchain_create_info\0";
pub const META_performance_metrics_SPEC_VERSION: u32 = 2u32;
pub const META_PERFORMANCE_METRICS_EXTENSION_NAME: &[u8] = b"XR_META_performance_metrics\0";
pub const META_headset_id_SPEC_VERSION: u32 = 2u32;
pub const META_HEADSET_ID_EXTENSION_NAME: &[u8] = b"XR_META_headset_id\0";
pub const META_recommended_layer_resolution_SPEC_VERSION: u32 = 1u32;
pub const META_RECOMMENDED_LAYER_RESOLUTION_EXTENSION_NAME: &[u8] =
    b"XR_META_recommended_layer_resolution\0";
pub const META_passthrough_color_lut_SPEC_VERSION: u32 = 1u32;
pub const META_PASSTHROUGH_COLOR_LUT_EXTENSION_NAME: &[u8] = b"XR_META_passthrough_color_lut\0";
pub const META_spatial_entity_mesh_SPEC_VERSION: u32 = 1u32;
pub const META_SPATIAL_ENTITY_MESH_EXTENSION_NAME: &[u8] = b"XR_META_spatial_entity_mesh\0";
pub const META_automatic_layer_filter_SPEC_VERSION: u32 = 1u32;
pub const META_AUTOMATIC_LAYER_FILTER_EXTENSION_NAME: &[u8] = b"XR_META_automatic_layer_filter\0";
pub const META_touch_controller_plus_SPEC_VERSION: u32 = 1u32;
pub const META_TOUCH_CONTROLLER_PLUS_EXTENSION_NAME: &[u8] = b"XR_META_touch_controller_plus\0";
pub const META_environment_depth_SPEC_VERSION: u32 = 1u32;
pub const META_ENVIRONMENT_DEPTH_EXTENSION_NAME: &[u8] = b"XR_META_environment_depth\0";
pub const ML_ml2_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const ML_ML2_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_ML_ml2_controller_interaction\0";
pub const ML_frame_end_info_SPEC_VERSION: u32 = 1u32;
pub const ML_FRAME_END_INFO_EXTENSION_NAME: &[u8] = b"XR_ML_frame_end_info\0";
pub const ML_global_dimmer_SPEC_VERSION: u32 = 1u32;
pub const ML_GLOBAL_DIMMER_EXTENSION_NAME: &[u8] = b"XR_ML_global_dimmer\0";
pub const ML_compat_SPEC_VERSION: u32 = 1u32;
pub const ML_COMPAT_EXTENSION_NAME: &[u8] = b"XR_ML_compat\0";
pub const ML_marker_understanding_SPEC_VERSION: u32 = 1u32;
pub const ML_MARKER_UNDERSTANDING_EXTENSION_NAME: &[u8] = b"XR_ML_marker_understanding\0";
pub const ML_localization_map_SPEC_VERSION: u32 = 1u32;
pub const ML_LOCALIZATION_MAP_EXTENSION_NAME: &[u8] = b"XR_ML_localization_map\0";
pub const ML_user_calibration_SPEC_VERSION: u32 = 1u32;
pub const ML_USER_CALIBRATION_EXTENSION_NAME: &[u8] = b"XR_ML_user_calibration\0";
pub const MND_headless_SPEC_VERSION: u32 = 2u32;
pub const MND_HEADLESS_EXTENSION_NAME: &[u8] = b"XR_MND_headless\0";
pub const MND_swapchain_usage_input_attachment_bit_SPEC_VERSION: u32 = 2u32;
pub const MND_SWAPCHAIN_USAGE_INPUT_ATTACHMENT_BIT_EXTENSION_NAME: &[u8] =
    b"XR_MND_swapchain_usage_input_attachment_bit\0";
pub const MSFT_unbounded_reference_space_SPEC_VERSION: u32 = 1u32;
pub const MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_unbounded_reference_space\0";
pub const MSFT_spatial_anchor_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SPATIAL_ANCHOR_EXTENSION_NAME: &[u8] = b"XR_MSFT_spatial_anchor\0";
pub const MSFT_spatial_graph_bridge_SPEC_VERSION: u32 = 2u32;
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
pub const MSFT_scene_understanding_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SCENE_UNDERSTANDING_EXTENSION_NAME: &[u8] = b"XR_MSFT_scene_understanding\0";
pub const MSFT_scene_understanding_serialization_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SCENE_UNDERSTANDING_SERIALIZATION_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_scene_understanding_serialization\0";
pub const MSFT_spatial_anchor_persistence_SPEC_VERSION: u32 = 2u32;
pub const MSFT_SPATIAL_ANCHOR_PERSISTENCE_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_spatial_anchor_persistence\0";
pub const MSFT_scene_marker_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SCENE_MARKER_EXTENSION_NAME: &[u8] = b"XR_MSFT_scene_marker\0";
#[cfg(target_os = "android")]
pub const OCULUS_android_session_state_enable_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME: &[u8] =
    b"XR_OCULUS_android_session_state_enable\0";
pub const OCULUS_audio_device_guid_SPEC_VERSION: u32 = 1u32;
pub const OCULUS_AUDIO_DEVICE_GUID_EXTENSION_NAME: &[u8] = b"XR_OCULUS_audio_device_guid\0";
pub const OCULUS_external_camera_SPEC_VERSION: u32 = 1u32;
pub const OCULUS_EXTERNAL_CAMERA_EXTENSION_NAME: &[u8] = b"XR_OCULUS_external_camera\0";
pub const OPPO_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const OPPO_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_OPPO_controller_interaction\0";
pub const QCOM_tracking_optimization_settings_SPEC_VERSION: u32 = 1u32;
pub const QCOM_TRACKING_OPTIMIZATION_SETTINGS_EXTENSION_NAME: &[u8] =
    b"XR_QCOM_tracking_optimization_settings\0";
pub const ULTRALEAP_hand_tracking_forearm_SPEC_VERSION: u32 = 1u32;
pub const ULTRALEAP_HAND_TRACKING_FOREARM_EXTENSION_NAME: &[u8] =
    b"XR_ULTRALEAP_hand_tracking_forearm\0";
pub const VALVE_analog_threshold_SPEC_VERSION: u32 = 2u32;
pub const VALVE_ANALOG_THRESHOLD_EXTENSION_NAME: &[u8] = b"XR_VALVE_analog_threshold\0";
pub const VARJO_quad_views_SPEC_VERSION: u32 = 2u32;
pub const VARJO_QUAD_VIEWS_EXTENSION_NAME: &[u8] = b"XR_VARJO_quad_views\0";
pub const VARJO_foveated_rendering_SPEC_VERSION: u32 = 3u32;
pub const VARJO_FOVEATED_RENDERING_EXTENSION_NAME: &[u8] = b"XR_VARJO_foveated_rendering\0";
pub const VARJO_composition_layer_depth_test_SPEC_VERSION: u32 = 2u32;
pub const VARJO_COMPOSITION_LAYER_DEPTH_TEST_EXTENSION_NAME: &[u8] =
    b"XR_VARJO_composition_layer_depth_test\0";
pub const VARJO_environment_depth_estimation_SPEC_VERSION: u32 = 1u32;
pub const VARJO_ENVIRONMENT_DEPTH_ESTIMATION_EXTENSION_NAME: &[u8] =
    b"XR_VARJO_environment_depth_estimation\0";
pub const VARJO_marker_tracking_SPEC_VERSION: u32 = 1u32;
pub const VARJO_MARKER_TRACKING_EXTENSION_NAME: &[u8] = b"XR_VARJO_marker_tracking\0";
pub const VARJO_view_offset_SPEC_VERSION: u32 = 1u32;
pub const VARJO_VIEW_OFFSET_EXTENSION_NAME: &[u8] = b"XR_VARJO_view_offset\0";
pub const VARJO_xr4_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const VARJO_XR4_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_VARJO_xr4_controller_interaction\0";
pub const YVR_controller_interaction_SPEC_VERSION: u32 = 1u32;
pub const YVR_CONTROLLER_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_YVR_controller_interaction\0";
pub const EXTX_overlay_SPEC_VERSION: u32 = 5u32;
pub const EXTX_OVERLAY_EXTENSION_NAME: &[u8] = b"XR_EXTX_overlay\0";
pub const MNDX_egl_enable_SPEC_VERSION: u32 = 2u32;
pub const MNDX_EGL_ENABLE_EXTENSION_NAME: &[u8] = b"XR_MNDX_egl_enable\0";
pub const MNDX_force_feedback_curl_SPEC_VERSION: u32 = 1u32;
pub const MNDX_FORCE_FEEDBACK_CURL_EXTENSION_NAME: &[u8] = b"XR_MNDX_force_feedback_curl\0";
pub const HTCX_vive_tracker_interaction_SPEC_VERSION: u32 = 3u32;
pub const HTCX_VIVE_TRACKER_INTERACTION_EXTENSION_NAME: &[u8] =
    b"XR_HTCX_vive_tracker_interaction\0";
#[cfg(feature = "linked")]
unsafe extern "system" {
    #[link_name = "xrNegotiateLoaderRuntimeInterface"]
    pub fn negotiate_loader_runtime_interface(
        loader_info: *const NegotiateLoaderInfo,
        runtime_request: *mut NegotiateRuntimeRequest,
    ) -> Result;
    #[link_name = "xrNegotiateLoaderApiLayerInterface"]
    pub fn negotiate_loader_api_layer_interface(
        loader_info: *const NegotiateLoaderInfo,
        layer_name: *const c_char,
        api_layer_request: *mut NegotiateApiLayerRequest,
    ) -> Result;
    #[link_name = "xrCreateApiLayerInstance"]
    pub fn create_api_layer_instance(
        info: *const InstanceCreateInfo,
        layer_info: *const ApiLayerCreateInfo,
        instance: *mut Instance,
    ) -> Result;
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
    #[link_name = "xrLocateSpaces"]
    pub fn locate_spaces(
        session: Session,
        locate_info: *const SpacesLocateInfo,
        space_locations: *mut SpaceLocations,
    ) -> Result;
    #[link_name = "xrLocateSpacesKHR"]
    pub fn locate_spaces_khr(
        session: Session,
        locate_info: *const SpacesLocateInfo,
        space_locations: *mut SpaceLocations,
    ) -> Result;
}

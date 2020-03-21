# ! [ doc = r" Automatically generated code; do not edit!" ] # ! [ allow ( non_upper_case_globals , clippy :: unreadable_literal , clippy :: identity_op ) ]use crate::platform::*;
use crate::support::*;
use crate::*;
use libc::timespec;
use std::fmt;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};
pub const CURRENT_API_VERSION: Version = Version::new(1u16, 0u16, 7u32);
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
#[doc = "Structure type enumerant - see [XrStructureType](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrStructureType)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StructureType(i32);
impl StructureType {
    pub const UNKNOWN: StructureType = StructureType(0i32);
    pub const API_LAYER_PROPERTIES: StructureType = StructureType(1i32);
    pub const EXTENSION_PROPERTIES: StructureType = StructureType(2i32);
    pub const INSTANCE_CREATE_INFO: StructureType = StructureType(3i32);
    pub const SYSTEM_GET_INFO: StructureType = StructureType(4i32);
    pub const SYSTEM_PROPERTIES: StructureType = StructureType(5i32);
    pub const VIEW_LOCATE_INFO: StructureType = StructureType(6i32);
    pub const VIEW: StructureType = StructureType(7i32);
    pub const SESSION_CREATE_INFO: StructureType = StructureType(8i32);
    pub const SWAPCHAIN_CREATE_INFO: StructureType = StructureType(9i32);
    pub const SESSION_BEGIN_INFO: StructureType = StructureType(10i32);
    pub const VIEW_STATE: StructureType = StructureType(11i32);
    pub const FRAME_END_INFO: StructureType = StructureType(12i32);
    pub const HAPTIC_VIBRATION: StructureType = StructureType(13i32);
    pub const EVENT_DATA_BUFFER: StructureType = StructureType(16i32);
    pub const EVENT_DATA_INSTANCE_LOSS_PENDING: StructureType = StructureType(17i32);
    pub const EVENT_DATA_SESSION_STATE_CHANGED: StructureType = StructureType(18i32);
    pub const ACTION_STATE_BOOLEAN: StructureType = StructureType(23i32);
    pub const ACTION_STATE_FLOAT: StructureType = StructureType(24i32);
    pub const ACTION_STATE_VECTOR2F: StructureType = StructureType(25i32);
    pub const ACTION_STATE_POSE: StructureType = StructureType(27i32);
    pub const ACTION_SET_CREATE_INFO: StructureType = StructureType(28i32);
    pub const ACTION_CREATE_INFO: StructureType = StructureType(29i32);
    pub const INSTANCE_PROPERTIES: StructureType = StructureType(32i32);
    pub const FRAME_WAIT_INFO: StructureType = StructureType(33i32);
    pub const COMPOSITION_LAYER_PROJECTION: StructureType = StructureType(35i32);
    pub const COMPOSITION_LAYER_QUAD: StructureType = StructureType(36i32);
    pub const REFERENCE_SPACE_CREATE_INFO: StructureType = StructureType(37i32);
    pub const ACTION_SPACE_CREATE_INFO: StructureType = StructureType(38i32);
    pub const EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING: StructureType = StructureType(40i32);
    pub const VIEW_CONFIGURATION_VIEW: StructureType = StructureType(41i32);
    pub const SPACE_LOCATION: StructureType = StructureType(42i32);
    pub const SPACE_VELOCITY: StructureType = StructureType(43i32);
    pub const FRAME_STATE: StructureType = StructureType(44i32);
    pub const VIEW_CONFIGURATION_PROPERTIES: StructureType = StructureType(45i32);
    pub const FRAME_BEGIN_INFO: StructureType = StructureType(46i32);
    pub const COMPOSITION_LAYER_PROJECTION_VIEW: StructureType = StructureType(48i32);
    pub const EVENT_DATA_EVENTS_LOST: StructureType = StructureType(49i32);
    pub const INTERACTION_PROFILE_SUGGESTED_BINDING: StructureType = StructureType(51i32);
    pub const EVENT_DATA_INTERACTION_PROFILE_CHANGED: StructureType = StructureType(52i32);
    pub const INTERACTION_PROFILE_STATE: StructureType = StructureType(53i32);
    pub const SWAPCHAIN_IMAGE_ACQUIRE_INFO: StructureType = StructureType(55i32);
    pub const SWAPCHAIN_IMAGE_WAIT_INFO: StructureType = StructureType(56i32);
    pub const SWAPCHAIN_IMAGE_RELEASE_INFO: StructureType = StructureType(57i32);
    pub const ACTION_STATE_GET_INFO: StructureType = StructureType(58i32);
    pub const HAPTIC_ACTION_INFO: StructureType = StructureType(59i32);
    pub const SESSION_ACTION_SETS_ATTACH_INFO: StructureType = StructureType(60i32);
    pub const ACTIONS_SYNC_INFO: StructureType = StructureType(61i32);
    pub const BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO: StructureType = StructureType(62i32);
    pub const INPUT_SOURCE_LOCALIZED_NAME_GET_INFO: StructureType = StructureType(63i32);
    pub const COMPOSITION_LAYER_CUBE_KHR: StructureType = StructureType(1000006000i32);
    pub const INSTANCE_CREATE_INFO_ANDROID_KHR: StructureType = StructureType(1000008000i32);
    pub const COMPOSITION_LAYER_DEPTH_INFO_KHR: StructureType = StructureType(1000010000i32);
    pub const VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR: StructureType =
        StructureType(1000014000i32);
    pub const EVENT_DATA_PERF_SETTINGS_EXT: StructureType = StructureType(1000015000i32);
    pub const COMPOSITION_LAYER_CYLINDER_KHR: StructureType = StructureType(1000017000i32);
    pub const COMPOSITION_LAYER_EQUIRECT_KHR: StructureType = StructureType(1000018000i32);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: StructureType = StructureType(1000019000i32);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: StructureType = StructureType(1000019001i32);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: StructureType = StructureType(1000019002i32);
    pub const DEBUG_UTILS_LABEL_EXT: StructureType = StructureType(1000019003i32);
    pub const GRAPHICS_BINDING_OPENGL_WIN32_KHR: StructureType = StructureType(1000023000i32);
    pub const GRAPHICS_BINDING_OPENGL_XLIB_KHR: StructureType = StructureType(1000023001i32);
    pub const GRAPHICS_BINDING_OPENGL_XCB_KHR: StructureType = StructureType(1000023002i32);
    pub const GRAPHICS_BINDING_OPENGL_WAYLAND_KHR: StructureType = StructureType(1000023003i32);
    pub const SWAPCHAIN_IMAGE_OPENGL_KHR: StructureType = StructureType(1000023004i32);
    pub const GRAPHICS_REQUIREMENTS_OPENGL_KHR: StructureType = StructureType(1000023005i32);
    pub const GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR: StructureType = StructureType(1000024001i32);
    pub const SWAPCHAIN_IMAGE_OPENGL_ES_KHR: StructureType = StructureType(1000024002i32);
    pub const GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR: StructureType = StructureType(1000024003i32);
    pub const GRAPHICS_BINDING_VULKAN_KHR: StructureType = StructureType(1000025000i32);
    pub const SWAPCHAIN_IMAGE_VULKAN_KHR: StructureType = StructureType(1000025001i32);
    pub const GRAPHICS_REQUIREMENTS_VULKAN_KHR: StructureType = StructureType(1000025002i32);
    pub const GRAPHICS_BINDING_D3D11_KHR: StructureType = StructureType(1000027000i32);
    pub const SWAPCHAIN_IMAGE_D3D11_KHR: StructureType = StructureType(1000027001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D11_KHR: StructureType = StructureType(1000027002i32);
    pub const GRAPHICS_BINDING_D3D12_KHR: StructureType = StructureType(1000028000i32);
    pub const SWAPCHAIN_IMAGE_D3D12_KHR: StructureType = StructureType(1000028001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D12_KHR: StructureType = StructureType(1000028002i32);
    pub const VISIBILITY_MASK_KHR: StructureType = StructureType(1000031000i32);
    pub const EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR: StructureType = StructureType(1000031001i32);
    pub const SESSION_CREATE_INFO_OVERLAY_EXTX: StructureType = StructureType(1000033000i32);
    pub const EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX: StructureType =
        StructureType(1000033003i32);
    pub const SPATIAL_ANCHOR_CREATE_INFO_MSFT: StructureType = StructureType(1000039000i32);
    pub const SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT: StructureType = StructureType(1000039001i32);
    pub const VIEW_CONFIGURATION_DEPTH_RANGE_EXT: StructureType = StructureType(1000046000i32);
    pub const VIEW_CONFIGURATION_VIEW_FOV_EPIC: StructureType = StructureType(1000059000i32);
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
            Self::VISIBILITY_MASK_KHR => Some("VISIBILITY_MASK_KHR"),
            Self::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
                Some("EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR")
            }
            Self::SESSION_CREATE_INFO_OVERLAY_EXTX => Some("SESSION_CREATE_INFO_OVERLAY_EXTX"),
            Self::EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX => {
                Some("EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX")
            }
            Self::SPATIAL_ANCHOR_CREATE_INFO_MSFT => Some("SPATIAL_ANCHOR_CREATE_INFO_MSFT"),
            Self::SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT => {
                Some("SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT")
            }
            Self::VIEW_CONFIGURATION_DEPTH_RANGE_EXT => Some("VIEW_CONFIGURATION_DEPTH_RANGE_EXT"),
            Self::VIEW_CONFIGURATION_VIEW_FOV_EPIC => Some("VIEW_CONFIGURATION_VIEW_FOV_EPIC"),
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
    pub const SUCCESS: Result = Result(0i32);
    #[doc = "The specified timeout time occurred before the operation could complete."]
    pub const TIMEOUT_EXPIRED: Result = Result(1i32);
    #[doc = "The session will be lost soon."]
    pub const SESSION_LOSS_PENDING: Result = Result(3i32);
    #[doc = "No event was available."]
    pub const EVENT_UNAVAILABLE: Result = Result(4i32);
    #[doc = "The space's bounds are not known at the moment."]
    pub const SPACE_BOUNDS_UNAVAILABLE: Result = Result(7i32);
    #[doc = "The session is not in the focused state."]
    pub const SESSION_NOT_FOCUSED: Result = Result(8i32);
    #[doc = "A frame has been discarded from composition."]
    pub const FRAME_DISCARDED: Result = Result(9i32);
    #[doc = "The function usage was invalid in some way."]
    pub const ERROR_VALIDATION_FAILURE: Result = Result(-1i32);
    #[doc = "The runtime failed to handle the function in an unexpected way that is not covered by another error result."]
    pub const ERROR_RUNTIME_FAILURE: Result = Result(-2i32);
    #[doc = "A memory allocation has failed."]
    pub const ERROR_OUT_OF_MEMORY: Result = Result(-3i32);
    #[doc = "The runtime does not support the requested API version."]
    pub const ERROR_API_VERSION_UNSUPPORTED: Result = Result(-4i32);
    #[doc = "Initialization of object could not be completed."]
    pub const ERROR_INITIALIZATION_FAILED: Result = Result(-6i32);
    #[doc = "The requested function was not found or is otherwise unsupported."]
    pub const ERROR_FUNCTION_UNSUPPORTED: Result = Result(-7i32);
    #[doc = "The requested feature is not supported."]
    pub const ERROR_FEATURE_UNSUPPORTED: Result = Result(-8i32);
    #[doc = "A requested extension is not supported."]
    pub const ERROR_EXTENSION_NOT_PRESENT: Result = Result(-9i32);
    #[doc = "The runtime supports no more of the requested resource."]
    pub const ERROR_LIMIT_REACHED: Result = Result(-10i32);
    #[doc = "The supplied size was smaller than required."]
    pub const ERROR_SIZE_INSUFFICIENT: Result = Result(-11i32);
    #[doc = "A supplied object handle was invalid."]
    pub const ERROR_HANDLE_INVALID: Result = Result(-12i32);
    #[doc = "The XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated."]
    pub const ERROR_INSTANCE_LOST: Result = Result(-13i32);
    #[doc = "The session is already running."]
    pub const ERROR_SESSION_RUNNING: Result = Result(-14i32);
    #[doc = "The session is not yet running."]
    pub const ERROR_SESSION_NOT_RUNNING: Result = Result(-16i32);
    #[doc = "The XrSession was lost. It will need to be destroyed and optionally recreated."]
    pub const ERROR_SESSION_LOST: Result = Result(-17i32);
    #[doc = "The provided XrSystemId was invalid."]
    pub const ERROR_SYSTEM_INVALID: Result = Result(-18i32);
    #[doc = "The provided XrPath was not valid."]
    pub const ERROR_PATH_INVALID: Result = Result(-19i32);
    #[doc = "The maximum number of supported semantic paths has been reached."]
    pub const ERROR_PATH_COUNT_EXCEEDED: Result = Result(-20i32);
    #[doc = "The semantic path character format is invalid."]
    pub const ERROR_PATH_FORMAT_INVALID: Result = Result(-21i32);
    #[doc = "The semantic path is unsupported."]
    pub const ERROR_PATH_UNSUPPORTED: Result = Result(-22i32);
    #[doc = "The layer was NULL or otherwise invalid."]
    pub const ERROR_LAYER_INVALID: Result = Result(-23i32);
    #[doc = "The number of specified layers is greater than the supported number."]
    pub const ERROR_LAYER_LIMIT_EXCEEDED: Result = Result(-24i32);
    #[doc = "The image rect was negatively sized or otherwise invalid."]
    pub const ERROR_SWAPCHAIN_RECT_INVALID: Result = Result(-25i32);
    #[doc = "The image format is not supported by the runtime or platform."]
    pub const ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED: Result = Result(-26i32);
    #[doc = "The API used to retrieve an action's state does not match the action's type."]
    pub const ERROR_ACTION_TYPE_MISMATCH: Result = Result(-27i32);
    #[doc = "The session is not in the ready state."]
    pub const ERROR_SESSION_NOT_READY: Result = Result(-28i32);
    #[doc = "The session is not in the stopping state."]
    pub const ERROR_SESSION_NOT_STOPPING: Result = Result(-29i32);
    #[doc = "The provided XrTime was zero, negative, or out of range."]
    pub const ERROR_TIME_INVALID: Result = Result(-30i32);
    #[doc = "The specified reference space is not supported by the runtime or system."]
    pub const ERROR_REFERENCE_SPACE_UNSUPPORTED: Result = Result(-31i32);
    #[doc = "The file could not be accessed."]
    pub const ERROR_FILE_ACCESS_ERROR: Result = Result(-32i32);
    #[doc = "The file's contents were invalid."]
    pub const ERROR_FILE_CONTENTS_INVALID: Result = Result(-33i32);
    #[doc = "The specified form factor is not supported by the current runtime or platform."]
    pub const ERROR_FORM_FACTOR_UNSUPPORTED: Result = Result(-34i32);
    #[doc = "The specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off."]
    pub const ERROR_FORM_FACTOR_UNAVAILABLE: Result = Result(-35i32);
    #[doc = "A requested API layer is not present or could not be loaded."]
    pub const ERROR_API_LAYER_NOT_PRESENT: Result = Result(-36i32);
    #[doc = "The call was made without having made a previously required call."]
    pub const ERROR_CALL_ORDER_INVALID: Result = Result(-37i32);
    #[doc = "The given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements."]
    pub const ERROR_GRAPHICS_DEVICE_INVALID: Result = Result(-38i32);
    #[doc = "The supplied pose was invalid with respect to the requirements."]
    pub const ERROR_POSE_INVALID: Result = Result(-39i32);
    #[doc = "The supplied index was outside the range of valid indices."]
    pub const ERROR_INDEX_OUT_OF_RANGE: Result = Result(-40i32);
    #[doc = "The specified view configuration type is not supported by the runtime or platform."]
    pub const ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED: Result = Result(-41i32);
    #[doc = "The specified environment blend mode is not supported by the runtime or platform."]
    pub const ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED: Result = Result(-42i32);
    #[doc = "The name provided was a duplicate of an already-existing resource."]
    pub const ERROR_NAME_DUPLICATED: Result = Result(-44i32);
    #[doc = "The name provided was invalid."]
    pub const ERROR_NAME_INVALID: Result = Result(-45i32);
    #[doc = "A referenced action set is not attached to the session."]
    pub const ERROR_ACTIONSET_NOT_ATTACHED: Result = Result(-46i32);
    #[doc = "The session already has attached action sets."]
    pub const ERROR_ACTIONSETS_ALREADY_ATTACHED: Result = Result(-47i32);
    #[doc = "The localized name provided was a duplicate of an already-existing resource."]
    pub const ERROR_LOCALIZED_NAME_DUPLICATED: Result = Result(-48i32);
    #[doc = "The localized name provided was invalid."]
    pub const ERROR_LOCALIZED_NAME_INVALID: Result = Result(-49i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed as thread id is invalid."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR: Result = Result(-1000003000i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR: Result = Result(-1000003001i32);
    #[doc = "Spatial anchor could not be created at that location."]
    pub const ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT: Result = Result(-1000039001i32);
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
            Self::ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR")
            }
            Self::ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR")
            }
            Self::ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => {
                Some("ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let reason = match * self { Self :: SUCCESS => Some ( "function successfully completed" ) , Self :: TIMEOUT_EXPIRED => Some ( "the specified timeout time occurred before the operation could complete" ) , Self :: SESSION_LOSS_PENDING => Some ( "the session will be lost soon" ) , Self :: EVENT_UNAVAILABLE => Some ( "no event was available" ) , Self :: SPACE_BOUNDS_UNAVAILABLE => Some ( "the space's bounds are not known at the moment" ) , Self :: SESSION_NOT_FOCUSED => Some ( "the session is not in the focused state" ) , Self :: FRAME_DISCARDED => Some ( "a frame has been discarded from composition" ) , Self :: ERROR_VALIDATION_FAILURE => Some ( "the function usage was invalid in some way" ) , Self :: ERROR_RUNTIME_FAILURE => Some ( "the runtime failed to handle the function in an unexpected way that is not covered by another error result" ) , Self :: ERROR_OUT_OF_MEMORY => Some ( "a memory allocation has failed" ) , Self :: ERROR_API_VERSION_UNSUPPORTED => Some ( "the runtime does not support the requested API version" ) , Self :: ERROR_INITIALIZATION_FAILED => Some ( "initialization of object could not be completed" ) , Self :: ERROR_FUNCTION_UNSUPPORTED => Some ( "the requested function was not found or is otherwise unsupported" ) , Self :: ERROR_FEATURE_UNSUPPORTED => Some ( "the requested feature is not supported" ) , Self :: ERROR_EXTENSION_NOT_PRESENT => Some ( "a requested extension is not supported" ) , Self :: ERROR_LIMIT_REACHED => Some ( "the runtime supports no more of the requested resource" ) , Self :: ERROR_SIZE_INSUFFICIENT => Some ( "the supplied size was smaller than required" ) , Self :: ERROR_HANDLE_INVALID => Some ( "a supplied object handle was invalid" ) , Self :: ERROR_INSTANCE_LOST => Some ( "the XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated" ) , Self :: ERROR_SESSION_RUNNING => Some ( "the session is already running" ) , Self :: ERROR_SESSION_NOT_RUNNING => Some ( "the session is not yet running" ) , Self :: ERROR_SESSION_LOST => Some ( "the XrSession was lost. It will need to be destroyed and optionally recreated" ) , Self :: ERROR_SYSTEM_INVALID => Some ( "the provided XrSystemId was invalid" ) , Self :: ERROR_PATH_INVALID => Some ( "the provided XrPath was not valid" ) , Self :: ERROR_PATH_COUNT_EXCEEDED => Some ( "the maximum number of supported semantic paths has been reached" ) , Self :: ERROR_PATH_FORMAT_INVALID => Some ( "the semantic path character format is invalid" ) , Self :: ERROR_PATH_UNSUPPORTED => Some ( "the semantic path is unsupported" ) , Self :: ERROR_LAYER_INVALID => Some ( "the layer was NULL or otherwise invalid" ) , Self :: ERROR_LAYER_LIMIT_EXCEEDED => Some ( "the number of specified layers is greater than the supported number" ) , Self :: ERROR_SWAPCHAIN_RECT_INVALID => Some ( "the image rect was negatively sized or otherwise invalid" ) , Self :: ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some ( "the image format is not supported by the runtime or platform" ) , Self :: ERROR_ACTION_TYPE_MISMATCH => Some ( "the API used to retrieve an action's state does not match the action's type" ) , Self :: ERROR_SESSION_NOT_READY => Some ( "the session is not in the ready state" ) , Self :: ERROR_SESSION_NOT_STOPPING => Some ( "the session is not in the stopping state" ) , Self :: ERROR_TIME_INVALID => Some ( "the provided XrTime was zero, negative, or out of range" ) , Self :: ERROR_REFERENCE_SPACE_UNSUPPORTED => Some ( "the specified reference space is not supported by the runtime or system" ) , Self :: ERROR_FILE_ACCESS_ERROR => Some ( "the file could not be accessed" ) , Self :: ERROR_FILE_CONTENTS_INVALID => Some ( "the file's contents were invalid" ) , Self :: ERROR_FORM_FACTOR_UNSUPPORTED => Some ( "the specified form factor is not supported by the current runtime or platform" ) , Self :: ERROR_FORM_FACTOR_UNAVAILABLE => Some ( "the specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off" ) , Self :: ERROR_API_LAYER_NOT_PRESENT => Some ( "a requested API layer is not present or could not be loaded" ) , Self :: ERROR_CALL_ORDER_INVALID => Some ( "the call was made without having made a previously required call" ) , Self :: ERROR_GRAPHICS_DEVICE_INVALID => Some ( "the given graphics device is not in a valid state. The graphics device could be lost or initialized without meeting graphics requirements" ) , Self :: ERROR_POSE_INVALID => Some ( "the supplied pose was invalid with respect to the requirements" ) , Self :: ERROR_INDEX_OUT_OF_RANGE => Some ( "the supplied index was outside the range of valid indices" ) , Self :: ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => Some ( "the specified view configuration type is not supported by the runtime or platform" ) , Self :: ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => Some ( "the specified environment blend mode is not supported by the runtime or platform" ) , Self :: ERROR_NAME_DUPLICATED => Some ( "the name provided was a duplicate of an already-existing resource" ) , Self :: ERROR_NAME_INVALID => Some ( "the name provided was invalid" ) , Self :: ERROR_ACTIONSET_NOT_ATTACHED => Some ( "a referenced action set is not attached to the session" ) , Self :: ERROR_ACTIONSETS_ALREADY_ATTACHED => Some ( "the session already has attached action sets" ) , Self :: ERROR_LOCALIZED_NAME_DUPLICATED => Some ( "the localized name provided was a duplicate of an already-existing resource" ) , Self :: ERROR_LOCALIZED_NAME_INVALID => Some ( "the localized name provided was invalid" ) , Self :: ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => Some ( "xrSetAndroidApplicationThreadKHR failed as thread id is invalid" ) , Self :: ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => Some ( "xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority" ) , Self :: ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => Some ( "spatial anchor could not be created at that location" ) , _ => None , } ;
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
    pub const UNKNOWN: ObjectType = ObjectType(0i32);
    #[doc = "XrInstance"]
    pub const INSTANCE: ObjectType = ObjectType(1i32);
    #[doc = "XrSession"]
    pub const SESSION: ObjectType = ObjectType(2i32);
    #[doc = "XrSwapchain"]
    pub const SWAPCHAIN: ObjectType = ObjectType(3i32);
    #[doc = "XrSpace"]
    pub const SPACE: ObjectType = ObjectType(4i32);
    #[doc = "XrActionSet"]
    pub const ACTION_SET: ObjectType = ObjectType(5i32);
    #[doc = "XrAction"]
    pub const ACTION: ObjectType = ObjectType(6i32);
    #[doc = "XrDebugUtilsMessengerEXT"]
    pub const DEBUG_UTILS_MESSENGER_EXT: ObjectType = ObjectType(1000019000i32);
    #[doc = "XrSpatialAnchorMSFT"]
    pub const SPATIAL_ANCHOR_MSFT: ObjectType = ObjectType(1000039000i32);
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
    pub const APPLICATION_MAIN: AndroidThreadTypeKHR = AndroidThreadTypeKHR(1i32);
    pub const APPLICATION_WORKER: AndroidThreadTypeKHR = AndroidThreadTypeKHR(2i32);
    pub const RENDERER_MAIN: AndroidThreadTypeKHR = AndroidThreadTypeKHR(3i32);
    pub const RENDERER_WORKER: AndroidThreadTypeKHR = AndroidThreadTypeKHR(4i32);
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
    pub const BOTH: EyeVisibility = EyeVisibility(0i32);
    #[doc = "Display in the left eye only."]
    pub const LEFT: EyeVisibility = EyeVisibility(1i32);
    #[doc = "Display in the right eye only."]
    pub const RIGHT: EyeVisibility = EyeVisibility(2i32);
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
    pub const BOOLEAN_INPUT: ActionType = ActionType(1i32);
    pub const FLOAT_INPUT: ActionType = ActionType(2i32);
    pub const VECTOR2F_INPUT: ActionType = ActionType(3i32);
    pub const POSE_INPUT: ActionType = ActionType(4i32);
    pub const VIBRATION_OUTPUT: ActionType = ActionType(100i32);
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
    pub const VIEW: ReferenceSpaceType = ReferenceSpaceType(1i32);
    pub const LOCAL: ReferenceSpaceType = ReferenceSpaceType(2i32);
    pub const STAGE: ReferenceSpaceType = ReferenceSpaceType(3i32);
    pub const UNBOUNDED_MSFT: ReferenceSpaceType = ReferenceSpaceType(1000038000i32);
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
    pub const HEAD_MOUNTED_DISPLAY: FormFactor = FormFactor(1i32);
    pub const HANDHELD_DISPLAY: FormFactor = FormFactor(2i32);
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
    pub const PRIMARY_MONO: ViewConfigurationType = ViewConfigurationType(1i32);
    pub const PRIMARY_STEREO: ViewConfigurationType = ViewConfigurationType(2i32);
    pub const PRIMARY_QUAD_VARJO: ViewConfigurationType = ViewConfigurationType(1000037000i32);
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
    pub const OPAQUE: EnvironmentBlendMode = EnvironmentBlendMode(1i32);
    pub const ADDITIVE: EnvironmentBlendMode = EnvironmentBlendMode(2i32);
    pub const ALPHA_BLEND: EnvironmentBlendMode = EnvironmentBlendMode(3i32);
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
    pub const UNKNOWN: SessionState = SessionState(0i32);
    pub const IDLE: SessionState = SessionState(1i32);
    pub const READY: SessionState = SessionState(2i32);
    pub const SYNCHRONIZED: SessionState = SessionState(3i32);
    pub const VISIBLE: SessionState = SessionState(4i32);
    pub const FOCUSED: SessionState = SessionState(5i32);
    pub const STOPPING: SessionState = SessionState(6i32);
    pub const LOSS_PENDING: SessionState = SessionState(7i32);
    pub const EXITING: SessionState = SessionState(8i32);
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
    pub const CPU: PerfSettingsDomainEXT = PerfSettingsDomainEXT(1i32);
    #[doc = "Indicates that the performance settings or notification applies to GPU domain"]
    pub const GPU: PerfSettingsDomainEXT = PerfSettingsDomainEXT(2i32);
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
    pub const COMPOSITING: PerfSettingsSubDomainEXT = PerfSettingsSubDomainEXT(1i32);
    #[doc = "Indicates that the performance notification originates from the RENDERING sub-domain"]
    pub const RENDERING: PerfSettingsSubDomainEXT = PerfSettingsSubDomainEXT(2i32);
    #[doc = "Indicates that the performance notification originates from the THERMAL sub-domain"]
    pub const THERMAL: PerfSettingsSubDomainEXT = PerfSettingsSubDomainEXT(3i32);
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
    pub const POWER_SAVINGS: PerfSettingsLevelEXT = PerfSettingsLevelEXT(0i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters a low\n                 and stable complexity section, during which reducing power is more important than\n                 occasional late rendering frames"]
    pub const SUSTAINED_LOW: PerfSettingsLevelEXT = PerfSettingsLevelEXT(25i32);
    #[doc = "Performance settings hint used by the application to indicate that it enters\n                 a high or dynamic complexity section, during which the XR Runtime strives for consistent\n                 XR compositing and frame rendering within a thermally sustainable range"]
    pub const SUSTAINED_HIGH: PerfSettingsLevelEXT = PerfSettingsLevelEXT(50i32);
    #[doc = "Performance settings hint used by the application to indicate that the application enters\n                 a section with very high complexity, during which the XR Runtime is allowed to step\n                 up beyond the thermally sustainable range"]
    pub const BOOST: PerfSettingsLevelEXT = PerfSettingsLevelEXT(75i32);
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
    pub const NORMAL: PerfSettingsNotificationLevelEXT = PerfSettingsNotificationLevelEXT(0i32);
    #[doc = "Notifies that the sub-domain has reached an early warning level\n                 where the application should start proactive mitigation actions\n                 with the goal to return to the XR_PERF_NOTIF_LEVEL_NORMAL level"]
    pub const WARNING: PerfSettingsNotificationLevelEXT = PerfSettingsNotificationLevelEXT(25i32);
    #[doc = "Notifies that the sub-domain has reached a critical\n                 level with significant performance degradation.\n                 The application should take drastic mitigation action"]
    pub const IMPAIRED: PerfSettingsNotificationLevelEXT = PerfSettingsNotificationLevelEXT(75i32);
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
    pub const HIDDEN_TRIANGLE_MESH: VisibilityMaskTypeKHR = VisibilityMaskTypeKHR(1i32);
    #[doc = "inclusive mesh; indicates strictly that which the viewer can see."]
    pub const VISIBLE_TRIANGLE_MESH: VisibilityMaskTypeKHR = VisibilityMaskTypeKHR(2i32);
    #[doc = "line loop; traces the outline of the area the viewer can see."]
    pub const LINE_LOOP: VisibilityMaskTypeKHR = VisibilityMaskTypeKHR(3i32);
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
    pub const PROTECTED_CONTENT: SwapchainCreateFlags = SwapchainCreateFlags(1 << 0u64);
    #[doc = "Only one image will be acquired from this swapchain over its lifetime"]
    pub const STATIC_IMAGE: SwapchainCreateFlags = SwapchainCreateFlags(1 << 1u64);
}
bitmask!(SwapchainCreateFlags);
#[doc = "See [XrSwapchainUsageFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainUsageFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainUsageFlags(u64);
impl SwapchainUsageFlags {
    #[doc = "Specifies that the image can: be a color rendering target."]
    pub const COLOR_ATTACHMENT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 0u64);
    #[doc = "Specifies that the image can: be a depth/stencil rendering target."]
    pub const DEPTH_STENCIL_ATTACHMENT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 1u64);
    #[doc = "Specifies that the image can: be used as data/compute."]
    pub const UNORDERED_ACCESS: SwapchainUsageFlags = SwapchainUsageFlags(1 << 2u64);
    #[doc = "Specifies that the image can: be used as the source of a transfer command."]
    pub const TRANSFER_SRC: SwapchainUsageFlags = SwapchainUsageFlags(1 << 3u64);
    #[doc = "Specifies that the image can: be used as the destination of a transfer command."]
    pub const TRANSFER_DST: SwapchainUsageFlags = SwapchainUsageFlags(1 << 4u64);
    #[doc = "Specifies that the image can: be sampled by a shader."]
    pub const SAMPLED: SwapchainUsageFlags = SwapchainUsageFlags(1 << 5u64);
    #[doc = "Specifies that the image can: be reinterpreted as another image format."]
    pub const MUTABLE_FORMAT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 6u64);
}
bitmask!(SwapchainUsageFlags);
#[doc = "See [XrViewStateFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewStateFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ViewStateFlags(u64);
impl ViewStateFlags {
    #[doc = "Indicates validity of all XrView orientations"]
    pub const ORIENTATION_VALID: ViewStateFlags = ViewStateFlags(1 << 0u64);
    #[doc = "Indicates validity of all XrView positions"]
    pub const POSITION_VALID: ViewStateFlags = ViewStateFlags(1 << 1u64);
    #[doc = "Indicates whether all XrView orientations are actively tracked"]
    pub const ORIENTATION_TRACKED: ViewStateFlags = ViewStateFlags(1 << 2u64);
    #[doc = "Indicates whether all XrView positions are actively tracked"]
    pub const POSITION_TRACKED: ViewStateFlags = ViewStateFlags(1 << 3u64);
}
bitmask!(ViewStateFlags);
#[doc = "See [XrCompositionLayerFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerFlags(u64);
impl CompositionLayerFlags {
    #[doc = "Enables chromatic aberration correction when not done by default."]
    pub const CORRECT_CHROMATIC_ABERRATION: CompositionLayerFlags =
        CompositionLayerFlags(1 << 0u64);
    #[doc = "Enables the layer texture alpha channel."]
    pub const BLEND_TEXTURE_SOURCE_ALPHA: CompositionLayerFlags = CompositionLayerFlags(1 << 1u64);
    #[doc = "Indicates the texture color channels have not been premultiplied by the texture alpha channel."]
    pub const UNPREMULTIPLIED_ALPHA: CompositionLayerFlags = CompositionLayerFlags(1 << 2u64);
}
bitmask!(CompositionLayerFlags);
#[doc = "See [XrSpaceLocationFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceLocationFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceLocationFlags(u64);
impl SpaceLocationFlags {
    #[doc = "Indicates validity of orientation member"]
    pub const ORIENTATION_VALID: SpaceLocationFlags = SpaceLocationFlags(1 << 0u64);
    #[doc = "Indicates validity of position member"]
    pub const POSITION_VALID: SpaceLocationFlags = SpaceLocationFlags(1 << 1u64);
    #[doc = "Indicates whether pose member contains an actively tracked orientation"]
    pub const ORIENTATION_TRACKED: SpaceLocationFlags = SpaceLocationFlags(1 << 2u64);
    #[doc = "Indicates whether pose member contains an actively tracked position"]
    pub const POSITION_TRACKED: SpaceLocationFlags = SpaceLocationFlags(1 << 3u64);
}
bitmask!(SpaceLocationFlags);
#[doc = "See [XrSpaceVelocityFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSpaceVelocityFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceVelocityFlags(u64);
impl SpaceVelocityFlags {
    #[doc = "Indicates validity of linearVelocity member"]
    pub const LINEAR_VALID: SpaceVelocityFlags = SpaceVelocityFlags(1 << 0u64);
    #[doc = "Indicates validity of angularVelocity member"]
    pub const ANGULAR_VALID: SpaceVelocityFlags = SpaceVelocityFlags(1 << 1u64);
}
bitmask!(SpaceVelocityFlags);
#[doc = "See [XrInputSourceLocalizedNameFlagBits](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrInputSourceLocalizedNameFlagBits)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InputSourceLocalizedNameFlags(u64);
impl InputSourceLocalizedNameFlags {
    #[doc = "Asks for the part of the string which indicates the top level user path the source represents"]
    pub const USER_PATH: InputSourceLocalizedNameFlags = InputSourceLocalizedNameFlags(1 << 0u64);
    #[doc = "Asks for the part of the string which represents the interaction profile of the source"]
    pub const INTERACTION_PROFILE: InputSourceLocalizedNameFlags =
        InputSourceLocalizedNameFlags(1 << 1u64);
    #[doc = "Asks for the part of the string which represents the component on the device which needs to be interacted with"]
    pub const COMPONENT: InputSourceLocalizedNameFlags = InputSourceLocalizedNameFlags(1 << 2u64);
}
bitmask!(InputSourceLocalizedNameFlags);
#[doc = "See [XrDebugUtilsMessageSeverityFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessageSeverityFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageSeverityFlagsEXT(u64);
impl DebugUtilsMessageSeverityFlagsEXT {
    #[doc = "Most verbose output severity, typically used for debugging."]
    pub const VERBOSE: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 0u64);
    #[doc = "General info message"]
    pub const INFO: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 4u64);
    #[doc = "Indicates the item may be the cause of issues."]
    pub const WARNING: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 8u64);
    #[doc = "Indicates that the item is definitely related to erroneous behavior."]
    pub const ERROR: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 12u64);
}
bitmask!(DebugUtilsMessageSeverityFlagsEXT);
#[doc = "See [XrDebugUtilsMessageTypeFlagsEXT](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrDebugUtilsMessageTypeFlagsEXT)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageTypeFlagsEXT(u64);
impl DebugUtilsMessageTypeFlagsEXT {
    #[doc = "Indicates this is a general message"]
    pub const GENERAL: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 0u64);
    #[doc = "Indicates the message is related to a validation message"]
    pub const VALIDATION: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 1u64);
    #[doc = "Indicates the message is related to a potential performance situation"]
    pub const PERFORMANCE: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 2u64);
    #[doc = "Indicates the message is related to a non-conformant runtime result"]
    pub const CONFORMANCE: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 3u64);
}
bitmask!(DebugUtilsMessageTypeFlagsEXT);
#[doc = "See [XrOverlayMainSessionFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOverlayMainSessionFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlayMainSessionFlagsEXTX(u64);
impl OverlayMainSessionFlagsEXTX {
    #[doc = "Indicates the main session enabled XR_KHR_extra_layer_info_depth"]
    pub const ENABLED_COMPOSITION_LAYER_INFO_DEPTH: OverlayMainSessionFlagsEXTX =
        OverlayMainSessionFlagsEXTX(1 << 0u64);
}
bitmask!(OverlayMainSessionFlagsEXTX);
#[doc = "See [XrOverlaySessionCreateFlagsEXTX](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOverlaySessionCreateFlagsEXTX)"]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct OverlaySessionCreateFlagsEXTX(u64);
impl OverlaySessionCreateFlagsEXTX {
    #[doc = "Indicates the runtime does not need to attempt to lock the overlay session displayTime to the main session displayTime"]
    pub const RELAXED_DISPLAY_TIME: OverlaySessionCreateFlagsEXTX =
        OverlaySessionCreateFlagsEXTX(1 << 0u64);
}
bitmask!(OverlaySessionCreateFlagsEXTX);
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
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrVector2f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector2f)"]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrVector3f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector3f)"]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrVector4f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrVector4f)"]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrColor4f](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrColor4f)"]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrQuaternionf](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrQuaternionf)"]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrPosef](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPosef)"]
pub struct Posef {
    pub orientation: Quaternionf,
    pub position: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrOffset2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOffset2Df)"]
pub struct Offset2Df {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrExtent2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrExtent2Df)"]
pub struct Extent2Df {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrRect2Df](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRect2Df)"]
pub struct Rect2Df {
    pub offset: Offset2Df,
    pub extent: Extent2Df,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrOffset2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrOffset2Di)"]
pub struct Offset2Di {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrExtent2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrExtent2Di)"]
pub struct Extent2Di {
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrRect2Di](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrRect2Di)"]
pub struct Rect2Di {
    pub offset: Offset2Di,
    pub extent: Extent2Di,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrBaseInStructure](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBaseInStructure)"]
pub struct BaseInStructure {
    pub ty: StructureType,
    pub next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrBaseOutStructure](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrBaseOutStructure)"]
pub struct BaseOutStructure {
    pub ty: StructureType,
    pub next: *mut BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrApplicationInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrApplicationInfo)"]
pub struct ApplicationInfo {
    pub application_name: [c_char; MAX_APPLICATION_NAME_SIZE],
    pub application_version: u32,
    pub engine_name: [c_char; MAX_ENGINE_NAME_SIZE],
    pub engine_version: u32,
    pub api_version: Version,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrSystemGraphicsProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemGraphicsProperties)"]
pub struct SystemGraphicsProperties {
    pub max_swapchain_image_height: u32,
    pub max_swapchain_image_width: u32,
    pub max_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrSystemTrackingProperties](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSystemTrackingProperties)"]
pub struct SystemTrackingProperties {
    pub orientation_tracking: Bool32,
    pub position_tracking: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrSwapchainImageBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageBaseHeader)"]
pub struct SwapchainImageBaseHeader {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrSwapchainImageAcquireInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageAcquireInfo)"]
pub struct SwapchainImageAcquireInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageAcquireInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_ACQUIRE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrSwapchainImageReleaseInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainImageReleaseInfo)"]
pub struct SwapchainImageReleaseInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl SwapchainImageReleaseInfo {
    pub const TYPE: StructureType = StructureType::SWAPCHAIN_IMAGE_RELEASE_INFO;
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrFovf](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFovf)"]
pub struct Fovf {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrSwapchainSubImage](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrSwapchainSubImage)"]
pub struct SwapchainSubImage {
    pub swapchain: Swapchain,
    pub image_rect: Rect2Di,
    pub image_array_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "See [XrCompositionLayerBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrCompositionLayerBaseHeader)"]
pub struct CompositionLayerBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrFrameBeginInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameBeginInfo)"]
pub struct FrameBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameBeginInfo {
    pub const TYPE: StructureType = StructureType::FRAME_BEGIN_INFO;
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrFrameWaitInfo](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrFrameWaitInfo)"]
pub struct FrameWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
impl FrameWaitInfo {
    pub const TYPE: StructureType = StructureType::FRAME_WAIT_INFO;
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrHapticBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrHapticBaseHeader)"]
pub struct HapticBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrEventDataBaseHeader](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrEventDataBaseHeader)"]
pub struct EventDataBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrActionSuggestedBinding](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActionSuggestedBinding)"]
pub struct ActionSuggestedBinding {
    pub action: Action,
    pub binding: Path,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrActiveActionSet](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrActiveActionSet)"]
pub struct ActiveActionSet {
    pub action_set: ActionSet,
    pub subaction_path: Path,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
#[doc = "See [XrViewConfigurationViewFovEPIC](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrViewConfigurationViewFovEPIC) - defined by [XR_EPIC_view_configuration_fov](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_EPIC_view_configuration_fov)"]
pub struct ViewConfigurationViewFovEPIC {
    pub ty: StructureType,
    pub next: *const c_void,
    pub recommended_mutable_fov: Fovf,
    pub max_mutable_fov: Fovf,
}
impl ViewConfigurationViewFovEPIC {
    pub const TYPE: StructureType = StructureType::VIEW_CONFIGURATION_VIEW_FOV_EPIC;
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
}
pub const EPIC_view_configuration_fov_SPEC_VERSION: u32 = 1u32;
pub const EPIC_VIEW_CONFIGURATION_FOV_EXTENSION_NAME: &[u8] = b"XR_EPIC_view_configuration_fov\0";
pub const EXT_performance_settings_SPEC_VERSION: u32 = 1u32;
pub const EXT_PERFORMANCE_SETTINGS_EXTENSION_NAME: &[u8] = b"XR_EXT_performance_settings\0";
pub const EXT_thermal_query_SPEC_VERSION: u32 = 1u32;
pub const EXT_THERMAL_QUERY_EXTENSION_NAME: &[u8] = b"XR_EXT_thermal_query\0";
pub const EXT_debug_utils_SPEC_VERSION: u32 = 3u32;
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &[u8] = b"XR_EXT_debug_utils\0";
pub const EXT_view_configuration_depth_range_SPEC_VERSION: u32 = 1u32;
pub const EXT_VIEW_CONFIGURATION_DEPTH_RANGE_EXTENSION_NAME: &[u8] =
    b"XR_EXT_view_configuration_depth_range\0";
pub const EXT_conformance_automation_SPEC_VERSION: u32 = 1u32;
pub const EXT_CONFORMANCE_AUTOMATION_EXTENSION_NAME: &[u8] = b"XR_EXT_conformance_automation\0";
#[cfg(windows)]
pub const EXT_win32_appcontainer_compatible_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const EXT_WIN32_APPCONTAINER_COMPATIBLE_EXTENSION_NAME: &[u8] =
    b"XR_EXT_win32_appcontainer_compatible\0";
pub const EXTX_overlay_SPEC_VERSION: u32 = 3u32;
pub const EXTX_OVERLAY_EXTENSION_NAME: &[u8] = b"XR_EXTX_overlay\0";
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
pub const KHR_vulkan_swapchain_format_list_SPEC_VERSION: u32 = 2u32;
pub const KHR_VULKAN_SWAPCHAIN_FORMAT_LIST_EXTENSION_NAME: &[u8] =
    b"XR_KHR_vulkan_swapchain_format_list\0";
pub const KHR_composition_layer_cylinder_SPEC_VERSION: u32 = 4u32;
pub const KHR_COMPOSITION_LAYER_CYLINDER_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_cylinder\0";
pub const KHR_composition_layer_equirect_SPEC_VERSION: u32 = 3u32;
pub const KHR_COMPOSITION_LAYER_EQUIRECT_EXTENSION_NAME: &[u8] =
    b"XR_KHR_composition_layer_equirect\0";
pub const KHR_opengl_enable_SPEC_VERSION: u32 = 8u32;
pub const KHR_OPENGL_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_opengl_enable\0";
pub const KHR_opengl_es_enable_SPEC_VERSION: u32 = 6u32;
pub const KHR_OPENGL_ES_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_opengl_es_enable\0";
pub const KHR_vulkan_enable_SPEC_VERSION: u32 = 6u32;
pub const KHR_VULKAN_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_vulkan_enable\0";
#[cfg(windows)]
pub const KHR_D3D11_enable_SPEC_VERSION: u32 = 4u32;
#[cfg(windows)]
pub const KHR_D3D11_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D11_enable\0";
#[cfg(windows)]
pub const KHR_D3D12_enable_SPEC_VERSION: u32 = 6u32;
#[cfg(windows)]
pub const KHR_D3D12_ENABLE_EXTENSION_NAME: &[u8] = b"XR_KHR_D3D12_enable\0";
pub const KHR_visibility_mask_SPEC_VERSION: u32 = 2u32;
pub const KHR_VISIBILITY_MASK_EXTENSION_NAME: &[u8] = b"XR_KHR_visibility_mask\0";
#[cfg(windows)]
pub const KHR_win32_convert_performance_counter_time_SPEC_VERSION: u32 = 1u32;
#[cfg(windows)]
pub const KHR_WIN32_CONVERT_PERFORMANCE_COUNTER_TIME_EXTENSION_NAME: &[u8] =
    b"XR_KHR_win32_convert_performance_counter_time\0";
pub const KHR_convert_timespec_time_SPEC_VERSION: u32 = 1u32;
pub const KHR_CONVERT_TIMESPEC_TIME_EXTENSION_NAME: &[u8] = b"XR_KHR_convert_timespec_time\0";
pub const MND_headless_SPEC_VERSION: u32 = 2u32;
pub const MND_HEADLESS_EXTENSION_NAME: &[u8] = b"XR_MND_headless\0";
pub const MSFT_unbounded_reference_space_SPEC_VERSION: u32 = 1u32;
pub const MSFT_UNBOUNDED_REFERENCE_SPACE_EXTENSION_NAME: &[u8] =
    b"XR_MSFT_unbounded_reference_space\0";
pub const MSFT_spatial_anchor_SPEC_VERSION: u32 = 1u32;
pub const MSFT_SPATIAL_ANCHOR_EXTENSION_NAME: &[u8] = b"XR_MSFT_spatial_anchor\0";
pub const MSFT_hand_interaction_SPEC_VERSION: u32 = 1u32;
pub const MSFT_HAND_INTERACTION_EXTENSION_NAME: &[u8] = b"XR_MSFT_hand_interaction\0";
#[cfg(target_os = "android")]
pub const OCULUS_android_session_state_enable_SPEC_VERSION: u32 = 1u32;
#[cfg(target_os = "android")]
pub const OCULUS_ANDROID_SESSION_STATE_ENABLE_EXTENSION_NAME: &[u8] =
    b"XR_OCULUS_android_session_state_enable\0";
pub const VARJO_quad_views_SPEC_VERSION: u32 = 1u32;
pub const VARJO_QUAD_VIEWS_EXTENSION_NAME: &[u8] = b"XR_VARJO_quad_views\0";
#[cfg(feature = "prototypes")]
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

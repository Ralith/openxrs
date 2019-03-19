use crate::support::*;
use crate::*;
use std::fmt;
use std::os::raw::c_void;
pub const MAX_EXTENSION_NAME_SIZE: usize = 128usize;
pub const MAX_API_LAYER_NAME_SIZE: usize = 256usize;
pub const MAX_API_LAYER_DESCRIPTION_SIZE: usize = 256usize;
pub const MAX_SYSTEM_NAME_SIZE: usize = 256usize;
pub const MAX_APPLICATION_NAME_SIZE: usize = 128usize;
pub const MAX_ENGINE_NAME_SIZE: usize = 128usize;
pub const MAX_RUNTIME_NAME_SIZE: usize = 128usize;
pub const MAX_TOUCH_COUNT: usize = 2usize;
pub const MAX_ACTION_SOURCES_COUNT: usize = 8usize;
pub const MAX_PATH_LENGTH: usize = 256usize;
pub const MAX_STRUCTURE_NAME_SIZE: usize = 64usize;
pub const MAX_RESULT_STRING_SIZE: usize = 64usize;
pub const MAX_GRAPHICS_APIS_SUPPORTED: usize = 32usize;
pub const MAX_ACTION_SET_NAME_SIZE: usize = 64usize;
pub const MAX_ACTION_NAME_SIZE: usize = 64usize;
pub const MAX_LOCALIZED_ACTION_SET_NAME_SIZE: usize = 128usize;
pub const MAX_LOCALIZED_ACTION_NAME_SIZE: usize = 128usize;
pub const MIN_COMPOSITION_LAYERS_SUPPORTED: usize = 16usize;
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActionType(i32);
impl ActionType {
    pub const INPUT_BOOLEAN: ActionType = ActionType(1i32);
    pub const INPUT_VECTOR1F: ActionType = ActionType(2i32);
    pub const INPUT_VECTOR2F: ActionType = ActionType(3i32);
    pub const INPUT_POSE: ActionType = ActionType(4i32);
    pub const OUTPUT_VIBRATION: ActionType = ActionType(100i32);
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
            Self::INPUT_BOOLEAN => Some("INPUT_BOOLEAN"),
            Self::INPUT_VECTOR1F => Some("INPUT_VECTOR1F"),
            Self::INPUT_VECTOR2F => Some("INPUT_VECTOR2F"),
            Self::INPUT_POSE => Some("INPUT_POSE"),
            Self::OUTPUT_VIBRATION => Some("OUTPUT_VIBRATION"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Android Thread Types"]
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
#[doc = "eye visibility selector"]
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
#[doc = "Enums to track objects of various types"]
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
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PerfSettingsNotificationLevelEXT(i32);
impl PerfSettingsNotificationLevelEXT {
    #[doc = "Notifies that the sub-domain has reached a level\n                 where no further actions other than currently applied are necessary"]
    pub const NORMAL: PerfSettingsNotificationLevelEXT = PerfSettingsNotificationLevelEXT(0i32);
    #[doc = "Notifies that the sub-domain has reached an early warning level\n                 where the application should start proactive mitigation actions\n                 with the goal to return to the ename:XR_PERF_NOTIF_LEVEL_NORMAL level"]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReferenceSpaceType(i32);
impl ReferenceSpaceType {
    pub const VIEW: ReferenceSpaceType = ReferenceSpaceType(1i32);
    pub const LOCAL: ReferenceSpaceType = ReferenceSpaceType(2i32);
    pub const STAGE: ReferenceSpaceType = ReferenceSpaceType(3i32);
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
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = "Error and return codes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(i32);
impl Result {
    #[doc = "Command completed successfully."]
    pub const SUCCESS: Result = Result(0i32);
    #[doc = "The specified timeout time occurred before the operation could complete."]
    pub const TIMEOUT_EXPIRED: Result = Result(1i32);
    #[doc = "The session has started but cannot be made visible at the moment."]
    pub const SESSION_VISIBILITY_UNAVAILABLE: Result = Result(2i32);
    #[doc = "The session will be lost soon."]
    pub const SESSION_LOSS_PENDING: Result = Result(3i32);
    #[doc = "No event was available."]
    pub const EVENT_UNAVAILABLE: Result = Result(4i32);
    #[doc = "No state (of any type) is available for the provided handle."]
    pub const STATE_UNAVAILABLE: Result = Result(5i32);
    #[doc = "The state of the given type is not available for the provided handle."]
    pub const STATE_TYPE_UNAVAILABLE: Result = Result(6i32);
    #[doc = "The space\'s bounds are not known at the moment."]
    pub const SPACE_BOUNDS_UNAVAILABLE: Result = Result(7i32);
    #[doc = "The session is not in the focused state."]
    pub const SESSION_NOT_FOCUSED: Result = Result(8i32);
    #[doc = "A frame has been discarded from composition."]
    pub const FRAME_DISCARDED: Result = Result(9i32);
    #[doc = "The function usage was invalid in some way."]
    pub const ERROR_VALIDATION_FAILURE: Result = Result(-1i32);
    #[doc = "The runtime failed to handle the function in an unexpected way that is not covered by another error result. "]
    pub const ERROR_RUNTIME_FAILURE: Result = Result(-2i32);
    #[doc = "A memory allocation has failed."]
    pub const ERROR_OUT_OF_MEMORY: Result = Result(-3i32);
    #[doc = "The runtime version is incompatible with the requested or required version."]
    pub const ERROR_RUNTIME_VERSION_INCOMPATIBLE: Result = Result(-4i32);
    #[doc = "The driver is incompatible with the runtime."]
    pub const ERROR_DRIVER_INCOMPATIBLE: Result = Result(-5i32);
    #[doc = "Initialization of object could not be completed."]
    pub const ERROR_INITIALIZATION_FAILED: Result = Result(-6i32);
    #[doc = "The requested function was not found or is otherwise unsupported."]
    pub const ERROR_FUNCTION_UNSUPPORTED: Result = Result(-7i32);
    #[doc = "The requested feature is unavailable."]
    pub const ERROR_FEATURE_UNSUPPORTED: Result = Result(-8i32);
    #[doc = "A requested extension is not supported."]
    pub const ERROR_EXTENSION_NOT_PRESENT: Result = Result(-9i32);
    #[doc = "The runtime supports no more of the requested resource."]
    pub const ERROR_LIMIT_REACHED: Result = Result(-10i32);
    #[doc = "The supplied size was smaller than required."]
    pub const ERROR_SIZE_INSUFFICIENT: Result = Result(-11i32);
    #[doc = "A supplied object was invalid."]
    pub const ERROR_HANDLE_INVALID: Result = Result(-12i32);
    #[doc = "The XrInstance was lost or could not be found. It will need to be destroyed and optionally recreated."]
    pub const ERROR_INSTANCE_LOST: Result = Result(-13i32);
    #[doc = "This session is already running."]
    pub const ERROR_SESSION_RUNNING: Result = Result(-14i32);
    #[doc = "The operation requires this session to be in the running state."]
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
    #[doc = "The layer was NULL or otherwise invalid."]
    pub const ERROR_LAYER_INVALID: Result = Result(-22i32);
    #[doc = "The number of specified layers is greater than the supported number."]
    pub const ERROR_LAYER_LIMIT_EXCEEDED: Result = Result(-23i32);
    #[doc = "The image rect was negatively sized or otherwise invalid."]
    pub const ERROR_SWAPCHAIN_RECT_INVALID: Result = Result(-25i32);
    #[doc = "The image format isn\'t supported by the runtime or platform."]
    pub const ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED: Result = Result(-26i32);
    #[doc = "The API used to retrieve an action\'s state does not match the action\'s type."]
    pub const ERROR_ACTION_TYPE_MISMATCH: Result = Result(-27i32);
    #[doc = "The specified reference space is not supported by the runtime or system."]
    pub const ERROR_REFERENCE_SPACE_UNSUPPORTED: Result = Result(-31i32);
    #[doc = "The file could not be accessed."]
    pub const ERROR_FILE_ACCESS_ERROR: Result = Result(-32i32);
    #[doc = "The file\'s contents were invalid."]
    pub const ERROR_FILE_CONTENTS_INVALID: Result = Result(-33i32);
    #[doc = "The specified form factor isn\'t supported by the current runtime or platform."]
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
    #[doc = "The application specified bindings for an input form factor it had already suggested bindings for."]
    pub const ERROR_BINDINGS_DUPLICATED: Result = Result(-43i32);
    #[doc = "The name provided was a duplicate of an already-existing resource."]
    pub const ERROR_NAME_DUPLICATED: Result = Result(-44i32);
    #[doc = "The name provided was invalid."]
    pub const ERROR_NAME_INVALID: Result = Result(-45i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed as thread id is invalid."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR: Result = Result(-1000003000i32);
    #[doc = "xrSetAndroidApplicationThreadKHR failed setting the thread attributes/priority."]
    pub const ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR: Result = Result(-1000003001i32);
    pub const ERROR_DEBUG_UTILS_MESSENGER_INVALID_EXT: Result = Result(-1000019000i32);
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
            Self::SESSION_VISIBILITY_UNAVAILABLE => Some("SESSION_VISIBILITY_UNAVAILABLE"),
            Self::SESSION_LOSS_PENDING => Some("SESSION_LOSS_PENDING"),
            Self::EVENT_UNAVAILABLE => Some("EVENT_UNAVAILABLE"),
            Self::STATE_UNAVAILABLE => Some("STATE_UNAVAILABLE"),
            Self::STATE_TYPE_UNAVAILABLE => Some("STATE_TYPE_UNAVAILABLE"),
            Self::SPACE_BOUNDS_UNAVAILABLE => Some("SPACE_BOUNDS_UNAVAILABLE"),
            Self::SESSION_NOT_FOCUSED => Some("SESSION_NOT_FOCUSED"),
            Self::FRAME_DISCARDED => Some("FRAME_DISCARDED"),
            Self::ERROR_VALIDATION_FAILURE => Some("ERROR_VALIDATION_FAILURE"),
            Self::ERROR_RUNTIME_FAILURE => Some("ERROR_RUNTIME_FAILURE"),
            Self::ERROR_OUT_OF_MEMORY => Some("ERROR_OUT_OF_MEMORY"),
            Self::ERROR_RUNTIME_VERSION_INCOMPATIBLE => Some("ERROR_RUNTIME_VERSION_INCOMPATIBLE"),
            Self::ERROR_DRIVER_INCOMPATIBLE => Some("ERROR_DRIVER_INCOMPATIBLE"),
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
            Self::ERROR_LAYER_INVALID => Some("ERROR_LAYER_INVALID"),
            Self::ERROR_LAYER_LIMIT_EXCEEDED => Some("ERROR_LAYER_LIMIT_EXCEEDED"),
            Self::ERROR_SWAPCHAIN_RECT_INVALID => Some("ERROR_SWAPCHAIN_RECT_INVALID"),
            Self::ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some("ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED"),
            Self::ERROR_ACTION_TYPE_MISMATCH => Some("ERROR_ACTION_TYPE_MISMATCH"),
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
            Self::ERROR_BINDINGS_DUPLICATED => Some("ERROR_BINDINGS_DUPLICATED"),
            Self::ERROR_NAME_DUPLICATED => Some("ERROR_NAME_DUPLICATED"),
            Self::ERROR_NAME_INVALID => Some("ERROR_NAME_INVALID"),
            Self::ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR")
            }
            Self::ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => {
                Some("ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR")
            }
            Self::ERROR_DEBUG_UTILS_MESSENGER_INVALID_EXT => {
                Some("ERROR_DEBUG_UTILS_MESSENGER_INVALID_EXT")
            }
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let reason = match * self { Self :: SUCCESS => Some ( "command completed successfully" ) , Self :: TIMEOUT_EXPIRED => Some ( "the specified timeout time occurred before the operation could complete" ) , Self :: SESSION_VISIBILITY_UNAVAILABLE => Some ( "the session has started but cannot be made visible at the moment" ) , Self :: SESSION_LOSS_PENDING => Some ( "the session will be lost soon" ) , Self :: EVENT_UNAVAILABLE => Some ( "no event was available" ) , Self :: STATE_UNAVAILABLE => Some ( "no state (of any type) is available for the provided handle" ) , Self :: STATE_TYPE_UNAVAILABLE => Some ( "the state of the given type is not available for the provided handle" ) , Self :: SPACE_BOUNDS_UNAVAILABLE => Some ( "the space\'s bounds are not known at the moment" ) , Self :: SESSION_NOT_FOCUSED => Some ( "the session is not in the focused state" ) , Self :: FRAME_DISCARDED => Some ( "a frame has been discarded from composition" ) , Self :: ERROR_VALIDATION_FAILURE => Some ( "the function usage was invalid in some way" ) , Self :: ERROR_RUNTIME_FAILURE => Some ( "the runtime failed to handle the function in an unexpected way that is not covered by another error result" ) , Self :: ERROR_OUT_OF_MEMORY => Some ( "a memory allocation has failed" ) , Self :: ERROR_RUNTIME_VERSION_INCOMPATIBLE => Some ( "the runtime version is incompatible with the requested or required version" ) , Self :: ERROR_DRIVER_INCOMPATIBLE => Some ( "the driver is incompatible with the runtime" ) , Self :: ERROR_INITIALIZATION_FAILED => Some ( "initialization of object could not be completed" ) , Self :: ERROR_FUNCTION_UNSUPPORTED => Some ( "the requested function was not found or is otherwise unsupported" ) , Self :: ERROR_FEATURE_UNSUPPORTED => Some ( "the requested feature is unavailable" ) , Self :: ERROR_EXTENSION_NOT_PRESENT => Some ( "a requested extension is not supported" ) , Self :: ERROR_LIMIT_REACHED => Some ( "the runtime supports no more of the requested resource" ) , Self :: ERROR_SIZE_INSUFFICIENT => Some ( "the supplied size was smaller than required" ) , Self :: ERROR_HANDLE_INVALID => Some ( "a supplied object was invalid" ) , Self :: ERROR_INSTANCE_LOST => Some ( "the xrinstance was lost or could not be found. it will need to be destroyed and optionally recreated" ) , Self :: ERROR_SESSION_RUNNING => Some ( "this session is already running" ) , Self :: ERROR_SESSION_NOT_RUNNING => Some ( "the operation requires this session to be in the running state" ) , Self :: ERROR_SESSION_LOST => Some ( "the xrsession was lost. it will need to be destroyed and optionally recreated" ) , Self :: ERROR_SYSTEM_INVALID => Some ( "the provided xrsystemid was invalid" ) , Self :: ERROR_PATH_INVALID => Some ( "the provided xrpath was not valid" ) , Self :: ERROR_PATH_COUNT_EXCEEDED => Some ( "the maximum number of supported semantic paths has been reached" ) , Self :: ERROR_PATH_FORMAT_INVALID => Some ( "the semantic path character format is invalid" ) , Self :: ERROR_LAYER_INVALID => Some ( "the layer was null or otherwise invalid" ) , Self :: ERROR_LAYER_LIMIT_EXCEEDED => Some ( "the number of specified layers is greater than the supported number" ) , Self :: ERROR_SWAPCHAIN_RECT_INVALID => Some ( "the image rect was negatively sized or otherwise invalid" ) , Self :: ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some ( "the image format isn\'t supported by the runtime or platform" ) , Self :: ERROR_ACTION_TYPE_MISMATCH => Some ( "the api used to retrieve an action\'s state does not match the action\'s type" ) , Self :: ERROR_REFERENCE_SPACE_UNSUPPORTED => Some ( "the specified reference space is not supported by the runtime or system" ) , Self :: ERROR_FILE_ACCESS_ERROR => Some ( "the file could not be accessed" ) , Self :: ERROR_FILE_CONTENTS_INVALID => Some ( "the file\'s contents were invalid" ) , Self :: ERROR_FORM_FACTOR_UNSUPPORTED => Some ( "the specified form factor isn\'t supported by the current runtime or platform" ) , Self :: ERROR_FORM_FACTOR_UNAVAILABLE => Some ( "the specified form factor is supported, but the device is currently not available, e.g. not plugged in or powered off" ) , Self :: ERROR_API_LAYER_NOT_PRESENT => Some ( "a requested api layer is not present or could not be loaded" ) , Self :: ERROR_CALL_ORDER_INVALID => Some ( "the call was made without having made a previously required call" ) , Self :: ERROR_GRAPHICS_DEVICE_INVALID => Some ( "the given graphics device is not in a valid state. the graphics device could be lost or initialized without meeting graphics requirements" ) , Self :: ERROR_POSE_INVALID => Some ( "the supplied pose was invalid with respect to the requirements" ) , Self :: ERROR_INDEX_OUT_OF_RANGE => Some ( "the supplied index was outside the range of valid indices" ) , Self :: ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => Some ( "the specified view configuration type is not supported by the runtime or platform" ) , Self :: ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => Some ( "the specified environment blend mode is not supported by the runtime or platform" ) , Self :: ERROR_BINDINGS_DUPLICATED => Some ( "the application specified bindings for an input form factor it had already suggested bindings for" ) , Self :: ERROR_NAME_DUPLICATED => Some ( "the name provided was a duplicate of an already-existing resource" ) , Self :: ERROR_NAME_INVALID => Some ( "the name provided was invalid" ) , Self :: ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => Some ( "xrsetandroidapplicationthreadkhr failed as thread id is invalid" ) , Self :: ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => Some ( "xrsetandroidapplicationthreadkhr failed setting the thread attributes/priority" ) , Self :: ERROR_DEBUG_UTILS_MESSENGER_INVALID_EXT => Some ( "ERROR_DEBUG_UTILS_MESSENGER_INVALID_EXT" ) , _ => None , } ;
        if let Some(reason) = reason {
            fmt.pad(reason)
        } else {
            write!(fmt, "unknown error (code {})", self.0)
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SessionState(i32);
impl SessionState {
    pub const UNKNOWN: SessionState = SessionState(0i32);
    pub const IDLE: SessionState = SessionState(1i32);
    pub const READY: SessionState = SessionState(2i32);
    pub const RUNNING: SessionState = SessionState(3i32);
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
            Self::RUNNING => Some("RUNNING"),
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
#[doc = "Structure type enumerant"]
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
    pub const ACTION_STATE_VECTOR1F: StructureType = StructureType(24i32);
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
    pub const SPACE_RELATION: StructureType = StructureType(39i32);
    pub const EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING: StructureType = StructureType(40i32);
    pub const VIEW_CONFIGURATION_VIEW: StructureType = StructureType(41i32);
    pub const FRAME_STATE: StructureType = StructureType(44i32);
    pub const VIEW_CONFIGURATION_PROPERTIES: StructureType = StructureType(45i32);
    pub const FRAME_BEGIN_INFO: StructureType = StructureType(46i32);
    pub const COMPOSITION_LAYER_PROJECTION_VIEW: StructureType = StructureType(48i32);
    pub const EVENT_DATA_EVENTS_LOST: StructureType = StructureType(49i32);
    pub const INTERACTION_PROFILE_SUGGESTED_BINDING: StructureType = StructureType(51i32);
    pub const EVENT_DATA_INTERACTION_PROFILE_CHANGED: StructureType = StructureType(52i32);
    pub const INTERACTION_PROFILE_INFO: StructureType = StructureType(53i32);
    pub const ACTIVE_ACTION_SET: StructureType = StructureType(54i32);
    pub const SWAPCHAIN_IMAGE_ACQUIRE_INFO: StructureType = StructureType(55i32);
    pub const SWAPCHAIN_IMAGE_WAIT_INFO: StructureType = StructureType(56i32);
    pub const SWAPCHAIN_IMAGE_RELEASE_INFO: StructureType = StructureType(57i32);
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
    pub const GRAPHICS_BINDING_D3D10_KHR: StructureType = StructureType(1000026000i32);
    pub const SWAPCHAIN_IMAGE_D3D10_KHR: StructureType = StructureType(1000026001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D10_KHR: StructureType = StructureType(1000026002i32);
    pub const GRAPHICS_BINDING_D3D11_KHR: StructureType = StructureType(1000027000i32);
    pub const SWAPCHAIN_IMAGE_D3D11_KHR: StructureType = StructureType(1000027001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D11_KHR: StructureType = StructureType(1000027002i32);
    pub const GRAPHICS_BINDING_D3D12_KHR: StructureType = StructureType(1000028000i32);
    pub const SWAPCHAIN_IMAGE_D3D12_KHR: StructureType = StructureType(1000028001i32);
    pub const GRAPHICS_REQUIREMENTS_D3D12_KHR: StructureType = StructureType(1000028002i32);
    pub const VISIBILITY_MASK_KHR: StructureType = StructureType(1000031000i32);
    pub const EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR: StructureType = StructureType(1000031001i32);
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
            Self::ACTION_STATE_VECTOR1F => Some("ACTION_STATE_VECTOR1F"),
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
            Self::SPACE_RELATION => Some("SPACE_RELATION"),
            Self::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
                Some("EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING")
            }
            Self::VIEW_CONFIGURATION_VIEW => Some("VIEW_CONFIGURATION_VIEW"),
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
            Self::INTERACTION_PROFILE_INFO => Some("INTERACTION_PROFILE_INFO"),
            Self::ACTIVE_ACTION_SET => Some("ACTIVE_ACTION_SET"),
            Self::SWAPCHAIN_IMAGE_ACQUIRE_INFO => Some("SWAPCHAIN_IMAGE_ACQUIRE_INFO"),
            Self::SWAPCHAIN_IMAGE_WAIT_INFO => Some("SWAPCHAIN_IMAGE_WAIT_INFO"),
            Self::SWAPCHAIN_IMAGE_RELEASE_INFO => Some("SWAPCHAIN_IMAGE_RELEASE_INFO"),
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
            Self::GRAPHICS_BINDING_D3D10_KHR => Some("GRAPHICS_BINDING_D3D10_KHR"),
            Self::SWAPCHAIN_IMAGE_D3D10_KHR => Some("SWAPCHAIN_IMAGE_D3D10_KHR"),
            Self::GRAPHICS_REQUIREMENTS_D3D10_KHR => Some("GRAPHICS_REQUIREMENTS_D3D10_KHR"),
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
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ViewConfigurationType(i32);
impl ViewConfigurationType {
    pub const PRIMARY_MONO: ViewConfigurationType = ViewConfigurationType(1i32);
    pub const PRIMARY_STEREO: ViewConfigurationType = ViewConfigurationType(2i32);
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
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}
#[doc = ""]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VisibilityMaskTypeKHR(i32);
impl VisibilityMaskTypeKHR {
    #[doc = "exclusive mesh; indicates that which the viewer cannot see."]
    pub const HIDDEN_TRIANGLE_MESH: VisibilityMaskTypeKHR = VisibilityMaskTypeKHR(1i32);
    #[doc = "incluse mesh; indicates strictly that which the viewer can see."]
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
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CompositionLayerFlags(i32);
impl CompositionLayerFlags {
    #[doc = "Enables chromatic aberration correction when not done by default."]
    pub const CORRECT_CHROMATIC_ABERRATION: CompositionLayerFlags =
        CompositionLayerFlags(1 << 0i32);
    #[doc = "Enables the layer texture alpha channel."]
    pub const BLEND_TEXTURE_SOURCE_ALPHA: CompositionLayerFlags = CompositionLayerFlags(1 << 1i32);
}
bitmask!(CompositionLayerFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageSeverityFlagsEXT(i32);
impl DebugUtilsMessageSeverityFlagsEXT {
    #[doc = "Most verbose output severity, typically used for debugging."]
    pub const VERBOSE: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 0i32);
    #[doc = "General info message"]
    pub const INFO: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 4i32);
    #[doc = "Indicates the item may be the cause of issues."]
    pub const WARNING: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 8i32);
    #[doc = "Indicates that the item is definitely related to erroneous behavior."]
    pub const ERROR: DebugUtilsMessageSeverityFlagsEXT =
        DebugUtilsMessageSeverityFlagsEXT(1 << 12i32);
}
bitmask!(DebugUtilsMessageSeverityFlagsEXT);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessageTypeFlagsEXT(i32);
impl DebugUtilsMessageTypeFlagsEXT {
    #[doc = "Indicates this is a general message"]
    pub const GENERAL: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 0i32);
    #[doc = "Indicates the message is related to a validation message"]
    pub const VALIDATION: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 1i32);
    #[doc = "Indicates the message is related to a potential performance situation"]
    pub const PERFORMANCE: DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagsEXT(1 << 2i32);
}
bitmask!(DebugUtilsMessageTypeFlagsEXT);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InputSourceLocalizedNameFlags(i32);
impl InputSourceLocalizedNameFlags {
    #[doc = "Asks for the part of the string which indicates the top level user path the source represents"]
    pub const USER_PATH: InputSourceLocalizedNameFlags = InputSourceLocalizedNameFlags(1 << 0i32);
    #[doc = "Asks for the part of the string which represents the interaction profile of the source"]
    pub const INTERACTION_PROFILE: InputSourceLocalizedNameFlags =
        InputSourceLocalizedNameFlags(1 << 1i32);
    #[doc = "Asks for the part of the string which represents the component on the device which needs to be interacted with"]
    pub const COMPONENT: InputSourceLocalizedNameFlags = InputSourceLocalizedNameFlags(1 << 2i32);
}
bitmask!(InputSourceLocalizedNameFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InstanceCreateFlags(i32);
impl InstanceCreateFlags {}
bitmask!(InstanceCreateFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SessionCreateFlags(i32);
impl SessionCreateFlags {}
bitmask!(SessionCreateFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SpaceRelationFlags(i32);
impl SpaceRelationFlags {
    #[doc = "Indicates validity of orientation member"]
    pub const ORIENTATION_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 0i32);
    #[doc = "Indicates validity of position member"]
    pub const POSITION_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 1i32);
    #[doc = "Indicates validity of linearVelocity member"]
    pub const LINEAR_VELOCITY_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 2i32);
    #[doc = "Indicates validity of angularVelocity member"]
    pub const ANGULAR_VELOCITY_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 3i32);
    #[doc = "Indicates validity of linearAcceleration member"]
    pub const LINEAR_ACCELERATION_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 4i32);
    #[doc = "Indicates validity of angularAcceleration member"]
    pub const ANGULAR_ACCELERATION_VALID: SpaceRelationFlags = SpaceRelationFlags(1 << 5i32);
    #[doc = "Indicates whether pose member contains an actively tracked orientation"]
    pub const ORIENTATION_TRACKED: SpaceRelationFlags = SpaceRelationFlags(1 << 6i32);
    #[doc = "Indicates whether pose member contains an actively tracked position"]
    pub const POSITION_TRACKED: SpaceRelationFlags = SpaceRelationFlags(1 << 7i32);
}
bitmask!(SpaceRelationFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainCreateFlags(i32);
impl SwapchainCreateFlags {
    #[doc = "Content will be protected from CPU access"]
    pub const PROTECTED_CONTENT: SwapchainCreateFlags = SwapchainCreateFlags(1 << 0i32);
    #[doc = "Only one image will be acquired from this swapchain over its lifetime"]
    pub const STATIC_IMAGE: SwapchainCreateFlags = SwapchainCreateFlags(1 << 1i32);
}
bitmask!(SwapchainCreateFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SwapchainUsageFlags(i32);
impl SwapchainUsageFlags {
    #[doc = "Specifies that the image can: be a color rendering target."]
    pub const COLOR_ATTACHMENT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 0i32);
    #[doc = "Specifies that the image can: be a depth/stencil rendering target."]
    pub const DEPTH_STENCIL_ATTACHMENT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 1i32);
    #[doc = "Specifies that the image can: be used as data/compute."]
    pub const UNORDERED_ACCESS: SwapchainUsageFlags = SwapchainUsageFlags(1 << 2i32);
    #[doc = "Specifies that the image can: be used as the source of a transfer command."]
    pub const TRANSFER_SRC: SwapchainUsageFlags = SwapchainUsageFlags(1 << 3i32);
    #[doc = "Specifies that the image can: be used as the destination of a transfer command."]
    pub const TRANSFER_DST: SwapchainUsageFlags = SwapchainUsageFlags(1 << 4i32);
    #[doc = "Specifies that the image can: be sampled by a shader."]
    pub const SAMPLED: SwapchainUsageFlags = SwapchainUsageFlags(1 << 5i32);
    #[doc = "Specifies that the image can: be reinterpreted as another image format."]
    pub const MUTABLE_FORMAT: SwapchainUsageFlags = SwapchainUsageFlags(1 << 6i32);
}
bitmask!(SwapchainUsageFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ViewStateFlags(i32);
impl ViewStateFlags {
    #[doc = "Indicates validity of all XrView orientations"]
    pub const ORIENTATION_VALID: ViewStateFlags = ViewStateFlags(1 << 0i32);
    #[doc = "Indicates validity of all XrView positions"]
    pub const POSITION_VALID: ViewStateFlags = ViewStateFlags(1 << 1i32);
    #[doc = "Indicates whether all XrView orientations are actively tracked"]
    pub const ORIENTATION_TRACKED: ViewStateFlags = ViewStateFlags(1 << 2i32);
    #[doc = "Indicates whether all XrView positions are actively tracked"]
    pub const POSITION_TRACKED: ViewStateFlags = ViewStateFlags(1 << 3i32);
}
bitmask!(ViewStateFlags);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instance(u64);
impl Instance {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Session(u64);
impl Session {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Swapchain(u64);
impl Swapchain {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Space(u64);
impl Space {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ActionSet(u64);
impl ActionSet {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Action(u64);
impl Action {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DebugUtilsMessengerEXT(u64);
impl DebugUtilsMessengerEXT {
    #[inline]
    pub fn from_raw(x: u64) -> Self {
        Self(x)
    }
    #[inline]
    pub fn into_raw(self) -> u64 {
        self.0
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_name: [u8; MAX_ACTION_NAME_SIZE],
    pub action_type: ActionType,
    pub count_subaction_paths: u32,
    pub subaction_paths: *const Path,
    pub localized_action_name: [u8; MAX_LOCALIZED_ACTION_NAME_SIZE],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionSetCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_set_name: [u8; MAX_ACTION_SET_NAME_SIZE],
    pub localized_action_set_name: [u8; MAX_LOCALIZED_ACTION_SET_NAME_SIZE],
    pub priority: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionSpaceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub subaction_path: Path,
    pub pose_in_action_space: Posef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionStateBoolean {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: Bool32,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionStatePose {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub is_active: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionStateVector1f {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: f32,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionStateVector2f {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub current_state: Vector2f,
    pub changed_since_last_sync: Bool32,
    pub last_change_time: Time,
    pub is_active: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActionSuggestedBinding {
    pub action: Action,
    pub binding: Path,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ActiveActionSet {
    pub ty: StructureType,
    pub next: *const c_void,
    pub action_set: ActionSet,
    pub subaction_path: Path,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApiLayerProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub layer_name: [u8; MAX_API_LAYER_NAME_SIZE],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [u8; MAX_API_LAYER_DESCRIPTION_SIZE],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub application_name: [u8; MAX_APPLICATION_NAME_SIZE],
    pub application_version: u32,
    pub engine_name: [u8; MAX_ENGINE_NAME_SIZE],
    pub engine_version: u32,
    pub api_version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseInStructure {
    pub ty: StructureType,
    pub next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseOutStructure {
    pub ty: StructureType,
    pub next: *mut BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompositionLayerBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompositionLayerColorModulationInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub color_scale: Color4f,
    pub color_offset: Color4f,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_composition_layer_cube"]
pub struct CompositionLayerCubeKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub swapchain: Swapchain,
    pub image_array_index: u32,
    pub orientation: Quaternionf,
    pub offset: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_composition_layer_cylinder"]
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
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_composition_layer_depth"]
pub struct CompositionLayerDepthInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub sub_image: SwapchainSubImage,
    pub min_depth: f32,
    pub max_depth: f32,
    pub near_z: f32,
    pub far_z: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_composition_layer_equirect"]
pub struct CompositionLayerEquirectKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub offset: Vector3f,
    pub scale: Vector2f,
    pub bias: Vector2f,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompositionLayerProjection {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub view_count: u32,
    pub views: *const CompositionLayerProjectionView,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompositionLayerProjectionView {
    pub ty: StructureType,
    pub next: *const c_void,
    pub pose: Posef,
    pub fov: Fovf,
    pub sub_image: SwapchainSubImage,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompositionLayerQuad {
    pub ty: StructureType,
    pub next: *const c_void,
    pub layer_flags: CompositionLayerFlags,
    pub space: Space,
    pub eye_visibility: EyeVisibility,
    pub sub_image: SwapchainSubImage,
    pub pose: Posef,
    pub size: Vector2f,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_EXT_debug_utils"]
pub struct DebugUtilsLabelEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub label_name: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_EXT_debug_utils"]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub message_id: *const u8,
    pub function_name: *const u8,
    pub message: *const u8,
    pub object_count: u32,
    pub objects: *mut DebugUtilsObjectNameInfoEXT,
    pub session_label_count: u32,
    pub session_labels: *mut DebugUtilsLabelEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_EXT_debug_utils"]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub message_severities: DebugUtilsMessageSeverityFlagsEXT,
    pub message_types: DebugUtilsMessageTypeFlagsEXT,
    pub user_callback: pfn::DebugUtilsMessengerCallbackEXT,
    pub user_data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_EXT_debug_utils"]
pub struct DebugUtilsObjectNameInfoEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub object_name: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataBuffer {
    pub ty: StructureType,
    pub next: *const c_void,
    pub varying: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataEventsLost {
    pub ty: StructureType,
    pub next: *const c_void,
    pub lost_event_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataInstanceLossPending {
    pub ty: StructureType,
    pub next: *const c_void,
    pub loss_time: Time,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataInteractionProfileChanged {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_EXT_performance_settings"]
pub struct EventDataPerfSettingsEXT {
    pub ty: StructureType,
    pub next: *const c_void,
    pub domain: PerfSettingsDomainEXT,
    pub sub_domain: PerfSettingsSubDomainEXT,
    pub from_level: PerfSettingsNotificationLevelEXT,
    pub to_level: PerfSettingsNotificationLevelEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataReferenceSpaceChangePending {
    pub ty: StructureType,
    pub next: *const c_void,
    pub reference_space_type: ReferenceSpaceType,
    pub change_time: Time,
    pub pose_valid: Bool32,
    pub pose_in_previous_space: Posef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventDataSessionStateChanged {
    pub ty: StructureType,
    pub next: *const c_void,
    pub session: Session,
    pub state: SessionState,
    pub time: Time,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_visibility_mask"]
pub struct EventDataVisibilityMaskChangedKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub view_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub extension_name: [u8; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2Df {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2Di {
    pub width: i32,
    pub height: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Fovf {
    pub angle_left: f32,
    pub angle_right: f32,
    pub angle_up: f32,
    pub angle_down: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameEndInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display_time: Time,
    pub environment_blend_mode: EnvironmentBlendMode,
    pub layer_count: u32,
    pub layers: *const *const CompositionLayerBaseHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameState {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub predicted_display_time: Time,
    pub predicted_display_period: Duration,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D10_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsBindingD3D10KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub device: *mut ID3D10Device,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D11_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsBindingD3D11KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub device: *mut ID3D11Device,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D12_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsBindingD3D12KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub device: *mut ID3D12Device,
    pub queue: *mut ID3D12CommandQueue,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_es_enable"]
#[cfg(all(target_os = "android", feature = "opengles"))]
pub struct GraphicsBindingOpenGLESAndroidKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display: EGLDisplay,
    pub config: EGLConfig,
    pub context: EGLContext,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(all(feature = "opengl", feature = "wayland"))]
pub struct GraphicsBindingOpenGLWaylandKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display: *mut wl_display,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(all(target_os = "windows", feature = "opengl"))]
pub struct GraphicsBindingOpenGLWin32KHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub h_dc: HDC,
    pub h_glrc: HGLRC,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(all(feature = "opengl", feature = "xcb"))]
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
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(all(feature = "opengl", feature = "x11"))]
pub struct GraphicsBindingOpenGLXlibKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub x_display: *mut Display,
    pub visualid: u32,
    pub glx_fb_config: GLXFBConfig,
    pub glx_drawable: GLXDrawable,
    pub glx_context: GLXContext,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_vulkan_enable"]
#[cfg(feature = "ash")]
pub struct GraphicsBindingVulkanKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub instance: vk::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D10_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsRequirementsD3D10KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub adapter_luid: LUID,
    pub min_feature_level: D3D10_FEATURE_LEVEL1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D11_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsRequirementsD3D11KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub adapter_luid: LUID,
    pub min_feature_level: D3D_FEATURE_LEVEL,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D12_enable"]
#[cfg(feature = "d3d")]
pub struct GraphicsRequirementsD3D12KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub adapter_luid: LUID,
    pub min_feature_level: D3D_FEATURE_LEVEL,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_es_enable"]
#[cfg(feature = "opengles")]
pub struct GraphicsRequirementsOpenGLESKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: u32,
    pub max_api_version_supported: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(feature = "opengl")]
pub struct GraphicsRequirementsOpenGLKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: u32,
    pub max_api_version_supported: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_vulkan_enable"]
#[cfg(feature = "ash")]
pub struct GraphicsRequirementsVulkanKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub min_api_version_supported: u32,
    pub max_api_version_supported: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HapticBaseHeader {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HapticVibration {
    pub ty: StructureType,
    pub next: *const c_void,
    pub duration: Duration,
    pub frequency: f32,
    pub amplitude: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: InstanceCreateFlags,
    pub application_info: ApplicationInfo,
    pub enabled_api_layer_count: u32,
    pub enabled_api_layer_names: *const *const u8,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_android_create_instance"]
#[cfg(target_os = "android")]
pub struct InstanceCreateInfoAndroidKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub application_vm: *mut c_void,
    pub application_activity: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub runtime_version: u32,
    pub runtime_name: [u8; MAX_RUNTIME_NAME_SIZE],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InteractionProfileInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub interaction_profile: Path,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InteractionProfileSuggestedBinding {
    pub ty: StructureType,
    pub next: *const c_void,
    pub interaction_profile: Path,
    pub count_suggested_bindings: u32,
    pub suggested_bindings: *const ActionSuggestedBinding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset2Df {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset2Di {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Posef {
    pub orientation: Quaternionf,
    pub position: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Quaternionf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect2Df {
    pub offset: Offset2Df,
    pub extent: Extent2Df,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect2Di {
    pub offset: Offset2Di,
    pub extent: Extent2Di,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReferenceSpaceCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub reference_space_type: ReferenceSpaceType,
    pub pose_in_reference_space: Posef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SessionBeginInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub primary_view_configuration_type: ViewConfigurationType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SessionCreateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub create_flags: SessionCreateFlags,
    pub system_id: SystemId,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpaceRelation {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub relation_flags: SpaceRelationFlags,
    pub time: Time,
    pub pose: Posef,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
    pub linear_acceleration: Vector3f,
    pub angular_acceleration: Vector3f,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainImageAcquireInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainImageBaseHeader {
    pub ty: StructureType,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D10_enable"]
#[cfg(feature = "d3d")]
pub struct SwapchainImageD3D10KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture: *mut ID3D10Texture2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D11_enable"]
#[cfg(feature = "d3d")]
pub struct SwapchainImageD3D11KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture: *mut ID3D11Texture2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_D3D12_enable"]
#[cfg(feature = "d3d")]
pub struct SwapchainImageD3D12KHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub texture: *mut ID3D12Resource,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_es_enable"]
#[cfg(feature = "opengles")]
pub struct SwapchainImageOpenGLESKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_opengl_enable"]
#[cfg(feature = "opengl")]
pub struct SwapchainImageOpenGLKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainImageReleaseInfo {
    pub ty: StructureType,
    pub next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_vulkan_enable"]
#[cfg(feature = "ash")]
pub struct SwapchainImageVulkanKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub image: vk::Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainImageWaitInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub timeout: Duration,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainSubImage {
    pub swapchain: Swapchain,
    pub image_rect: Rect2Di,
    pub image_array_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemGetInfo {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub form_factor: FormFactor,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemGraphicsProperties {
    pub max_swapchain_image_height: u32,
    pub max_swapchain_image_width: u32,
    pub max_view_count: u32,
    pub max_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub system_id: SystemId,
    pub vendor_id: u32,
    pub system_name: [u8; MAX_SYSTEM_NAME_SIZE],
    pub graphics_properties: SystemGraphicsProperties,
    pub tracking_properties: SystemTrackingProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemTrackingProperties {
    pub orientation_tracking: Bool32,
    pub position_tracking: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct View {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub pose: Posef,
    pub fov: Fovf,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewConfigurationProperties {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_configuration_type: ViewConfigurationType,
    pub fov_mutable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewLocateInfo {
    pub ty: StructureType,
    pub next: *const c_void,
    pub display_time: Time,
    pub space: Space,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewState {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub view_state_flags: ViewStateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_visibility_mask"]
pub struct VisibilityMaskKHR {
    pub ty: StructureType,
    pub next: *mut c_void,
    pub vertex_count: u32,
    pub vertices: *mut Vector2f,
    pub index_count: u32,
    pub indices: *mut u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[doc = "From XR_KHR_vulkan_swapchain_format_list"]
#[cfg(feature = "ash")]
pub struct VulkanSwapchainFormatListCreateInfoKHR {
    pub ty: StructureType,
    pub next: *const c_void,
    pub view_format_count: u32,
    pub view_formats: *const vk::Format,
}
#[doc = r" Function pointer prototypes"]
pub mod pfn {
    use super::*;
    pub type VoidFunction = Option<unsafe extern "system" fn()>;
    pub type DebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
        DebugUtilsMessageSeverityFlagsEXT,
        DebugUtilsMessageTypeFlagsEXT,
        *const DebugUtilsMessengerCallbackDataEXT,
        *mut c_void,
    ) -> Bool32;
    pub type AcquireSwapchainImage = unsafe extern "C" fn(
        swapchain: Swapchain,
        acquire_info: *const SwapchainImageAcquireInfo,
        index: *mut u32,
    ) -> Result;
    pub type ApplyHapticFeedback = unsafe extern "C" fn(
        haptic_action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        haptic_event: *const HapticBaseHeader,
    ) -> Result;
    pub type BeginFrame =
        unsafe extern "C" fn(session: Session, frame_begin_info: *const FrameBeginInfo) -> Result;
    pub type BeginSession =
        unsafe extern "C" fn(session: Session, begin_info: *const SessionBeginInfo) -> Result;
    #[cfg(feature = "libc")]
    #[doc = "From XR_KHR_convert_timespec_time"]
    pub type ConvertTimeToTimespecTimeKHR = unsafe extern "C" fn(
        instance: Instance,
        time: Time,
        timespec_time: *mut timespec,
    ) -> Result;
    #[cfg(target_os = "windows")]
    #[doc = "From XR_KHR_win32_convert_performance_counter_time"]
    pub type ConvertTimeToWin32PerformanceCounterKHR = unsafe extern "C" fn(
        instance: Instance,
        time: Time,
        performance_counter: *mut LARGE_INTEGER,
    ) -> Result;
    #[cfg(feature = "libc")]
    #[doc = "From XR_KHR_convert_timespec_time"]
    pub type ConvertTimespecTimeToTimeKHR = unsafe extern "C" fn(
        instance: Instance,
        timespec_time: *const timespec,
        time: *mut Time,
    ) -> Result;
    #[cfg(target_os = "windows")]
    #[doc = "From XR_KHR_win32_convert_performance_counter_time"]
    pub type ConvertWin32PerformanceCounterToTimeKHR = unsafe extern "C" fn(
        instance: Instance,
        performance_counter: *const LARGE_INTEGER,
        time: *mut Time,
    ) -> Result;
    pub type CreateAction = unsafe extern "C" fn(
        action_set: ActionSet,
        create_info: *const ActionCreateInfo,
        action: *mut Action,
    ) -> Result;
    pub type CreateActionSet = unsafe extern "C" fn(
        session: Session,
        create_info: *const ActionSetCreateInfo,
        action_set: *mut ActionSet,
    ) -> Result;
    pub type CreateActionSpace = unsafe extern "C" fn(
        action: Action,
        create_info: *const ActionSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type CreateDebugUtilsMessengerEXT = unsafe extern "C" fn(
        instance: Instance,
        create_info: *const DebugUtilsMessengerCreateInfoEXT,
        messenger: *mut DebugUtilsMessengerEXT,
    ) -> Result;
    pub type CreateInstance = unsafe extern "C" fn(
        create_info: *const InstanceCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    pub type CreateReferenceSpace = unsafe extern "C" fn(
        session: Session,
        create_info: *const ReferenceSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    pub type CreateSession = unsafe extern "C" fn(
        instance: Instance,
        create_info: *const SessionCreateInfo,
        session: *mut Session,
    ) -> Result;
    pub type CreateSwapchain = unsafe extern "C" fn(
        session: Session,
        create_info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
    ) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "From XR_KHR_android_surface_swapchain"]
    pub type CreateSwapchainAndroidSurfaceKHR = unsafe extern "C" fn(
        session: Session,
        info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
        surface: *mut jobject,
    ) -> Result;
    pub type DestroyAction = unsafe extern "C" fn(action: Action) -> Result;
    pub type DestroyActionSet = unsafe extern "C" fn(action_set: ActionSet) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type DestroyDebugUtilsMessengerEXT =
        unsafe extern "C" fn(messenger: DebugUtilsMessengerEXT) -> Result;
    pub type DestroyInstance = unsafe extern "C" fn(instance: Instance) -> Result;
    pub type DestroySession = unsafe extern "C" fn(session: Session) -> Result;
    pub type DestroySpace = unsafe extern "C" fn(space: Space) -> Result;
    pub type DestroySwapchain = unsafe extern "C" fn(swapchain: Swapchain) -> Result;
    pub type EndFrame =
        unsafe extern "C" fn(session: Session, frame_end_info: *const FrameEndInfo) -> Result;
    pub type EndSession = unsafe extern "C" fn(session: Session) -> Result;
    pub type EnumerateApiLayerProperties = unsafe extern "C" fn(
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ApiLayerProperties,
    ) -> Result;
    pub type EnumerateEnvironmentBlendModes = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        environment_blend_mode_capacity_input: u32,
        environment_blend_mode_count_output: *mut u32,
        environment_blend_modes: *mut EnvironmentBlendMode,
    ) -> Result;
    pub type EnumerateInstanceExtensionProperties = unsafe extern "C" fn(
        layer_name: *const u8,
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result;
    pub type EnumerateReferenceSpaces = unsafe extern "C" fn(
        session: Session,
        space_capacity_input: u32,
        space_count_output: *mut u32,
        spaces: *mut ReferenceSpaceType,
    ) -> Result;
    pub type EnumerateSwapchainFormats = unsafe extern "C" fn(
        session: Session,
        format_capacity_input: u32,
        format_count_output: *mut u32,
        formats: *mut i64,
    ) -> Result;
    pub type EnumerateSwapchainImages = unsafe extern "C" fn(
        swapchain: Swapchain,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    ) -> Result;
    pub type EnumerateViewConfigurationViews = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut ViewConfigurationView,
    ) -> Result;
    pub type EnumerateViewConfigurations = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type_capacity_input: u32,
        view_configuration_type_count_output: *mut u32,
        view_configuration_types: *mut ViewConfigurationType,
    ) -> Result;
    pub type GetActionStateBoolean = unsafe extern "C" fn(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateBoolean,
    ) -> Result;
    pub type GetActionStatePose = unsafe extern "C" fn(
        action: Action,
        subaction_path: Path,
        data: *mut ActionStatePose,
    ) -> Result;
    pub type GetActionStateVector1f = unsafe extern "C" fn(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateVector1f,
    ) -> Result;
    pub type GetActionStateVector2f = unsafe extern "C" fn(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateVector2f,
    ) -> Result;
    pub type GetBoundSourcesForAction = unsafe extern "C" fn(
        action: Action,
        source_capacity_input: u32,
        source_count_output: *mut u32,
        sources: *mut Path,
    ) -> Result;
    pub type GetCurrentInteractionProfile = unsafe extern "C" fn(
        session: Session,
        top_level_user_path: Path,
        interaction_profile: *mut InteractionProfileInfo,
    ) -> Result;
    #[cfg(feature = "d3d")]
    #[doc = "From XR_KHR_D3D10_enable"]
    pub type GetD3D10GraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D10KHR,
    ) -> Result;
    #[cfg(feature = "d3d")]
    #[doc = "From XR_KHR_D3D11_enable"]
    pub type GetD3D11GraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D11KHR,
    ) -> Result;
    #[cfg(feature = "d3d")]
    #[doc = "From XR_KHR_D3D12_enable"]
    pub type GetD3D12GraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsD3D12KHR,
    ) -> Result;
    pub type GetInputSourceLocalizedName = unsafe extern "C" fn(
        session: Session,
        source: Path,
        which_components: InputSourceLocalizedNameFlags,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    pub type GetInstanceProcAddr = unsafe extern "C" fn(
        instance: Instance,
        name: *const u8,
        function: *mut pfn::VoidFunction,
    ) -> Result;
    pub type GetInstanceProperties = unsafe extern "C" fn(
        instance: Instance,
        instance_properties: *mut InstanceProperties,
    ) -> Result;
    #[cfg(feature = "opengles")]
    #[doc = "From XR_KHR_opengl_es_enable"]
    pub type GetOpenGLESGraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLESKHR,
    ) -> Result;
    #[cfg(feature = "opengl")]
    #[doc = "From XR_KHR_opengl_enable"]
    pub type GetOpenGLGraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsOpenGLKHR,
    ) -> Result;
    pub type GetReferenceSpaceBoundsRect = unsafe extern "C" fn(
        session: Session,
        reference_space_type: ReferenceSpaceType,
        bounds: *mut Extent2Df,
    ) -> Result;
    pub type GetSystem = unsafe extern "C" fn(
        instance: Instance,
        get_info: *const SystemGetInfo,
        system_id: *mut SystemId,
    ) -> Result;
    pub type GetSystemProperties = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        properties: *mut SystemProperties,
    ) -> Result;
    pub type GetViewConfigurationProperties = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        configuration_properties: *mut ViewConfigurationProperties,
    ) -> Result;
    #[doc = "From XR_KHR_visibility_mask"]
    pub type GetVisibilityMaskKHR = unsafe extern "C" fn(
        session: Session,
        view_configuration_type: ViewConfigurationType,
        view_index: u32,
        visibility_mask_type: VisibilityMaskTypeKHR,
        visibility_mask: *mut VisibilityMaskKHR,
    ) -> Result;
    #[cfg(feature = "ash")]
    #[doc = "From XR_KHR_vulkan_enable"]
    pub type GetVulkanDeviceExtensionsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        names_capacity_input: u32,
        names_count_output: *mut u32,
        names_string: *mut u8,
    ) -> Result;
    #[cfg(feature = "ash")]
    #[doc = "From XR_KHR_vulkan_enable"]
    pub type GetVulkanGraphicsDeviceKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        vk_instance: vk::Instance,
        vk_physical_device: *mut vk::PhysicalDevice,
    ) -> Result;
    #[cfg(feature = "ash")]
    #[doc = "From XR_KHR_vulkan_enable"]
    pub type GetVulkanGraphicsRequirementsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        graphics_requirements: *mut GraphicsRequirementsVulkanKHR,
    ) -> Result;
    #[cfg(feature = "ash")]
    #[doc = "From XR_KHR_vulkan_enable"]
    pub type GetVulkanInstanceExtensionsKHR = unsafe extern "C" fn(
        instance: Instance,
        system_id: SystemId,
        names_capacity_input: u32,
        names_count_output: *mut u32,
        names_string: *mut u8,
    ) -> Result;
    pub type LocateSpace = unsafe extern "C" fn(
        space: Space,
        base_space: Space,
        time: Time,
        relation: *mut SpaceRelation,
    ) -> Result;
    pub type LocateViews = unsafe extern "C" fn(
        session: Session,
        view_locate_info: *const ViewLocateInfo,
        view_state: *mut ViewState,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut View,
    ) -> Result;
    pub type PathToString = unsafe extern "C" fn(
        instance: Instance,
        path: Path,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    #[doc = "From XR_EXT_performance_settings"]
    pub type PerfSettingsSetPerformanceLevelEXT = unsafe extern "C" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        level: PerfSettingsLevelEXT,
    ) -> Result;
    pub type PollEvent =
        unsafe extern "C" fn(instance: Instance, event_data: *mut EventDataBuffer) -> Result;
    pub type ReleaseSwapchainImage = unsafe extern "C" fn(
        swapchain: Swapchain,
        release_info: *const SwapchainImageReleaseInfo,
    ) -> Result;
    pub type ResultToString = unsafe extern "C" fn(
        instance: Instance,
        value: Result,
        buffer: [u8; MAX_RESULT_STRING_SIZE],
    ) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type SessionBeginDebugUtilsLabelRegionEXT =
        unsafe extern "C" fn(session: Session, label_info: *const DebugUtilsLabelEXT) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type SessionEndDebugUtilsLabelRegionEXT = unsafe extern "C" fn(session: Session) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type SessionInsertDebugUtilsLabelEXT =
        unsafe extern "C" fn(session: Session, label_info: *const DebugUtilsLabelEXT) -> Result;
    #[cfg(target_os = "android")]
    #[doc = "From XR_KHR_android_thread_settings"]
    pub type SetAndroidApplicationThreadKHR = unsafe extern "C" fn(
        session: Session,
        thread_type: AndroidThreadTypeKHR,
        thread_id: u32,
    ) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type SetDebugUtilsObjectNameEXT = unsafe extern "C" fn(
        instance: Instance,
        name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> Result;
    pub type SetInteractionProfileSuggestedBindings = unsafe extern "C" fn(
        session: Session,
        suggested_bindings: *const InteractionProfileSuggestedBinding,
    ) -> Result;
    pub type StopHapticFeedback = unsafe extern "C" fn(
        haptic_action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
    ) -> Result;
    pub type StringToPath =
        unsafe extern "C" fn(instance: Instance, path_string: *const u8, path: *mut Path) -> Result;
    pub type StructureTypeToString = unsafe extern "C" fn(
        instance: Instance,
        value: StructureType,
        buffer: [u8; MAX_STRUCTURE_NAME_SIZE],
    ) -> Result;
    #[doc = "From XR_EXT_debug_utils"]
    pub type SubmitDebugUtilsMessageEXT = unsafe extern "C" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) -> Result;
    pub type SyncActionData = unsafe extern "C" fn(
        session: Session,
        count_action_sets: u32,
        action_sets: *const ActiveActionSet,
    ) -> Result;
    #[doc = "From XR_EXT_thermal_query"]
    pub type ThermalGetTemperatureTrendEXT = unsafe extern "C" fn(
        session: Session,
        domain: PerfSettingsDomainEXT,
        notification_level: *mut PerfSettingsNotificationLevelEXT,
        temp_headroom: *mut f32,
        temp_slope: *mut f32,
    ) -> Result;
    pub type WaitFrame = unsafe extern "C" fn(
        session: Session,
        frame_wait_info: *const FrameWaitInfo,
        frame_state: *mut FrameState,
    ) -> Result;
    pub type WaitSwapchainImage = unsafe extern "C" fn(
        swapchain: Swapchain,
        wait_info: *const SwapchainImageWaitInfo,
    ) -> Result;
}
#[cfg(feature = "prototypes")]
extern "C" {
    pub fn xrAcquireSwapchainImage(
        swapchain: Swapchain,
        acquire_info: *const SwapchainImageAcquireInfo,
        index: *mut u32,
    ) -> Result;
    pub fn xrApplyHapticFeedback(
        haptic_action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        haptic_event: *const HapticBaseHeader,
    ) -> Result;
    pub fn xrBeginFrame(session: Session, frame_begin_info: *const FrameBeginInfo) -> Result;
    pub fn xrBeginSession(session: Session, begin_info: *const SessionBeginInfo) -> Result;
    pub fn xrCreateAction(
        action_set: ActionSet,
        create_info: *const ActionCreateInfo,
        action: *mut Action,
    ) -> Result;
    pub fn xrCreateActionSet(
        session: Session,
        create_info: *const ActionSetCreateInfo,
        action_set: *mut ActionSet,
    ) -> Result;
    pub fn xrCreateActionSpace(
        action: Action,
        create_info: *const ActionSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    pub fn xrCreateInstance(
        create_info: *const InstanceCreateInfo,
        instance: *mut Instance,
    ) -> Result;
    pub fn xrCreateReferenceSpace(
        session: Session,
        create_info: *const ReferenceSpaceCreateInfo,
        space: *mut Space,
    ) -> Result;
    pub fn xrCreateSession(
        instance: Instance,
        create_info: *const SessionCreateInfo,
        session: *mut Session,
    ) -> Result;
    pub fn xrCreateSwapchain(
        session: Session,
        create_info: *const SwapchainCreateInfo,
        swapchain: *mut Swapchain,
    ) -> Result;
    pub fn xrDestroyAction(action: Action) -> Result;
    pub fn xrDestroyActionSet(action_set: ActionSet) -> Result;
    pub fn xrDestroyInstance(instance: Instance) -> Result;
    pub fn xrDestroySession(session: Session) -> Result;
    pub fn xrDestroySpace(space: Space) -> Result;
    pub fn xrDestroySwapchain(swapchain: Swapchain) -> Result;
    pub fn xrEndFrame(session: Session, frame_end_info: *const FrameEndInfo) -> Result;
    pub fn xrEndSession(session: Session) -> Result;
    pub fn xrEnumerateApiLayerProperties(
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ApiLayerProperties,
    ) -> Result;
    pub fn xrEnumerateEnvironmentBlendModes(
        instance: Instance,
        system_id: SystemId,
        environment_blend_mode_capacity_input: u32,
        environment_blend_mode_count_output: *mut u32,
        environment_blend_modes: *mut EnvironmentBlendMode,
    ) -> Result;
    pub fn xrEnumerateInstanceExtensionProperties(
        layer_name: *const u8,
        property_capacity_input: u32,
        property_count_output: *mut u32,
        properties: *mut ExtensionProperties,
    ) -> Result;
    pub fn xrEnumerateReferenceSpaces(
        session: Session,
        space_capacity_input: u32,
        space_count_output: *mut u32,
        spaces: *mut ReferenceSpaceType,
    ) -> Result;
    pub fn xrEnumerateSwapchainFormats(
        session: Session,
        format_capacity_input: u32,
        format_count_output: *mut u32,
        formats: *mut i64,
    ) -> Result;
    pub fn xrEnumerateSwapchainImages(
        swapchain: Swapchain,
        image_capacity_input: u32,
        image_count_output: *mut u32,
        images: *mut SwapchainImageBaseHeader,
    ) -> Result;
    pub fn xrEnumerateViewConfigurationViews(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut ViewConfigurationView,
    ) -> Result;
    pub fn xrEnumerateViewConfigurations(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type_capacity_input: u32,
        view_configuration_type_count_output: *mut u32,
        view_configuration_types: *mut ViewConfigurationType,
    ) -> Result;
    pub fn xrGetActionStateBoolean(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateBoolean,
    ) -> Result;
    pub fn xrGetActionStatePose(
        action: Action,
        subaction_path: Path,
        data: *mut ActionStatePose,
    ) -> Result;
    pub fn xrGetActionStateVector1f(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateVector1f,
    ) -> Result;
    pub fn xrGetActionStateVector2f(
        action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
        data: *mut ActionStateVector2f,
    ) -> Result;
    pub fn xrGetBoundSourcesForAction(
        action: Action,
        source_capacity_input: u32,
        source_count_output: *mut u32,
        sources: *mut Path,
    ) -> Result;
    pub fn xrGetCurrentInteractionProfile(
        session: Session,
        top_level_user_path: Path,
        interaction_profile: *mut InteractionProfileInfo,
    ) -> Result;
    pub fn xrGetInputSourceLocalizedName(
        session: Session,
        source: Path,
        which_components: InputSourceLocalizedNameFlags,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    pub fn xrGetInstanceProcAddr(
        instance: Instance,
        name: *const u8,
        function: *mut pfn::VoidFunction,
    ) -> Result;
    pub fn xrGetInstanceProperties(
        instance: Instance,
        instance_properties: *mut InstanceProperties,
    ) -> Result;
    pub fn xrGetReferenceSpaceBoundsRect(
        session: Session,
        reference_space_type: ReferenceSpaceType,
        bounds: *mut Extent2Df,
    ) -> Result;
    pub fn xrGetSystem(
        instance: Instance,
        get_info: *const SystemGetInfo,
        system_id: *mut SystemId,
    ) -> Result;
    pub fn xrGetSystemProperties(
        instance: Instance,
        system_id: SystemId,
        properties: *mut SystemProperties,
    ) -> Result;
    pub fn xrGetViewConfigurationProperties(
        instance: Instance,
        system_id: SystemId,
        view_configuration_type: ViewConfigurationType,
        configuration_properties: *mut ViewConfigurationProperties,
    ) -> Result;
    pub fn xrLocateSpace(
        space: Space,
        base_space: Space,
        time: Time,
        relation: *mut SpaceRelation,
    ) -> Result;
    pub fn xrLocateViews(
        session: Session,
        view_locate_info: *const ViewLocateInfo,
        view_state: *mut ViewState,
        view_capacity_input: u32,
        view_count_output: *mut u32,
        views: *mut View,
    ) -> Result;
    pub fn xrPathToString(
        instance: Instance,
        path: Path,
        buffer_capacity_input: u32,
        buffer_count_output: *mut u32,
        buffer: *mut u8,
    ) -> Result;
    pub fn xrPollEvent(instance: Instance, event_data: *mut EventDataBuffer) -> Result;
    pub fn xrReleaseSwapchainImage(
        swapchain: Swapchain,
        release_info: *const SwapchainImageReleaseInfo,
    ) -> Result;
    pub fn xrResultToString(
        instance: Instance,
        value: Result,
        buffer: [u8; MAX_RESULT_STRING_SIZE],
    ) -> Result;
    pub fn xrSetInteractionProfileSuggestedBindings(
        session: Session,
        suggested_bindings: *const InteractionProfileSuggestedBinding,
    ) -> Result;
    pub fn xrStopHapticFeedback(
        haptic_action: Action,
        count_subaction_paths: u32,
        subaction_paths: *const Path,
    ) -> Result;
    pub fn xrStringToPath(instance: Instance, path_string: *const u8, path: *mut Path) -> Result;
    pub fn xrStructureTypeToString(
        instance: Instance,
        value: StructureType,
        buffer: [u8; MAX_STRUCTURE_NAME_SIZE],
    ) -> Result;
    pub fn xrSyncActionData(
        session: Session,
        count_action_sets: u32,
        action_sets: *const ActiveActionSet,
    ) -> Result;
    pub fn xrWaitFrame(
        session: Session,
        frame_wait_info: *const FrameWaitInfo,
        frame_state: *mut FrameState,
    ) -> Result;
    pub fn xrWaitSwapchainImage(
        swapchain: Swapchain,
        wait_info: *const SwapchainImageWaitInfo,
    ) -> Result;
}
#[doc = "Multivendor extensions"]
pub mod ext {
    use super::*;
    pub struct PerformanceSettings {
        pub perf_settings_set_performance_level: pfn::PerfSettingsSetPerformanceLevelEXT,
    }
    impl PerformanceSettings {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_EXT_performance_settings\0";
    }
    pub struct ThermalQuery {
        pub thermal_get_temperature_trend: pfn::ThermalGetTemperatureTrendEXT,
    }
    impl ThermalQuery {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_EXT_thermal_query\0";
    }
    pub struct DebugUtils {
        pub set_debug_utils_object_name: pfn::SetDebugUtilsObjectNameEXT,
        pub create_debug_utils_messenger: pfn::CreateDebugUtilsMessengerEXT,
        pub destroy_debug_utils_messenger: pfn::DestroyDebugUtilsMessengerEXT,
        pub submit_debug_utils_message: pfn::SubmitDebugUtilsMessageEXT,
        pub session_begin_debug_utils_label_region: pfn::SessionBeginDebugUtilsLabelRegionEXT,
        pub session_end_debug_utils_label_region: pfn::SessionEndDebugUtilsLabelRegionEXT,
        pub session_insert_debug_utils_label: pfn::SessionInsertDebugUtilsLabelEXT,
    }
    impl DebugUtils {
        pub const VERSION: u32 = 2u32;
        pub const NAME: &'static [u8] = b"XR_EXT_debug_utils\0";
    }
}
#[doc = "Khronos extensions"]
pub mod khr {
    use super::*;
    #[cfg(target_os = "android")]
    pub struct AndroidThreadSettings {
        pub set_android_application_thread: pfn::SetAndroidApplicationThreadKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidThreadSettings {
        pub const VERSION: u32 = 4u32;
        pub const NAME: &'static [u8] = b"XR_KHR_android_thread_settings\0";
    }
    #[cfg(target_os = "android")]
    pub struct AndroidSurfaceSwapchain {
        pub create_swapchain_android_surface: pfn::CreateSwapchainAndroidSurfaceKHR,
    }
    #[cfg(target_os = "android")]
    impl AndroidSurfaceSwapchain {
        pub const VERSION: u32 = 4u32;
        pub const NAME: &'static [u8] = b"XR_KHR_android_surface_swapchain\0";
    }
    pub struct CompositionLayerCube {}
    impl CompositionLayerCube {
        pub const VERSION: u32 = 8u32;
        pub const NAME: &'static [u8] = b"XR_KHR_composition_layer_cube\0";
    }
    #[cfg(target_os = "android")]
    pub struct AndroidCreateInstance {}
    #[cfg(target_os = "android")]
    impl AndroidCreateInstance {
        pub const VERSION: u32 = 2u32;
        pub const NAME: &'static [u8] = b"XR_KHR_android_create_instance\0";
    }
    pub struct CompositionLayerDepth {}
    impl CompositionLayerDepth {
        pub const VERSION: u32 = 5u32;
        pub const NAME: &'static [u8] = b"XR_KHR_composition_layer_depth\0";
    }
    pub struct Headless {}
    impl Headless {
        pub const VERSION: u32 = 2u32;
        pub const NAME: &'static [u8] = b"XR_KHR_headless\0";
    }
    #[cfg(feature = "ash")]
    pub struct VulkanSwapchainFormatList {}
    #[cfg(feature = "ash")]
    impl VulkanSwapchainFormatList {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_vulkan_swapchain_format_list\0";
    }
    pub struct CompositionLayerCylinder {}
    impl CompositionLayerCylinder {
        pub const VERSION: u32 = 4u32;
        pub const NAME: &'static [u8] = b"XR_KHR_composition_layer_cylinder\0";
    }
    pub struct CompositionLayerEquirect {}
    impl CompositionLayerEquirect {
        pub const VERSION: u32 = 3u32;
        pub const NAME: &'static [u8] = b"XR_KHR_composition_layer_equirect\0";
    }
    #[cfg(feature = "opengl")]
    pub struct OpenglEnable {
        pub get_open_gl_graphics_requirements: pfn::GetOpenGLGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengl")]
    impl OpenglEnable {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_opengl_enable\0";
    }
    #[cfg(feature = "opengles")]
    pub struct OpenglEsEnable {
        pub get_open_gles_graphics_requirements: pfn::GetOpenGLESGraphicsRequirementsKHR,
    }
    #[cfg(feature = "opengles")]
    impl OpenglEsEnable {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_opengl_es_enable\0";
    }
    #[cfg(feature = "ash")]
    pub struct VulkanEnable {
        pub get_vulkan_instance_extensions: pfn::GetVulkanInstanceExtensionsKHR,
        pub get_vulkan_device_extensions: pfn::GetVulkanDeviceExtensionsKHR,
        pub get_vulkan_graphics_device: pfn::GetVulkanGraphicsDeviceKHR,
        pub get_vulkan_graphics_requirements: pfn::GetVulkanGraphicsRequirementsKHR,
    }
    #[cfg(feature = "ash")]
    impl VulkanEnable {
        pub const VERSION: u32 = 6u32;
        pub const NAME: &'static [u8] = b"XR_KHR_vulkan_enable\0";
    }
    #[cfg(feature = "d3d")]
    pub struct D3d10Enable {
        pub get_d3d10_graphics_requirements: pfn::GetD3D10GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d10Enable {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_D3D10_enable\0";
    }
    #[cfg(feature = "d3d")]
    pub struct D3d11Enable {
        pub get_d3d11_graphics_requirements: pfn::GetD3D11GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d11Enable {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_D3D11_enable\0";
    }
    #[cfg(feature = "d3d")]
    pub struct D3d12Enable {
        pub get_d3d12_graphics_requirements: pfn::GetD3D12GraphicsRequirementsKHR,
    }
    #[cfg(feature = "d3d")]
    impl D3d12Enable {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_D3D12_enable\0";
    }
    pub struct VisibilityMask {
        pub get_visibility_mask: pfn::GetVisibilityMaskKHR,
    }
    impl VisibilityMask {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_visibility_mask\0";
    }
    #[cfg(target_os = "windows")]
    pub struct Win32ConvertPerformanceCounterTime {
        pub convert_win32_performance_counter_to_time: pfn::ConvertWin32PerformanceCounterToTimeKHR,
        pub convert_time_to_win32_performance_counter: pfn::ConvertTimeToWin32PerformanceCounterKHR,
    }
    #[cfg(target_os = "windows")]
    impl Win32ConvertPerformanceCounterTime {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_win32_convert_performance_counter_time\0";
    }
    #[cfg(feature = "libc")]
    pub struct ConvertTimespecTime {
        pub convert_timespec_time_to_time: pfn::ConvertTimespecTimeToTimeKHR,
        pub convert_time_to_timespec_time: pfn::ConvertTimeToTimespecTimeKHR,
    }
    #[cfg(feature = "libc")]
    impl ConvertTimespecTime {
        pub const VERSION: u32 = 1u32;
        pub const NAME: &'static [u8] = b"XR_KHR_convert_timespec_time\0";
    }
}

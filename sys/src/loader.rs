use crate::generated::*;
use crate::support::fmt_enum;
use crate::Version;

/// Function pointer prototype for the xrCreateApiLayerInstance function used in place of xrCreateInstance.
/// This function allows us to pass special API layer information to each layer during the process of creating an Instance.
pub type FnCreateApiLayerInstance = unsafe extern "system" fn(
    info: *const InstanceCreateInfo,
    api_layer_info: *const ApiLayerCreateInfo,
    instance: *mut Instance,
) -> Result;

/// Loader/API Layer Interface versions
///
///  1 - First version, introduces negotiation structure and functions
pub const CURRENT_LOADER_API_LAYER_VERSION: u32 = 1;

/// Loader/Runtime Interface versions
///
///  1 - First version, introduces negotiation structure and functions
pub const CURRENT_LOADER_RUNTIME_VERSION: u32 = 1;

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LoaderInterfaceStructureType(i32);
impl LoaderInterfaceStructureType {
    pub const UNINTIALIZED: Self = Self(0);
    pub const LOADER_INFO: Self = Self(1);
    pub const API_LAYER_REQUEST: Self = Self(2);
    pub const RUNTIME_REQUEST: Self = Self(3);
    pub const API_LAYER_CREATE_INFO: Self = Self(4);
    pub const API_LAYER_NEXT_INFO: Self = Self(5);
}

impl core::fmt::Debug for LoaderInterfaceStructureType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNINTIALIZED => Some("UNINTIALIZED"),
            Self::LOADER_INFO => Some("LOADER_INFO"),
            Self::API_LAYER_REQUEST => Some("API_LAYER_REQUEST"),
            Self::RUNTIME_REQUEST => Some("RUNTIME_REQUEST"),
            Self::API_LAYER_CREATE_INFO => Some("API_LAYER_CREATE_INFO"),
            Self::API_LAYER_NEXT_INFO => Some("API_LAYER_NEXT_INFO"),
            _ => None,
        };
        fmt_enum(fmt, self.0, name)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XrNegotiateLoaderInfo {
    pub ty: LoaderInterfaceStructureType,
    pub struct_version: u32,
    pub struct_size: usize,
    pub min_interface_version: u32,
    pub max_interface_version: u32,
    pub min_api_version: Version,
    pub max_api_version: Version,
}
impl XrNegotiateLoaderInfo {
    pub const TYPE: LoaderInterfaceStructureType = LoaderInterfaceStructureType::LOADER_INFO;
    pub const VERSION: u32 = 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrNegotiateApiLayerRequest {
    pub ty: LoaderInterfaceStructureType,
    pub struct_version: u32,
    pub struct_size: usize,
    pub layer_interface_version: u32,
    pub layer_api_version: Version,
    pub get_instance_proc_addr: Option<pfn::GetInstanceProcAddr>,
    pub create_api_layer_instance: Option<FnCreateApiLayerInstance>,
}
impl XrNegotiateApiLayerRequest {
    pub const TYPE: LoaderInterfaceStructureType = LoaderInterfaceStructureType::API_LAYER_REQUEST;
    pub const VERSION: u32 = 1;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrNegotiateRuntimeRequest {
    pub ty: LoaderInterfaceStructureType,
    pub struct_version: u32,
    pub struct_size: usize,
    pub runtime_interface_version: u32,
    pub runtime_api_version: Version,
    pub get_instance_proc_addr: Option<pfn::GetInstanceProcAddr>,
}
impl XrNegotiateRuntimeRequest {
    pub const TYPE: LoaderInterfaceStructureType = LoaderInterfaceStructureType::RUNTIME_REQUEST;
    pub const VERSION: u32 = 1;
}

/// Function used to negotiate an interface between the loader and an API layer.  Each library exposing one or
/// more API layers needs to expose at least this function.
pub type FnNegotiateLoaderApiLayerInterface = unsafe extern "system" fn(
    loader_info: *const XrNegotiateLoaderInfo,
    api_layer_name: *const i8,
    api_layer_request: *mut XrNegotiateApiLayerRequest,
) -> Result;

/// Function used to negotiate an interface between the loader and a runtime.  Each runtime should expose
/// at least this function.
pub type FnNegotiateLoaderRuntimeInterface = unsafe extern "system" fn(
    loader_info: *const XrNegotiateLoaderInfo,
    runtime_request: *mut XrNegotiateRuntimeRequest,
) -> Result;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XrApiLayerNextInfo {
    pub ty: LoaderInterfaceStructureType,
    pub struct_version: u32,
    pub struct_size: usize,
    /// Name of API layer which should receive this info
    pub layer_name: [i8; MAX_API_LAYER_NAME_SIZE],
    /// Pointer to next API layer's xrGetInstanceProcAddr
    pub next_get_instance_proc_addr: pfn::GetInstanceProcAddr,
    /// Pointer to next API layer's xrCreateApiLayerInstance
    pub next_create_api_layer_instance: FnCreateApiLayerInstance,
    /// Pointer to the next API layer info in the sequence
    pub next: *mut XrApiLayerNextInfo,
}
impl XrApiLayerNextInfo {
    pub const TYPE: LoaderInterfaceStructureType =
        LoaderInterfaceStructureType::API_LAYER_NEXT_INFO;
    pub const VERSION: u32 = 1;
}

pub const API_LAYER_MAX_SETTINGS_PATH_SIZE: usize = 512;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ApiLayerCreateInfo {
    pub ty: LoaderInterfaceStructureType,
    pub struct_version: u32,
    pub struct_size: usize,
    /// Pointer to the LoaderInstance class
    pub loader_instance: *const (),
    /// Location to the found settings file (or empty '\0')
    pub settings_file_location: [i8; API_LAYER_MAX_SETTINGS_PATH_SIZE],
    /// Pointer to the next API layer's Info
    pub next_info: *mut XrApiLayerNextInfo,
}
impl ApiLayerCreateInfo {
    pub const TYPE: LoaderInterfaceStructureType =
        LoaderInterfaceStructureType::API_LAYER_CREATE_INFO;
    pub const VERSION: u32 = 1;
}

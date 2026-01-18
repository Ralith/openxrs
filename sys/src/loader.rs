use crate::generated::*;

// =============================================================================
// Loader Negotiation Interface Definitions
// =============================================================================
//
// NOTE: The following `impl` blocks are MANUALLY maintained.
//
// While the structs are generated from `xr.xml`, the registry is missing the
// necessary metadata (specifically `values` attributes) to link these structs
// to their corresponding `TYPE` enums and `VERSION` constants.
//
// These relationships are implicitly defined in the C header `openxr_loader_negotiation.h`.
// DO NOT remove or modify these values without verifying against that header file.
//
// =============================================================================

impl NegotiateLoaderInfo {
    pub const TYPE: LoaderInterfaceStructs = LoaderInterfaceStructs::LOADER_INFO;
    pub const VERSION: u32 = 1;
}

impl NegotiateApiLayerRequest {
    pub const TYPE: LoaderInterfaceStructs = LoaderInterfaceStructs::API_LAYER_REQUEST;
    pub const VERSION: u32 = 1;
}

impl NegotiateRuntimeRequest {
    pub const TYPE: LoaderInterfaceStructs = LoaderInterfaceStructs::RUNTIME_REQUEST;
    pub const VERSION: u32 = 1;
}

impl ApiLayerNextInfo {
    pub const TYPE: LoaderInterfaceStructs = LoaderInterfaceStructs::API_LAYER_NEXT_INFO;
    pub const VERSION: u32 = 1;
}

impl ApiLayerCreateInfo {
    pub const TYPE: LoaderInterfaceStructs = LoaderInterfaceStructs::API_LAYER_CREATE_INFO;
    pub const VERSION: u32 = 1;
}

#[cfg(feature = "loaded")]
use shared_library::dynamic_library::DynamicLibrary;
use std::{ffi::CStr, mem, ptr, sync::Arc};
#[cfg(feature = "loaded")]
use std::{fmt, os::raw::c_void, path::Path};

use crate::*;

/// Entry point to the API. Start here.
///
/// An `Entry` represents access to an OpenXR implementation. When the crate is built with the
/// `linked` feature, this can be obtained at compile time with the `linked` constructor. The
/// `static` feature provides a built-in copy of the Khronos OpenXR loader for use in this
/// pattern. Alternatively, the `loaded` feature exposes the `load` and `load_from` constructors to
/// manually load an OpenXR implementation at run-time.
#[derive(Clone)]
pub struct Entry {
    inner: Arc<Inner>,
}

impl Entry {
    /// Access entry points linked directly into the binary at compile time
    ///
    /// Available if the `linked` feature is enabled. You must ensure that the entry points are
    /// actually linked into the binary, e.g. by enabling the `static` feature.
    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        Self {
            inner: Arc::new(Inner {
                raw: RawEntry {
                    get_instance_proc_addr: sys::get_instance_proc_addr,
                    create_instance: sys::create_instance,
                    enumerate_instance_extension_properties:
                        sys::enumerate_instance_extension_properties,
                    enumerate_api_layer_properties: sys::enumerate_api_layer_properties,
                },
                #[cfg(feature = "loaded")]
                _lib_guard: None,
            }),
        }
    }

    /// Load entry points at run time from the dynamic library `openxr_loader` according to the
    /// target platform's naming conventions
    ///
    /// Available if the `loaded` feature is enabled.
    #[cfg(feature = "loaded")]
    pub fn load() -> std::result::Result<Self, LoadError> {
        #[cfg(target_os = "windows")]
        const PATH: &str = "openxr_loader.dll";
        #[cfg(target_os = "macos")]
        const PATH: &str = "libopenxr_loader.dylib";
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        const PATH: &str = "libopenxr_loader.so";
        Self::load_from(Path::new(PATH))
    }

    /// Load entry points at run time from the dynamic library identified by `path`
    ///
    /// Available if the `loaded` feature is enabled.
    #[cfg(feature = "loaded")]
    pub fn load_from(path: &Path) -> std::result::Result<Self, LoadError> {
        let lib = DynamicLibrary::open(Some(path)).map_err(LoadError)?;
        Ok(Self {
            inner: Arc::new(Inner {
                raw: unsafe {
                    RawEntry {
                        get_instance_proc_addr: mem::transmute(
                            lib.symbol::<c_void>("xrGetInstanceProcAddr")
                                .map_err(LoadError)?,
                        ),
                        create_instance: mem::transmute(
                            lib.symbol::<c_void>("xrCreateInstance")
                                .map_err(LoadError)?,
                        ),
                        enumerate_instance_extension_properties: mem::transmute(
                            lib.symbol::<c_void>("xrEnumerateInstanceExtensionProperties")
                                .map_err(LoadError)?,
                        ),
                        enumerate_api_layer_properties: mem::transmute(
                            lib.symbol::<c_void>("xrEnumerateApiLayerProperties")
                                .map_err(LoadError)?,
                        ),
                    }
                },
                _lib_guard: Some(lib),
            }),
        })
    }

    /// Access the raw function pointers
    #[inline]
    pub fn fp(&self) -> &RawEntry {
        &self.inner.raw
    }

    #[inline]
    pub(crate) unsafe fn get_instance_proc_addr(
        &self,
        instance: sys::Instance,
        name: &CStr,
    ) -> Result<unsafe extern "system" fn()> {
        let mut f = None;
        cvt((self.fp().get_instance_proc_addr)(
            instance,
            name.as_ptr(),
            &mut f,
        ))?;
        Ok(f.unwrap())
    }

    /// Create an OpenXR instance with certain extensions enabled
    ///
    /// Most applications will want to enable at least one graphics API extension
    /// (e.g. `khr_vulkan_enable`) so that a `Session` can be created for rendering.
    pub fn create_instance(
        &self,
        app_info: &ApplicationInfo,
        required_extensions: &ExtensionSet,
    ) -> Result<Instance> {
        assert!(
            app_info.application_name.len() < sys::MAX_APPLICATION_NAME_SIZE,
            "application names are limited to {} bytes",
            sys::MAX_APPLICATION_NAME_SIZE
        );
        assert!(
            app_info.engine_name.len() < sys::MAX_ENGINE_NAME_SIZE,
            "engine names are limited to {} bytes",
            sys::MAX_ENGINE_NAME_SIZE
        );
        let ext_names = required_extensions.names();
        let mut info = sys::InstanceCreateInfo {
            ty: sys::InstanceCreateInfo::TYPE,
            next: ptr::null(),
            create_flags: Default::default(),
            application_info: sys::ApplicationInfo {
                application_name: [0; sys::MAX_APPLICATION_NAME_SIZE],
                application_version: app_info.application_version,
                engine_name: [0; sys::MAX_ENGINE_NAME_SIZE],
                engine_version: app_info.engine_version,
                api_version: CURRENT_API_VERSION.into_raw(),
            },
            enabled_api_layer_count: 0,
            enabled_api_layer_names: ptr::null(),
            enabled_extension_count: ext_names.len() as _,
            enabled_extension_names: ext_names.as_ptr(),
        };
        place_cstr(
            &mut info.application_info.application_name,
            &app_info.application_name,
        );
        place_cstr(
            &mut info.application_info.engine_name,
            &app_info.engine_name,
        );
        unsafe {
            let mut handle = sys::Instance::NULL;
            cvt((self.fp().create_instance)(&info, &mut handle))?;

            let exts = InstanceExtensions::load(self, handle, required_extensions)?;
            Instance::from_raw(self.clone(), handle, exts)
        }
    }

    /// Determine the set of extensions supported by this OpenXR implementation
    pub fn enumerate_extensions(&self) -> Result<ExtensionSet> {
        let exts = unsafe {
            get_arr_init(
                sys::ExtensionProperties {
                    ty: sys::ExtensionProperties::TYPE,
                    next: ptr::null_mut(),
                    ..mem::uninitialized()
                },
                |cap, count, buf| {
                    (self.fp().enumerate_instance_extension_properties)(
                        ptr::null(),
                        cap,
                        count,
                        buf,
                    )
                },
            )?
        };
        Ok(ExtensionSet::from_properties(&exts))
    }
}

struct Inner {
    raw: RawEntry,
    #[cfg(feature = "loaded")]
    _lib_guard: Option<DynamicLibrary>,
}

pub struct RawEntry {
    pub get_instance_proc_addr: sys::pfn::GetInstanceProcAddr,
    pub create_instance: sys::pfn::CreateInstance,
    pub enumerate_instance_extension_properties: sys::pfn::EnumerateInstanceExtensionProperties,
    pub enumerate_api_layer_properties: sys::pfn::EnumerateApiLayerProperties,
}

/// An error encountered while loading entry points from a dynamic library at run time
#[cfg(feature = "loaded")]
#[derive(Clone)]
pub struct LoadError(String);

#[cfg(feature = "loaded")]
impl fmt::Debug for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "loaded")]
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "loaded")]
impl std::error::Error for LoadError {}

pub struct ApplicationInfo<'a> {
    pub application_name: &'a str,
    pub application_version: u32,
    pub engine_name: &'a str,
    pub engine_version: u32,
}

impl<'a> Default for ApplicationInfo<'a> {
    fn default() -> Self {
        Self {
            application_name: "",
            application_version: 0,
            engine_name: "",
            engine_version: 0,
        }
    }
}

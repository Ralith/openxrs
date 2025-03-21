#[cfg(feature = "loaded")]
use libloading::Library;
use std::{
    ffi::{CStr, CString},
    mem, ptr,
    sync::Arc,
};
#[cfg(feature = "loaded")]
use std::{fmt, path::Path};

use crate::sys::Handle as _;
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
    /// actually linked into the binary, e.g. by enabling the `static` feature or manually linking
    /// to an external loader or implementation.
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
    ///
    /// # Safety
    ///
    /// The OpenXR loader shared library in the dynamic loader's search path must conform to the
    /// OpenXR specification.
    #[cfg(feature = "loaded")]
    pub unsafe fn load() -> std::result::Result<Self, LoadError> {
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
    ///
    /// # Safety
    ///
    /// `path` must be a shared library that provides OpenXR-compliant definitions for every core
    /// OpenXR entry point.
    #[cfg(feature = "loaded")]
    pub unsafe fn load_from(path: &Path) -> std::result::Result<Self, LoadError> {
        let lib = Library::new(path).map_err(LoadError)?;
        Ok(Self {
            inner: Arc::new(Inner {
                raw: RawEntry {
                    get_instance_proc_addr: *lib
                        .get(b"xrGetInstanceProcAddr\0")
                        .map_err(LoadError)?,
                    create_instance: *lib.get(b"xrCreateInstance\0").map_err(LoadError)?,
                    enumerate_instance_extension_properties: *lib
                        .get(b"xrEnumerateInstanceExtensionProperties\0")
                        .map_err(LoadError)?,
                    enumerate_api_layer_properties: lib
                        .get(b"xrEnumerateApiLayerProperties\0")
                        .map(|s| *s)
                        .unwrap_or(crate::stub_enumerate_api_layer_properties),
                },
                _lib_guard: Some(lib),
            }),
        })
    }

    /// Load entry points using an arbitrary `xrGetInstanceProcAddr` implementation
    ///
    /// # Safety
    ///
    /// For all core OpenXR instance functions, `get_instance_proc_addr` must yield function
    /// pointers that satisfy the semantics given in the OpenXR specification.
    #[allow(clippy::missing_transmute_annotations)]
    pub unsafe fn from_get_instance_proc_addr(
        get_instance_proc_addr: sys::pfn::GetInstanceProcAddr,
    ) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(Inner {
                raw: RawEntry {
                    get_instance_proc_addr,
                    create_instance: mem::transmute(get_instance_proc_addr_helper(
                        get_instance_proc_addr,
                        sys::Instance::NULL,
                        CStr::from_bytes_with_nul_unchecked(b"xrCreateInstance\0"),
                    )?),
                    enumerate_instance_extension_properties: mem::transmute(
                        get_instance_proc_addr_helper(
                            get_instance_proc_addr,
                            sys::Instance::NULL,
                            CStr::from_bytes_with_nul_unchecked(
                                b"xrEnumerateInstanceExtensionProperties\0",
                            ),
                        )?,
                    ),
                    enumerate_api_layer_properties: get_instance_proc_addr_helper(
                        get_instance_proc_addr,
                        sys::Instance::NULL,
                        CStr::from_bytes_with_nul_unchecked(b"xrEnumerateApiLayerProperties\0"),
                    )
                    .map(|s| unsafe { mem::transmute(s) })
                    .unwrap_or(crate::stub_enumerate_api_layer_properties),
                },
                #[cfg(feature = "loaded")]
                _lib_guard: None,
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
        get_instance_proc_addr_helper(self.fp().get_instance_proc_addr, instance, name)
    }

    /// Initialize Android loader. This must be called before `create_instance()`.
    #[cfg(target_os = "android")]
    pub fn initialize_android_loader(&self) -> Result<()> {
        let loader_init = unsafe { raw::LoaderInitKHR::load(self, sys::Instance::NULL)? };

        let context = ndk_context::android_context();

        let loader_info = sys::LoaderInitInfoAndroidKHR {
            ty: sys::LoaderInitInfoAndroidKHR::TYPE,
            next: ptr::null(),
            application_vm: context.vm(),
            application_context: context.context(),
        };

        unsafe {
            cvt((loader_init.initialize_loader)(
                &loader_info as *const _ as _,
            ))?;
        }

        Ok(())
    }

    /// Create an OpenXR instance with certain extensions enabled. Android support can be enabled by
    /// setting `khr_android_create_instance` to `true`.
    ///
    /// Most applications will want to enable at least one graphics API extension
    /// (e.g. `khr_vulkan_enable2`) so that a `Session` can be created for rendering.
    pub fn create_instance(
        &self,
        app_info: &ApplicationInfo,
        required_extensions: &ExtensionSet,
        layers: &[&str],
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
        let ext_ptrs = ext_names
            .iter()
            .map(|x| x.as_ptr() as *const _)
            .collect::<Vec<_>>();
        let layer_names = layers
            .iter()
            .filter_map(|&x| CString::new(x).ok())
            .collect::<Vec<_>>();
        let layer_ptrs = layer_names
            .iter()
            .map(|x| x.as_ptr() as *const _)
            .collect::<Vec<_>>();

        #[cfg(not(target_os = "android"))]
        let next = ptr::null();
        #[cfg(target_os = "android")]
        let android_info = {
            let context = ndk_context::android_context();

            sys::InstanceCreateInfoAndroidKHR {
                ty: sys::InstanceCreateInfoAndroidKHR::TYPE,
                next: ptr::null(),
                application_vm: context.vm(),
                application_activity: context.context(),
            }
        };
        #[cfg(target_os = "android")]
        let next = if required_extensions.khr_android_create_instance {
            &android_info as *const _ as _
        } else {
            ptr::null()
        };

        let mut info = sys::InstanceCreateInfo {
            ty: sys::InstanceCreateInfo::TYPE,
            next,
            create_flags: Default::default(),
            application_info: sys::ApplicationInfo {
                application_name: [0; sys::MAX_APPLICATION_NAME_SIZE],
                application_version: app_info.application_version,
                engine_name: [0; sys::MAX_ENGINE_NAME_SIZE],
                engine_version: app_info.engine_version,
                api_version: app_info.api_version,
            },
            enabled_api_layer_count: layer_ptrs.len() as _,
            enabled_api_layer_names: layer_ptrs.as_ptr(),
            enabled_extension_count: ext_ptrs.len() as _,
            enabled_extension_names: ext_ptrs.as_ptr(),
        };
        place_cstr(
            &mut info.application_info.application_name,
            app_info.application_name,
        );
        place_cstr(&mut info.application_info.engine_name, app_info.engine_name);
        unsafe {
            let mut handle = sys::Instance::NULL;
            cvt((self.fp().create_instance)(&info, &mut handle))?;

            let exts = InstanceExtensions::load(self, handle, required_extensions)?;
            Instance::from_raw(self.clone(), handle, exts)
        }
    }

    /// Determine the set of extensions supported by this OpenXR implementation
    pub fn enumerate_extensions(&self) -> Result<ExtensionSet> {
        unsafe {
            let exts = get_arr_init(
                sys::ExtensionProperties::out(ptr::null_mut()),
                |cap, count, buf| {
                    (self.fp().enumerate_instance_extension_properties)(
                        ptr::null(),
                        cap,
                        count,
                        buf as _,
                    )
                },
            )?;
            // https://github.com/rust-lang/rust/issues/63569
            Ok(ExtensionSet::from_properties(mem::transmute::<
                &[std::mem::MaybeUninit<sys::ExtensionProperties>],
                &[sys::ExtensionProperties],
            >(&exts[..])))
        }
    }

    pub fn enumerate_layers(&self) -> Result<Vec<ApiLayerProperties>> {
        unsafe {
            let layers = get_arr_init(
                sys::ApiLayerProperties::out(ptr::null_mut()),
                |cap, count, buf| (self.fp().enumerate_api_layer_properties)(cap, count, buf as _),
            )?;
            Ok(layers
                .into_iter()
                .map(|x| {
                    let x = x.assume_init();
                    ApiLayerProperties {
                        layer_name: fixed_str(&x.layer_name).into(),
                        spec_version: x.spec_version,
                        layer_version: x.layer_version,
                        description: fixed_str(&x.description).into(),
                    }
                })
                .collect())
        }
    }
}

#[inline]
unsafe fn get_instance_proc_addr_helper(
    get_instance_proc_addr: sys::pfn::GetInstanceProcAddr,
    instance: sys::Instance,
    name: &CStr,
) -> Result<unsafe extern "system" fn()> {
    let mut f = None;
    cvt((get_instance_proc_addr)(instance, name.as_ptr(), &mut f))?;
    Ok(f.unwrap())
}

struct Inner {
    raw: RawEntry,
    #[cfg(feature = "loaded")]
    _lib_guard: Option<Library>,
}

pub struct RawEntry {
    pub get_instance_proc_addr: sys::pfn::GetInstanceProcAddr,
    pub create_instance: sys::pfn::CreateInstance,
    pub enumerate_instance_extension_properties: sys::pfn::EnumerateInstanceExtensionProperties,
    pub enumerate_api_layer_properties: sys::pfn::EnumerateApiLayerProperties,
}

/// An error encountered while loading entry points from a dynamic library at run time
#[cfg(feature = "loaded")]
pub struct LoadError(libloading::Error);

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
impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ApplicationInfo<'a> {
    pub application_name: &'a str,
    pub application_version: u32,
    pub engine_name: &'a str,
    pub engine_version: u32,
    pub api_version: Version,
}

impl<'a> Default for ApplicationInfo<'a> {
    fn default() -> Self {
        Self {
            application_name: Default::default(),
            application_version: Default::default(),
            engine_name: Default::default(),
            engine_version: Default::default(),
            api_version: Version::new(1, 0, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "See [XrApiLayerProperties](https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XrApiLayerProperties)"]
pub struct ApiLayerProperties {
    pub layer_name: String,
    pub spec_version: Version,
    pub layer_version: u32,
    pub description: String,
}

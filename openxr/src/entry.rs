use std::{ptr, ffi::CStr};

use crate::{cvt, Result, Instance};

pub trait Entry {
    /// Access the raw function pointers
    fn raw(&self) -> &RawEntry;

    #[inline]
    #[doc(hidden)]
    unsafe fn get_instance_proc_addr(
        &self,
        instance: sys::Instance,
        name: &CStr,
    ) -> Result<unsafe extern "system" fn()> {
        let mut f = None;
        cvt((self.raw().get_instance_proc_addr)(
            instance,
            name.as_ptr(),
            &mut f,
        ))?;
        Ok(f.unwrap())
    }

    fn create_instance(&self, app_info: &ApplicationInfo) -> Result<Instance<Self>>
        where Self: Sized + Clone
    {
        let info = sys::InstanceCreateInfo {
            ty: sys::InstanceCreateInfo::TYPE,
            next: ptr::null(),
            create_flags: Default::default(),
            application_info: sys::ApplicationInfo {
                application_name: fixed_cstr!(app_info.application_name, MAX_APPLICATION_NAME_SIZE),
                application_version: app_info.application_version,
                engine_name: fixed_cstr!(app_info.engine_name, MAX_ENGINE_NAME_SIZE),
                engine_version: app_info.engine_version,
                api_version: sys::CURRENT_API_VERSION,
            },
            enabled_api_layer_count: 0,
            enabled_api_layer_names: ptr::null(),
            enabled_extension_count: 0,
            enabled_extension_names: ptr::null(),
        };
        unsafe {
            let mut i = sys::Instance::NULL;
            cvt((self.raw().create_instance)(&info, &mut i))?;
            Instance::from_raw(self.clone(), i)
        }
    }
}

pub struct ApplicationInfo<'a> {
    pub application_name: &'a str,
    pub application_version: u32,
    pub engine_name: &'a str,
    pub engine_version: u32,
}

pub struct RawEntry {
    pub get_instance_proc_addr: sys::pfn::GetInstanceProcAddr,
    pub create_instance: sys::pfn::CreateInstance,
    pub enumerate_instance_extension_properties: sys::pfn::EnumerateInstanceExtensionProperties,
    pub enumerate_api_layer_properties: sys::pfn::EnumerateApiLayerProperties,
}

#[cfg(feature = "linked")]
mod linked {
    use super::*;

    /// Entry point to an OpenXR implementation linked at compile time
    ///
    /// Requires the `linked` feature.
    #[derive(Copy, Clone)]
    pub struct LinkedEntry;

    impl LinkedEntry {
        #[inline]
        pub fn new() -> Self {
            Self
        }

        pub fn create_instance(&self, app_info: &ApplicationInfo) -> Result<Instance<Self>> {
            Entry::create_instance(self, app_info)
        }
    }

    impl Entry for LinkedEntry {
        #[inline]
        fn raw(&self) -> &RawEntry {
            &RawEntry {
                get_instance_proc_addr: sys::get_instance_proc_addr,
                create_instance: sys::create_instance,
                enumerate_instance_extension_properties:
                    sys::enumerate_instance_extension_properties,
                enumerate_api_layer_properties: sys::enumerate_api_layer_properties,
            }
        }
    }
}
#[cfg(feature = "linked")]
pub use linked::LinkedEntry;

#[cfg(feature = "loaded")]
mod loaded {
    use super::*;

    use shared_library::dynamic_library::DynamicLibrary;
    use std::{mem, os::raw::c_void, path::Path, sync::Arc};

    /// Entry point to an OpenXR implementation loaded from a dynamic library at run time
    ///
    /// Requires the `loaded` future.
    #[derive(Clone)]
    pub struct LoadedEntry {
        inner: Arc<Inner>,
    }

    struct Inner {
        raw: RawEntry,
        _lib_guard: DynamicLibrary,
    }

    impl LoadedEntry {
        pub fn load() -> std::result::Result<Self, String> {
            Self::load_from(Path::new("openxr_loader"))
        }

        pub fn load_from(path: &Path) -> std::result::Result<Self, String> {
            let lib = DynamicLibrary::open(Some(path))?;
            Ok(Self {
                inner: Arc::new(Inner {
                    raw: unsafe {
                        RawEntry {
                            get_instance_proc_addr: mem::transmute(
                                lib.symbol::<c_void>("xrGetInstanceProcAddr")?,
                            ),
                            create_instance: mem::transmute(lib.symbol::<c_void>("xrCreateInstance")?),
                            enumerate_instance_extension_properties: mem::transmute(
                                lib.symbol::<c_void>("xrEnumerateInstanceExtensionProperties")?,
                            ),
                            enumerate_api_layer_properties: mem::transmute(
                                lib.symbol::<c_void>("xrEnumerateApiLayerProperties")?,
                            ),
                        }
                    },
                    _lib_guard: lib,
                })
            })
        }

        pub fn create_instance(&self, app_info: &ApplicationInfo) -> Result<Instance<Self>> {
            Entry::create_instance(self, app_info)
        }
    }

    impl Entry for LoadedEntry {
        fn raw(&self) -> &RawEntry {
            &self.inner.raw
        }
    }
}
#[cfg(feature = "loaded")]
pub use loaded::LoadedEntry;

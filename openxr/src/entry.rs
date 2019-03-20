use std::ffi::CStr;
use std::sync::Arc;

use crate::{cvt, Result};

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

    #[inline]
    #[doc(hidden)]
    unsafe fn create_instance(&self, info: &sys::InstanceCreateInfo) -> Result<sys::Instance> {
        let mut i = sys::Instance::NULL;
        cvt((self.raw().create_instance)(info, &mut i))?;
        Ok(i)
    }
}

impl<T: Entry> Entry for Arc<T> {
    fn raw(&self) -> &RawEntry {
        self.as_ref().raw()
    }
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
    use std::{mem, os::raw::c_void, path::Path};

    /// Entry point to an OpenXR implementation loaded from a dynamic library at run time
    ///
    /// Requires the `loaded` future. Wrap in an `Arc` to support loading extensions.
    pub struct LoadedEntry {
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
        }
    }

    impl Entry for LoadedEntry {
        fn raw(&self) -> &RawEntry {
            &self.raw
        }
    }
}
#[cfg(feature = "loaded")]
pub use loaded::LoadedEntry;

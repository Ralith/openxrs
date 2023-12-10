use crate::*;

#[derive(Clone)]
pub struct LocalizationMapML {
    inner: sys::LocalizationMapML,
}

impl LocalizationMapML {
    #[inline]
    pub fn from_raw(inner: sys::LocalizationMapML) -> Self {
        Self { inner }
    }
    #[inline]
    pub fn as_raw(&self) -> sys::LocalizationMapML {
        self.inner
    }
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr(self.inner.name.as_ptr())
                .to_str()
                .unwrap()
        }
    }
    #[inline]
    pub fn map_uuid(&self) -> UuidEXT {
        self.inner.map_uuid
    }
    #[inline]
    pub fn map_type(&self) -> LocalizationMapTypeML {
        self.inner.map_type
    }
}

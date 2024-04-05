use crate::*;

#[derive(Clone)]
pub struct LocalizationMapML {
    pub name: String,
    pub map_uuid: UuidEXT,
    pub map_type: LocalizationMapTypeML,
}

impl LocalizationMapML {
    #[inline]
    pub fn from_raw(inner: sys::LocalizationMapML) -> Self {
        Self {
            name: unsafe { fixed_str(&inner.name).into() },
            map_uuid: inner.map_uuid,
            map_type: inner.map_type,
        }
    }
}

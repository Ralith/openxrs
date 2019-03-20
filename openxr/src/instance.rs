use std::mem;
use crate::{cvt, fixed_str, Entry, Instance, Result};

impl<E: Entry> Instance<E> {
    #[inline]
    pub fn properties(&self) -> Result<InstanceProperties> {
        unsafe {
            let mut p = sys::InstanceProperties {
                ty: sys::InstanceProperties::TYPE,
                ..mem::zeroed()
            };
            cvt((self.raw().get_instance_properties)(self.as_raw(), &mut p))?;
            Ok(InstanceProperties {
                runtime_version: p.runtime_version,
                runtime_name: fixed_str(&p.runtime_name).into(),
            })
        }
    }

    #[inline]
    pub fn result_to_string(&self, result: sys::Result) -> Result<String> {
        unsafe {
            let mut s = [0; sys::MAX_RESULT_STRING_SIZE];
            cvt((self.raw().result_to_string)(self.as_raw(), result, s.as_mut_ptr()))?;
            Ok(fixed_str(&s).into())
        }
    }

    #[inline]
    pub fn structure_type_to_string(&self, ty: sys::StructureType) -> Result<String> {
        unsafe {
            let mut s = [0; sys::MAX_STRUCTURE_NAME_SIZE];
            cvt((self.raw().structure_type_to_string)(self.as_raw(), ty, s.as_mut_ptr()))?;
            Ok(fixed_str(&s).into())
        }
    }
}

impl<E: Entry> Drop for Instance<E> {
    fn drop(&mut self) {
        unsafe {
            (self.raw().destroy_instance)(self.as_raw());
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstanceProperties {
    pub runtime_version: u32,
    pub runtime_name: String,
}

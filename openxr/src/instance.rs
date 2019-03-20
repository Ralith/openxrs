use std::{mem, ptr};
use crate::{cvt, fixed_str, Entry, Instance, Result};

impl<E: Entry> Instance<E> {
    #[inline]
    pub fn properties(&self) -> Result<InstanceProperties> {
        unsafe {
            let mut p = sys::InstanceProperties {
                ty: sys::InstanceProperties::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((self.raw().get_instance_properties)(self.as_raw(), &mut p))?;
            Ok(InstanceProperties {
                runtime_version: p.runtime_version,
                runtime_name: fixed_str(&p.runtime_name).into(),
            })
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

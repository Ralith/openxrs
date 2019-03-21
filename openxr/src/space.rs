use std::{mem, ptr, sync::Arc};

use crate::*;

pub struct Space {
    session: Arc<session::SessionInner>,
    handle: sys::Space,
}

impl Space {
    pub(crate) fn new(session: Arc<session::SessionInner>, handle: sys::Space) -> Self {
        Self { session, handle }
    }

    /// Take ownership of an existing space handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid swapchain handle.
    #[inline]
    pub unsafe fn from_raw<G: Graphics>(session: Session<G>, handle: sys::Space) -> Self {
        Self {
            session: session.inner,
            handle,
        }
    }

    /// Access the raw swapchain handle
    #[inline]
    pub fn as_raw(&self) -> sys::Space {
        self.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.session.instance
    }

    /// Determine the physical relationship of a space relative to a base space at a specified time,
    /// if currently known by the runtime.
    #[inline]
    pub fn locate(&self, base: &Space, time: Time) -> Result<SpaceRelation> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*base.session as *const session::SessionInner,
                   "`self` and `base` must have been created, allocated, or retrieved from the same `Session`");
        let mut out;
        unsafe {
            out = sys::SpaceRelation {
                ty: sys::SpaceRelation::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((self.fp().locate_space)(
                self.as_raw(),
                base.as_raw(),
                time,
                &mut out,
            ))?;
        }
        Ok(SpaceRelation {
            relation_flags: out.relation_flags,
            time: out.time,
            pose: out.pose,
            linear_velocity: out.linear_velocity,
            angular_velocity: out.angular_velocity,
            linear_acceleration: out.linear_acceleration,
            angular_acceleration: out.angular_acceleration,
        })
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance.fp()
    }
}

impl Drop for Space {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_space)(self.handle);
        }
    }
}

#[derive(Copy, Clone)]
pub struct SpaceRelation {
    pub relation_flags: SpaceRelationFlags,
    pub time: Time,
    pub pose: Posef,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
    pub linear_acceleration: Vector3f,
    pub angular_acceleration: Vector3f,
}

use std::{ffi::CString, mem, ptr, sync::Arc};

use crate::*;

pub struct Space {
    session: Arc<session::SessionInner>,
    _action_guard: Option<Action<Posef>>,
    handle: sys::Space,
}

impl Space {
    /// Take ownership of an existing reference space handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid reference space handle associated with `session`.
    #[inline]
    pub unsafe fn reference_from_raw<G: Graphics>(session: Session<G>, handle: sys::Space) -> Self {
        Self {
            session: session.inner,
            _action_guard: None,
            handle,
        }
    }

    /// Take ownership of an existing action space handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid action space handle associated with `session`.
    #[inline]
    pub unsafe fn action_from_raw<G: Graphics>(
        action: Action<Posef>,
        session: Session<G>,
        handle: sys::Space,
    ) -> Self {
        Self {
            session: session.inner,
            _action_guard: Some(action),
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

    /// Set the debug name of this `Space`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        // We don't forward to the locking version on Instance because this object can't be cloned
        if let Some(fp) = self.instance().exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::SPACE,
                object_handle: self.as_raw().into_raw(),
                object_name: name.as_ptr(),
            };
            unsafe {
                cvt((fp.set_debug_utils_object_name)(
                    self.instance().as_raw(),
                    &info,
                ))?;
            }
        }
        Ok(())
    }

    /// Determine the location of a space relative to a base space at a specified time, if currently
    /// known by the runtime.
    #[inline]
    pub fn locate(&self, base: &Space, time: Time) -> Result<SpaceLocation> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*base.session as *const session::SessionInner,
                   "`self` and `base` must have been created, allocated, or retrieved from the same `Session`");
        let mut out;
        unsafe {
            out = sys::SpaceLocation {
                ty: sys::SpaceLocation::TYPE,
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
        Ok(SpaceLocation {
            location_flags: out.location_flags,
            pose: out.pose,
            // linear_velocity: out.linear_velocity,
            // angular_velocity: out.angular_velocity,
            // linear_acceleration: out.linear_acceleration,
            // angular_acceleration: out.angular_acceleration,
        })
    }

    /// Determine the location and velocity of a space relative to a base space at a specified time,
    /// if currently known by the runtime.
    #[inline]
    pub fn relate(&self, base: &Space, time: Time) -> Result<(SpaceLocation, SpaceVelocity)> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*base.session as *const session::SessionInner,
                   "`self` and `base` must have been created, allocated, or retrieved from the same `Session`");
        let mut velocity;
        let mut location;
        unsafe {
            velocity = sys::SpaceVelocity {
                ty: sys::SpaceVelocity::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            location = sys::SpaceLocation {
                ty: sys::SpaceLocation::TYPE,
                next: &mut velocity as *mut _ as _,
                ..mem::uninitialized()
            };
            cvt((self.fp().locate_space)(
                self.as_raw(),
                base.as_raw(),
                time,
                &mut location,
            ))?;
        }
        Ok((
            SpaceLocation {
                location_flags: location.location_flags,
                pose: location.pose,
            },
            SpaceVelocity {
                velocity_flags: velocity.velocity_flags,
                linear_velocity: velocity.linear_velocity,
                angular_velocity: velocity.angular_velocity,
            },
        ))
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
pub struct SpaceLocation {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
}

#[derive(Copy, Clone)]
pub struct SpaceVelocity {
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}

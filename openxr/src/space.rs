use std::{ffi::CString, mem::MaybeUninit, ptr, sync::Arc};

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
    pub unsafe fn reference_from_raw<G>(session: Session<G>, handle: sys::Space) -> Self {
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
    /// `handle` must be a valid action space handle for `action` and associated with `session`.
    #[inline]
    pub unsafe fn action_from_raw<G>(
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
        unsafe {
            let mut x = sys::SpaceLocation::out(ptr::null_mut());
            cvt((self.fp().locate_space)(
                self.as_raw(),
                base.as_raw(),
                time,
                x.as_mut_ptr(),
            ))?;
            Ok(SpaceLocation::new(&x))
        }
    }

    /// Determine the location and velocity of a space relative to a base space at a specified time,
    /// if currently known by the runtime.
    #[inline]
    pub fn relate(&self, base: &Space, time: Time) -> Result<(SpaceLocation, SpaceVelocity)> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*base.session as *const session::SessionInner,
                   "`self` and `base` must have been created, allocated, or retrieved from the same `Session`");
        unsafe {
            let mut velocity = sys::SpaceVelocity::out(ptr::null_mut());
            let mut location = sys::SpaceLocation::out(&mut velocity as *mut _ as _);
            cvt((self.fp().locate_space)(
                self.as_raw(),
                base.as_raw(),
                time,
                location.as_mut_ptr(),
            ))?;
            Ok((SpaceLocation::new(&location), SpaceVelocity::new(&velocity)))
        }
    }

    /// Determine the locations of the joints of a hand tracker relative to this space at a
    /// specified time, if currently known by the runtime.
    ///
    /// XR_EXT_hand_tracking must be enabled.
    #[inline]
    pub fn locate_hand_joints(
        &self,
        tracker: &HandTracker,
        time: Time,
    ) -> Result<Option<HandJointLocations>> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*tracker.session as *const session::SessionInner,
                   "`self` and `tracker` must have been created, allocated, or retrieved from the same `Session`");
        unsafe {
            let locate_info = sys::HandJointsLocateInfoEXT {
                ty: sys::HandJointsLocateInfoEXT::TYPE,
                next: ptr::null(),
                base_space: self.as_raw(),
                time,
            };
            let mut locations = MaybeUninit::<[HandJointLocation; HAND_JOINT_COUNT]>::uninit();
            let mut location_info = sys::HandJointLocationsEXT {
                ty: sys::HandJointLocationsEXT::TYPE,
                next: ptr::null_mut(),
                is_active: false.into(),
                joint_count: HAND_JOINT_COUNT as u32,
                joint_locations: locations.as_mut_ptr() as _,
            };
            cvt((tracker.fp().locate_hand_joints)(
                tracker.as_raw(),
                &locate_info,
                &mut location_info,
            ))?;
            Ok(if location_info.is_active.into() {
                Some(locations.assume_init())
            } else {
                None
            })
        }
    }

    /// Determine the locations and velocities of the joints of a hand tracker relative to this
    /// space at a specified time, if currently known by the runtime.
    ///
    /// XR_EXT_hand_tracking must be enabled.
    #[inline]
    pub fn relate_hand_joints(
        &self,
        tracker: &HandTracker,
        time: Time,
    ) -> Result<Option<(HandJointLocations, HandJointVelocities)>> {
        // This assert allows this function to be safe.
        assert_eq!(&*self.session as *const session::SessionInner, &*tracker.session as *const session::SessionInner,
                   "`self` and `tracker` must have been created, allocated, or retrieved from the same `Session`");
        unsafe {
            let locate_info = sys::HandJointsLocateInfoEXT {
                ty: sys::HandJointsLocateInfoEXT::TYPE,
                next: ptr::null(),
                base_space: self.as_raw(),
                time,
            };
            let mut velocities = MaybeUninit::<[HandJointVelocity; HAND_JOINT_COUNT]>::uninit();
            let mut velocity_info = sys::HandJointVelocitiesEXT {
                ty: sys::HandJointVelocitiesEXT::TYPE,
                next: ptr::null_mut(),
                joint_count: HAND_JOINT_COUNT as u32,
                joint_velocities: velocities.as_mut_ptr() as _,
            };
            let mut locations = MaybeUninit::<[HandJointLocation; HAND_JOINT_COUNT]>::uninit();
            let mut location_info = sys::HandJointLocationsEXT {
                ty: sys::HandJointLocationsEXT::TYPE,
                next: &mut velocity_info as *mut _ as _,
                is_active: false.into(),
                joint_count: HAND_JOINT_COUNT as u32,
                joint_locations: locations.as_mut_ptr() as _,
            };
            cvt((tracker.fp().locate_hand_joints)(
                tracker.as_raw(),
                &locate_info,
                &mut location_info,
            ))?;
            Ok(if location_info.is_active.into() {
                Some((locations.assume_init(), velocities.assume_init()))
            } else {
                None
            })
        }
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

#[derive(Copy, Clone, Default, PartialEq)]
pub struct SpaceLocation {
    pub location_flags: SpaceLocationFlags,
    pub pose: Posef,
}

impl SpaceLocation {
    unsafe fn new(raw: &MaybeUninit<sys::SpaceLocation>) -> Self {
        // Applications *must* not read invalid parts of a pose, i.e. they may be uninitialized
        let ptr = raw.as_ptr();
        let flags = *ptr::addr_of!((*ptr).location_flags);
        Self {
            location_flags: flags,
            pose: Posef {
                orientation: flags
                    .contains(sys::SpaceLocationFlags::ORIENTATION_VALID)
                    .then(|| *ptr::addr_of!((*ptr).pose.orientation))
                    .unwrap_or_default(),
                position: flags
                    .contains(sys::SpaceLocationFlags::POSITION_VALID)
                    .then(|| *ptr::addr_of!((*ptr).pose.position))
                    .unwrap_or_default(),
            },
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct SpaceVelocity {
    pub linear_velocity: Option<Vector3f>,
    pub angular_velocity: Option<Vector3f>,
}

impl SpaceVelocity {
    unsafe fn new(raw: &MaybeUninit<sys::SpaceVelocity>) -> Self {
        // Applications *must* not read invalid velocities, i.e. they may be uninitialized
        let ptr = raw.as_ptr();
        let flags = *ptr::addr_of!((*ptr).velocity_flags);
        Self {
            linear_velocity: flags
                .contains(sys::SpaceVelocityFlags::LINEAR_VALID)
                .then(|| *ptr::addr_of!((*ptr).linear_velocity)),
            angular_velocity: flags
                .contains(sys::SpaceVelocityFlags::ANGULAR_VALID)
                .then(|| *ptr::addr_of!((*ptr).angular_velocity)),
        }
    }
}

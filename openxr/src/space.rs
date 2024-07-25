use std::{ffi::CString, mem::MaybeUninit, ptr, sync::Arc};

use crate::sys::Handle as _;
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

    #[inline]
    pub(crate) fn session_handle(&self) -> sys::Session {
        self.session.handle
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
        assert_eq!(
            &*self.session as *const session::SessionInner,
            &*base.session as *const session::SessionInner,
            "`self` and `base` must have been created, allocated, or retrieved from the same `Session`"
        );
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
        assert_eq!(
            &*self.session as *const session::SessionInner,
            &*base.session as *const session::SessionInner,
            "`self` and `base` must have been created, allocated, or retrieved from the same `Session`"
        );
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

    fn locate_hand_joints_helper(
        &self,
        tracker: &HandTracker,
        time: Time,
        next: *mut sys::BaseOutStructure,
    ) -> Result<Option<HandJointLocations>> {
        // This assert allows this function to be safe.
        assert_eq!(
            &*self.session as *const session::SessionInner,
            &*tracker.session as *const session::SessionInner,
            "`self` and `tracker` must have been created, allocated, or retrieved from the same `Session`"
        );
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
                next: next as _,
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
        self.locate_hand_joints_helper(tracker, time, ptr::null_mut())
    }

    /// Determine the locations of the joints of a hand tracker relative to this space at a
    /// specified time, if currently known by the runtime, as well as the data source used
    /// to generate the hand joints.
    ///
    /// XR_EXT_hand_tracking and XR_EXT_hand_tracking_data_source must be enabled, and the
    /// hand tracker must have been created with [`HandTracker::create_with_data_sources`].
    #[inline]
    pub fn locate_hand_joints_with_data_source(
        &self,
        tracker: &HandTracker,
        time: Time,
    ) -> Result<Option<(HandJointLocations, HandTrackingDataSourceEXT)>> {
        let mut data_source_state = sys::HandTrackingDataSourceStateEXT::out(ptr::null_mut());
        Ok(self
            .locate_hand_joints_helper(tracker, time, data_source_state.as_mut_ptr() as _)?
            .map(|locations| {
                (
                    locations,
                    unsafe { data_source_state.assume_init() }.data_source,
                )
            }))
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
        let mut velocities = MaybeUninit::<[HandJointVelocity; HAND_JOINT_COUNT]>::uninit();
        let mut velocity_info = sys::HandJointVelocitiesEXT {
            ty: sys::HandJointVelocitiesEXT::TYPE,
            next: ptr::null_mut(),
            joint_count: HAND_JOINT_COUNT as u32,
            joint_velocities: velocities.as_mut_ptr() as _,
        };

        Ok(self
            .locate_hand_joints_helper(tracker, time, &mut velocity_info as *mut _ as _)?
            .map(|locations| (locations, unsafe { velocities.assume_init() })))
    }

    /// Determine the locations and velocities of the joints of a hand tracker relative to this
    /// space at a specified time, if currently known by the runtime, as well as the data source
    /// used to generate the hand joints.
    ///
    /// XR_EXT_hand_tracking and XR_EXT_hand_tracking_data_source must be enabled, and the
    /// hand tracker must have been created with [`HandTracker::create_with_data_sources`].
    #[inline]
    pub fn relate_hand_joints_with_data_source(
        &self,
        tracker: &HandTracker,
        time: Time,
    ) -> Result<
        Option<(
            HandJointLocations,
            HandJointVelocities,
            HandTrackingDataSourceEXT,
        )>,
    > {
        let mut data_source_state = sys::HandTrackingDataSourceStateEXT::out(ptr::null_mut());

        let mut velocities = MaybeUninit::<[HandJointVelocity; HAND_JOINT_COUNT]>::uninit();
        let mut velocity_info = sys::HandJointVelocitiesEXT {
            ty: sys::HandJointVelocitiesEXT::TYPE,
            next: data_source_state.as_mut_ptr() as _,
            joint_count: HAND_JOINT_COUNT as u32,
            joint_velocities: velocities.as_mut_ptr() as _,
        };

        Ok(self
            .locate_hand_joints_helper(tracker, time, &mut velocity_info as *mut _ as _)?
            .map(|locations| {
                (
                    locations,
                    unsafe { velocities.assume_init() },
                    unsafe { data_source_state.assume_init() }.data_source,
                )
            }))
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance.fp()
    }
}

impl AsHandle for Space {
    type Handle = sys::Space;
    fn as_handle(&self) -> Self::Handle {
        self.handle
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
        unsafe {
            let flags = *ptr::addr_of!((*ptr).location_flags);
            Self {
                location_flags: flags,
                pose: Posef {
                    orientation: if flags.contains(sys::SpaceLocationFlags::ORIENTATION_VALID) {
                        *ptr::addr_of!((*ptr).pose.orientation)
                    } else {
                        Default::default()
                    },
                    position: if flags.contains(sys::SpaceLocationFlags::POSITION_VALID) {
                        *ptr::addr_of!((*ptr).pose.position)
                    } else {
                        Default::default()
                    },
                },
            }
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct SpaceVelocity {
    pub velocity_flags: SpaceVelocityFlags,
    pub linear_velocity: Vector3f,
    pub angular_velocity: Vector3f,
}

impl SpaceVelocity {
    unsafe fn new(raw: &MaybeUninit<sys::SpaceVelocity>) -> Self {
        // Applications *must* not read invalid velocities, i.e. they may be uninitialized
        let ptr = raw.as_ptr();
        unsafe {
            let flags = *ptr::addr_of!((*ptr).velocity_flags);
            Self {
                velocity_flags: flags,
                linear_velocity: if flags.contains(sys::SpaceVelocityFlags::LINEAR_VALID) {
                    *ptr::addr_of!((*ptr).linear_velocity)
                } else {
                    Default::default()
                },
                angular_velocity: if flags.contains(sys::SpaceVelocityFlags::ANGULAR_VALID) {
                    *ptr::addr_of!((*ptr).angular_velocity)
                } else {
                    Default::default()
                },
            }
        }
    }
}

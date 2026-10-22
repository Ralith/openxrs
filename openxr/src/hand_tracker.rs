use std::{ptr, sync::Arc};

use crate::sys::Handle as _;
use crate::*;

pub use sys::HandEXT as Hand;
pub use sys::HandJointEXT as HandJoint;
pub use sys::HandJointLocationEXT as HandJointLocation;
pub use sys::HandJointVelocityEXT as HandJointVelocity;

pub const HAND_JOINT_COUNT: usize = sys::HAND_JOINT_COUNT_EXT;

pub struct HandTracker {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::HandTrackerEXT,
}

impl HandTracker {
    #[inline]
    pub fn as_raw(&self) -> sys::HandTrackerEXT {
        self.handle
    }

    /// Take ownership of an existing hand tracker
    ///
    /// # Safety
    ///
    /// `handle` must be a valid hand tracker handle associated with `session`.
    #[inline]
    pub unsafe fn from_raw<G>(session: &Session<G>, handle: sys::HandTrackerEXT) -> Self {
        Self {
            handle,
            session: session.inner.clone(),
        }
    }

    fn create_helper<G>(
        session: &Session<G>,
        hand: Hand,
        next: *const sys::BaseInStructure,
    ) -> Result<Self> {
        let Some(fp) = session.inner.instance.exts().ext_hand_tracking.as_ref() else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut handle = sys::HandTrackerEXT::NULL;
        let info = sys::HandTrackerCreateInfoEXT {
            ty: sys::HandTrackerCreateInfoEXT::TYPE,
            next: next as _,
            hand,
            // If this ever changes, update the joint_counts set in `Space::locate_hand_joints`
            hand_joint_set: sys::HandJointSetEXT::DEFAULT,
        };
        unsafe {
            cvt((fp.create_hand_tracker)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?;
        }
        Ok(HandTracker {
            session: session.inner.clone(),
            handle,
        })
    }

    pub(crate) fn create<G>(session: &Session<G>, hand: Hand) -> Result<Self> {
        Self::create_helper(session, hand, ptr::null())
    }

    pub(crate) fn create_with_data_sources<G>(
        session: &Session<G>,
        hand: Hand,
        data_sources: &[HandTrackingDataSourceEXT],
    ) -> Result<Self> {
        if session
            .inner
            .instance
            .exts()
            .ext_hand_tracking_data_source
            .is_none()
        {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        }

        let data_source_info = sys::HandTrackingDataSourceInfoEXT {
            ty: sys::HandTrackingDataSourceInfoEXT::TYPE,
            next: ptr::null(),
            requested_data_source_count: data_sources.len() as u32,
            requested_data_sources: data_sources.as_ptr() as *mut _, // Runtime shouldn't mutate this anyway
        };

        Self::create_helper(session, hand, &data_source_info as *const _ as _)
    }

    #[inline]
    pub(crate) fn fp(&self) -> &raw::HandTrackingEXT {
        self.session
            .instance
            .exts()
            .ext_hand_tracking
            .as_ref()
            .expect("Somehow created HandTracker without XR_EXT_hand_tracking being enabled")
    }
}

impl AsHandle for HandTracker {
    type Handle = sys::HandTrackerEXT;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Drop for HandTracker {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_hand_tracker)(self.handle);
        }
    }
}

/// An array of `HandJointLocation`s, one for each `HandJoint`.
///
/// `HandJoint`s can be used directly as an index for convenience.
pub type HandJointLocations = [HandJointLocation; HAND_JOINT_COUNT];

/// An array of `HandJointVelocity`s, one for each `HandJoint`.
///
/// `HandJoint`s can be used directly as an index for convenience.
pub type HandJointVelocities = [HandJointVelocity; HAND_JOINT_COUNT];

use std::{ptr, sync::Arc};

use crate::*;

pub use sys::HandEXT as Hand;
pub use sys::HandJointEXT as HandJoint;
pub use sys::HandJointLocationEXT as HandJointLocation;
pub use sys::HandJointVelocityEXT as HandJointVelocity;

pub const HAND_JOINT_COUNT: usize = sys::HAND_JOINT_COUNT_EXT as usize;

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

    pub(crate) fn create<G>(session: &Session<G>, hand: Hand) -> Result<Self> {
        let fp = session.inner.instance.exts().ext_hand_tracking.as_ref();
        let fp = if let Some(fp) = fp {
            fp
        } else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut out = sys::HandTrackerEXT::NULL;
        let info = sys::HandTrackerCreateInfoEXT {
            ty: sys::HandTrackerCreateInfoEXT::TYPE,
            next: ptr::null(),
            hand,
            // If this ever changes, update the joint_counts set in `Space::locate_hand_joints`
            hand_joint_set: sys::HandJointSetEXT::DEFAULT,
        };
        let handle = unsafe {
            cvt((fp.create_hand_tracker)(session.as_raw(), &info, &mut out))?;
            out
        };
        Ok(HandTracker {
            session: session.inner.clone(),
            handle,
        })
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

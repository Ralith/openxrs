use std::{mem, ptr, sync::Arc};

use crate::sys::Handle as _;
use crate::*;

const FACE_CONFIDENCE_REGIONS_COUNT: usize =
    FaceConfidenceRegionsANDROID::RIGHT_UPPER.into_raw() as usize + 1;

pub struct FaceTrackerANDROID {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::FaceTrackerANDROID,
}

impl FaceTrackerANDROID {
    #[inline]
    pub fn as_raw(&self) -> sys::FaceTrackerANDROID {
        self.handle
    }

    /// Take ownership of an existing face tracker
    ///
    /// # Safety
    ///
    /// `handle` must be a valid face tracker handle associated with `session`.
    #[inline]
    pub unsafe fn from_raw<G>(session: &Session<G>, handle: sys::FaceTrackerANDROID) -> Self {
        Self {
            handle,
            session: session.inner.clone(),
        }
    }

    pub(crate) fn create<G>(session: &Session<G>) -> Result<Self> {
        let Some(fp) = session.inner.instance.exts().android_face_tracking.as_ref() else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut handle = sys::FaceTrackerANDROID::NULL;
        let info = sys::FaceTrackerCreateInfoANDROID {
            ty: sys::FaceTrackerCreateInfoANDROID::TYPE,
            next: ptr::null(),
        };
        unsafe {
            cvt((fp.create_face_tracker)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?
        };
        Ok(FaceTrackerANDROID {
            session: session.inner.clone(),
            handle,
        })
    }

    pub fn get_calibration_state(&self) -> Result<bool> {
        let fp = self.fp();

        let mut is_calibrated = sys::FALSE;
        unsafe {
            cvt((fp.get_face_calibration_state)(
                self.handle,
                &mut is_calibrated,
            ))?;
        }
        Ok(is_calibrated.into())
    }

    pub fn get_state(&self, time: Time) -> Result<Option<FaceStateANDROID>> {
        let fp = self.fp();

        let mut parameters: [f32; sys::FACE_PARAMETER_COUNT_ANDROID] = [0.0; _];
        let mut region_confidences: [f32; FACE_CONFIDENCE_REGIONS_COUNT] = [0.0; _];
        let mut out = sys::FaceStateANDROID {
            ty: sys::FaceStateANDROID::TYPE,
            next: ptr::null_mut(),
            parameters_capacity_input: parameters.len() as _,
            parameters: parameters.as_mut_ptr(),
            region_confidences_capacity_input: region_confidences.len() as _,
            region_confidences: region_confidences.as_mut_ptr(),
            ..unsafe { mem::zeroed() }
        };
        let info = sys::FaceStateGetInfoANDROID {
            ty: sys::FaceStateGetInfoANDROID::TYPE,
            next: ptr::null(),
            time,
        };
        unsafe {
            cvt((fp.get_face_state)(self.handle, &info, &mut out))?;
        }
        if out.is_valid.into() {
            Ok(Some(FaceStateANDROID {
                tracking_state: out.face_tracking_state,
                sample_time: out.sample_time,
                parameters,
                region_confidences,
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn fp(&self) -> &raw::FaceTrackingANDROID {
        self.session
            .instance
            .exts()
            .android_face_tracking
            .as_ref()
            .expect(
                "Somehow created FaceTrackerANDROID without XR_ANDROID_face_tracking being enabled",
            )
    }
}

/// Result from calls to [`FaceTrackerANDROID::get_state`]
#[derive(Debug, Copy, Clone)]
pub struct FaceStateANDROID {
    pub tracking_state: FaceTrackingStateANDROID,
    pub sample_time: Time,
    pub parameters: [f32; sys::FACE_PARAMETER_COUNT_ANDROID],
    pub region_confidences: [f32; FACE_CONFIDENCE_REGIONS_COUNT],
}

impl AsHandle for FaceTrackerANDROID {
    type Handle = sys::FaceTrackerANDROID;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Drop for FaceTrackerANDROID {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_face_tracker)(self.handle);
        }
    }
}

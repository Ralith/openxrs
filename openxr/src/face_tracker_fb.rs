use std::{mem, ptr, sync::Arc};

use crate::sys::Handle as _;
use crate::*;

const FACE_EXPRESSION_COUNT: usize = FaceExpression2FB::COUNT.into_raw() as _;
const FACE_CONFIDENCE_COUNT: usize = FaceConfidence2FB::COUNT.into_raw() as _;

pub struct FaceTrackerFB {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::FaceTracker2FB,
}

impl FaceTrackerFB {
    #[inline]
    pub fn as_raw(&self) -> sys::FaceTracker2FB {
        self.handle
    }

    /// Take ownership of an existing face tracker
    ///
    /// # Safety
    ///
    /// `handle` must be a valid face tracker handle associated with `session`.
    #[inline]
    pub unsafe fn from_raw<G>(session: &Session<G>, handle: sys::FaceTracker2FB) -> Self {
        Self {
            handle,
            session: session.inner.clone(),
        }
    }

    pub(crate) fn create<G>(
        session: &Session<G>,
        face_expression_set: FaceExpressionSet2FB,
        data_sources: &[FaceTrackingDataSource2FB],
    ) -> Result<Self> {
        let Some(fp) = session.inner.instance.exts().fb_face_tracking2.as_ref() else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut handle = sys::FaceTracker2FB::NULL;
        let info = sys::FaceTrackerCreateInfo2FB {
            ty: sys::FaceTrackerCreateInfo2FB::TYPE,
            next: ptr::null(),
            face_expression_set,
            requested_data_source_count: data_sources.len().try_into().unwrap(),
            requested_data_sources: data_sources.as_ptr().cast_mut(),
        };
        unsafe {
            cvt((fp.create_face_tracker2)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?
        };
        Ok(FaceTrackerFB {
            session: session.inner.clone(),
            handle,
        })
    }

    pub fn get_expression_weights(&self, time: Time) -> Result<Option<FaceExpressionWeightsFB>> {
        let fp = self.fp();

        let mut weights: [f32; FACE_EXPRESSION_COUNT] = [0.0; _];
        let mut confidences: [f32; FACE_CONFIDENCE_COUNT] = [0.0; _];
        let mut out = sys::FaceExpressionWeights2FB {
            ty: sys::FaceExpressionWeights2FB::TYPE,
            next: ptr::null_mut(),
            weight_count: weights.len() as _,
            weights: weights.as_mut_ptr(),
            confidence_count: confidences.len() as _,
            confidences: confidences.as_mut_ptr(),
            ..unsafe { mem::zeroed() }
        };
        let info = sys::FaceExpressionInfo2FB {
            ty: sys::FaceExpressionInfo2FB::TYPE,
            next: ptr::null(),
            time,
        };
        unsafe {
            cvt((fp.get_face_expression_weights2)(
                self.handle,
                &info,
                &mut out,
            ))?;
        }
        if out.is_valid.into() {
            Ok(Some(FaceExpressionWeightsFB {
                time: out.time,
                data_source: out.data_source,
                eye_following_blendshapes_valid: out.is_eye_following_blendshapes_valid.into(),
                weights,
                confidences,
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn fp(&self) -> &raw::FaceTracking2FB {
        self.session
            .instance
            .exts()
            .fb_face_tracking2
            .as_ref()
            .expect("Somehow created FaceTrackerFB without XR_FB_face_tracking2 being enabled")
    }
}

/// Result from calls to [`FaceTrackerFB::get_expression_weights`]
#[derive(Debug, Copy, Clone)]
pub struct FaceExpressionWeightsFB {
    pub time: Time,
    pub data_source: FaceTrackingDataSource2FB,
    pub eye_following_blendshapes_valid: bool,
    pub weights: [f32; FACE_EXPRESSION_COUNT],
    pub confidences: [f32; FACE_CONFIDENCE_COUNT],
}

impl AsHandle for FaceTrackerFB {
    type Handle = sys::FaceTracker2FB;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Drop for FaceTrackerFB {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_face_tracker2)(self.handle);
        }
    }
}

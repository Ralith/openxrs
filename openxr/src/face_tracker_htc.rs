use std::{mem, ptr, sync::Arc};

use crate::sys::Handle as _;
use crate::*;

pub struct FacialTrackerHTC {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::FacialTrackerHTC,
    tracking_type: FacialTrackingTypeHTC,
}

impl FacialTrackerHTC {
    #[inline]
    pub fn as_raw(&self) -> sys::FacialTrackerHTC {
        self.handle
    }

    /// Take ownership of an existing face tracker
    ///
    /// # Safety
    ///
    /// `handle` must be a valid face tracker handle associated with `session`.
    #[inline]
    pub unsafe fn from_raw<G>(
        session: &Session<G>,
        handle: sys::FacialTrackerHTC,
        tracking_type: FacialTrackingTypeHTC,
    ) -> Self {
        Self {
            handle,
            session: session.inner.clone(),
            tracking_type,
        }
    }

    pub(crate) fn create<G>(
        session: &Session<G>,
        tracking_type: FacialTrackingTypeHTC,
    ) -> Result<Self> {
        let Some(fp) = session.inner.instance.exts().htc_facial_tracking.as_ref() else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut handle = sys::FacialTrackerHTC::NULL;
        let info = sys::FacialTrackerCreateInfoHTC {
            ty: sys::FacialTrackerCreateInfoHTC::TYPE,
            next: ptr::null(),
            facial_tracking_type: tracking_type,
        };
        unsafe {
            cvt((fp.create_facial_tracker)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?
        };
        Ok(FacialTrackerHTC {
            session: session.inner.clone(),
            handle,
            tracking_type,
        })
    }

    pub fn get_expressions(&self, time: Time) -> Result<Option<FacialExpressionsHTC>> {
        let fp = self.fp();

        let mut expressions = Vec::with_capacity(match self.tracking_type {
            FacialTrackingTypeHTC::EYE_DEFAULT => sys::FACIAL_EXPRESSION_EYE_COUNT_HTC,
            FacialTrackingTypeHTC::LIP_DEFAULT => sys::FACIAL_EXPRESSION_LIP_COUNT_HTC,
            _ => unreachable!(),
        });
        let mut out = sys::FacialExpressionsHTC {
            ty: sys::FacialExpressionsHTC::TYPE,
            next: ptr::null_mut(),
            sample_time: time,
            expression_count: expressions.capacity() as _,
            expression_weightings: expressions.as_mut_ptr(),
            ..unsafe { mem::zeroed() }
        };
        unsafe {
            cvt((fp.get_facial_expressions)(self.handle, &mut out))?;
        }
        if out.is_active.into() {
            Ok(Some(FacialExpressionsHTC {
                sample_time: out.sample_time,
                expressions,
            }))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub(crate) fn fp(&self) -> &raw::FacialTrackingHTC {
        self.session
            .instance
            .exts()
            .htc_facial_tracking
            .as_ref()
            .expect("Somehow created FacialTrackerHTC without XR_HTC_facial_tracking being enabled")
    }
}

/// Result from calls to [`FacialTrackerHTC::get_expressions`]
#[derive(Debug, Clone)]
pub struct FacialExpressionsHTC {
    pub sample_time: Time,
    pub expressions: Vec<f32>,
}

impl AsHandle for FacialTrackerHTC {
    type Handle = sys::FacialTrackerHTC;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl Drop for FacialTrackerHTC {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_facial_tracker)(self.handle);
        }
    }
}

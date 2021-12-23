use std::sync::Arc;

use crate::*;

#[derive(Clone)]
pub struct FoveationProfileFB {
    inner: Arc<FoveationProfileFBInner>,
}

pub struct FoveationLevelProfile {
    pub level: FoveationLevelFB,
    pub vertical_offset: f32,
    pub dynamic: FoveationDynamicFB,
}

impl FoveationProfileFB {
    /// Take ownership of an existing foveation profile handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid foveation profile handle created with a [Session] associated with `instance`.
    #[inline]
    pub unsafe fn from_raw(instance: Instance, handle: sys::FoveationProfileFB) -> Self {
        Self {
            inner: Arc::new(FoveationProfileFBInner { instance, handle }),
        }
    }

    #[inline]
    pub fn as_raw(&self) -> sys::FoveationProfileFB {
        self.inner.handle
    }
}

impl<G> Session<G> {
    pub fn create_foveation_profile(
        &self,
        level_profile: Option<FoveationLevelProfile>,
    ) -> Result<FoveationProfileFB> {
        let fp = self
            .instance()
            .exts()
            .fb_foveation
            .as_ref()
            .ok_or(sys::Result::ERROR_EXTENSION_NOT_PRESENT)?;

        let mut level_profile = level_profile.map(|lp| sys::FoveationLevelProfileCreateInfoFB {
            ty: sys::FoveationLevelProfileCreateInfoFB::TYPE,
            next: std::ptr::null_mut(),
            vertical_offset: lp.vertical_offset,
            level: lp.level,
            dynamic: lp.dynamic,
        });
        let next = if let Some(level_profile) = level_profile.as_mut() {
            level_profile as *mut _ as *mut _
        } else {
            std::ptr::null_mut()
        };

        let mut create_info = sys::FoveationProfileCreateInfoFB {
            ty: sys::FoveationProfileCreateInfoFB::TYPE,
            next,
        };
        let mut profile = sys::FoveationProfileFB::NULL;
        let res =
            unsafe { (fp.create_foveation_profile)(self.as_raw(), &mut create_info, &mut profile) };
        cvt(res)?;

        Ok(unsafe { FoveationProfileFB::from_raw(self.instance().clone(), profile) })
    }
}

struct FoveationProfileFBInner {
    instance: Instance,
    handle: sys::FoveationProfileFB,
}

impl Drop for FoveationProfileFBInner {
    fn drop(&mut self) {
        if let Some(fp) = self.instance.exts().fb_foveation.as_ref() {
            unsafe { (fp.destroy_foveation_profile)(self.handle) };
        }
    }
}

use sys::BaseOutStructure;

use crate::*;

pub struct FoveationProfileFB {
    inner: FoveationProfileFBInner,
}

pub struct FoveationLevelProfile {
    pub level: FoveationLevelFB,
    pub vertical_offset: f32,
    pub dynamic: FoveationDynamicFB,
}

impl FoveationProfileFB {
    pub unsafe fn create<G>(
        session: Session<G>,
        level_profile: Option<FoveationLevelProfile>,
    ) -> Result<Self> {
        let instance = session.instance();
        let entry = instance.entry();
        let fp = raw::FoveationFB::load(entry, instance.as_raw())?;

        let mut level_profile = level_profile.map(|lp| {
            let mut c = sys::FoveationLevelProfileCreateInfoFB::out(std::ptr::null_mut());
            (*c.as_mut_ptr()).vertical_offset = lp.vertical_offset;
            (*c.as_mut_ptr()).level = lp.level;
            (*c.as_mut_ptr()).dynamic = lp.dynamic;
            c.assume_init()
        });
        let next = if let Some(level_profile) = level_profile.as_mut() {
            std::mem::transmute::<_, &mut BaseOutStructure>(level_profile)
        } else {
            std::ptr::null_mut()
        };

        let mut create_info = sys::FoveationProfileCreateInfoFB::out(next).assume_init();
        let mut profile = sys::FoveationProfileFB::NULL;
        let res = (fp.create_foveation_profile)(session.as_raw(), &mut create_info, &mut profile);

        if res.into_raw() < 0 {
            return Err(res);
        }

        Ok(Self {
            inner: FoveationProfileFBInner(profile),
        })
    }

    pub unsafe fn destroy<G>(self, session: Session<G>) -> Result<()> {
        let instance = session.instance();
        let entry = instance.entry();
        let fp = raw::FoveationFB::load(entry, instance.as_raw())?;

        let res = (fp.destroy_foveation_profile)(self.inner.0);
        if res.into_raw() < 0 {
            return Err(res);
        }

        Ok(())
    }

    #[inline]
    pub unsafe fn from_raw(handle: sys::FoveationProfileFB) -> Self {
        Self {
            inner: FoveationProfileFBInner(handle),
        }
    }

    #[inline]
    pub fn as_raw(&self) -> sys::FoveationProfileFB {
        self.inner.0
    }
}

struct FoveationProfileFBInner(sys::FoveationProfileFB);

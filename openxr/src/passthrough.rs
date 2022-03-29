//! The [Passthrough feature](https://support.oculus.com/articles/in-vr-experiences/oculus-features/what-is-passthrough/)
//! allows you to step outside your view in VR to see a real-time view of your surroundings.
//!
//! This feature is exclusive to the Oculus Quest 2 as of March 2022.
//!
//! More details about passthrough can be found in the [Oculus Native SDK documentation](https://developer.oculus.com/documentation/native/android/mobile-passthrough/)
//! as well as in the [OpenXR specification](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XrPassthroughFB)
//!
//! Not all features are currently implemented. Execution control features are implemented, style-oriented features are not.
use crate::{
    cvt, raw, session, sys, PassthroughFlagsFB, PassthroughLayerPurposeFB, Result, Session,
    SessionInner,
};
use std::ptr;
use std::sync::Arc;
use sys::PassthroughLayerFB;

/// A [passthrough feature].
///
/// Requires [`XR_FB_passthrough`] to be enabled.
///
/// See the [`PassthroughFB struct`].
///
/// [`XR_FB_passthrough`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough
/// [`PassthroughFB struct`]: https://docs.rs/openxr-sys/latest/openxr_sys/struct.PassthroughFB.html
/// [passthrough feature]: https://developer.oculus.com/documentation/native/android/mobile-passthrough/#create-and-start-a-passthrough-feature
pub struct Passthrough {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::PassthroughFB,
}

impl Passthrough {
    /// [Create](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#xrCreatePassthroughFB)
    /// a passthrough feature.
    pub(crate) fn create<G>(session: &Session<G>, flags: PassthroughFlagsFB) -> Result<Self> {
        let info = sys::PassthroughCreateInfoFB {
            ty: sys::PassthroughCreateInfoFB::TYPE,
            next: ptr::null(),
            flags,
        };
        let fp = fp(&session.inner);
        let mut handle = sys::PassthroughFB::NULL;
        unsafe {
            cvt((fp.create_passthrough)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?;
        }
        Ok(Passthrough {
            session: session.inner.clone(),
            handle,
        })
    }

    /// [Start](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#_xrpassthroughstartfb3)
    /// a passthrough feature.
    pub fn start(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.passthrough_pause)(self.handle))?;
        }
        Ok(())
    }

    /// [Pause](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#_xrpassthroughstartfb3)
    /// a passthrough feature.
    pub fn pause(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.passthrough_pause)(self.handle))?;
        }
        Ok(())
    }
}

impl Drop for Passthrough {
    fn drop(&mut self) {
        unsafe {
            (fp(&self.session).destroy_passthrough)(self.handle);
        }
    }
}

/// A [passthrough layer].
///
/// Requires [`XR_FB_passthrough`].
///
/// [`XR_FB_passthrough`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_passthrough
/// [passthrough layer]: https://developer.oculus.com/documentation/native/android/mobile-passthrough/#create-and-start-a-passthrough-layer
pub struct PassthroughLayer {
    pub(crate) session: Arc<session::SessionInner>,
    handle: sys::PassthroughLayerFB,
}

impl PassthroughLayer {
    /// [Create](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#xrCreatePassthroughLayerFB)
    /// a passthrough layer.
    pub(crate) fn create<G>(
        session: &Session<G>,
        passthrough: &Passthrough,
        flags: PassthroughFlagsFB,
        purpose: PassthroughLayerPurposeFB,
    ) -> Result<Self> {
        let info = sys::PassthroughLayerCreateInfoFB {
            ty: sys::PassthroughLayerCreateInfoFB::TYPE,
            next: ptr::null(),
            passthrough: passthrough.handle,
            flags,
            purpose,
        };
        let fp = fp(&session.inner);
        let mut handle = sys::PassthroughLayerFB::NULL;
        unsafe {
            cvt((fp.create_passthrough_layer)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?;
        }
        Ok(PassthroughLayer {
            session: session.inner.clone(),
            handle,
        })
    }

    /// [Resume](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#_xrpassthroughlayerresumefb3)
    /// a passthrough layer.
    pub fn resume(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.passthrough_layer_resume)(self.handle))?;
        }
        Ok(())
    }

    /// [Pause](https://www.khronos.org/registry/OpenXR/specs/1.0/man/html/openxr.html#_xrpassthroughlayerpausefb3)
    /// a passthrough layer.
    pub fn pause(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.passthrough_layer_pause)(self.handle))?;
        }
        Ok(())
    }

    pub fn inner(&self) -> &PassthroughLayerFB {
        &self.handle
    }
}

impl Drop for PassthroughLayer {
    fn drop(&mut self) {
        unsafe {
            (fp(&self.session).destroy_passthrough_layer)(self.handle);
        }
    }
}

#[inline]
pub(crate) fn fp(session: &SessionInner) -> &raw::PassthroughFB {
    session
        .instance
        .exts()
        .fb_passthrough
        .as_ref()
        .expect("`XR_FB_passthrough` not loaded")
}

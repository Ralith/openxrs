//! Implements the functions from [`XR_FB_display_refresh_rate`].
//! The extension also includes a new event ([`Event::DisplayRefreshRateChangedFB`]) that gets
//! sent when the display refresh rate changes.
//!
//! [`XR_FB_display_refresh_rate`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate
//! [`Event::DisplayRefreshRateChangedFB`]: crate::Event::DisplayRefreshRateChangedFB

use crate::{cvt, Session};
use crate::{get_arr, Result};
use std::mem::MaybeUninit;

impl<G> Session<G> {
    /// [Enumerates] the supported display refresh rates.
    /// Requires [`XR_FB_display_refresh_rate`]
    ///
    /// [Enumerates]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateDisplayRefreshRatesFB
    /// [`XR_FB_display_refresh_rate`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate
    pub fn enumerate_display_refresh_rates(&self) -> Result<Vec<f32>> {
        let ext = self
            .inner
            .instance
            .exts()
            .fb_display_refresh_rate
            .as_ref()
            .expect("XR_FB_display_refresh_rate not loaded");
        get_arr(|cap, count, buf| unsafe {
            (ext.enumerate_display_refresh_rates)(self.as_raw(), cap, count, buf)
        })
    }

    /// Retrieves the [current display refresh rate].
    /// Requires [`XR_FB_display_refresh_rate`]
    ///
    /// [current display refresh rate]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetDisplayRefreshRateFB
    /// [`XR_FB_display_refresh_rate`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate
    pub fn get_display_refresh_rate(&self) -> Result<f32> {
        let ext = self
            .inner
            .instance
            .exts()
            .fb_display_refresh_rate
            .as_ref()
            .expect("XR_FB_display_refresh_rate not loaded");
        unsafe {
            let mut out = MaybeUninit::uninit();
            cvt((ext.get_display_refresh_rate)(
                self.as_raw(),
                out.as_mut_ptr(),
            ))?;
            Ok(out.assume_init())
        }
    }

    /// [Requests] a change to the `display_refresh_rate`.
    /// Requires [`XR_FB_display_refresh_rate`]
    ///
    /// [Requests]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestDisplayRefreshRateFB
    /// [`XR_FB_display_refresh_rate`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_FB_display_refresh_rate
    pub fn request_display_refresh_rate(&self, display_refresh_rate: f32) -> Result<()> {
        let ext = self
            .inner
            .instance
            .exts()
            .fb_display_refresh_rate
            .as_ref()
            .expect("XR_FB_display_refresh_rate not loaded");
        cvt(unsafe { (ext.request_display_refresh_rate)(self.as_raw(), display_refresh_rate) })?;
        Ok(())
    }
}

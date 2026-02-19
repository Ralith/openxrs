//! Implements the functions from [`XR_META_recommended_layer_resolution`].
//!
//! [`XR_META_recommended_layer_resolution`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_recommended_layer_resolution

use crate::*;

use std::ptr;

impl<G: Graphics> Session<G> {
    /// Requires [`XR_META_recommended_layer_resolution`]
    ///
    /// [`XR_META_recommended_layer_resolution`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_META_recommended_layer_resolution
    pub fn get_recommended_layer_resolution(
        &self,
        display_time: Time,
        layer: &CompositionLayerBase<'_, G>,
    ) -> Result<Option<Extent2Di>> {
        let ext = self
            .inner
            .instance
            .exts()
            .meta_recommended_layer_resolution
            .as_ref()
            .expect("XR_META_recommended_layer_resolution not loaded");

        let resolution = unsafe {
            let mut resolution = sys::RecommendedLayerResolutionMETA::out(ptr::null_mut());
            let info = sys::RecommendedLayerResolutionGetInfoMETA {
                ty: sys::RecommendedLayerResolutionGetInfoMETA::TYPE,
                next: ptr::null(),
                layer: layer as *const _ as *const _,
                predicted_display_time: display_time,
            };
            cvt((ext.get_recommended_layer_resolution)(
                self.as_raw(),
                &info,
                resolution.as_mut_ptr(),
            ))?;
            resolution.assume_init()
        };

        if resolution.is_valid.into() {
            Ok(Some(resolution.recommended_image_dimensions))
        } else {
            Ok(None)
        }
    }
}

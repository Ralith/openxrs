use std::ptr;

use crate::*;

/// Handle for managing frame presentation
///
/// This is a secondary interface to a `Session` object that exposes only the frame wait/begin/end
/// operations. These are separated so that `&mut self` receivers can be used to statically
/// guarantee that calls are synchronized as required by OpenXR, enabling a safe interface.
///
/// # Example
///
/// A typical presentation loop body should look roughly as follows:
///
/// ```no_run
/// # fn dummy<G: openxr::Graphics>(
/// #     session: &openxr::Session<G>,
/// #     swapchain: &mut openxr::Swapchain<G>,
/// #     frame_waiter: &mut openxr::FrameWaiter,
/// #     frame_stream: &mut openxr::FrameStream<G>,
/// #     world_space: &openxr::Space,
/// #     view_resolution: &[openxr::Extent2Di],
/// # ) {
/// let state = frame_waiter.wait().unwrap();
/// let image = swapchain.acquire_image().unwrap();
/// swapchain.wait_image(openxr::Duration::INFINITE).unwrap();
///
/// frame_stream.begin().unwrap();
///
/// if state.should_render {
///     // draw scene...
/// }
///
/// let (view_flags, views) = session
///     .locate_views(
///         openxr::ViewConfigurationType::PRIMARY_STEREO,
///         state.predicted_display_time,
///         world_space,
///     )
///     .unwrap();
///
/// // set view matrices and submit to GPU...
///
/// swapchain.release_image().unwrap();
/// frame_stream
///     .end(
///         state.predicted_display_time,
///         openxr::EnvironmentBlendMode::OPAQUE,
///         &[&openxr::CompositionLayerProjection::new()
///             .space(world_space)
///             .views(&[
///                 openxr::CompositionLayerProjectionView::new()
///                     .pose(views[0].pose)
///                     .fov(views[0].fov)
///                     .sub_image(
///                         openxr::SwapchainSubImage::new()
///                             .swapchain(swapchain)
///                             .image_array_index(0)
///                             .image_rect(openxr::Rect2Di {
///                                 offset: openxr::Offset2Di { x: 0, y: 0 },
///                                 extent: view_resolution[0],
///                             }),
///                     ),
///                 openxr::CompositionLayerProjectionView::new()
///                     .pose(views[1].pose)
///                     .fov(views[1].fov)
///                     .sub_image(
///                         openxr::SwapchainSubImage::new()
///                             .swapchain(swapchain)
///                             .image_array_index(1)
///                             .image_rect(openxr::Rect2Di {
///                                 offset: openxr::Offset2Di { x: 0, y: 0 },
///                                 extent: view_resolution[1],
///                             }),
///                     ),
///             ])],
///     )
///     .unwrap();
/// # }
/// ```
pub struct FrameStream<G: Graphics> {
    session: Session<G>,
}

impl<G: Graphics> FrameStream<G> {
    pub(crate) fn new(session: Session<G>) -> Self {
        Self { session }
    }

    /// Indicate that graphics device work is beginning
    #[inline]
    pub fn begin(&mut self) -> Result<()> {
        unsafe {
            cvt((self.fp().begin_frame)(self.session.as_raw(), ptr::null()))?;
        }
        Ok(())
    }

    /// Indicate that all graphics work for the frame has been submitted
    ///
    /// `layers` is an array of references to any type of composition layer,
    /// e.g. `CompositionLayerProjection`.
    #[inline]
    pub fn end(
        &mut self,
        display_time: Time,
        environment_blend_mode: EnvironmentBlendMode,
        layers: &[&CompositionLayerBase<'_, G>],
    ) -> Result<()> {
        assert!(layers.len() <= u32::max_value() as usize);
        let info = sys::FrameEndInfo {
            ty: sys::FrameEndInfo::TYPE,
            next: ptr::null(),
            display_time,
            environment_blend_mode,
            layer_count: layers.len() as u32,
            layers: layers.as_ptr() as _,
        };
        unsafe {
            cvt((self.fp().end_frame)(self.session.as_raw(), &info))?;
        }
        Ok(())
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance().fp()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FrameState {
    pub predicted_display_time: Time,
    pub predicted_display_period: Duration,
    pub should_render: bool,
}

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
        self.assert_layers_validity(layers);
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

    /// Indicate that all graphics work for the frame has been submitted
    ///
    /// `layers` is an array of references to any type of composition layer,
    /// e.g. `CompositionLayerProjection`.
    ///
    /// `secondary_info` has the same information for the secondary layer
    ///
    /// `XR_MSFT_secondary_view_configuration` must be loaded and the session
    /// must have been started with begin_secondary
    #[inline]
    pub fn end_secondary(
        &mut self,
        display_time: Time,
        environment_blend_mode: EnvironmentBlendMode,
        layers: &[&CompositionLayerBase<'_, G>],
        secondary_info: SecondaryEndInfo<'_, '_, '_, G>,
    ) -> Result<()> {
        self.assert_layers_validity(layers);
        self.assert_layers_validity(secondary_info.layers);
        let single_secondary_info = [sys::SecondaryViewConfigurationLayerInfoMSFT {
            ty: sys::SecondaryViewConfigurationLayerInfoMSFT::TYPE,
            next: ptr::null(),
            view_configuration_type: secondary_info.ty,
            environment_blend_mode: secondary_info.environment_blend_mode,
            layer_count: secondary_info.layers.len() as u32,
            layers: secondary_info.layers.as_ptr() as *const _,
        }];
        let secondary_info = sys::SecondaryViewConfigurationFrameEndInfoMSFT {
            ty: sys::SecondaryViewConfigurationFrameEndInfoMSFT::TYPE,
            next: ptr::null(),
            view_configuration_count: 1,
            view_configuration_layers_info: single_secondary_info.as_ptr() as *const _,
        };
        let info = sys::FrameEndInfo {
            ty: sys::FrameEndInfo::TYPE,
            next: &secondary_info as *const _ as *const _,
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

    /// Check the invariants of the passed `layers`.
    /// The lifetimes guarantee the validity of non-null handles.
    fn assert_layers_validity(&self, layers: &[&CompositionLayerBase<'_, G>]) {
        // TODO `space` and `swapchain` must be from the same session (only in the spec for cube)
        assert!(layers.len() <= u32::MAX as usize);
        for &layer in layers {
            match layer.as_raw().ty {
                sys::CompositionLayerProjection::TYPE => {
                    let layer = unsafe {
                        std::mem::transmute::<
                            &CompositionLayerBase<G>,
                            &CompositionLayerProjection<G>,
                        >(layer)
                    }
                    .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert!(layer.view_count > 0);
                    let views = unsafe {
                        std::slice::from_raw_parts(layer.views, layer.view_count as usize)
                    };
                    for view in views {
                        assert_ne!(view.sub_image.swapchain, sys::Swapchain::NULL);
                    }
                }
                sys::CompositionLayerQuad::TYPE => {
                    let layer = unsafe {
                        std::mem::transmute::<&CompositionLayerBase<G>, &CompositionLayerQuad<G>>(
                            layer,
                        )
                    }
                    .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert_ne!(layer.sub_image.swapchain, sys::Swapchain::NULL);
                }
                sys::CompositionLayerCylinderKHR::TYPE => {
                    assert!(self
                        .session
                        .instance()
                        .exts()
                        .khr_composition_layer_cylinder
                        .is_some());
                    let layer = unsafe {
                        std::mem::transmute::<
                            &CompositionLayerBase<G>,
                            &CompositionLayerCylinderKHR<G>,
                        >(layer)
                    }
                    .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert_ne!(layer.sub_image.swapchain, sys::Swapchain::NULL);
                }
                sys::CompositionLayerCubeKHR::TYPE => {
                    assert!(self
                        .session
                        .instance()
                        .exts()
                        .khr_composition_layer_cube
                        .is_some());
                    let layer =
                        unsafe {
                            std::mem::transmute::<
                                &CompositionLayerBase<G>,
                                &CompositionLayerCubeKHR<G>,
                            >(layer)
                        }
                        .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert_ne!(layer.swapchain, sys::Swapchain::NULL);
                }
                sys::CompositionLayerEquirectKHR::TYPE => {
                    assert!(self
                        .session
                        .instance()
                        .exts()
                        .khr_composition_layer_equirect
                        .is_some());
                    let layer = unsafe {
                        std::mem::transmute::<
                            &CompositionLayerBase<G>,
                            &CompositionLayerEquirectKHR<G>,
                        >(layer)
                    }
                    .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert_ne!(layer.sub_image.swapchain, sys::Swapchain::NULL);
                }
                sys::CompositionLayerEquirect2KHR::TYPE => {
                    assert!(self
                        .session
                        .instance()
                        .exts()
                        .khr_composition_layer_equirect2
                        .is_some());
                    let layer = unsafe {
                        std::mem::transmute::<
                            &CompositionLayerBase<G>,
                            &CompositionLayerEquirect2KHR<G>,
                        >(layer)
                    }
                    .as_raw();
                    assert_ne!(layer.space, sys::Space::NULL);
                    assert_ne!(layer.sub_image.swapchain, sys::Swapchain::NULL);
                }
                ty => {
                    panic!("unsupported layer type: {:?}", ty)
                }
            }
        }
    }
}

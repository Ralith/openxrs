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
///         &[&openxr::CompositionLayer::Projection {
///             layer_flags: Default::default(),
///             space: world_space,
///             views: &[
///                 openxr::CompositionLayerProjectionView {
///                     pose: views[0].pose,
///                     fov: views[0].fov,
///                     sub_image: openxr::SwapchainSubImage {
///                         swapchain,
///                         image_array_index: 0,
///                         image_rect: openxr::Rect2Di {
///                             offset: openxr::Offset2Di { x: 0, y: 0 },
///                             extent: view_resolution[0],
///                         },
///                     },
///                 },
///                 openxr::CompositionLayerProjectionView {
///                     pose: views[1].pose,
///                     fov: views[1].fov,
///                     sub_image: openxr::SwapchainSubImage {
///                         swapchain,
///                         image_array_index: 1,
///                         image_rect: openxr::Rect2Di {
///                             offset: openxr::Offset2Di { x: 0, y: 0 },
///                             extent: view_resolution[1],
///                         },
///                     },
///                 },
///             ],
///         }],
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
        layers: &[&CompositionLayer<'_, G>],
    ) -> Result<()> {
        let layers = layers
            .iter()
            .inspect(|layer| self.assert_layer_validity(layer))
            .map(|layer| layer.as_raw())
            .collect::<Vec<_>>();
        assert!(layers.len() <= u32::MAX as usize);
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
        layers: &[&CompositionLayer<'_, G>],
        secondary_info: SecondaryEndInfo<'_, '_, '_, G>,
    ) -> Result<()> {
        let layers = layers
            .iter()
            .inspect(|layer| self.assert_layer_validity(layer))
            .map(|layer| layer.as_raw())
            .collect::<Vec<_>>();
        assert!(layers.len() <= u32::MAX as usize);
        let secondary_layers = secondary_info
            .layers
            .iter()
            .inspect(|layer| self.assert_layer_validity(layer))
            .map(|layer| layer.as_raw())
            .collect::<Vec<_>>();
        assert!(secondary_layers.len() <= u32::MAX as usize);
        let single_secondary_info = [sys::SecondaryViewConfigurationLayerInfoMSFT {
            ty: sys::SecondaryViewConfigurationLayerInfoMSFT::TYPE,
            next: ptr::null(),
            view_configuration_type: secondary_info.ty,
            environment_blend_mode: secondary_info.environment_blend_mode,
            layer_count: secondary_layers.len() as u32,
            layers: secondary_layers.as_ptr() as *const _,
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

    /// Check the invariants of the passed `layer`.
    /// The lifetimes guarantee the validity of non-null handles.
    fn assert_layer_validity(&self, layer: &CompositionLayer<'_, G>) {
        match layer {
            CompositionLayer::Projection { space, views, .. } => {
                assert!(!views.is_empty());
                assert_eq!(self.session.as_raw(), space.session_handle());
                for CompositionLayerProjectionView { sub_image, .. } in *views {
                    assert_eq!(self.session.as_raw(), sub_image.swapchain.session_handle());
                }
            }
            CompositionLayer::Quad {
                space, sub_image, ..
            } => {
                assert_eq!(space.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.face_count(), 1);
            }
            CompositionLayer::CylinderKHR {
                space, sub_image, ..
            } => {
                assert!(self
                    .session
                    .instance()
                    .exts()
                    .khr_composition_layer_cylinder
                    .is_some());
                assert_eq!(space.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.face_count(), 1);
            }
            CompositionLayer::CubeKHR {
                space, swapchain, ..
            } => {
                assert!(self
                    .session
                    .instance()
                    .exts()
                    .khr_composition_layer_cube
                    .is_some());
                assert_eq!(space.session_handle(), self.session.as_raw());
                assert_eq!(swapchain.session_handle(), self.session.as_raw());
                assert_eq!(swapchain.face_count(), 6);
            }
            CompositionLayer::EquirectKHR {
                space, sub_image, ..
            } => {
                assert!(self
                    .session
                    .instance()
                    .exts()
                    .khr_composition_layer_equirect
                    .is_some());
                assert_eq!(space.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.face_count(), 1);
            }
            CompositionLayer::Equirect2KHR {
                space, sub_image, ..
            } => {
                assert!(self
                    .session
                    .instance()
                    .exts()
                    .khr_composition_layer_equirect2
                    .is_some());
                assert_eq!(space.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.session_handle(), self.session.as_raw());
                assert_eq!(sub_image.swapchain.face_count(), 1);
            }
        }
    }
}

use std::{mem, ptr};

use crate::*;

/// Handle for managing frame presentation
///
///
/// # Example
/// ```no_run
/// # fn dummy<G: openxr::Graphics>(
/// #     session: &openxr::Session<G>,
/// #     swapchain: &openxr::Swapchain<G>,
/// #     frame_stream: &mut openxr::FrameStream<G>,
/// #     world_space: &openxr::Space,
/// #     view_resolution: &[openxr::Extent2Di],
/// # ) {
///     let state = frame_stream.wait().unwrap();
///     let (view_flags, views) = session
///         .locate_views(state.predicted_display_time, world_space)
///         .unwrap();
///     let status = frame_stream.begin().unwrap();
///     if status != openxr::sys::Result::SESSION_VISIBILITY_UNAVAILABLE {
///         // draw views...
///     }
///     frame_stream
///         .end(
///             state.predicted_display_time,
///             openxr::EnvironmentBlendMode::OPAQUE,
///             &[&openxr::CompositionLayerProjection::new()
///                 .space(world_space)
///                 .views(&[
///                     openxr::CompositionLayerProjectionView::new()
///                         .pose(views[0].pose)
///                         .fov(views[0].fov)
///                         .sub_image(
///                             openxr::SwapchainSubImage::new()
///                                 .swapchain(swapchain)
///                                 .image_array_index(0)
///                                 .image_rect(openxr::Rect2Di {
///                                     offset: openxr::Offset2Di { x: 0, y: 0 },
///                                     extent: view_resolution[0],
///                                 }),
///                         ),
///                     openxr::CompositionLayerProjectionView::new()
///                         .pose(views[1].pose)
///                         .fov(views[1].fov)
///                         .sub_image(
///                             openxr::SwapchainSubImage::new()
///                                 .swapchain(swapchain)
///                                 .image_array_index(1)
///                                 .image_rect(openxr::Rect2Di {
///                                     offset: openxr::Offset2Di { x: 0, y: 0 },
///                                     extent: view_resolution[1],
///                                 }),
///                         ),
///                 ])],
///         )
///         .unwrap();
/// # }
/// ```
pub struct FrameStream<G: Graphics> {
    session: Session<G>,
    state: State,
}

impl<G: Graphics> FrameStream<G> {
    pub(crate) fn new(session: Session<G>) -> Self {
        Self {
            session,
            state: State::End,
        }
    }

    /// Block until rendering should begin
    #[inline]
    pub fn wait(&mut self) -> Result<FrameState> {
        assert_eq!(self.state, State::End, "wait must be called after end");
        let mut out = sys::FrameState {
            ty: sys::FrameState::TYPE,
            next: ptr::null_mut(),
            ..unsafe { mem::uninitialized() }
        };
        unsafe {
            cvt((self.fp().wait_frame)(
                self.session.as_raw(),
                builder::FrameWaitInfo::new().as_raw(),
                &mut out,
            ))?;
        }
        self.state = State::Wait;
        Ok(FrameState {
            predicted_display_time: out.predicted_display_time,
            predicted_display_period: out.predicted_display_period,
        })
    }

    /// Indicate that graphics device work is beginning
    #[inline]
    pub fn begin(&mut self) -> Result<sys::Result> {
        assert_eq!(self.state, State::Wait, "begin must be called after wait");
        let x = unsafe {
            cvt((self.fp().begin_frame)(
                self.session.as_raw(),
                builder::FrameBeginInfo::new().as_raw(),
            ))?
        };
        self.state = State::Begin;
        Ok(x)
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
        assert_eq!(self.state, State::Begin, "end must be called after begin");
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
        self.state = State::End;
        Ok(())
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance().fp()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    End,
    Wait,
    Begin,
}

#[derive(Debug, Copy, Clone)]
pub struct FrameState {
    pub predicted_display_time: Time,
    pub predicted_display_period: Duration,
}

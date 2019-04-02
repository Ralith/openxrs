use std::{mem, ptr};

use crate::*;

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

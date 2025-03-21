use std::{marker::PhantomData, ptr, sync::Arc};

use crate::sys::Handle as _;
use crate::*;

pub struct Action<T: ActionTy> {
    inner: Arc<ActionInner>,
    _marker: PhantomData<T>,
}

impl<T: ActionTy> Action<T> {
    /// Take ownership of an existing action handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid action handle associated with `set`.
    #[inline]
    pub unsafe fn from_raw(set: ActionSet, handle: sys::Action) -> Self {
        Self {
            inner: Arc::new(ActionInner { set, handle }),
            _marker: PhantomData,
        }
    }

    /// Access the raw swapchain handle
    #[inline]
    pub fn as_raw(&self) -> sys::Action {
        self.inner.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        self.inner.set.instance()
    }

    /// Set the debug name of this `Action`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        self.instance().set_name_raw(self.as_raw().into_raw(), name)
    }

    /// Input sources currently bound to this action
    #[inline]
    pub fn bound_sources<G>(&self, session: &Session<G>) -> Result<Vec<Path>> {
        let info = sys::BoundSourcesForActionEnumerateInfo {
            ty: sys::BoundSourcesForActionEnumerateInfo::TYPE,
            next: ptr::null(),
            action: self.as_raw(),
        };
        get_arr(|cap, count, buf| unsafe {
            (self.fp().enumerate_bound_sources_for_action)(session.as_raw(), &info, cap, count, buf)
        })
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.instance().fp()
    }
}

impl<T: ActionTy> AsHandle for Action<T> {
    type Handle = sys::Action;
    fn as_handle(&self) -> Self::Handle {
        self.inner.handle
    }
}

impl<T: ActionTy> Clone for Action<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: ActionInput> Action<T> {
    /// Retrieve the current state
    pub fn state<G>(&self, session: &Session<G>, subaction_path: Path) -> Result<ActionState<T>> {
        T::get(self, session, subaction_path)
    }
}

impl Action<Posef> {
    /// Creates a `Space` relative to this action
    pub fn create_space<G>(
        &self,
        session: &Session<G>,
        subaction_path: Path,
        pose_in_action_space: Posef,
    ) -> Result<Space> {
        let info = sys::ActionSpaceCreateInfo {
            ty: sys::ActionSpaceCreateInfo::TYPE,
            next: ptr::null(),
            action: self.as_raw(),
            subaction_path,
            pose_in_action_space,
        };
        let mut out = sys::Space::NULL;
        unsafe {
            cvt((self.fp().create_action_space)(
                session.as_raw(),
                &info,
                &mut out,
            ))?;
            Ok(Space::action_from_raw(self.clone(), session.clone(), out))
        }
    }

    pub fn is_active<G>(&self, session: &Session<G>, subaction_path: Path) -> Result<bool> {
        let info = sys::ActionStateGetInfo {
            ty: sys::ActionStateGetInfo::TYPE,
            next: ptr::null(),
            action: self.as_raw(),
            subaction_path,
        };
        let out = unsafe {
            let mut out = sys::ActionStatePose::out(ptr::null_mut());
            cvt((self.fp().get_action_state_pose)(
                session.as_raw(),
                &info,
                out.as_mut_ptr(),
            ))?;
            out.assume_init()
        };
        Ok(out.is_active.into())
    }
}

impl Action<Haptic> {
    pub fn apply_feedback<G>(
        &self,
        session: &Session<G>,
        subaction_path: Path,
        event: &HapticBase,
    ) -> Result<()> {
        let info = sys::HapticActionInfo {
            ty: sys::HapticActionInfo::TYPE,
            next: ptr::null(),
            action: self.as_raw(),
            subaction_path,
        };
        unsafe {
            cvt((self.fp().apply_haptic_feedback)(
                session.as_raw(),
                &info,
                event as *const _ as _,
            ))?;
        }
        Ok(())
    }

    pub fn stop_feedback<G>(&self, session: &Session<G>, subaction_path: Path) -> Result<()> {
        let info = sys::HapticActionInfo {
            ty: sys::HapticActionInfo::TYPE,
            next: ptr::null(),
            action: self.as_raw(),
            subaction_path,
        };
        unsafe {
            cvt((self.fp().stop_haptic_feedback)(session.as_raw(), &info))?;
        }
        Ok(())
    }
}

pub trait ActionTy: Sized {
    #[doc(hidden)]
    const TYPE: ActionType;
}

#[derive(Debug, Copy, Clone)]
pub struct ActionState<T: ActionInput> {
    pub current_state: T,
    pub changed_since_last_sync: bool,
    pub last_change_time: Time,
    pub is_active: bool,
}

pub trait ActionInput: ActionTy {
    #[doc(hidden)]
    fn get<G>(
        action: &Action<Self>,
        session: &Session<G>,
        subaction_path: Path,
    ) -> Result<ActionState<Self>>;
}

impl ActionTy for bool {
    const TYPE: ActionType = ActionType::BOOLEAN_INPUT;
}

impl ActionInput for bool {
    fn get<G>(
        action: &Action<Self>,
        session: &Session<G>,
        subaction_path: Path,
    ) -> Result<ActionState<Self>> {
        let info = sys::ActionStateGetInfo {
            ty: sys::ActionStateGetInfo::TYPE,
            next: ptr::null_mut(),
            action: action.as_raw(),
            subaction_path,
        };
        unsafe {
            let mut out = sys::ActionStateBoolean::out(ptr::null_mut());
            cvt((action.fp().get_action_state_boolean)(
                session.as_raw(),
                &info,
                out.as_mut_ptr(),
            ))?;
            let out = out.assume_init();
            Ok(ActionState {
                current_state: out.current_state.into(),
                changed_since_last_sync: out.changed_since_last_sync.into(),
                last_change_time: out.last_change_time,
                is_active: out.is_active.into(),
            })
        }
    }
}

impl ActionTy for f32 {
    const TYPE: ActionType = ActionType::FLOAT_INPUT;
}

impl ActionInput for f32 {
    fn get<G>(
        action: &Action<Self>,
        session: &Session<G>,
        subaction_path: Path,
    ) -> Result<ActionState<Self>> {
        let info = sys::ActionStateGetInfo {
            ty: sys::ActionStateGetInfo::TYPE,
            next: ptr::null_mut(),
            action: action.as_raw(),
            subaction_path,
        };
        unsafe {
            let mut out = sys::ActionStateFloat::out(ptr::null_mut());
            cvt((action.fp().get_action_state_float)(
                session.as_raw(),
                &info,
                out.as_mut_ptr(),
            ))?;
            let out = out.assume_init();
            Ok(ActionState {
                current_state: out.current_state,
                changed_since_last_sync: out.changed_since_last_sync.into(),
                last_change_time: out.last_change_time,
                is_active: out.is_active.into(),
            })
        }
    }
}

impl ActionTy for Vector2f {
    const TYPE: ActionType = ActionType::VECTOR2F_INPUT;
}

impl ActionInput for Vector2f {
    fn get<G>(
        action: &Action<Self>,
        session: &Session<G>,
        subaction_path: Path,
    ) -> Result<ActionState<Self>> {
        let info = sys::ActionStateGetInfo {
            ty: sys::ActionStateGetInfo::TYPE,
            next: ptr::null_mut(),
            action: action.as_raw(),
            subaction_path,
        };
        unsafe {
            let mut out = sys::ActionStateVector2f::out(ptr::null_mut());
            cvt((action.fp().get_action_state_vector2f)(
                session.as_raw(),
                &info,
                out.as_mut_ptr(),
            ))?;
            let out = out.assume_init();
            Ok(ActionState {
                current_state: out.current_state,
                changed_since_last_sync: out.changed_since_last_sync.into(),
                last_change_time: out.last_change_time,
                is_active: out.is_active.into(),
            })
        }
    }
}

impl ActionTy for Posef {
    const TYPE: ActionType = ActionType::POSE_INPUT;
}

/// Tag for haptic output actions
pub struct Haptic;

impl ActionTy for Haptic {
    const TYPE: ActionType = ActionType::VIBRATION_OUTPUT;
}

pub(crate) struct ActionInner {
    set: ActionSet,
    handle: sys::Action,
}

impl Drop for ActionInner {
    fn drop(&mut self) {
        unsafe {
            (self.set.instance().fp().destroy_action)(self.handle);
        }
    }
}

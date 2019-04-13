use std::{ffi::CString, marker::PhantomData, mem, ptr, sync::Arc};

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
        if let Some(fp) = self.instance().exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::ACTION,
                object_handle: self.as_raw().into_raw(),
                object_name: name.as_ptr(),
            };
            unsafe {
                cvt((fp.set_debug_utils_object_name)(
                    self.instance().as_raw(),
                    &info,
                ))?;
            }
        }
        Ok(())
    }

    /// Input sources currently bound to this action
    #[inline]
    pub fn bound_sources(&self) -> Result<Vec<Path>> {
        get_arr(|cap, count, buf| unsafe {
            (self.fp().get_bound_sources_for_action)(self.as_raw(), cap, count, buf)
        })
    }

    pub(crate) fn set(&self) -> &ActionSet {
        &self.inner.set
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.instance().fp()
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
    pub fn state(&self, subaction_paths: &[Path]) -> Result<ActionState<T>> {
        T::get(self, subaction_paths)
    }
}

impl Action<Posef> {
    /// Creates a `Space` relative to this action
    pub fn create_space(&self, subaction_path: Path, pose_in_action_space: Posef) -> Result<Space> {
        let info = sys::ActionSpaceCreateInfo {
            ty: sys::ActionSpaceCreateInfo::TYPE,
            next: ptr::null(),
            subaction_path,
            pose_in_action_space,
        };
        let mut out = sys::Space::NULL;
        unsafe {
            cvt((self.fp().create_action_space)(
                self.as_raw(),
                &info,
                &mut out,
            ))?;
            Ok(Space::action_from_raw(self.clone(), out))
        }
    }
}

impl Action<Haptic> {
    pub fn apply_feedback(&self, subaction_paths: &[Path], event: &HapticBase) -> Result<()> {
        unsafe {
            cvt((self.fp().apply_haptic_feedback)(
                self.as_raw(),
                subaction_paths.len() as u32,
                subaction_paths.as_ptr(),
                event as *const _ as _,
            ))?;
        }
        Ok(())
    }

    pub fn stop_feedback(&self, subaction_paths: &[Path]) -> Result<()> {
        unsafe {
            cvt((self.fp().stop_haptic_feedback)(
                self.as_raw(),
                subaction_paths.len() as u32,
                subaction_paths.as_ptr(),
            ))?;
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
    fn get(action: &Action<Self>, subaction_paths: &[Path]) -> Result<ActionState<Self>>;
}

impl ActionTy for bool {
    const TYPE: ActionType = ActionType::INPUT_BOOLEAN;
}

impl ActionInput for bool {
    fn get(action: &Action<Self>, subaction_paths: &[Path]) -> Result<ActionState<Self>> {
        unsafe {
            let mut out = sys::ActionStateBoolean {
                ty: sys::ActionStateBoolean::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((action.fp().get_action_state_boolean)(
                action.as_raw(),
                subaction_paths.len() as u32,
                subaction_paths.as_ptr(),
                &mut out,
            ))?;
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
    const TYPE: ActionType = ActionType::INPUT_VECTOR1F;
}

impl ActionInput for f32 {
    fn get(action: &Action<Self>, subaction_paths: &[Path]) -> Result<ActionState<Self>> {
        unsafe {
            let mut out = sys::ActionStateVector1f {
                ty: sys::ActionStateBoolean::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((action.fp().get_action_state_vector1f)(
                action.as_raw(),
                subaction_paths.len() as u32,
                subaction_paths.as_ptr(),
                &mut out,
            ))?;
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
    const TYPE: ActionType = ActionType::INPUT_VECTOR2F;
}

impl ActionInput for Vector2f {
    fn get(action: &Action<Self>, subaction_paths: &[Path]) -> Result<ActionState<Self>> {
        unsafe {
            let mut out = sys::ActionStateVector2f {
                ty: sys::ActionStateBoolean::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            cvt((action.fp().get_action_state_vector2f)(
                action.as_raw(),
                subaction_paths.len() as u32,
                subaction_paths.as_ptr(),
                &mut out,
            ))?;
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
    const TYPE: ActionType = ActionType::INPUT_POSE;
}

/// Tag for haptic output actions
pub struct Haptic;

impl ActionTy for Haptic {
    const TYPE: ActionType = ActionType::OUTPUT_VIBRATION;
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

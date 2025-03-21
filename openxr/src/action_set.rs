use std::sync::Arc;

use crate::sys::Handle as _;
use crate::*;

#[derive(Clone)]
pub struct ActionSet {
    inner: Arc<ActionSetInner>,
}

impl ActionSet {
    /// Take ownership of an existing action set handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid action set handle associated with `session`.
    #[inline]
    pub unsafe fn from_raw(instance: Instance, handle: sys::ActionSet) -> Self {
        Self {
            inner: Arc::new(ActionSetInner { instance, handle }),
        }
    }

    /// Access the raw swapchain handle
    #[inline]
    pub fn as_raw(&self) -> sys::ActionSet {
        self.inner.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.inner.instance
    }

    /// Set the debug name of this `ActionSet`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        self.instance().set_name_raw(self.as_raw().into_raw(), name)
    }

    /// Create a new logical input action
    #[inline]
    pub fn create_action<T: ActionTy>(
        &self,
        name: &str,
        localized_name: &str,
        subaction_paths: &[Path],
    ) -> Result<Action<T>> {
        let info = builder::ActionCreateInfo::new()
            .action_name(name)
            .localized_action_name(localized_name)
            .subaction_paths(subaction_paths)
            .action_type(T::TYPE);
        unsafe {
            let mut out = sys::Action::NULL;
            cvt((self.fp().create_action)(
                self.as_raw(),
                info.as_raw(),
                &mut out,
            ))?;
            Ok(Action::from_raw(self.clone(), out))
        }
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.inner.instance.fp()
    }

    // Private because safety requires that only one copy of the `ActionSet` exist externally.
    pub(crate) fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl AsHandle for ActionSet {
    type Handle = sys::ActionSet;
    fn as_handle(&self) -> Self::Handle {
        self.inner.handle
    }
}

struct ActionSetInner {
    instance: Instance,
    handle: sys::ActionSet,
}

impl Drop for ActionSetInner {
    fn drop(&mut self) {
        unsafe {
            (self.instance.fp().destroy_action_set)(self.handle);
        }
    }
}

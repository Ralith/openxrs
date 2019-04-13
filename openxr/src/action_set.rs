use std::{ffi::CString, ptr, sync::Arc};

use crate::*;

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
    pub unsafe fn from_raw<G: Graphics>(session: Session<G>, handle: sys::ActionSet) -> Self {
        Self {
            inner: Arc::new(ActionSetInner {
                session: session.inner,
                handle,
            }),
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
        &self.inner.session.instance
    }

    /// Set the debug name of this `ActionSet`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        if let Some(fp) = self.instance().exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::ACTION_SET,
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

    pub(crate) fn session(&self) -> &Arc<session::SessionInner> {
        &self.inner.session
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.inner.session.instance.fp()
    }

    // Private because safety requires that only one copy of the `ActionSet` exist externally.
    pub(crate) fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ActionSetInner {
    session: Arc<session::SessionInner>,
    handle: sys::ActionSet,
}

impl Drop for ActionSetInner {
    fn drop(&mut self) {
        unsafe {
            (self.session.instance.fp().destroy_action_set)(self.handle);
        }
    }
}

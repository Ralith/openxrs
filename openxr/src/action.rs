use std::{ptr, sync::Arc, ffi::CString};

use crate::*;

pub struct Action {
    session: Arc<session::SessionInner>,
    handle: sys::Action,
}

impl Action {
    /// Take ownership of an existing action handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid action handle.
    #[inline]
    pub unsafe fn from_raw<G: Graphics>(session: Session<G>, handle: sys::Action) -> Self {
        Self {
            session: session.inner,
            handle,
        }
    }

    /// Access the raw swapchain handle
    #[inline]
    pub fn as_raw(&self) -> sys::Action {
        self.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.session.instance
    }

    /// Set the debug name of this `Action`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&self, name: &str) -> Result<()> {
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

    /// Creates a `Space` based on a chosen action space
    pub fn create_action_space(
        &self,
        subaction_path: Path,
        pose_in_action_space: Posef,
    ) -> Result<Space> {
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
            Ok(Space::new(self.session.clone(), out))
        }
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance.fp()
    }
}

impl Drop for Action {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_action)(self.handle);
        }
    }
}

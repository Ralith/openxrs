use std::{ffi::CString, marker::PhantomData, ptr};

use crate::sys::Handle as _;
use crate::*;

/// A set of images to be rendered to using a particular graphics API `G`
pub struct Swapchain<G: Graphics> {
    session: Session<G>,
    handle: sys::Swapchain,
    _marker: PhantomData<G>,
    /// Whether `wait_image` was called more recently than `release_image`
    waited: bool,
}

impl<G: Graphics> Swapchain<G> {
    /// Take ownership of an existing swapchain handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid swapchain handle associated with `session` and created with `flags`.
    #[inline]
    pub unsafe fn from_raw(session: Session<G>, handle: sys::Swapchain) -> Self {
        Self {
            session,
            handle,
            _marker: PhantomData,
            waited: false,
        }
    }

    /// Access the raw swapchain handle
    #[inline]
    pub fn as_raw(&self) -> sys::Swapchain {
        self.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        self.session.instance()
    }

    /// Set the debug name of this `Swapchain`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        // We don't forward to the locking version on Instance because this object can't be cloned
        if let Some(fp) = self.instance().exts().ext_debug_utils.as_ref() {
            let name = CString::new(name).unwrap();
            let info = sys::DebugUtilsObjectNameInfoEXT {
                ty: sys::DebugUtilsObjectNameInfoEXT::TYPE,
                next: ptr::null(),
                object_type: ObjectType::SWAPCHAIN,
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

    #[inline]
    pub fn enumerate_images(&self) -> Result<Vec<G::SwapchainImage>> {
        G::enumerate_swapchain_images(self)
    }

    /// Determine the index of the next image to render to in the swapchain image array
    #[inline]
    pub fn acquire_image(&mut self) -> Result<u32> {
        let mut out = 0;
        unsafe {
            cvt((self.fp().acquire_swapchain_image)(
                self.as_raw(),
                ptr::null(),
                &mut out,
            ))?;
        }
        Ok(out)
    }

    /// Wait for the compositor to finish reading from the oldest unwaited acquired image
    #[inline]
    pub fn wait_image(&mut self, timeout: Duration) -> Result<()> {
        assert!(
            !self.waited,
            "release_image must be called before wait_image can be called again"
        );
        let info = sys::SwapchainImageWaitInfo {
            ty: sys::SwapchainImageWaitInfo::TYPE,
            next: ptr::null_mut(),
            timeout,
        };
        unsafe {
            cvt((self.fp().wait_swapchain_image)(self.as_raw(), &info))?;
        }
        self.waited = true;
        Ok(())
    }

    /// Release the oldest acquired image
    #[inline]
    pub fn release_image(&mut self) -> Result<()> {
        assert!(
            self.waited,
            "wait_image must be called before release_image"
        );
        unsafe {
            cvt((self.fp().release_swapchain_image)(
                self.as_raw(),
                ptr::null(),
            ))?;
        }
        self.waited = false;
        Ok(())
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance().fp()
    }
}

impl<G: Graphics> AsHandle for Swapchain<G> {
    type Handle = sys::Swapchain;
    fn as_handle(&self) -> Self::Handle {
        self.handle
    }
}

impl<G: Graphics> Drop for Swapchain<G> {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_swapchain)(self.as_raw());
        }
    }
}

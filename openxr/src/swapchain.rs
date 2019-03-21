use std::{ptr, marker::PhantomData};

use crate::*;

/// A set of images to be rendered to using a particular graphics API `G`
pub struct Swapchain<G: Graphics> {
    session: Session<G>,
    handle: sys::Swapchain,
    _marker: PhantomData<G>,
}

impl<G: Graphics> Swapchain<G> {
    /// Take ownership of an existing swapchain handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid swapchain handle.
    #[inline]
    pub unsafe fn from_raw(session: Session<G>, handle: sys::Swapchain) -> Self {
        Self {
            session,
            handle,
            _marker: PhantomData,
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

    pub fn enumerate_images(&self) -> Result<Vec<G::SwapchainImage>> {
        G::enumerate_swapchain_images(self)
    }

    /// Determine the index of the next image to render to in the swapchain image array
    pub fn acquire_image(&self) -> Result<u32> {
        let info = sys::SwapchainImageAcquireInfo {
            ty: sys::SwapchainImageAcquireInfo::TYPE,
            next: ptr::null_mut(),
        };
        let mut out = 0;
        unsafe {
            cvt((self.fp().acquire_swapchain_image)(self.as_raw(), &info, &mut out))?;
        }
        Ok(out)
    }

    /// Wait for the compositor to finish reading from the oldest unwaited acquired image
    ///
    /// # Safety
    ///
    /// Once a swapchain image has been successfully waited on, it must be released before waiting
    /// on the next acquired swapchain image.
    pub unsafe fn wait_image(&self, timeout: Duration) -> Result<()> {
        let info = sys::SwapchainImageWaitInfo {
            ty: sys::SwapchainImageWaitInfo::TYPE,
            next: ptr::null_mut(),
            timeout,
        };
        cvt((self.fp().wait_swapchain_image)(self.as_raw(), &info))?;
        Ok(())
    }

    /// Release the oldest acquired image
    ///
    /// # Safety
    ///
    /// The swapchain image must have been successfully waited on before it is released.
    pub unsafe fn release_image(&self) -> Result<()> {
        let info = sys::SwapchainImageReleaseInfo {
            ty: sys::SwapchainImageReleaseInfo::TYPE,
            next: ptr::null_mut(),
        };
        cvt((self.fp().release_swapchain_image)(self.as_raw(), &info))?;
        Ok(())
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.session.instance().fp()
    }
}

impl<G: Graphics> Drop for Swapchain<G> {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_swapchain)(self.as_raw());
        }
    }
}

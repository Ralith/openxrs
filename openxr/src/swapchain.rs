use std::marker::PhantomData;

use crate::*;

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

    pub fn enumerate_swapchain_images(&self) -> Result<Vec<G::SwapchainImage>> {
        G::enumerate_swapchain_images(self)
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

use std::{marker::PhantomData, ptr, sync::Arc};

use crate::*;

pub struct Session<G: Graphics> {
    inner: Arc<SessionInner>,
    _marker: PhantomData<G>,
}

impl<G: Graphics> Session<G> {
    /// Take ownership of an existing session handle
    ///
    /// # Safety
    ///
    /// `handle` must be a valid session handle.
    #[inline]
    pub unsafe fn from_raw(instance: Instance, handle: sys::Session) -> Self {
        Self {
            inner: Arc::new(SessionInner { instance, handle }),
            _marker: PhantomData,
        }
    }

    /// Access the raw session handle
    #[inline]
    pub fn as_raw(&self) -> sys::Session {
        self.inner.handle
    }

    /// Access the `Instance` self is descended from
    #[inline]
    pub fn instance(&self) -> &Instance {
        &self.inner.instance
    }

    /// Request that the runtime show the application's rendered output to the user
    #[inline]
    pub fn begin(&self, ty: ViewConfigurationType) -> Result<sys::Result> {
        let info = sys::SessionBeginInfo {
            ty: sys::SessionBeginInfo::TYPE,
            next: ptr::null(),
            primary_view_configuration_type: ty,
        };
        unsafe { cvt((self.fp().begin_session)(self.as_raw(), &info)) }
    }

    /// Signals that the application no longer wishes to display rendered output, read input state,
    /// or control haptic events
    #[inline]
    pub fn end(&self) -> Result<sys::Result> {
        unsafe { cvt((self.fp().end_session)(self.as_raw())) }
    }

    /// Enumerate texture formats supported by the current session
    ///
    /// The type of formats returned are dependent on the graphics API for which the session was
    /// created.
    #[inline]
    pub fn enumerate_swapchain_formats(&self) -> Result<Vec<G::Format>> {
        let raw = get_arr(|capacity, count, buf| unsafe {
            (self.fp().enumerate_swapchain_formats)(self.as_raw(), capacity, count, buf)
        })?;
        Ok(raw.into_iter().map(G::raise_format).collect())
    }

    pub fn create_swapchain(&self, info: &SwapchainCreateInfo<G>) -> Result<Swapchain<G>> {
        let mut out = sys::Swapchain::NULL;
        let info = sys::SwapchainCreateInfo {
            ty: sys::SwapchainCreateInfo::TYPE,
            next: ptr::null(),
            create_flags: info.create_flags,
            usage_flags: info.usage_flags,
            format: G::lower_format(info.format),
            sample_count: info.sample_count,
            width: info.width,
            height: info.height,
            face_count: info.face_count,
            array_size: info.array_size,
            mip_count: info.mip_count,
        };
        unsafe {
            cvt((self.fp().create_swapchain)(self.as_raw(), &info, &mut out))?;
            Ok(Swapchain::from_raw(self.clone(), out))
        }
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.inner.instance.fp()
    }
}

impl<G: Graphics> Clone for Session<G> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

struct SessionInner {
    instance: Instance,
    handle: sys::Session,
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        unsafe {
            (self.instance.fp().destroy_session)(self.handle);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SwapchainCreateInfo<G: Graphics> {
    pub create_flags: SwapchainCreateFlags,
    pub usage_flags: SwapchainUsageFlags,
    pub format: G::Format,
    pub sample_count: u32,
    pub width: u32,
    pub height: u32,
    pub face_count: u32,
    pub array_size: u32,
    pub mip_count: u32,
}

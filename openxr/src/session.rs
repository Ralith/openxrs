use std::{marker::PhantomData, mem, ptr, sync::Arc};

use crate::*;

/// A rendering session using a particular graphics API `G`
pub struct Session<G: Graphics> {
    pub(crate) inner: Arc<SessionInner>,
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

    #[inline]
    pub fn reference_space_bounds_rect(&self, ty: ReferenceSpaceType) -> Result<Option<Extent2Df>> {
        unsafe {
            let mut out = mem::uninitialized();
            let status = cvt((self.fp().get_reference_space_bounds_rect)(
                self.as_raw(),
                ty,
                &mut out,
            ))?;
            Ok(if status == sys::Result::SPACE_BOUNDS_UNAVAILABLE {
                None
            } else {
                Some(out)
            })
        }
    }

    /// Enumerates the set of reference space types supported for this session
    ///
    /// Constant for the lifetime of the session.
    #[inline]
    pub fn enumerate_reference_spaces(&self) -> Result<Vec<ReferenceSpaceType>> {
        get_arr(|cap, count, buf| unsafe {
            (self.fp().enumerate_reference_spaces)(self.as_raw(), cap, count, buf)
        })
    }

    /// Creates a `Space` based on a chosen reference space
    pub fn create_reference_space(
        &self,
        reference_space_type: ReferenceSpaceType,
        pose_in_reference_space: Posef,
    ) -> Result<Space> {
        let info = sys::ReferenceSpaceCreateInfo {
            ty: sys::ReferenceSpaceCreateInfo::TYPE,
            next: ptr::null(),
            reference_space_type,
            pose_in_reference_space,
        };
        let mut out = sys::Space::NULL;
        unsafe {
            cvt((self.fp().create_reference_space)(
                self.as_raw(),
                &info,
                &mut out,
            ))?;
            Ok(Space::from_raw(self.clone(), out))
        }
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

    #[inline]
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

pub(crate) struct SessionInner {
    pub(crate) instance: Instance,
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

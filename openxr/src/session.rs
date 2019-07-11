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
    /// `handle` must be a valid session handle associated with `instance` which is not currently
    /// inside a frame and was created for graphics API `G`.
    #[inline]
    pub unsafe fn from_raw(instance: Instance, handle: sys::Session) -> (Self, FrameStream<G>) {
        let session = Self {
            inner: Arc::new(SessionInner {
                instance: instance.clone(),
                handle,
            }),
            _marker: PhantomData,
        };
        (session.clone(), FrameStream::new(session))
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

    /// Set the debug name of this `Session`, if `XR_EXT_debug_utils` is loaded
    #[inline]
    pub fn set_name(&mut self, name: &str) -> Result<()> {
        self.instance().set_name_raw(self.as_raw().into_raw(), name)
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

    /// Signal that the application no longer wishes to display rendered output, read input state,
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

    /// Enumerate the set of reference space types supported for this session
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
            Ok(Space::reference_from_raw(self.clone(), out))
        }
    }

    /// Enumerate texture formats supported by the current session
    ///
    /// The type of formats returned is dependent on the graphics API for which the session was
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
            Ok(Swapchain::from_raw(self.clone(), out, info.create_flags))
        }
    }

    /// Get the view and projection info for a particular display time
    ///
    /// When rendering, this should be called as late as possible before the GPU accesses it to
    /// provide the most accurate possible poses.
    #[inline]
    pub fn locate_views(
        &self,
        display_time: Time,
        space: &Space,
    ) -> Result<(ViewStateFlags, Vec<View>)> {
        let info = sys::ViewLocateInfo {
            ty: sys::ViewLocateInfo::TYPE,
            next: ptr::null(),
            display_time,
            space: space.as_raw(),
        };
        let (flags, raw) = unsafe {
            let mut out = sys::ViewState {
                ty: sys::ViewState::TYPE,
                next: ptr::null_mut(),
                ..mem::uninitialized()
            };
            let raw = get_arr_init(
                sys::View {
                    ty: sys::View::TYPE,
                    next: ptr::null_mut(),
                    ..mem::uninitialized()
                },
                |cap, count, buf| {
                    (self.fp().locate_views)(self.as_raw(), &info, &mut out, cap, count, buf)
                },
            )?;
            (out.view_state_flags, raw)
        };
        Ok((
            flags,
            raw.into_iter()
                .map(|x| View {
                    pose: x.pose,
                    fov: x.fov,
                })
                .collect(),
        ))
    }

    /// Allocate a new [`ActionSet`]
    ///
    /// [`ActionSet`]: https://www.khronos.org/registry/OpenXR/specs/0.90/html/xrspec.html#input-action-creation
    #[inline]
    pub fn create_action_set(
        &self,
        name: &str,
        localized_name: &str,
        priority: u32,
    ) -> Result<ActionSet> {
        let info = builder::ActionSetCreateInfo::new()
            .action_set_name(name)
            .localized_action_set_name(localized_name)
            .priority(priority);
        unsafe {
            let mut out = sys::ActionSet::NULL;
            cvt((self.fp().create_action_set)(
                self.as_raw(),
                info.as_raw(),
                &mut out,
            ))?;
            Ok(ActionSet::from_raw(self.clone(), out))
        }
    }

    #[inline]
    pub fn set_interaction_profile_suggested_bindings(
        &self,
        interaction_profile: Path,
        bindings: &[Binding],
    ) -> Result<()> {
        let info = sys::InteractionProfileSuggestedBinding {
            ty: sys::InteractionProfileSuggestedBinding::TYPE,
            next: ptr::null(),
            interaction_profile,
            count_suggested_bindings: bindings.len() as u32,
            suggested_bindings: bindings.as_ptr() as *const _ as _,
        };
        unsafe {
            cvt((self.fp().set_interaction_profile_suggested_bindings)(
                self.as_raw(),
                &info,
            ))?;
        }
        Ok(())
    }

    /// Get the suggested interaction profile in use for a top level user path
    ///
    /// May be NULL.
    #[inline]
    pub fn current_interaction_profile(&self, top_level_user_path: Path) -> Result<Path> {
        unsafe {
            let mut out = sys::InteractionProfileInfo {
                ty: sys::InteractionProfileInfo::TYPE,
                next: ptr::null(),
                ..mem::uninitialized()
            };
            cvt((self.fp().get_current_interaction_profile)(
                self.as_raw(),
                top_level_user_path,
                &mut out,
            ))?;
            Ok(out.interaction_profile)
        }
    }

    /// Designate active input actions and update their states
    #[inline]
    pub fn sync_action_data(&self, action_sets: &[ActiveActionSet<'_>]) -> Result<()> {
        unsafe {
            cvt((self.fp().sync_action_data)(
                self.as_raw(),
                action_sets.len() as u32,
                action_sets.as_ptr() as _,
            ))?;
        }
        Ok(())
    }

    /// Get a name for the input source in the current system locale
    #[inline]
    pub fn input_source_localized_name(
        &self,
        source: Path,
        flags: InputSourceLocalizedNameFlags,
    ) -> Result<String> {
        get_str(|cap, count, buf| unsafe {
            (self.fp().get_input_source_localized_name)(
                self.as_raw(),
                source,
                flags,
                cap,
                count,
                buf,
            )
        })
    }

    // Private helper
    #[inline]
    fn fp(&self) -> &raw::Instance {
        self.inner.instance.fp()
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
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
    pub(crate) handle: sys::Session,
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

#[derive(Copy, Clone)]
pub struct View {
    pub pose: Posef,
    pub fov: Fovf,
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Binding<'a> {
    _inner: sys::ActionSuggestedBinding,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Binding<'a> {
    #[inline]
    pub fn new<T: ActionTy>(action: &'a Action<T>, binding: Path) -> Self {
        Self {
            _inner: sys::ActionSuggestedBinding {
                action: action.as_raw(),
                binding: binding,
            },
            _marker: PhantomData,
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ActiveActionSet<'a> {
    _inner: sys::ActiveActionSet,
    _marker: PhantomData<&'a ActionSet>,
}

impl<'a> ActiveActionSet<'a> {
    #[inline]
    pub fn new(action_set: &'a ActionSet) -> Self {
        Self::with_subaction(action_set, Path::NULL)
    }

    #[inline]
    pub fn with_subaction(action_set: &'a ActionSet, subaction_path: Path) -> Self {
        Self {
            _inner: sys::ActiveActionSet {
                ty: sys::ActiveActionSet::TYPE,
                next: ptr::null(),
                action_set: action_set.as_raw(),
                subaction_path,
            },
            _marker: PhantomData,
        }
    }
}

impl<'a> From<&'a ActionSet> for ActiveActionSet<'a> {
    fn from(x: &'a ActionSet) -> Self {
        Self::new(x)
    }
}

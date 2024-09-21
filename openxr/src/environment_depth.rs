//! The [Environment Depth feature](https://developer.oculus.com/documentation/native/android/mobile-depth/)
//! allows you to access depth maps based on the user environment.
//! This is primarily used to occlude virtual objects by real-world objects and surfaces.
//!
//! This feature is exclusive to the Oculus Quest 3 as of September 2024.
//!
//! More details about the environment depth feature can be found in the [OpenXR specification](https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth).
use crate::{
    cvt, raw, sys, Graphics, Instance, Result, Session, SessionInner, Space, SystemId, Time, View,
};
use std::marker::PhantomData;
use std::ptr;
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub struct SystemEnvironmentDepthProperties {
    pub supports_environment_depth: bool,
    pub supports_hand_removal: bool,
}

impl Instance {
    #[inline]
    pub fn environment_depth_properties(
        &self,
        system: SystemId,
    ) -> Result<SystemEnvironmentDepthProperties> {
        unsafe {
            if self.exts().meta_environment_depth.is_none() {
                return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
            }
            let mut env_depth = sys::SystemEnvironmentDepthPropertiesMETA::out(ptr::null_mut());
            let mut p = sys::SystemProperties::out(&mut env_depth as *mut _ as _);
            cvt((self.fp().get_system_properties)(
                self.as_raw(),
                system,
                p.as_mut_ptr(),
            ))?;
            Ok(env_depth.assume_init().into())
        }
    }
}

impl From<sys::SystemEnvironmentDepthPropertiesMETA> for SystemEnvironmentDepthProperties {
    fn from(value: sys::SystemEnvironmentDepthPropertiesMETA) -> Self {
        SystemEnvironmentDepthProperties {
            supports_environment_depth: value.supports_environment_depth.into(),
            supports_hand_removal: value.supports_hand_removal.into(),
        }
    }
}

impl<G: Graphics> Session<G> {
    #[inline]
    /// Create an [`EnvironmentDepthProvider`].
    ///
    /// Requires [`XR_META_environment_depth`].
    ///
    /// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
    /// [`EnvironmentDepthProvider`]: environment_depth/struct.EnvironmentDepthProvider.html
    pub fn create_depth_environment_provider(&self) -> Result<EnvironmentDepthProvider> {
        EnvironmentDepthProvider::create(self)
    }

    #[inline]
    /// Create an [`EnvironmentDepthSwapchain`].
    ///
    /// Requires [`XR_META_environment_depth`].
    ///
    /// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
    /// [`EnvironmentDepthSwapchain`]: environment_depth/struct.EnvironmentDepthSwapchain.html
    pub fn create_depth_environment_swapchain(
        &self,
        provider: &EnvironmentDepthProvider,
    ) -> Result<EnvironmentDepthSwapchain<G>> {
        EnvironmentDepthSwapchain::create(self, provider)
    }
}

/// An [Environment Depth Provider].
///
/// Requires [`XR_META_environment_depth`] to be enabled.
///
/// See the [`EnvironmentDepthProviderMETA struct`].
///
/// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
/// [`EnvironmentDepthProviderMETA struct`]: https://docs.rs/openxr-sys/latest/openxr_sys/struct.EnvironmentDepthProviderMETA.html
/// [Environment Depth Provider]: https://developer.oculus.com/documentation/native/android/mobile-depth/#creating-a-depth-provider
pub struct EnvironmentDepthProvider {
    pub(crate) session: Arc<SessionInner>,
    handle: sys::EnvironmentDepthProviderMETA,
}

impl EnvironmentDepthProvider {
    /// [Create] an environment depth provider.
    ///
    /// [Create]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrCreateEnvironmentDepthProviderMETA
    pub(crate) fn create<G>(session: &Session<G>) -> Result<Self> {
        let info = sys::EnvironmentDepthProviderCreateInfoMETA {
            ty: sys::EnvironmentDepthProviderCreateInfoMETA::TYPE,
            next: ptr::null(),
            create_flags: sys::EnvironmentDepthProviderCreateFlagsMETA::EMPTY,
        };
        let fp = fp(&session.inner);
        let mut handle = sys::EnvironmentDepthProviderMETA::NULL;
        unsafe {
            cvt((fp.create_environment_depth_provider)(
                session.as_raw(),
                &info,
                &mut handle,
            ))?;
        }
        Ok(EnvironmentDepthProvider {
            session: session.inner.clone(),
            handle,
        })
    }

    /// [Start] an environment depth provider.
    ///
    /// [Start]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrStartEnvironmentDepthProviderMETA
    pub fn start(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.start_environment_depth_provider)(self.handle))?;
        }
        Ok(())
    }

    /// [Stop] an environment depth provider.
    ///
    /// [Stop]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrStopEnvironmentDepthProviderMETA
    pub fn stop(&self) -> Result<()> {
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.stop_environment_depth_provider)(self.handle))?;
        }
        Ok(())
    }

    /// Set the [hand removal option].
    ///
    /// [hand removal option]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrSetEnvironmentDepthHandRemovalMETA
    pub fn set_hand_removal(&self, enabled: bool) -> Result<()> {
        let info = sys::EnvironmentDepthHandRemovalSetInfoMETA {
            ty: sys::EnvironmentDepthHandRemovalSetInfoMETA::TYPE,
            next: ptr::null(),
            enabled: enabled.into(),
        };
        let fp = fp(&self.session);
        unsafe {
            cvt((fp.set_environment_depth_hand_removal)(self.handle, &info))?;
        }
        Ok(())
    }

    /// [Acquire] the latest available swapchain image that has been generated.
    ///
    /// [Acquire]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrAcquireEnvironmentDepthImageMETA
    pub fn acquire_image(
        &self,
        space: &Space,
        display_time: Time,
    ) -> Result<EnvironmentDepthImage> {
        let info = sys::EnvironmentDepthImageAcquireInfoMETA {
            ty: sys::EnvironmentDepthImageAcquireInfoMETA::TYPE,
            next: ptr::null(),
            space: space.as_raw(),
            display_time,
        };
        let fp = fp(&self.session);
        let default_img_view = sys::EnvironmentDepthImageViewMETA {
            ty: sys::EnvironmentDepthImageViewMETA::TYPE,
            next: ptr::null(),
            pose: sys::Posef::default(),
            fov: sys::Fovf::default(),
        };
        let mut img = sys::EnvironmentDepthImageMETA {
            ty: sys::EnvironmentDepthImageMETA::TYPE,
            next: ptr::null(),
            near_z: 0.0,
            far_z: 0.0,
            swapchain_index: 0,
            views: [default_img_view, default_img_view],
        };
        unsafe {
            cvt((fp.acquire_environment_depth_image)(
                self.handle,
                &info,
                &mut img,
            ))?;
            Ok(img.into())
        }
    }
}

impl Drop for EnvironmentDepthProvider {
    fn drop(&mut self) {
        unsafe {
            ((fp(&self.session).destroy_environment_depth_provider)(self.handle));
        }
    }
}

/// An [Environment Depth Swapchain].
///
/// Requires [`XR_META_environment_depth`] to be enabled.
///
/// See the [`EnvironmentDepthSwapchainMETA struct`].
///
/// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
/// [`EnvironmentDepthSwapchainMETA struct`]: https://docs.rs/openxr-sys/latest/openxr_sys/struct.EnvironmentDepthSwapchainMETA.html
/// [Environment Depth Swapchain]: https://developer.oculus.com/documentation/native/android/mobile-depth/#creating-and-enumerating-a-depth-swapchain
pub struct EnvironmentDepthSwapchain<G> {
    pub(crate) session: Arc<SessionInner>,
    pub(crate) handle: sys::EnvironmentDepthSwapchainMETA,
    _marker: PhantomData<G>,
}

impl<G: Graphics> EnvironmentDepthSwapchain<G> {
    /// [Create] an environment depth swapchain.
    ///
    /// [Create]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrCreateEnvironmentDepthSwapchainMETA
    pub(crate) fn create(
        session: &Session<G>,
        provider: &EnvironmentDepthProvider,
    ) -> Result<Self> {
        let info = sys::EnvironmentDepthSwapchainCreateInfoMETA {
            ty: sys::EnvironmentDepthSwapchainCreateInfoMETA::TYPE,
            next: ptr::null(),
            create_flags: sys::EnvironmentDepthSwapchainCreateFlagsMETA::EMPTY,
        };
        let fp = fp(&session.inner);
        let mut handle = sys::EnvironmentDepthSwapchainMETA::NULL;
        unsafe {
            cvt((fp.create_environment_depth_swapchain)(
                provider.handle,
                &info,
                &mut handle,
            ))?;
        }
        Ok(EnvironmentDepthSwapchain {
            session: session.inner.clone(),
            handle,
            _marker: PhantomData,
        })
    }

    /// Retrieve the [environment depth swapchain state].
    ///
    /// [environment depth swapchain state]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrGetEnvironmentDepthSwapchainStateMETA
    pub fn get_state(&self) -> Result<EnvironmentDepthSwapchainState> {
        let fp = fp(&self.session);
        let mut state = sys::EnvironmentDepthSwapchainStateMETA {
            ty: sys::EnvironmentDepthSwapchainStateMETA::TYPE,
            next: ptr::null_mut(),
            width: 0,
            height: 0,
        };
        unsafe {
            cvt((fp.get_environment_depth_swapchain_state)(
                self.handle,
                &mut state,
            ))?;
            Ok(state.into())
        }
    }

    #[inline]
    /// [Enumerate] the environment depth swapchain images.
    ///
    /// [`Enumerate`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#xrEnumerateEnvironmentDepthSwapchainImagesMETA
    pub fn enumerate_images(&self) -> Result<Vec<G::SwapchainImage>> {
        G::enumerate_depth_environment_swapchain_images(self)
    }

    #[inline]
    pub(crate) fn fp(&self) -> &raw::EnvironmentDepthMETA {
        fp(&self.session)
    }
}

impl<G> Drop for EnvironmentDepthSwapchain<G> {
    fn drop(&mut self) {
        unsafe {
            ((fp(&self.session).destroy_environment_depth_swapchain)(self.handle));
        }
    }
}

/// An [Environment Depth Swapchain State].
///
/// Requires [`XR_META_environment_depth`] to be enabled.
///
/// See the [`EnvironmentDepthSwapchainStateMETA struct`].
///
/// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
/// [`EnvironmentDepthSwapchainStateMETA struct`]: https://docs.rs/openxr-sys/latest/openxr_sys/struct.EnvironmentDepthSwapchainStateMETA.html
/// [Environment Depth Swapchain State]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthSwapchainStateMETA
#[derive(Debug, Copy, Clone)]
pub struct EnvironmentDepthSwapchainState {
    pub width: u32,
    pub height: u32,
}

impl From<sys::EnvironmentDepthSwapchainStateMETA> for EnvironmentDepthSwapchainState {
    fn from(value: sys::EnvironmentDepthSwapchainStateMETA) -> Self {
        EnvironmentDepthSwapchainState {
            width: value.width,
            height: value.height,
        }
    }
}

/// An [Environment Depth Image].
///
/// Requires [`XR_META_environment_depth`] to be enabled.
///
/// See the [`EnvironmentDepthImageMETA struct`].
///
/// [`XR_META_environment_depth`]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XR_META_environment_depth
/// [`EnvironmentDepthImageMETA struct`]: https://docs.rs/openxr-sys/latest/openxr_sys/struct.EnvironmentDepthImageMETA.html
/// [Environment Depth Image]: https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#XrEnvironmentDepthImageMETA
#[derive(Copy, Clone)]
pub struct EnvironmentDepthImage {
    pub swapchain_index: u32,
    pub near_z: f32,
    pub far_z: f32,
    pub views: [View; 2],
}

impl From<sys::EnvironmentDepthImageMETA> for EnvironmentDepthImage {
    fn from(value: sys::EnvironmentDepthImageMETA) -> Self {
        EnvironmentDepthImage {
            swapchain_index: value.swapchain_index,
            near_z: value.near_z,
            far_z: value.far_z,
            views: value.views.map(|v| View {
                pose: v.pose,
                fov: v.fov,
            }),
        }
    }
}

#[inline]
fn fp(session: &SessionInner) -> &raw::EnvironmentDepthMETA {
    session
        .instance
        .exts()
        .meta_environment_depth
        .as_ref()
        .expect("`XR_META_environment_depth` not loaded")
}

use crate::*;

/// Used to create a session without graphics.
///
/// See [`XR_MND_headless`] for safety details.
///
/// [`XR_MND_headless`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MND_headless
pub enum Headless {}

impl Graphics for Headless {
    type Requirements = Requirements;
    type SessionCreateInfo = SessionCreateInfo;
    type SwapchainImage = u32;
    type Format = i64;

    fn raise_format(x: i64) -> Self::Format {
        x
    }
    fn lower_format(x: Self::Format) -> i64 {
        x
    }

    fn requirements(_instance: &Instance, _system: SystemId) -> Result<Requirements> {
        Ok(Requirements {})
    }

    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        _info: &SessionCreateInfo,
    ) -> Result<sys::Session> {
        let info = sys::SessionCreateInfo {
            ty: sys::SessionCreateInfo::TYPE,
            next: std::ptr::null(),
            create_flags: Default::default(),
            system_id: system,
        };
        let mut out = sys::Session::NULL;
        cvt((instance.fp().create_session)(
            instance.as_raw(),
            &info,
            &mut out,
        ))?;
        Ok(out)
    }

    fn enumerate_swapchain_images(
        _swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        // in a MND_headless session, xrEnumerateSwapchainFormats will always
        // enumerate 0 formats, and so it's not possible to create a swapchain
        unreachable!();
    }
}

#[derive(Clone, Copy)]
pub struct Requirements {}

#[derive(Clone, Copy)]
pub struct SessionCreateInfo {}

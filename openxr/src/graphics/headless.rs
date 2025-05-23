use crate::sys::Handle as _;
use crate::*;

/// Used to create a session without graphics.
///
/// See [`XR_MND_headless`] for details.
///
/// [`XR_MND_headless`]: https://www.khronos.org/registry/OpenXR/specs/1.1/html/xrspec.html#XR_MND_headless
pub enum Headless {}

impl Graphics for Headless {
    type Requirements = Requirements;
    type SessionCreateInfo = SessionCreateInfo;
    type SwapchainImage = HeadlessSwapchainImage;
    type Format = HeadlessFormat;

    fn raise_format(_: i64) -> Self::Format {
        // used by enumerate_swapchain_formats, which returns empty
        unreachable!()
    }
    fn lower_format(f: Self::Format) -> i64 {
        // used by create_swapchain, which is not available in headless
        match f {}
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
        unsafe {
            cvt((instance.fp().create_session)(
                instance.as_raw(),
                &info,
                &mut out,
            ))?;
        }
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

#[derive(Clone, Copy)]
pub enum HeadlessSwapchainImage {}

#[derive(Clone, Copy)]
pub enum HeadlessFormat {}

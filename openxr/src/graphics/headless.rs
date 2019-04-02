use std::ptr;

use crate::*;

/// No graphics API
///
/// See [`XR_KHR_headless`] for details.
///
/// [`XR_KHR_headless`]: https://www.khronos.org/registry/OpenXR/specs/0.90/html/xrspec.html#XR_KHR_headless
pub enum Headless {}

impl Graphics for Headless {
    type Requirements = ();
    type Format = void::Void;
    type SessionCreateInfo = ();
    type SwapchainImage = void::Void;

    fn raise_format(_: i64) -> Self::Format {
        panic!("tried to construct a headless image format")
    }
    fn lower_format(x: Self::Format) -> i64 {
        void::unreachable(x)
    }

    fn requirements(_: &Instance, _: SystemId) -> Result<()> {
        Ok(())
    }

    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        _: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        let info = sys::SessionCreateInfo {
            ty: sys::SessionCreateInfo::TYPE,
            next: ptr::null(),
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

    fn enumerate_swapchain_images(_: &Swapchain<Self>) -> Result<Vec<Self::SwapchainImage>> {
        Ok(Vec::new())
    }
}

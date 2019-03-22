use crate::*;

/// Static dispatch for OpenXR graphics bindings
///
/// The types and functions defined by this trait are an implementation detail, and should not be
/// referenced externally.
pub trait Graphics: Sized {
    /// Compatibility details within this graphics API
    type Requirements;
    /// Parameters required to construct a session for use with this graphics API
    type SessionCreateInfo;
    /// Swapchain image formats
    type Format: Copy;
    /// Identifiers for images to render to
    type SwapchainImage;

    #[doc(hidden)]
    fn raise_format(x: i64) -> Self::Format;
    #[doc(hidden)]
    fn lower_format(x: Self::Format) -> i64;

    #[doc(hidden)]
    fn requirements(instance: &Instance, system: SystemId) -> Result<Self::Requirements>;

    #[doc(hidden)]
    unsafe fn create_session(
        instance: Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<Session<Self>>;

    #[doc(hidden)]
    fn enumerate_swapchain_images(swapchain: &Swapchain<Self>) -> Result<Vec<Self::SwapchainImage>>;
}

mod headless;
pub use headless::Headless;

#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "vulkan")]
pub use vulkan::Vulkan;

#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "opengl")]
pub use opengl::OpenGL;

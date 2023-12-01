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
        instance: &Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session>;

    #[doc(hidden)]
    fn enumerate_swapchain_images(swapchain: &Swapchain<Self>)
        -> Result<Vec<Self::SwapchainImage>>;
}

#[cfg(windows)]
pub mod d3d;
#[cfg(windows)]
pub use d3d::D3D11;
#[cfg(windows)]
pub use d3d::D3D12;

pub mod vulkan;
pub use vulkan::Vulkan;

pub mod opengl;
pub use opengl::OpenGL;

pub mod opengles;
pub use opengles::OpenGlEs;

use crate::*;

/// Static dispatch for OpenXR graphics bindings
pub trait Graphics: Sized {
    type Requirements;
    type SessionCreateInfo;
    /// Swapchain image formats
    type Format: Copy;
    type SwapchainImage;

    fn raise_format(x: i64) -> Self::Format;
    fn lower_format(x: Self::Format) -> i64;

    fn requirements(instance: &Instance, system: SystemId) -> Result<Self::Requirements>;

    unsafe fn create_session(
        instance: Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<Session<Self>>;

    fn enumerate_swapchain_images(swapchain: &Swapchain<Self>) -> Result<Vec<Self::SwapchainImage>>;
}

#[cfg(feature = "vulkan")]
pub mod vulkan;
#[cfg(feature = "vulkan")]
pub use vulkan::Vulkan;

#[cfg(feature = "opengl")]
pub mod opengl;
#[cfg(feature = "opengl")]
pub use opengl::OpenGL;

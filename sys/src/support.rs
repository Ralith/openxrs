//! Static helpers called from generated code
use std::os::raw::{c_ulong, c_void};
use std::fmt;

#[macro_export]
macro_rules! wrapper {
    {$(#[$meta: meta])* $ident:ident($ty:ty)} => {
        $(#[$meta])* #[repr(transparent)]
        pub struct $ident($ty);
        impl $ident {
            pub fn from_raw(x: $ty) -> Self { Self(x) }
            pub fn into_raw(self) -> $ty { self.0 }
        }
    }
}

pub fn fmt_enum(f: &mut fmt::Formatter, value: i32, name: Option<&'static str>) -> fmt::Result {
    match name {
        Some(x) => f.pad(x),
        None => {
            <i32 as fmt::Debug>::fmt(&value, f)
        },
    }
}

#[cfg(feature = "ash")]
pub use ash::vk;

#[cfg(all(feature = "x11", feature = "opengl"))]
mod x11 {
    use super::*;

    pub type XID = c_ulong;
    pub type GLXFBConfig = *mut c_void;
    pub type GLXDrawable = XID;
    pub type GLXContext = *mut c_void;
}
#[cfg(all(feature = "x11", feature = "opengl"))]
pub use x11::*;
#[cfg(feature = "x11")]
pub type Display = c_void;

#[cfg(feature = "libc")]
pub use libc::timespec;

// TODO: OpenGLES, D3D, windows

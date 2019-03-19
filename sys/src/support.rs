//! Static helpers called from generated code
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

#[cfg(feature = "x11")]
pub use x11::glx::{
    GLXFBConfig,
    GLXDrawable,
    GLXContext,
};

#[cfg(feature = "x11-dl")]
pub use x11_dl::glx::{
    GLXFBConfig,
    GLXDrawable,
    GLXContext,
};

#[cfg(feature = "libc")]
pub use libc::timespec;

// TODO: OpenGLES, D3D, windows

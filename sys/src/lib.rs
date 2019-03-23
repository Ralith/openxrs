use std::fmt;

#[macro_use]
mod support;
mod generated;

// Hand-written bindings for cases which are too few or weird to bother automating

wrapper! {
    #[derive(Copy, Clone, Eq, PartialEq)]
    Bool32(u32)
}
pub const TRUE: Bool32 = Bool32(1);
pub const FALSE: Bool32 = Bool32(0);
impl fmt::Display for Bool32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (*self != FALSE).fmt(fmt)
    }
}

impl fmt::Debug for Bool32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (*self != FALSE).fmt(fmt)
    }
}

impl From<Bool32> for bool {
    fn from(x: Bool32) -> Self {
        x != FALSE
    }
}

impl From<bool> for Bool32 {
    fn from(x: bool) -> Self {
        Self(x as _)
    }
}

wrapper! {
    #[derive(Copy, Clone, Eq, PartialEq)]
    Time(i64)
}

impl fmt::Debug for Time {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

wrapper! {
    #[derive(Copy, Clone, Eq, PartialEq)]
    Duration(i64)
}

impl Duration {
    pub const NONE: Self = Self(0);
    pub const INFINITE: Self = Self(0x7fffffffffffffff);
    pub const MIN_HAPTIC: Self = Self(-1);
}

impl fmt::Debug for Duration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    Path(u64)
}

impl Path {
    pub const NULL: Path = Path(0);
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    SystemId(u64)
}

impl SystemId {
    pub const NULL: SystemId = SystemId(0);
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    Version(u32)
}

impl Version {
    #[inline]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(major << 22 | minor << 12 | patch)
    }

    #[inline]
    pub const fn major(self) -> u32 {
        self.0 >> 22
    }

    #[inline]
    pub const fn minor(self) -> u32 {
        (self.0 >> 12) & 0x3fff
    }

    #[inline]
    pub const fn patch(self) -> u32 {
        self.0 & 0xfff
    }
}

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}


#[cfg(all(feature = "xlib", feature = "opengl"))]
mod xlib {
    use std::os::raw::{c_ulong, c_void};

    pub type XID = c_ulong;
    pub type GLXFBConfig = *mut c_void;
    pub type GLXDrawable = XID;
    pub type GLXContext = *mut c_void;
}
#[cfg(all(feature = "xlib", feature = "opengl"))]
pub use xlib::*;
#[cfg(feature = "xlib")]
pub type Display = std::os::raw::c_void;

// TODO: XCB, OpenGLES, D3D, windows

pub use generated::*;

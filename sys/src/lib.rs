use std::fmt;

#[macro_use]
mod support;
mod generated;
pub mod platform;

#[cfg(feature = "nalgebra")]
mod nalgebra;

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

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Time(i64);
impl Time {
    pub fn from_raw(x: i64) -> Self {
        Self(x)
    }

    pub fn as_nanos(&self) -> i64 {
        self.0
    }
}

impl fmt::Debug for Time {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl std::ops::Sub<Time> for Time {
    type Output = Duration;

    fn sub(self, other: Time) -> Duration {
        Duration(self.0.wrapping_sub(other.0))
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Duration(i64);
impl Duration {
    pub fn from_raw(x: i64) -> Self {
        Self(x)
    }

    pub fn as_nanos(&self) -> i64 {
        self.0
    }
}

impl Duration {
    pub const NONE: Self = Self(0);
    pub const INFINITE: Self = Self(0x7fffffffffffffff);
    pub const MIN_HAPTIC: Self = Self(-1);
}

impl fmt::Debug for Duration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        std::time::Duration::from_nanos(self.0 as u64).fmt(fmt)
    }
}

impl From<Duration> for std::time::Duration {
    fn from(x: Duration) -> Self {
        Self::from_nanos(x.0.abs() as u64)
    }
}

wrapper! {
    #[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    Path(u64)
}

impl Path {
    pub const NULL: Path = Path(0);
}

wrapper! {
    #[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
    SystemId(u64)
}

impl SystemId {
    pub const NULL: SystemId = SystemId(0);
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    Version(u64)
}

impl Version {
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u32) -> Self {
        Self((major as u64) << 48 | (minor as u64) << 32 | patch as u64)
    }

    #[inline]
    pub const fn major(self) -> u16 {
        (self.0 >> 48) as u16
    }

    #[inline]
    pub const fn minor(self) -> u16 {
        (self.0 >> 32) as u16
    }

    #[inline]
    pub const fn patch(self) -> u32 {
        self.0 as u32
    }
}

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

pub const FREQUENCY_UNSPECIFIED: f32 = 0.0;

pub use generated::*;

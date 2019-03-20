use std::fmt;

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
        fmt.pad(match *self {
            TRUE => "true",
            FALSE => "false",
            _ => unreachable!("invalid Bool32 value"),
        })
    }
}

impl fmt::Debug for Bool32 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, fmt)
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

impl fmt::Debug for Duration {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    Path(u64)
}

wrapper! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    SystemId(u64)
}

pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    major << 22 | minor << 12 | patch
}

pub const fn version_major(version: u32) -> u32 {
    version >> 22
}

pub const fn version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3fff
}

pub const fn version_patch(version: u32) -> u32 {
    version & 0xfff
}

pub use generated::*;

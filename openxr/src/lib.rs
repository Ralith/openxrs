mod entry;
// mod support;
mod generated;

pub use sys;
pub use entry::*;
pub use generated::*;

pub type Result<T> = std::result::Result<T, sys::Result>;

fn cvt(x: sys::Result) -> Result<()> {
    if x.into_raw() >= 0 { Ok(()) }
    else { Err(x) }
}

//! To get started, construct an `Entry` object.

// deref_addrof false positive: https://github.com/rust-lang/rust-clippy/issues/8247
#![allow(clippy::transmute_ptr_to_ptr, clippy::deref_addrof)]
use std::os::raw::c_char;

pub use sys::{
    self, Duration, Path, SystemId, Time, Version, CURRENT_API_VERSION, FREQUENCY_UNSPECIFIED,
};

mod generated;
pub use generated::*;
mod entry;
pub use entry::*;
mod instance;
pub use instance::*;
mod session;
pub use session::*;
mod frame_stream;
pub use frame_stream::*;
mod graphics;
pub use graphics::*;
mod swapchain;
pub use swapchain::*;
mod space;
pub use space::*;
mod action_set;
pub use action_set::*;
mod action;
pub use action::*;
mod hand_tracker;
pub use hand_tracker::*;
mod secondary_view;
pub use secondary_view::*;
mod foveation_profile_fb;
pub use foveation_profile_fb::*;
mod vive_tracker_paths;
pub use vive_tracker_paths::*;
mod passthrough;
pub use passthrough::*;

pub use builder::{
    CompositionLayerBase, CompositionLayerCubeKHR, CompositionLayerCylinderKHR,
    CompositionLayerEquirectKHR, CompositionLayerProjection, CompositionLayerProjectionView,
    CompositionLayerQuad, HapticBase, HapticVibration, SwapchainSubImage,
};

pub type Result<T, E = sys::Result> = std::result::Result<T, E>;

// Reserved semantic paths
pub const USER_HAND_LEFT: &str = "/user/hand/left";
pub const USER_HAND_RIGHT: &str = "/user/hand/right";
pub const USER_HEAD: &str = "/user/head";
pub const USER_GAMEPAD: &str = "/user/gamepad";
pub const USER_TREADMILL: &str = "/user/treadmill";

// FFI helpers
fn cvt(x: sys::Result) -> Result<sys::Result> {
    if x.into_raw() >= 0 {
        Ok(x)
    } else {
        Err(x)
    }
}

fn place_cstr(out: &mut [c_char], s: &str) {
    if s.len() + 1 > out.len() {
        panic!(
            "string requires {} > {} bytes (including trailing null)",
            s.len(),
            out.len()
        );
    }
    for (i, o) in s.bytes().zip(out.iter_mut()) {
        *o = i as c_char;
    }
    out[s.len()] = 0;
}

unsafe fn fixed_str(x: &[c_char]) -> &str {
    std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(x.as_ptr()).to_bytes())
}

/// Includes null for convenience of comparison with C string constants
fn fixed_str_bytes(x: &[c_char]) -> &[u8] {
    let end = x.iter().position(|&x| x == 0).unwrap();
    unsafe { std::mem::transmute(&x[..=end]) }
}

fn get_str(mut getter: impl FnMut(u32, &mut u32, *mut c_char) -> sys::Result) -> Result<String> {
    let mut bytes = get_arr(|x, y, z| getter(x, y, z as _))?;
    // Truncate at first null byte
    let first_nt = bytes
        .iter()
        .rposition(|&x| x != 0)
        .map(|x| x + 1)
        .unwrap_or(0);
    bytes.truncate(first_nt);

    unsafe { Ok(String::from_utf8_unchecked(bytes)) }
}

fn get_arr<T: Copy>(
    mut getter: impl FnMut(u32, &mut u32, *mut T) -> sys::Result,
) -> Result<Vec<T>> {
    let mut output = 0;
    cvt(getter(0, &mut output, std::ptr::null_mut()))?;
    let mut buffer = Vec::with_capacity(output as usize);
    loop {
        match cvt(getter(
            buffer.capacity() as u32,
            &mut output,
            buffer.as_mut_ptr() as _,
        )) {
            Ok(_) => {
                unsafe {
                    buffer.set_len(output as usize);
                }
                return Ok(buffer);
            }
            Err(sys::Result::ERROR_SIZE_INSUFFICIENT) => {
                buffer.reserve(output as usize - buffer.capacity());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn get_arr_init<T: Copy>(
    init: T,
    mut getter: impl FnMut(u32, &mut u32, *mut T) -> sys::Result,
) -> Result<Vec<T>> {
    let mut output = 0;
    cvt(getter(0, &mut output, std::ptr::null_mut()))?;
    let mut buffer = vec![init; output as usize];
    loop {
        match cvt(getter(output, &mut output, buffer.as_mut_ptr() as _)) {
            Ok(_) => {
                buffer.truncate(output as usize);
                return Ok(buffer);
            }
            Err(sys::Result::ERROR_SIZE_INSUFFICIENT) => {
                buffer.resize(output as usize, init);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

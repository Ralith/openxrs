//! To get started, construct an `Entry` object.

// deref_addrof false positive: https://github.com/rust-lang/rust-clippy/issues/8247
#![allow(clippy::transmute_ptr_to_ptr, clippy::deref_addrof)]
use std::ffi::CStr;
use std::os::raw::c_char;

pub use sys::{
    self, AsyncRequestIdFB, Duration, Path, SystemId, Time, UuidEXT, Version, CURRENT_API_VERSION,
    FREQUENCY_UNSPECIFIED, MAX_VIRTUAL_KEYBOARD_COMMIT_TEXT_SIZE_META,
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
mod display_refresh_rate;
mod passthrough;
pub use passthrough::*;
mod localization_map_ml;
pub use localization_map_ml::*;

pub use builder::{
    CompositionLayerBase, CompositionLayerCubeKHR, CompositionLayerCylinderKHR,
    CompositionLayerEquirect2KHR, CompositionLayerEquirectKHR, CompositionLayerProjection,
    CompositionLayerProjectionView, CompositionLayerQuad, HapticBase, HapticPcmVibrationFB,
    HapticVibration, SwapchainSubImage,
};

pub type Result<T, E = sys::Result> = std::result::Result<T, E>;

// Reserved semantic paths
pub const USER_HAND_LEFT: &str = "/user/hand/left";
pub const USER_HAND_RIGHT: &str = "/user/hand/right";
pub const USER_HEAD: &str = "/user/head";
pub const USER_GAMEPAD: &str = "/user/gamepad";
pub const USER_TREADMILL: &str = "/user/treadmill";

/// Stub for the `xrEnumerateApiLayerProperties` entry point when it's not provided by the runtime.
///
/// # Safety
///
/// No this is actually safe but we need the type to match.
unsafe extern "system" fn stub_enumerate_api_layer_properties(
    _a: u32,
    _b: *mut u32,
    _c: *mut sys::ApiLayerProperties,
) -> sys::Result {
    panic!("Runtime didn't provide a xrEnumerateApiLayers entry point");
}

// Helper traits
pub trait AsHandle {
    type Handle: sys::Handle;
    fn as_handle(&self) -> Self::Handle;
}

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
    unsafe { std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(x.as_ptr()).to_bytes()) }
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

impl<'a> From<&'a [sys::ExtensionProperties]> for generated::ExtensionSet {
    fn from(properties: &'a [sys::ExtensionProperties]) -> Self {
        properties
            .iter()
            .filter_map(|ext| {
                // Safety: `c_char` and `u8` have identical layout and no padding
                CStr::from_bytes_until_nul(unsafe {
                    std::mem::transmute::<&[c_char], &[u8]>(&ext.extension_name[..])
                })
                .ok()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn extension_set_from_iter() {
        use crate::generated::{raw, ExtensionSet};
        use std::convert::TryInto;
        let extensions = [
            raw::PassthroughFB::NAME,
            raw::PassthroughHTC::NAME,
            raw::Maintenance1KHR::NAME,
        ];
        let set: ExtensionSet = IntoIterator::into_iter(extensions).collect();
        assert!(set.fb_passthrough);
        assert!(set.htc_passthrough);
        assert!(set.khr_maintenance1);

        let _: [&[u8]; 3] = set.names().as_slice().try_into().unwrap();
    }

    #[test]
    fn extension_set_from_properties() {
        use crate::generated::{raw, ExtensionSet};
        use std::convert::TryInto;
        const EXTENSION_PROP_INIT: sys::ExtensionProperties = sys::ExtensionProperties {
            extension_name: [0; sys::MAX_EXTENSION_NAME_SIZE],
            ty: sys::ExtensionProperties::TYPE,
            next: std::ptr::null_mut(),
            extension_version: 0,
        };
        let extensions = [
            raw::PassthroughFB::NAME,
            raw::PassthroughHTC::NAME,
            raw::Maintenance1KHR::NAME,
        ];
        let mut props = [EXTENSION_PROP_INIT; 3];
        for i in 0..extensions.len() {
            // Safety: All extension names is less than `sys::MAX_EXTENSION_NAME_SIZE` bytes
            // long. And casting from `*const u8` to `*const c_char` is safe because they have
            // the same size, alignment, and representation. `props` can't overlap with constants
            // because they are in the stack.
            unsafe {
                props[i]
                    .extension_name
                    .as_mut_slice()
                    .as_mut_ptr()
                    .copy_from_nonoverlapping(
                        extensions[i].as_ptr() as *const _,
                        extensions[i].len(),
                    );
            }
        }
        let set: ExtensionSet = (&props[..]).into();
        assert!(set.fb_passthrough);
        assert!(set.htc_passthrough);
        assert!(set.khr_maintenance1);

        let _: [&[u8]; 3] = set.names().as_slice().try_into().unwrap();
    }
}

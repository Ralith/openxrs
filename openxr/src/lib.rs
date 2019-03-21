use std::os::raw::c_char;

macro_rules! fixed_cstr {
    ($str:expr, $n:ident) => {{
        let s = $str;
        unsafe {
            if s.len() + 1 > sys::$n {
                panic!("string exceeds {}", stringify!($n));
            }
            let mut x: [std::os::raw::c_char; sys::$n] = std::mem::uninitialized();
            for (i, o) in s.bytes().zip(x.iter_mut()) {
                *o = i as std::os::raw::c_char;
            }
            x[s.len()] = 0;
            x
        }
    }};
}

pub use sys::{self, Duration, Path, SystemId, Time, Version, CURRENT_API_VERSION};

mod generated;
pub use generated::*;
mod entry;
pub use entry::*;
mod instance;
pub use instance::*;
mod session;
pub use session::*;
mod graphics;
pub use graphics::*;
mod swapchain;
pub use swapchain::*;
mod space;
pub use space::*;
mod action;
pub use action::*;

pub type Result<T> = std::result::Result<T, sys::Result>;

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

unsafe fn fixed_str<'a>(x: &'a [c_char]) -> &'a str {
    std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(x.as_ptr()).to_bytes())
}

fn fixed_str_bytes<'a>(x: &'a [c_char]) -> &'a [u8] {
    let end = x.iter().position(|&x| x == 0).unwrap();
    unsafe { std::mem::transmute(&x[..end + 1]) }
}

fn get_str(mut getter: impl FnMut(u32, &mut u32, *mut c_char) -> sys::Result) -> Result<String> {
    unsafe {
        Ok(String::from_utf8_unchecked(get_arr(|x, y, z| {
            getter(x, y, z as _)
        })?))
    }
}

fn get_arr<T>(mut getter: impl FnMut(u32, &mut u32, *mut T) -> sys::Result) -> Result<Vec<T>> {
    let mut output = 0;
    cvt(getter(0, &mut output, std::ptr::null_mut()))?;
    let mut buffer = Vec::with_capacity(output as usize);
    cvt(getter(output, &mut output, buffer.as_mut_ptr() as _))?;
    unsafe {
        buffer.set_len(output as usize);
    }
    Ok(buffer)
}

fn get_arr_init<T: Clone>(
    init: T,
    mut getter: impl FnMut(u32, &mut u32, *mut T) -> sys::Result,
) -> Result<Vec<T>> {
    let mut output = 0;
    cvt(getter(0, &mut output, std::ptr::null_mut()))?;
    let mut buffer = vec![init; output as usize];
    cvt(getter(output, &mut output, buffer.as_mut_ptr() as _))?;
    buffer.truncate(output as usize);
    Ok(buffer)
}

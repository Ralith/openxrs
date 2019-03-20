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

pub use sys::{self, Duration, Path, SystemId, Time};

mod generated;
pub use generated::*;
mod entry;
pub use entry::*;
mod instance;
pub use instance::*;
mod session;
pub use session::*;

pub type Result<T> = std::result::Result<T, sys::Result>;

fn cvt(x: sys::Result) -> Result<sys::Result> {
    if x.into_raw() >= 0 {
        Ok(x)
    } else {
        Err(x)
    }
}

unsafe fn fixed_str<'a>(x: &'a [std::os::raw::c_char]) -> &'a str {
    std::str::from_utf8_unchecked(std::ffi::CStr::from_ptr(x.as_ptr()).to_bytes())
}

// Reserved semantic paths
pub const USER_HAND_LEFT: &str = "/user/hand/left";
pub const USER_HAND_RIGHT: &str = "/user/hand/right";
pub const USER_HEAD: &str = "/user/head";
pub const USER_GAMEPAD: &str = "/user/gamepad";
pub const USER_TREADMILL: &str = "/user/treadmill";

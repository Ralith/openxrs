//! Stubbed-out platform types for use in graphics bindings
//!
//! Cast to these from your bindings of choice.
#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_ulong, c_void};

// Vulkan
pub type VkInstance = *const c_void;
pub type VkPhysicalDevice = *const c_void;
pub type VkDevice = *const c_void;
pub type VkImage = u64;
pub type VkFormat = u32;
pub type VkInstanceCreateInfo = c_void;
pub type VkDeviceCreateInfo = c_void;
pub type VkAllocationCallbacks = c_void;
pub type VkResult = i32;
pub type VkGetInstanceProcAddr =
    unsafe extern "system" fn(VkInstance, *const c_char) -> Option<unsafe extern "system" fn()>;

// Xlib
pub type GLXFBConfig = *mut c_void;
pub type GLXDrawable = c_ulong;
pub type GLXContext = *mut c_void;
pub type Display = std::os::raw::c_void;

// Xcb
pub type xcb_connection_t = c_void;
pub type xcb_glx_fbconfig_t = u32;
pub type xcb_visualid_t = u32;
pub type xcb_glx_drawable_t = u32;
pub type xcb_glx_context_t = u32;

// Wayland
pub type wl_display = c_void;

#[cfg(target_os = "android")]
pub use jni::sys::jobject;

// Win32
#[cfg(windows)]
pub type ID3D10Device = *const c_void;
#[cfg(windows)]
pub type ID3D10Texture2D = *const c_void;
#[cfg(windows)]
pub type D3D10_FEATURE_LEVEL1 = u32;
#[cfg(windows)]
pub use winapi::{
    shared::{
        ntdef::LUID,
        windef::{HDC, HGLRC},
    },
    um::{
        d3d11::{ID3D11Device, ID3D11Texture2D},
        d3d12::{ID3D12CommandQueue, ID3D12Device, ID3D12Resource},
        d3dcommon::D3D_FEATURE_LEVEL,
        unknwnbase::IUnknown,
        winnt::LARGE_INTEGER,
    },
};

// EGL
pub type EGLConfig = *mut c_void;
pub type EGLContext = *mut c_void;
pub type EGLDisplay = *mut c_void;
pub type PFNEGLGETPROCADDRESSPROC =
    unsafe extern "system" fn(*const c_char) -> Option<unsafe extern "system" fn()>;

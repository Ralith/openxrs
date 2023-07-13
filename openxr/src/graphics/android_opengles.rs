use crate::Result;
use crate::{cvt, get_arr_init, Graphics, Instance, Swapchain};
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::{null, null_mut};
use sys::{
    create_session, GraphicsBindingOpenGLESAndroidKHR, GraphicsRequirementsOpenGLESKHR,
    SessionCreateInfo, SwapchainImageOpenGLESKHR, SystemId,
};

pub struct AndroidGLESCreateInfo {
    /// You probably want gst_gl_context_get_gl_context()
    pub gl_context: *mut c_void,
    /// You probably want gst_gl_display_get_handle()
    pub gl_display: *mut c_void,
    pub system_id: SystemId,
}

//

pub struct AndroidOpenGLES {}

impl Graphics for AndroidOpenGLES {
    type Requirements = GraphicsRequirementsOpenGLESKHR;
    type SessionCreateInfo = AndroidGLESCreateInfo; // XXX this should be a little more specific
    type Format = i64;
    type SwapchainImage = u32;

    fn raise_format(format: i64) -> Self::Format {
        format
    }

    fn lower_format(format: Self::Format) -> i64 {
        format
    }

    fn requirements(instance: &Instance, system_id: SystemId) -> Result<Self::Requirements> {
        let mut graphics_requirements = GraphicsRequirementsOpenGLESKHR::out(null_mut());

        let get_open_gles_graphics_requirements = instance
            .exts()
            .khr_opengl_es_enable
            .unwrap()
            .get_open_gles_graphics_requirements;

        /*debug!(
            "get_open_gles_graphics_requirements {:?}",
            get_open_gles_graphics_requirements as usize
        );*/
        let result = unsafe {
            (get_open_gles_graphics_requirements)(
                instance.as_raw(),
                system_id,
                graphics_requirements.as_mut_ptr(),
            )
        };
        cvt(result)?;
        Ok(unsafe { graphics_requirements.assume_init() })
    }

    unsafe fn create_session(
        instance: &Instance,
        _system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        let graphics_binding = GraphicsBindingOpenGLESAndroidKHR {
            ty: GraphicsBindingOpenGLESAndroidKHR::TYPE,
            next: null(),
            config: null_mut(),
            context: info.gl_context,
            display: info.gl_display,
        };
        let session_create_info = SessionCreateInfo {
            ty: SessionCreateInfo::TYPE,
            next: &graphics_binding as *const _ as *const c_void,
            create_flags: Default::default(),
            system_id: info.system_id,
        };

        let mut rval = MaybeUninit::uninit();
        create_session(instance.as_raw(), &session_create_info, rval.as_mut_ptr());
        Ok(rval.assume_init())
    }

    fn enumerate_swapchain_images(
        swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        let images = get_arr_init(
            SwapchainImageOpenGLESKHR {
                ty: sys::SwapchainImageOpenGLESKHR::TYPE,
                next: null_mut(),
                image: 0,
            },
            |capacity, count, buf| unsafe {
                (swapchain.instance().fp().enumerate_swapchain_images)(
                    swapchain.as_raw(),
                    capacity,
                    count,
                    buf as *mut _,
                )
            },
        )?;
        Ok(images.into_iter().map(|x| x.image).collect())
    }
}

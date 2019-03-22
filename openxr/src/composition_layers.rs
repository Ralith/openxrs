use std::{marker::PhantomData, mem, ops::Deref};

use crate::*;

#[repr(transparent)]
pub struct CompositionLayer<'a, G: Graphics> {
    _inner: sys::CompositionLayerBaseHeader,
    _marker: PhantomData<&'a G>,
}

macro_rules! setters {
    ($ident:ident { $($field:ident : $ty:ty,)* }) => {
        impl<'a, G: Graphics> $ident<'a, G> {
            $(
                pub fn $field(mut self, value: $ty) -> Self {
                    self.inner.$field = value;
                    self
                }
            )*
        }
    }
}

macro_rules! layer {
    ($ident:ident { $($field:ident : $ty:ty,)* }) => {
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        pub struct $ident<'a, G: Graphics> {
            inner: sys::$ident,
            _marker: PhantomData<&'a G>,
        }

        impl<'a, G: Graphics> $ident<'a, G> {
            pub fn new() -> Self {
                Self {
                    inner: sys::$ident {
                        ty: sys::$ident::TYPE,
                        ..unsafe { mem::zeroed() }
                    },
                    _marker: PhantomData,
                }
            }

            pub fn space(mut self, value: &'a Space) -> Self {
                self.inner.space = value.as_raw();
                self
            }
        }

        setters!($ident {
            layer_flags: CompositionLayerFlags,
            $($field : $ty,)*
        });

        impl<'a, G: Graphics> Deref for $ident<'a, G> {
            type Target = CompositionLayer<'a, G>;

            fn deref(&self) -> &Self::Target {
                unsafe { mem::transmute(&self.inner) }
            }
        }
    }
}

layer!(CompositionLayerProjection {});
impl<'a, G: Graphics> CompositionLayerProjection<'a, G> {
    pub fn views(mut self, views: &'a [CompositionLayerProjectionView<'a, G>]) -> Self {
        assert!(views.len() <= u32::max_value() as usize);
        self.inner.view_count = views.len() as u32;
        self.inner.views = views.as_ptr() as _;
        self
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct CompositionLayerProjectionView<'a, G: Graphics> {
    inner: sys::CompositionLayerProjectionView,
    _marker: PhantomData<&'a G>,
}

impl<'a, G: Graphics> CompositionLayerProjectionView<'a, G> {
    pub fn new() -> Self {
        Self {
            inner: sys::CompositionLayerProjectionView {
                ty: sys::CompositionLayerProjectionView::TYPE,
                ..unsafe { mem::zeroed() }
            },
            _marker: PhantomData,
        }
    }

    pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
        self.inner.sub_image = sys::SwapchainSubImage {
            swapchain: value.swapchain.as_raw(),
            image_rect: value.image_rect,
            image_array_index: value.image_array_index,
        };
        self
    }
    pub fn depth_info_khr(mut self, value: &'a CompositionLayerDepthInfoKHR<'a, G>) -> Self {
        self.inner.next = &value.inner as *const _ as _;
        self
    }
}

setters!(CompositionLayerProjectionView {
    pose: Posef,
    fov: Fovf,
});

#[derive(Copy, Clone)]
pub struct SwapchainSubImage<'a, G: Graphics> {
    pub swapchain: &'a Swapchain<G>,
    pub image_rect: Rect2Di,
    pub image_array_index: u32,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct CompositionLayerDepthInfoKHR<'a, G: Graphics> {
    inner: sys::CompositionLayerDepthInfoKHR,
    _marker: PhantomData<&'a G>,
}

impl<'a, G: Graphics> CompositionLayerDepthInfoKHR<'a, G> {
    pub fn new() -> Self {
        Self {
            inner: sys::CompositionLayerDepthInfoKHR {
                ty: sys::CompositionLayerDepthInfoKHR::TYPE,
                ..unsafe { mem::zeroed() }
            },
            _marker: PhantomData,
        }
    }

    pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
        self.inner.sub_image = sys::SwapchainSubImage {
            swapchain: value.swapchain.as_raw(),
            image_rect: value.image_rect,
            image_array_index: value.image_array_index,
        };
        self
    }
}

setters!(CompositionLayerDepthInfoKHR {
    min_depth: f32,
    max_depth: f32,
    near_z: f32,
    far_z: f32,
});

layer!(CompositionLayerQuad {
    eye_visibility: EyeVisibility,
    pose: Posef,
    size: Vector2f,
});

impl<'a, G: Graphics> CompositionLayerQuad<'a, G> {
    pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
        self.inner.sub_image = sys::SwapchainSubImage {
            swapchain: value.swapchain.as_raw(),
            image_rect: value.image_rect,
            image_array_index: value.image_array_index,
        };
        self
    }
}

layer!(CompositionLayerCubeKHR {
    eye_visibility: EyeVisibility,
    image_array_index: u32,
    orientation: Quaternionf,
    offset: Vector3f,
});

impl<'a, G: Graphics> CompositionLayerCubeKHR<'a, G> {
    pub fn swapchain(mut self, value: &'a Swapchain<G>) -> Self {
        self.inner.swapchain = value.as_raw();
        self
    }
}

layer!(CompositionLayerCylinderKHR {
    eye_visibility: EyeVisibility,
    pose: Posef,
    radius: f32,
    central_angle: f32,
    aspect_ratio: f32,
});

impl<'a, G: Graphics> CompositionLayerCylinderKHR<'a, G> {
    pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
        self.inner.sub_image = sys::SwapchainSubImage {
            swapchain: value.swapchain.as_raw(),
            image_rect: value.image_rect,
            image_array_index: value.image_array_index,
        };
        self
    }
}

layer!(CompositionLayerEquirectKHR {
    eye_visibility: EyeVisibility,
    pose: Posef,
    offset: Vector3f,
    scale: Vector2f,
    bias: Vector2f,
});

impl<'a, G: Graphics> CompositionLayerEquirectKHR<'a, G> {
    pub fn sub_image(mut self, value: SwapchainSubImage<'a, G>) -> Self {
        self.inner.sub_image = sys::SwapchainSubImage {
            swapchain: value.swapchain.as_raw(),
            image_rect: value.image_rect,
            image_array_index: value.image_array_index,
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn deref_tricks<G: Graphics>(sess: &Session<G>) {
        unsafe {
            sess.end_frame(
                Time::from_raw(0),
                EnvironmentBlendMode::OPAQUE,
                &[
                    &CompositionLayerProjection::new().views(&[]),
                    &CompositionLayerQuad::new(),
                ],
            )
            .unwrap();
        }
    }
}

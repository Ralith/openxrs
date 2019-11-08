use crate::{
    Extent2Df, Extent2Di, Offset2Df, Offset2Di, Quaternionf, Vector2f, Vector3f, Vector4f,
};
use mint::{Quaternion, Vector2, Vector3, Vector4};

impl From<Vector2f> for Vector2<f32> {
    fn from(v: Vector2f) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2<f32>> for Vector2f {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Offset2Df> for Vector2<f32> {
    fn from(v: Offset2Df) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2<f32>> for Offset2Df {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Offset2Di> for Vector2<i32> {
    fn from(v: Offset2Di) -> Self {
        Vector2 { x: v.x, y: v.y }
    }
}

impl From<Vector2<i32>> for Offset2Di {
    fn from(v: Vector2<i32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Extent2Df> for Vector2<f32> {
    fn from(v: Extent2Df) -> Self {
        Vector2 {
            x: v.width,
            y: v.height,
        }
    }
}

impl From<Vector2<f32>> for Extent2Df {
    fn from(v: Vector2<f32>) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

impl From<Extent2Di> for Vector2<i32> {
    fn from(v: Extent2Di) -> Self {
        Vector2 {
            x: v.width,
            y: v.height,
        }
    }
}

impl From<Vector2<i32>> for Extent2Di {
    fn from(v: Vector2<i32>) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

impl From<Vector3f> for Vector3<f32> {
    fn from(v: Vector3f) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector3<f32>> for Vector3f {
    fn from(v: Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vector4f> for Vector4<f32> {
    fn from(v: Vector4f) -> Self {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl From<Vector4<f32>> for Vector4f {
    fn from(v: Vector4<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl From<Quaternion<f32>> for Quaternionf {
    fn from(q: Quaternion<f32>) -> Self {
        Self {
            x: q.v.x,
            y: q.v.y,
            z: q.v.z,
            w: q.s,
        }
    }
}

impl From<Quaternionf> for Quaternion<f32> {
    fn from(q: Quaternionf) -> Quaternion<f32> {
        Quaternion {
            s: q.w,
            v: Vector3 {
                x: q.x,
                y: q.y,
                z: q.z,
            },
        }
    }
}

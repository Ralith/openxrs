use nalgebra as na;

use crate::{Extent2Df, Offset2Df, Posef, Quaternionf, Vector2f, Vector3f, Vector4f};

// Hack around a rustc bug
type Vector2<T> = na::Matrix<T, na::U2, na::U1, na::ArrayStorage<T, na::U2, na::U1>>;
type Vector3<T> = na::Matrix<T, na::U3, na::U1, na::ArrayStorage<T, na::U3, na::U1>>;
type Vector4<T> = na::Matrix<T, na::U4, na::U1, na::ArrayStorage<T, na::U4, na::U1>>;

impl From<Vector2f> for Vector2<f32> {
    fn from(v: Vector2f) -> Self {
        na::Vector2::new(v.x, v.y)
    }
}

impl From<Vector2<f32>> for Vector2f {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Offset2Df> for Vector2<f32> {
    fn from(v: Offset2Df) -> Self {
        na::Vector2::new(v.x, v.y)
    }
}

impl From<Vector2<f32>> for Offset2Df {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Extent2Df> for Vector2<f32> {
    fn from(v: Extent2Df) -> Self {
        na::Vector2::new(v.width, v.height)
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

impl From<Vector3f> for Vector3<f32> {
    fn from(v: Vector3f) -> Self {
        na::Vector3::new(v.x, v.y, v.z)
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
        na::Vector4::new(v.x, v.y, v.z, v.w)
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

impl From<na::UnitQuaternion<f32>> for Quaternionf {
    fn from(q: na::UnitQuaternion<f32>) -> Self {
        Self {
            x: q.i,
            y: q.j,
            z: q.k,
            w: q.w,
        }
    }
}

impl From<Quaternionf> for na::UnitQuaternion<f32> {
    fn from(q: Quaternionf) -> na::UnitQuaternion<f32> {
        na::Unit::new_unchecked(na::Quaternion::new(q.w, q.x, q.y, q.z))
    }
}

impl From<na::Isometry3<f32>> for Posef {
    fn from(x: na::Isometry3<f32>) -> Self {
        Self {
            orientation: x.rotation.into(),
            position: x.translation.vector.into(),
        }
    }
}

impl From<Posef> for na::Isometry3<f32> {
    fn from(p: Posef) -> na::Isometry3<f32> {
        let position: na::Vector3<f32> = p.position.into();
        na::Isometry3::from_parts(na::Translation3::from(position), p.orientation.into())
    }
}

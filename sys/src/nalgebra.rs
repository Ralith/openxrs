use nalgebra as na;

use crate::{Extent2Df, Offset2Df, Posef, Quaternionf, Vector2f, Vector3f, Vector4f};

impl Into<na::Vector2<f32>> for Vector2f {
    fn into(self) -> na::Vector2<f32> {
        na::Vector2::new(self.x, self.y)
    }
}

impl From<na::Vector2<f32>> for Vector2f {
    fn from(v: na::Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl Into<na::Vector3<f32>> for Vector3f {
    fn into(self) -> na::Vector3<f32> {
        na::Vector3::new(self.x, self.y, self.z)
    }
}

impl From<na::Vector3<f32>> for Vector3f {
    fn from(v: na::Vector3<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Into<na::Vector4<f32>> for Vector4f {
    fn into(self) -> na::Vector4<f32> {
        na::Vector4::new(self.x, self.y, self.z, self.w)
    }
}

impl From<na::Vector4<f32>> for Vector4f {
    fn from(v: na::Vector4<f32>) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl Into<na::UnitQuaternion<f32>> for Quaternionf {
    fn into(self) -> na::UnitQuaternion<f32> {
        na::Unit::new_unchecked(na::Quaternion::new(self.w, self.x, self.y, self.z))
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

impl Into<na::Vector2<f32>> for Extent2Df {
    fn into(self) -> na::Vector2<f32> {
        na::Vector2::new(self.width, self.height)
    }
}

impl From<na::Vector2<f32>> for Extent2Df {
    fn from(v: na::Vector2<f32>) -> Self {
        Self { width: v.x, height: v.y }
    }
}

impl Into<na::Vector2<f32>> for Offset2Df {
    fn into(self) -> na::Vector2<f32> {
        na::Vector2::new(self.x, self.y)
    }
}

impl From<na::Vector2<f32>> for Offset2Df {
    fn from(v: na::Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl Into<na::Isometry3<f32>> for Posef {
    fn into(self) -> na::Isometry3<f32> {
        let position: na::Vector3<f32> = self.position.into();
        na::Isometry3::from_parts(na::Translation3::from(position), self.orientation.into())
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

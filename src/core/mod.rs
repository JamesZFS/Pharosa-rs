pub use cgmath::*;
pub use num_traits::float::{FloatConst, FloatCore};

pub use film::*;
pub use intersection::*;
pub use ray::Ray;
pub use spectrum::Spectrum;

use crate::primitive::Primitive;

mod ray;
mod intersection;
mod spectrum;
mod film;

/// Global floating point precision
#[cfg(feature = "float32")]
pub type Real = f32;

// #[cfg(feature = "float64")]
// pub type Real = f64;

pub type Radf = Rad<Real>;
pub type Degf = Deg<Real>;
pub type Point2f = Point2<Real>;
pub type Point3f = Point3<Real>;
pub type Vector2f = Vector2<Real>;
pub type Vector3f = Vector3<Real>;
pub type Matrix4f = Matrix4<Real>;

#[inline]
pub fn pt3<S>(x: S, y: S, z: S) -> Point3<S> {
    Point3::new(x, y, z)
}

#[inline]
pub fn pt2<S>(x: S, y: S) -> Point2<S> {
    Point2::new(x, y)
}

pub trait TransformAny<T> {
    fn transform(&self, src: &T) -> T;
}

impl TransformAny<Point3f> for Matrix4f {
    #[inline]
    fn transform(&self, src: &Point3f) -> Point3f {
        self.transform_point(*src)
    }
}

impl TransformAny<Vector3f> for Matrix4f {
    #[inline]
    fn transform(&self, src: &Vector3f) -> Vector3f {
        self.transform_vector(*src)
    }
}

pub use cgmath::*;
pub use num_traits::float::{FloatConst, FloatCore};

pub use intersection::*;
pub use ray::Ray;
pub use spectrum::Spectrum;

use crate::primitive::Primitive;

mod ray;
mod intersection;
mod sampler;
mod spectrum;
mod film;

/// Global floating point precision
#[cfg(feature = "float32")]
pub type Real = f32;

// #[cfg(feature = "float64")]
// pub type Real = f64;

pub type Point2f = Point2<Real>;
pub type Point3f = Point3<Real>;
pub type Vector2f = Vector2<Real>;
pub type Vector3f = Vector3<Real>;
pub type Matrix4f = Matrix4<Real>;

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

pub trait Sampler {
    fn next(&mut self) -> Real;
    fn next2d(&mut self) -> Point2f;
}

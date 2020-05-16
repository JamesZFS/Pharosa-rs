pub use cgmath::*;
pub use num_traits::float::{FloatConst, FloatCore};

pub use film::*;
pub use intersection::*;
pub use ray::Ray;
pub use spectrum::Spectrum;

use std::ops::{Add, Sub, Mul};
use std::cell::UnsafeCell;

mod ray;
mod intersection;
mod spectrum;
mod film;

/// Global floating point precision
#[cfg(feature = "float32")]
pub type Float = f32;

// #[cfg(feature = "float64")]
// pub type Real = f64;

pub type Radf = Rad<Float>;
pub type Degf = Deg<Float>;
pub type Point2f = Point2<Float>;
pub type Point3f = Point3<Float>;
pub type Vector2f = Vector2<Float>;
pub type Vector3f = Vector3<Float>;
pub type Matrix4f = Matrix4<Float>;

#[inline(always)]
pub fn pt3<S>(x: S, y: S, z: S) -> Point3<S> {
    Point3::new(x, y, z)
}

#[inline(always)]
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

/// Linear interpolate
#[inline]
pub fn lerp<S, T>(a: S, b: S, t: Float) -> T where
    S: Copy + Add<T, Output=T> + Sub<S, Output=T>,
    T: Mul<Float, Output=T>
{
    a.clone() + (b - a) * t
}

/// To ensure performance, we could use an unsafe cell to sync data between threads
pub(super) struct UnsafeWrapper<T>(UnsafeCell<T>);

unsafe impl<T: Sync> Sync for UnsafeWrapper<T> {}

#[allow(dead_code)]
impl<T> UnsafeWrapper<T> {
    pub fn new(value: T) -> Self { Self(UnsafeCell::new(value)) }
    pub unsafe fn get(&self) -> &T { &*self.0.get() }
    pub unsafe fn get_mut(&self) -> &mut T { &mut *self.0.get() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(1f32, 2f32, 0.5), 1.5);
        let a = Spectrum::black();
        let b = Spectrum::white();
        for i in 0..10 {
            let t = i as Float / 10.;
            let c = lerp(&a, &b, t);
            assert_eq!(c, Spectrum::uniform(t));
        }
    }
}

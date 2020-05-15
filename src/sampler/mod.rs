use crate::core::*;

mod independent;
mod fake;

pub use independent::Independent;
pub use fake::Fake;
use std::fmt::Debug;

pub trait Sampler: Debug + Clone + Send + Sync + 'static {
    fn next(&mut self) -> Float;
    fn next2d(&mut self) -> Point2f;
}

pub fn cosine_on_hemisphere(samp: Point2f) -> Point3f {
    let Point2f { x, y } = uniform_on_disk(samp);
    pt3(x, y, (1. - x * x - y * y).max(0.).sqrt())
}

pub fn uniform_on_disk(samp: Point2f) -> Point2f {
    let Point2f { x, y } = samp;
    let (x, y) = (2. * x - 1., 2. * y - 1.); // [0, 1]^2 -> [-1, 1]^2
    if x == 0. && y == 0. { return pt2(0., 0.); }
    let (phi, r) = if x.abs() > y.abs() {
        (y / x * Float::FRAC_PI_4(), x)
    } else {
        (Float::FRAC_PI_2() - x / y * Float::FRAC_PI_4(), y)
    };
    pt2(r * phi.cos(), r * phi.sin())
}

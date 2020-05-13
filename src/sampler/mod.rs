use crate::core::*;

mod independent;
mod fake;

pub use independent::Independent;
pub use fake::Fake;

pub trait Sampler {
    fn next(&mut self) -> Real;
    fn next2d(&mut self) -> Point2f;
}

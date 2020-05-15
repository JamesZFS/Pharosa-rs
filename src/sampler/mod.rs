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

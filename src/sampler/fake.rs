use super::*;

pub struct Fake;

impl Sampler for Fake {
    fn next(&mut self) -> f32 { 0.5 }

    fn next2d(&mut self) -> Point2f { Point2::new(0.5, 0.5) }
}

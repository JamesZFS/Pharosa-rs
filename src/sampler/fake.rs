use super::*;

#[derive(Debug, Clone)]
pub struct Fake;

impl Sampler for Fake {
    fn next(&mut self) -> Float { 0.5 }

    fn next2d(&mut self) -> Point2f { pt2(0.5, 0.5) }
}

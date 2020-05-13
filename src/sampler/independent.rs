use super::*;
use rand::random;

pub struct Independent;

impl Sampler for Independent {
    fn next(&mut self) -> f32 { random() }

    fn next2d(&mut self) -> Point2f { Point2::new(random(), random()) }
}

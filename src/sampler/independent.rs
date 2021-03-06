use super::*;
use rand::random;

#[derive(Debug, Clone)]
pub struct Independent;

impl Sampler for Independent {
    fn next(&mut self) -> Float { random() }

    fn next2d(&mut self) -> Point2f { pt2(random(), random()) }
}

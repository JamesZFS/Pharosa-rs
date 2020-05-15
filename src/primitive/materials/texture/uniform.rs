use super::*;

#[derive(Debug, Default, Clone)]
pub struct Uniform(pub Spectrum);

impl Texture for Uniform {
    fn at(&self, _uv: Point2f) -> Spectrum { self.0 }
}

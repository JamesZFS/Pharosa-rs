use super::*;

#[derive(Debug, Default, Clone)]
pub struct Uniform(Spectrum);

impl Uniform {
    pub fn new(albedo: Spectrum) -> Self {
        Uniform(albedo)
    }
}

impl Texture for Uniform {
    fn at(&self, _uv: Point2f) -> Spectrum { self.0 }
}

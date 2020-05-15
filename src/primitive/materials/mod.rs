use super::*;

pub mod bsdf;
pub mod texture;

pub use bsdf::BSDF;
pub use texture::Texture;

#[derive(Debug)]
pub struct Material<B: BSDF, T: Texture> {
    pub bsdf: B,
    pub texture: T,
    pub emission: Spectrum,
}

impl<B, T> Material<B, T> where B: BSDF, T: Texture {
    /// Importance sample the BSDF, return the outgoing direction, **weight x albedo** and pdf
    pub fn sample_bsdf(&self, its: &GeometryIntersection, samp: Point2f) -> bsdf::SampleRecord {
        let mut rec = self.bsdf.sample(its, samp);
        rec.weight *= self.texture.at(Point2::origin()); // todo
        rec
    }
}

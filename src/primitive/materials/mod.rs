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


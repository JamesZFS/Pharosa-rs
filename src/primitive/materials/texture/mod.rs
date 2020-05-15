use super::*;

mod uniform;

pub use uniform::Uniform;

pub trait Texture: Debug {
    fn at(&self, uv: Point2f) -> Spectrum;
}

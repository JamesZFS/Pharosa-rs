use super::*;

mod uniform;

pub use uniform::Uniform;

pub trait Texture: Debug + Send + Sync + 'static {
    fn at(&self, uv: Point2f) -> Spectrum;
}

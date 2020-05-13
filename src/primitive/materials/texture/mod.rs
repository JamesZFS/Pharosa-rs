use super::*;

mod uniform;

pub use uniform::Uniform;

pub trait Texture: DynClone + Debug {
    fn at(&self, uv: Point2f) -> Spectrum;
}

dyn_clone::clone_trait_object!(Texture);

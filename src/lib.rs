#[macro_use]
pub mod macros;

pub mod core;
pub mod primitive;
pub mod camera;
pub mod utils;
pub mod scene;
pub mod sampler;
pub mod integrator;
pub mod gui;

pub use self::core::*;
pub use scene::*;
pub use camera::*;
pub use primitive::*;
pub use integrator::{Integrator, SampleIntegrator};
pub use sampler::Sampler;

/// All the data we need to do the rendering
///
/// To maximize the performance, we use a very generic representation
pub struct Context<G: Geometry, B: BSDF, T: Texture, C: CameraInner, S: Sampler, I: Integrator> {
    pub scene: Scene<G, B, T>,
    pub camera: Camera<C>,
    pub sampler: S,
    pub integrator: I,
}

#[macro_use]
mod macros;

mod core;
mod primitive;
mod camera;
pub mod utils;
mod scene;
mod sampler;
mod integrator;
mod gui;

pub use self::core::*;
pub use scene::*;
pub use camera::*;
pub use primitive::*;
pub use integrator::{Integrator, SampleIntegrator};
pub use sampler::Sampler;
pub use gui::gui;


#[derive(Debug)]
/// All the data we need to do the rendering
///
/// To maximize the performance, we use a very generic representation, though a bit awkward...
///
/// The `Context` can be a shared memory wrapped by a UnsafeWrapper
pub struct Context<G: Geometry, B: BSDF, T: Texture, C: CameraInner, S: Sampler> {
    pub scene: Scene<G, B, T>,
    pub camera: Camera<C>,
    pub sampler: S,
    /// `film` is read by gui thread, written by kernel thread
    pub film: Film,
    /// To indicate rendering progress, read by gui, written by kernel
    pub progress: Float,
    /// Gui can set this as true to terminate the rendering
    pub terminate_request: bool,
}

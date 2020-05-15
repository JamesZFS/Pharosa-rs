#![allow(non_snake_case)]

use crate::core::*;
use crate::camera::*;
use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::primitive::*;
use crate::Context;
use std::fmt::Debug;

mod simple;
mod smallpt;
mod path_tracing;

pub use simple::*;
pub use path_tracing::*;
pub use smallpt::*;

pub trait Integrator: Debug + Clone + Send + 'static {
    /// Render the scene, store the result in `film`
    fn render(&self, context: &mut Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>);
}

#[derive(Debug, Clone)]
pub struct SampleIntegrator<D: Debug + Clone> {
    pub n_spp: u32,
    pub delegate: D,
}

impl<D> Integrator for SampleIntegrator<D> where D: SampleIntegratorDelegate + Debug + Clone + Send + 'static {
    fn render(&self, context: &mut Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>) {
        // unpack
        let Context { scene, camera, sampler, film, progress } = context;
        let (height, width) = {
            let film = film.read().unwrap();
            (film.height(), film.width())
        };
        *progress.write().unwrap() = 0.;
        for spp in 0..self.n_spp {
            for y in 0..height {
                for x in 0..width {
                    let (ray, pdf) = camera.generate_ray(x, y, sampler.next2d());
                    let radiance = self.delegate.Li(ray, scene, sampler);
                    // accumulate pixel value
                    let mut film = film.write().unwrap();
                    let acc = if cfg!(debug_assertions) { film.at_mut(x, y) } else { unsafe { film.at_unchecked_mut(x, y) } };
                    *acc = lerp(*acc, radiance / pdf, 1. / (spp + 1) as Float);
                }
            }
            // notify progress
            *progress.write().unwrap() = spp as Float / self.n_spp as Float;
        }
    }
}

pub trait SampleIntegratorDelegate {
    /// Compute the incident radiance
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, sampler: &mut impl Sampler) -> Spectrum;
}

impl<D> Default for SampleIntegrator<D> where D: Default + Debug + Clone {
    fn default() -> Self {
        Self {
            n_spp: 1,
            delegate: D::default(),
        }
    }
}

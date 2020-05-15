#![allow(non_snake_case)]

use crate::core::*;
use crate::camera::*;
use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::primitive::*;

mod simple;
mod path_tracing;

pub use simple::*;
pub use path_tracing::*;
use std::fmt::Debug;

pub trait Integrator {
    /// Render the scene, store the result in `film`
    fn render<G, B, T, C, S>(&self, film: &mut Film, camera: &Camera<C>, scene: &Scene<G, B, T>, sampler: &mut S)
        where G: Geometry, B: BSDF, T: Texture, C: CameraInner, S: Sampler;
}

#[derive(Debug, Clone)]
pub struct SampleIntegrator<D: Debug + Clone> {
    pub n_spp: u32,
    pub delegate: D,
}

impl<D> Integrator for SampleIntegrator<D> where D: SampleIntegratorDelegate + Debug + Clone {
    fn render<G, B, T, C, S>(&self, film: &mut Film, camera: &Camera<C>, scene: &Scene<G, B, T>, sampler: &mut S)
        where G: Geometry, B: BSDF, T: Texture, C: CameraInner, S: Sampler {
        for y in 0..film.height() {
            for x in 0..film.width() {
                let acc = if cfg!(debug_assertions) { film.at_mut(x, y) } else { unsafe { film.at_unchecked_mut(x, y) } };
                for spp in 0..self.n_spp {
                    let (ray, pdf) = camera.generate_ray(x, y, sampler.next2d());
                    let radiance = self.delegate.Li(ray, scene, sampler);
                    // accumulate pixel value
                    *acc = lerp(*acc, radiance / pdf, 1. / (spp + 1) as Real);
                }
            }
        }
    }
}

pub trait SampleIntegratorDelegate {
    // has n_spp
    /// Compute the incident radiance
    fn Li<G, B, T>(&self, ray: Ray, scene: &Scene<G, B, T>, sampler: &mut impl Sampler) -> Spectrum where G: Geometry, B: BSDF, T: Texture;
}

impl<D> Default for SampleIntegrator<D> where D: Default + Debug + Clone {
    fn default() -> Self {
        Self {
            n_spp: 1,
            delegate: D::default(),
        }
    }
}

#![allow(non_snake_case)]

use crate::core::*;
use crate::camera::*;
use crate::scene::Scene;
use crate::sampler::Sampler;
use crate::primitive::*;
use crate::Context;
use std::fmt::Debug;
use rayon::prelude::*;

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

impl<D> Integrator for SampleIntegrator<D> where D: SampleIntegratorDelegate + Debug + Clone + Sync + Send + 'static {
    fn render(&self, context: &mut Context<impl Geometry, impl BSDF, impl Texture, impl CameraInner, impl Sampler>) {
        // unpack
        let Context { ref scene, ref camera, sampler, film, progress, ref terminate_request } = context;
        let (height, width) = (film.height(), film.width()); // assume size not change
        *progress = 0.;
        let film = UnsafeWrapper::new(film); // cast to pointer, for performance purpose
        for spp in 0..self.n_spp {
            if *terminate_request { return; } // early stop when a terminate_request is pending
            // parallel y
            (0..height).into_par_iter().for_each(|y| unsafe {
                let mut sampler = sampler.clone();
                let film = &mut *(*film.get_raw_mut() as *mut Film); // todo: this is too ugly...
                for x in 0..width {
                    let acc = film.at_unchecked_mut(x, y);
                    let (ray, pdf) = camera.generate_ray(x, y, sampler.next2d());
                    let mut radiance = self.delegate.Li(ray, scene, &mut sampler);
                    radiance /= pdf;
                    // accumulate pixel value
                    *acc = lerp(&*acc, &radiance, 1. / (spp + 1) as Float);
                }
            });
            // notify progress
            *progress = (spp + 1) as Float / self.n_spp as Float;
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

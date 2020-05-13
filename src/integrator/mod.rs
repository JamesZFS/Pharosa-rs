use crate::core::*;
use crate::camera::*;
use crate::scene::Scene;
use crate::sampler::Sampler;

pub mod albedo;

pub trait Integrator {
    /// Render the scene, store the result in `film`
    fn render<C, S>(film: &mut Film, camera: &Camera<C>, scene: &mut Scene, sampler: &mut S)
        where C: CameraInner, S: Sampler;
}

#[allow(non_snake_case)]
pub trait SampleIntegrator {
    /// Compute the incident radiance
    fn Li(ray: Ray, scene: &mut Scene, sampler: &mut impl Sampler) -> Spectrum;
}

impl<T> Integrator for T where T: SampleIntegrator {
    fn render<C, S>(film: &mut Film, camera: &Camera<C>, scene: &mut Scene, sampler: &mut S) where C: CameraInner, S: Sampler {
        for (x, y, pixel) in film.enumerate_pixels_mut() {
            let (ray, pdf) = camera.generate_ray(x, y, sampler.next2d());
            let radiance = Self::Li(ray, scene, sampler);
            *pixel = (radiance / pdf).into();
        }
    }
}

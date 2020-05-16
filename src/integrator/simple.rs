use super::*;
use crate::primitive::texture::Texture;
use num_traits::clamp_min;

#[derive(Default, Debug, Clone)]
pub struct Albedo;

impl SampleIntegratorDelegate for Albedo {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(its) => its.albedo().clone(),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Normal;

impl SampleIntegratorDelegate for Normal {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => Spectrum::black(),
            Some(Intersection(GeometryIntersection { normal, .. }, ..))
            => (normal.add_element_wise(1.) / 2.).into(), // [-1., 1.] to [0., 1.]
        }
    }
}

/// Primary hit position
#[derive(Default, Debug, Clone)]
pub struct Position;

impl SampleIntegratorDelegate for Position {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => Spectrum::black(),
            Some(Intersection(GeometryIntersection { pos, .. }, ..)) => pos.into(),
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct Shader;

impl SampleIntegratorDelegate for Shader {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(its) => {
                let mut attenuation: Float = dot(its.normal(), vec3(1.0, 1.0, 0.).normalize());
                attenuation = clamp_min(attenuation, 0.2);
                its.albedo() * attenuation
            }
        }
    }
}

use super::*;
use crate::primitive::texture::Texture;
use num_traits::clamp_min;

#[derive(Default, Debug, Clone)]
pub struct Albedo;

impl SampleIntegratorDelegate for Albedo {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(Intersection(_, prim)) => prim.material.texture.at(Point2::origin()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Normal;

impl SampleIntegratorDelegate for Normal {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => Spectrum::black(),
            Some(Intersection(geo, _prim)) => (geo.normal.add_element_wise(1.) / 2.).into(), // [-1., 1.] to [0., 1.]
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
            Some(Intersection(geo, _prim)) => geo.pos.into(),
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct Shader;

impl SampleIntegratorDelegate for Shader {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(Intersection(GeometryIntersection { normal, .. }, prim)) => {
                let mut attenuation: Float = dot(normal, vec3(1.0, 1.0, 0.).normalize());
                attenuation = clamp_min(attenuation, 0.2);
                prim.material.texture.at(Point2::origin()) * attenuation
            }
        }
    }
}

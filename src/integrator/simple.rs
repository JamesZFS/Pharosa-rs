use super::*;
use crate::primitive::texture::Texture;

pub struct Albedo;

#[allow(non_snake_case)]
impl SampleIntegrator for Albedo {
    fn Li(ray: Ray, scene: &Scene, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(Intersection(_, prim)) => prim.material.texture.at(Point2::origin()),
        }
    }
}

pub struct Normal;

#[allow(non_snake_case)]
impl SampleIntegrator for Normal {
    fn Li(ray: Ray, scene: &Scene, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => Spectrum::black(),
            Some(Intersection(geo, _prim)) => (geo.normal.add_element_wise(1.) / 2.).into(), // [-1., 1.] to [0., 1.]
        }
    }
}

/// Primary hit position
pub struct Position;

#[allow(non_snake_case)]
impl SampleIntegrator for Position {
    fn Li(ray: Ray, scene: &Scene, _sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => Spectrum::black(),
            Some(Intersection(geo, _prim)) => geo.pos.into(),
        }
    }
}

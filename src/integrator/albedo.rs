use super::*;
use crate::primitive::texture::Texture;

pub struct Albedo;

#[allow(non_snake_case)]
impl SampleIntegrator for Albedo {
    fn Li(ray: Ray, scene: &mut Scene, sampler: &mut impl Sampler) -> Spectrum {
        match scene.nearest_hit(&ray) {
            None => scene.environ_map(&ray),
            Some(Intersection(_, prim)) => prim.material.texture.at(Point2::origin()),
        }
    }
}
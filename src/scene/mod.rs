use crate::core::*;
use crate::primitive::*;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Scene<G: Geometry, B: BSDF, T: Texture> {
    primitives: Vec<Primitive<G, B, T>>,
}

impl<G, B, T> Scene<G, B, T> where G: Geometry, B: BSDF, T: Texture {
    #[inline]
    pub fn new() -> Self { Self { primitives: Vec::new() } }

    pub fn nearest_hit(&self, ray_world: &Ray) -> Option<Intersection<G, B, T>> {
        let mut isect: Option<Intersection<G, B, T>> = None;
        for prim in &self.primitives {
            let new_isect = prim.intersect(ray_world);
            if let Some(new_isect) = new_isect {
                match &isect {
                    None => isect = Some(Intersection(new_isect, prim)),
                    Some(Intersection(geo_isect, _hit))
                    // update nearer intersect:
                    if new_isect.t < geo_isect.t => isect = Some(Intersection(new_isect, prim)),
                    _ => {}
                }
            }
        }
        isect
    }

    pub fn environ_map(&self, _ray_world: &Ray) -> Spectrum {
        Spectrum::black() // todo
    }
}

impl<G, B, T> Deref for Scene<G, B, T> where G: Geometry, B: BSDF, T: Texture {
    type Target = Vec<Primitive<G, B, T>>;
    fn deref(&self) -> &Self::Target {
        &self.primitives
    }
}

impl<G, B, T> DerefMut for Scene<G, B, T> where G: Geometry, B: BSDF, T: Texture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.primitives
    }
}

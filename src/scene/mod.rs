use crate::primitive::Primitive;
use std::ops::{Deref, DerefMut};
use crate::core::*;

#[derive(Clone, Debug)]
pub struct Scene {
    primitives: Vec<Primitive>,
}

impl Scene {
    #[inline]
    pub fn new() -> Self { Self { primitives: Vec::new() } }

    pub fn nearest_hit(&self, ray_world: &Ray) -> Option<Intersection> {
        let mut isect: Option<Intersection> = None;
        for prim in &self.primitives {
            let new_isect = prim.intersect(ray_world);
            if let Some(new_isect) = new_isect {
                match &isect {
                    None => isect = Some(Intersection(new_isect, prim)),
                    Some(Intersection(geo_isect, hit))
                    // nearer intersect:
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

impl Deref for Scene {
    type Target = Vec<Primitive>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.primitives
    }
}

impl DerefMut for Scene {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.primitives
    }
}

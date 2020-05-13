use crate::core::*;
use std::fmt::Debug;
use dyn_clone::DynClone;

use geometries::Geometry;
use materials::Material;

mod geometries;
mod materials;

#[derive(Debug, Clone)]
pub struct Primitive {
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
    world_to_local: Matrix4f,
    local_to_world: Matrix4f,
}

impl Primitive {
    /// Construct a Primitive.
    ///
    /// `transform`: a matrix to transform the geometry from origin to where it should locate in the world
    pub fn new(geometry: Box<dyn Geometry>, material: Box<dyn Material>, transform: Matrix4f) -> Self {
        Self {
            geometry,
            material,
            world_to_local: transform.inverse_transform()
                .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform))),
            local_to_world: transform,
        }
    }
    pub fn intersect(&self, ray_world: &Ray) -> Option<GeometryIntersection> {
        let ray = self.world_to_local.transform(ray_world);
        self.geometry.intersect(&ray).map(|its| self.local_to_world.transform(&its))
    }
    #[inline]
    pub fn world_to_local(&self) -> &Matrix4f {
        &self.world_to_local
    }
    #[inline]
    pub fn local_to_world(&self) -> &Matrix4f {
        &self.local_to_world
    }
    /// Get world center
    #[inline]
    pub fn center(&self) -> Point3f {
        self.local_to_world.transform_point(Point3::origin())
    }
}

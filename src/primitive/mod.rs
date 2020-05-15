use crate::core::*;
use std::fmt::Debug;
use std::sync::{Mutex, Arc};
use lazy_static::*;

pub use geometries::{Sphere, Geometry};
pub use materials::Material;
pub use materials::{bsdf::{self, BSDF}, texture::{self, Texture}};

mod geometries;
mod materials;

#[derive(Debug, Clone)]
pub struct Primitive<G: Geometry, B: BSDF, T: Texture> {
    /// For debug
    pub label: String,
    /// Geometric instance: Sphere, Triangle, ..
    ///
    /// Owned by the primitive itself
    pub geometry: G,
    /// Physical material: bsdf + texture
    ///
    /// Material is sharable across multiple threads
    pub material: Arc<Material<B, T>>,
    world_to_local: Matrix4f,
    local_to_world: Matrix4f,
}

lazy_static! {
    static ref PRIMITIVE_COUNT: Mutex<usize> = Mutex::new(0);
}

impl<G, B, T> Primitive<G, B, T> where G: Geometry, B: BSDF, T: Texture {
    /// Construct a Primitive.
    ///
    /// `transform`: a matrix to transform the geometry from origin to where it should locate in the world
    pub fn new(geometry: G, material: Arc<Material<B, T>>, transform: Matrix4f) -> Self {
        let mut i = PRIMITIVE_COUNT.lock().unwrap();
        Self::new_with_label(format!("Unnamed primitive #{}", (*i, *i += 1).0), geometry, material, transform)
    }
    /// Construct a Primitive with custom label.
    ///
    /// `transform`: a matrix to transform the geometry from origin to where it should locate in the world
    pub fn new_with_label(label: String, geometry: G, material: Arc<Material<B, T>>, transform: Matrix4f) -> Self {
        Self {
            label,
            geometry,
            material,
            world_to_local: transform.inverse_transform()
                .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform))),
            local_to_world: transform,
        }
    }
    pub fn intersect(&self, ray_world: &Ray) -> Option<GeometryIntersection> {
        let ray = self.world_to_local.transform(ray_world);
        self.geometry.intersect(&ray).map(|its| {
            debug_assert_approx!(its.normal.magnitude(), 1.0);
            self.local_to_world.transform(&its)
        })
    }
    /// Set local_to_world transform, auto-set the counterpart
    pub fn set_transform(&mut self, transform: Matrix4f) {
        self.world_to_local = transform.inverse_transform()
            .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform)));
        self.local_to_world = transform;
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

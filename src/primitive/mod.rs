use std::fmt::Debug;
use std::rc::Rc;

use dyn_clone::DynClone;
use lazy_static::*;

use geometries::*;
pub use geometries::Sphere;
pub use materials::Material;
pub use materials::{bsdf, texture};

use crate::core::*;
use std::sync::Mutex;

mod geometries;
mod materials;

type SimpleMaterial = Rc<Material<bsdf::Simple, texture::Uniform>>;

#[derive(Debug, Clone)]
pub struct Primitive {
    /// for debug
    pub label: String,
    pub geometry: Box<Sphere>,
    pub material: SimpleMaterial,
    world_to_local: Matrix4f,
    local_to_world: Matrix4f,
}

lazy_static! {
    static ref PRIMITIVE_COUNT: Mutex<usize> = Mutex::new(0);
}

impl Primitive {
    /// Construct a Primitive.
    ///
    /// `transform`: a matrix to transform the geometry from origin to where it should locate in the world
    pub fn new(geometry: Box<Sphere>, material: SimpleMaterial, transform: Matrix4f) -> Self {
        let mut i = PRIMITIVE_COUNT.lock().unwrap();
        Self::new_with_label(format!("Unnamed primitive #{}", (*i, *i += 1).0), geometry, material, transform)
    }
    /// Construct a Primitive with custom label.
    ///
    /// `transform`: a matrix to transform the geometry from origin to where it should locate in the world
    pub fn new_with_label(label: String, geometry: Box<Sphere>, material: SimpleMaterial, transform: Matrix4f) -> Self {
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

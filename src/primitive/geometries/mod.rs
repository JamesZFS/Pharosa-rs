use super::*;

mod sphere;

pub use sphere::Sphere;

pub trait Geometry: Intersect + DynClone {
    // Others...
}

pub trait Intersect: Debug {
    /// Local ray to local intersection
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection>;
}

dyn_clone::clone_trait_object!(Geometry);

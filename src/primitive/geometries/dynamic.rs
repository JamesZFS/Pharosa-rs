use super::*;
use derive_more::*;


#[derive(Debug, Clone, From)]
/// Sphere or triangle
///
/// Note: Performance can be worse than static ones
pub enum DynamicGeometry {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Intersect for DynamicGeometry {
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection> {
        // dispatch job
        match self {
            DynamicGeometry::Sphere(s) => s.intersect(ray),
            DynamicGeometry::Triangle(t) => t.intersect(ray),
        }
    }
}

impl Geometry for DynamicGeometry {
    // todo
}

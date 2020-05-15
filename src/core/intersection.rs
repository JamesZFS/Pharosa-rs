use super::*;
use crate::primitive::*;

#[derive(Debug, Clone)]      /// Wraps geometric info of the intersection and the hit primitive
pub struct Intersection<'a, G, B, T>(pub GeometryIntersection, pub &'a Primitive<G, B, T>) where G: Geometry, B: BSDF, T: Texture;

#[derive(Debug, Clone, PartialEq)]
pub struct GeometryIntersection {
    pub pos: Point3f,
    pub normal: Vector3f,
    /// time from ray.org to the intersection
    pub t: Real,
}

impl TransformAny<GeometryIntersection> for Matrix4f {
    fn transform(&self, src: &GeometryIntersection) -> GeometryIntersection {
        GeometryIntersection {
            pos: self.transform_point(src.pos),
            normal: self.transform_vector(src.normal),
            t: src.t,
        }
    }
}

impl<'a, G, B, T> TransformAny<Intersection<'a, G, B, T>> for Matrix4f where G: Geometry, B: BSDF, T: Texture {
    #[inline]
    fn transform(&self, src: &Intersection<'a, G, B, T>) -> Intersection<'a, G, B, T> {
        Intersection(self.transform(&src.0), src.1)
    }
}

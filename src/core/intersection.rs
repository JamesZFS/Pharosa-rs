use super::*;
use crate::primitive::*;

#[derive(Debug, Clone)]       /// Wraps geometric info of the intersection and the hit primitive
pub struct Intersection<'a, G, B, T>(pub GeometryIntersection, pub &'a Primitive<G, B, T>) where G: Geometry, B: BSDF, T: Texture;

#[derive(Debug, Clone, PartialEq)]
pub struct GeometryIntersection {
    pub pos: Point3f,
    /// Regularized normal: on the same side with ray.dir
    pub normal: Vector3f,
    /// Incoming ray unit direction, pointing **out**
    pub wi: Vector3f,
    /// time from ray.org to the intersection
    pub t: Float,
    /// The ray is inside or outside the primitive?
    pub side: Side,
    // todo transform?
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    Outside,
    Inside,
}

impl TransformAny<GeometryIntersection> for Matrix4f {
    fn transform(&self, src: &GeometryIntersection) -> GeometryIntersection {
        GeometryIntersection {
            pos: self.transform_point(src.pos),
            normal: self.transform_vector(src.normal),
            wi: src.wi,
            t: src.t,
            side: src.side,
        }
    }
}

impl<'a, G, B, T> TransformAny<Intersection<'a, G, B, T>> for Matrix4f where G: Geometry, B: BSDF, T: Texture {
    #[inline]
    fn transform(&self, src: &Intersection<'a, G, B, T>) -> Intersection<'a, G, B, T> {
        Intersection(self.transform(&src.0), src.1)
    }
}

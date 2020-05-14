use super::*;

#[derive(Debug, Clone)]      /// Wraps geometric info of the intersection and the hit primitive
pub struct Intersection<'a>(pub GeometryIntersection, pub &'a Primitive);

#[derive(Debug, Clone, PartialEq)]
pub struct GeometryIntersection {
    pub pos: Point3f,
    pub normal: Vector3f,
    /// time from ray.org to the intersection
    pub t: Real,
}

impl Default for GeometryIntersection {
    fn default() -> Self {
        Self {
            pos: Point3::origin(),
            normal: Vector3::zero(),
            t: Real::infinity(),
        }
    }
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

impl<'a> TransformAny<Intersection<'a>> for Matrix4f {
    #[inline]
    fn transform(&self, src: &Intersection<'a>) -> Intersection<'a> {
        Intersection(self.transform(&src.0), src.1)
    }
}

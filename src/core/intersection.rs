use super::*;
use crate::primitive::*;

#[derive(Debug, Clone)]         /// Wraps geometric info of the intersection and the hit primitive
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
    pub uv: Point2f,
}

impl<'a, G, B, T> Intersection<'a, G, B, T> where G: Geometry, B: BSDF, T: Texture {
    /// Get the albedo at the intersection pos
    pub fn albedo(&self) -> &Spectrum {
        self.1.material.texture.at(self.0.uv)
    }
    pub fn emission(&self) -> &Spectrum { &self.1.material.emission }
    /// Importance sample the BSDF, return the outgoing direction, **weight x albedo** and pdf
    pub fn sample_bsdf(&self, samp: Point2f) -> bsdf::SampleRecord {
        let mut rec = self.1.material.bsdf.sample(&self.0, samp);
        rec.weight *= self.albedo();
        rec
    }
    pub fn pos(&self) -> Point3f { self.0.pos }
    pub fn normal(&self) -> Vector3f { self.0.normal }
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
            uv: src.uv,
        }
    }
}

impl<'a, G, B, T> TransformAny<Intersection<'a, G, B, T>> for Matrix4f where G: Geometry, B: BSDF, T: Texture {
    #[inline]
    fn transform(&self, src: &Intersection<'a, G, B, T>) -> Intersection<'a, G, B, T> {
        Intersection(self.transform(&src.0), src.1)
    }
}

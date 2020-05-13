use super::*;

#[derive(Clone)]
pub struct Ray {
    pub org: Point3f,
    pub dir: Vector3f,
    // todo min/max time
}

impl Ray {
    #[inline]
    pub fn new(org: Point3f, dir: Vector3f) -> Self { Self { org, dir } }

    #[inline]
    pub fn transport(&self, t: Real) -> Point3f { self.org + self.dir * t }
}

impl TransformAny<Ray> for Matrix4f {
    #[inline]
    fn transform(&self, src: &Ray) -> Ray {
        Ray::new(self.transform_point(src.org), self.transform_vector(src.dir))
    }
}

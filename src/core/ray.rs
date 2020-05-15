use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub org: Point3f,
    pub dir: Vector3f,
}

impl Ray {
    pub fn new(org: Point3f, dir: Vector3f) -> Self {
        debug_assert_approx!(dir.magnitude(), 1.0);
        Self { org, dir }
    }

    pub fn transport(&self, t: Float) -> Point3f { self.org + self.dir * t }
}

impl TransformAny<Ray> for Matrix4f {
    #[inline]
    fn transform(&self, src: &Ray) -> Ray {
        Ray::new(self.transform_point(src.org), self.transform_vector(src.dir))
    }
}

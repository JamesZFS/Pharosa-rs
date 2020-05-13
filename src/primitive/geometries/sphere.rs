use super::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    radius: Real,
    rad2: Real,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere { radius, rad2: radius * radius }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection> {
        let op: Vector3f = Point3f::origin() - ray.org;
        let b = op.dot(op);
        let det = b * b - op.dot(op) + self.rad2;
        if det < 0. {
            None
        } else {
            let det = det.sqrt();
            let t = b - det;
            if t > Real::epsilon() {
                Some(GeometryIntersection {
                    pos: ray.transport(t),
                    normal: op,
                    t,
                })
            } else {
                let t = b + det;
                if t > Real::epsilon() {
                    Some(GeometryIntersection {
                        pos: ray.transport(t),
                        normal: op,
                        t,
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Geometry for Sphere {

}

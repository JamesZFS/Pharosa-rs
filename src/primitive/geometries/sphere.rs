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
    /// Solve for ` (org + t dir)^2 == r^2 `
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection> {
        debug_assert_approx!(ray.dir.magnitude(), 1.0); // assume dir^2 == 1
        let org = ray.org.to_vec();
        let b: Real = org.dot(ray.dir);
        let delta = b * b - org.magnitude2() + self.rad2;
        if delta < 0. {
            None
        } else {
            let ds = delta.sqrt();
            let t = -b - ds;
            if t > Real::epsilon() { // front?
                let pos = ray.transport(t);
                Some(GeometryIntersection {
                    pos,
                    normal: pos.to_vec(),
                    t,
                })
            } else { // back?
                let t = -b + ds;
                if t > Real::epsilon() {
                    let pos = ray.transport(t);
                    Some(GeometryIntersection {
                        pos,
                        normal: pos.to_vec(),
                        t,
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Geometry for Sphere {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_intersect() {
        let s = Sphere::new(1.0);
        let r = Ray::new(pt3(0.0, 0.0, -10.0), vec3(0., 1., -1.).normalize());
        assert_eq!(s.intersect(&r), None);
        let r = Ray::new(pt3(0., 10., 0.), vec3(0., 1., 0.));
        assert_eq!(s.intersect(&r), None);
    }

    #[test]
    fn has_intersect() {
        let s = Sphere::new(1.0);
        let r = Ray::new(pt3(10., 0., 0.), vec3(-1., 0., 0.));
        assert_eq!(s.intersect(&r), Some(GeometryIntersection {
            pos: pt3(1., 0., 0.),
            normal: vec3(1., 0., 0.),
            t: 9.0,
        }));
        let r = Ray::new(pt3(0., 0., 0.), vec3(1., 1., 0.).normalize());
        let x = (2.0 as Real).sqrt() / 2.0;
        let p = pt3(x, x, 0.);
        assert_eq!(s.intersect(&r), Some(GeometryIntersection {
            pos: p,
            normal: p.to_vec(),
            t: 1.0,
        }));
    }
}

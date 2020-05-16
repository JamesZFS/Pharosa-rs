use super::*;
use crate::macros::*;
use std::mem::MaybeUninit;

#[derive(Debug, Clone)]
pub struct Sphere { // todo: store global coordinates instead of local
    radius: Float,
    rad2: Float,
}

impl Sphere {
    pub fn new(radius: Float) -> Self {
        debug_assert_gt!(radius, 0.);
        Sphere { radius, rad2: radius * radius }
    }
    pub fn set_radius(&mut self, new: Float) {
        debug_assert_gt!(new, 0.);
        self.radius = new;
        self.rad2 = new * new;
    }
}

impl Intersect for Sphere {
    /// Solve for ` (org + t dir)^2 == r^2 `
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection> {
        debug_assert_approx!(ray.dir.magnitude(), 1.0); // assume dir^2 == 1
        let org = ray.org.to_vec();
        let b: Float = org.dot(ray.dir);
        let delta = b * b - org.magnitude2() + self.rad2;
        if delta < 0. {
            None
        } else {
            let ds = delta.sqrt();
            let t = -b - ds;
            if t > Float::epsilon() { // front?
                let pos = ray.transport(t);
                Some(GeometryIntersection {
                    pos,
                    normal: pos.to_vec().normalize(),
                    wi: -ray.dir, // todo slow
                    t,
                    side: Side::Outside,
                    uv: unsafe { MaybeUninit::zeroed().assume_init() }
                })
            } else { // back?
                let t = -b + ds;
                if t > Float::epsilon() {
                    let pos = ray.transport(t);
                    Some(GeometryIntersection {
                        pos,
                        normal: -pos.to_vec().normalize(),
                        wi: -ray.dir,
                        t,
                        side: Side::Inside,
                        uv: unsafe { MaybeUninit::zeroed().assume_init() }
                    })
                } else {
                    None
                }
            }
        }
    }
}

impl Geometry for Sphere {
    // todo
}

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
            wi: -r.dir,
            t: 9.0,
            side: Side::Outside
        }));
        let r = Ray::new(pt3(0., 0., 0.), vec3(1., 1., 0.).normalize());
        let x = (2.0 as Float).sqrt() / 2.0;
        let p = pt3(x, x, 0.);
        let its = s.intersect(&r).unwrap();
        assert_approx!((its.pos - p).magnitude(), 0.);
        assert_approx!((-its.normal - p.to_vec()).magnitude(), 0.);
        assert_approx!(its.t - 1.0, 0.);
        assert_eq!(its.side, Side::Inside)
    }
}

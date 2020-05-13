pub use perspective::Perspective;

use crate::core::*;

mod perspective;
mod orthogonal;

#[derive(Debug, Clone)]
pub struct Camera<C: CameraInner> {
    inner: C,
    world_to_local: Matrix4f,
    local_to_world: Matrix4f,
}

impl<C> Camera<C> where C: CameraInner {
    /// Construct a Camera.
    ///
    /// `transform`: a matrix to transform the camera from origin to where it should locate and look at in the world
    pub fn new(inner: C, transform: Matrix4f) -> Self {
        Self {
            inner,
            world_to_local: transform.inverse_transform()
                .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform))),
            local_to_world: transform,
        }
    }

    /// Sample a ray, return the world ray and its pdf
    pub fn generate_ray(&self, x: u32, y: u32, aperture_samp: Point2f) -> (Ray, Real) {
        let (ray, pdf) = self.inner.generate_ray(x, y, aperture_samp);
        debug_assert_eq!(ray.dir.magnitude2(), 1.0);
        (self.local_to_world.transform(&ray), pdf)
    }

    pub fn set_transform(&mut self, transform: Matrix4f) {
        self.local_to_world = transform;
        self.world_to_local = transform.inverse_transform()
            .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform)));
    }
}

pub trait CameraInner {
    /// Sample a ray, return the local ray and its pdf
    fn generate_ray(&self, x: u32, y: u32, aperture_samp: Point2f) -> (Ray, Real);
}

#[cfg(test)]
mod test {
    use crate::sampler;
    use crate::sampler::Sampler;

    use super::*;

    #[test]
    fn camera() {
        let pers = Perspective::new(10, 10, Deg(90.));
        let camera = Camera::new(
            pers,
            Matrix4::look_at(
                Point3::new(0., 0., -1.),
                Point3::new(0., 0., 0.),
                vec3(0., 1., 0.)),
        );
        let mut samp = sampler::Fake;
        let (ray, pdf) = camera.generate_ray(5, 5, samp.next2d());
        assert_eq!(pdf, 1.0);
        assert_eq!(ray, Ray::new(Point3::new(0., 0., -1.), Vector3::unit_z()))
    }
}

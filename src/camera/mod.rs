pub use perspective::Perspective;

use crate::core::*;
use std::fmt::Debug;

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
    pub fn generate_ray(&self, x: u32, y: u32, aperture_samp: Point2f) -> (Ray, Float) {
        let (ray, pdf) = self.inner.generate_ray(x, y, aperture_samp);
        debug_assert_approx!(ray.dir.magnitude(), 1.0);
        (self.local_to_world.transform(&ray), pdf)
    }

    pub fn transform(&self) -> Matrix4f {
        self.local_to_world
    }

    pub fn set_transform(&mut self, transform: Matrix4f) {
        self.local_to_world = transform;
        self.world_to_local = transform.inverse_transform()
            .unwrap_or_else(|| panic!(format!("Singular transform {:?}", transform)));
    }

    pub fn translate(&mut self, translation: Vector3f) {
        self.set_transform(Matrix4::from_translation(translation) * self.local_to_world)
    }
}

pub trait CameraInner: Clone + Debug + Send + Sync + 'static {
    /// Sample a ray, return the local ray and its pdf
    fn generate_ray(&self, x: u32, y: u32, aperture_samp: Point2f) -> (Ray, Float);
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
                pt3(0., 0., -1.),
                pt3(0., 0., 0.),
                vec3(0., 1., 0.)),
        );
        let mut samp = sampler::Fake;
        let (ray, pdf) = camera.generate_ray(5, 5, samp.next2d());
        assert_approx!(pdf, 1.0);
        assert_eq!(ray, Ray::new(pt3(0., 0., -1.), Vector3::unit_z()))
    }
}

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
    /// Construct a Camera with the observer's parameters given
    pub fn new(inner: C, eye: Point3f, gaze: Point3f, up: Vector3f) -> Self {
        let world_to_local = Matrix4::look_at(eye, gaze, up);
        Self {
            inner,
            world_to_local,
            local_to_world: world_to_local.inverse_transform()
                .unwrap_or_else(|| panic!(format!("Singular transform when initializing camera: eye = {:?} gaze = {:?} up = {:?}", eye, gaze, up))),
        }
    }

    /// Sample a ray at **image** screen pixel (x, y), return the world ray and its pdf
    ///
    /// Notice: the screen coordinate y is inverse to physical coordinate!
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
        self.set_transform(self.local_to_world * Matrix4::from_translation(translation))
    }

    pub fn rotate<A: Into<Radf>>(&mut self, axis: Vector3f, angle: A) {
        self.set_transform(self.local_to_world * Matrix4::from_axis_angle(axis, angle));
    }
}

pub trait CameraInner: Clone + Debug + Send + Sync + 'static {
    /// Sample a ray at **image** screen pixel coordinate (x, y), return the local ray and its pdf
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
            pt3(0., 0., -1.),
            pt3(0., 0., 0.),
            vec3(0., 1., 0.),
        );
        let mut samp = sampler::Fake;
        let (ray, pdf) = camera.generate_ray(5, 5, samp.next2d());
        assert_approx!(pdf, 1.0);
        assert_eq!(ray, Ray::new(pt3(0., 0., -1.), Vector3::unit_z()))
    }

    #[test]
    fn transform() {
        let mut camera = Camera::new(
            Perspective::new(10, 10, Deg(90.)),
            pt3(10., 10., 1.),
            pt3(10., 10., 0.),
            vec3(0., 1., 0.),
        );
        let mut samp = sampler::Fake;
        let (ray, _) = camera.generate_ray(8, 8, samp.next2d());
        camera.translate(vec3(0., 0.5, 0.));
        let (ray2, _) = camera.generate_ray(8, 8, samp.next2d());
        assert_eq!(ray.dir, ray2.dir);
        assert_eq!(ray.org + vec3(0., 0.5, 0.), ray2.org);
    }
}

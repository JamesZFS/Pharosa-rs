use super::*;

#[derive(Clone, Debug)]
pub struct Perspective {
    width: Real,
    height: Real,
    /// width / height
    aspect: Real,
    fovy: Radf,
    /// -1/2 cot(fovy/2)
    dir_z: Real,
    fovx: Radf,
}

impl Perspective {
    pub fn new<A>(width: u32, height: u32, fovy: A) -> Self where A: Into<Radf> {
        let (width, height) = (width as Real, height as Real);
        let fovy = fovy.into();
        let tan_fovy_2 = (fovy.0 / 2.).tan();
        let aspect = width / height;
        Self {
            width,
            height,
            aspect,
            fovy,
            dir_z: -0.5 / tan_fovy_2,
            fovx: Rad(2. * (aspect * tan_fovy_2).atan()),
        }
    }
}

impl CameraInner for Perspective {
    fn generate_ray(&self, x: u32, y: u32, aperture_samp: Point2f) -> (Ray, Real) {
        debug_assert!((x as Real) < self.width && (y as Real) < self.height);
        // [0,1] -> [-0.5,0.5]
        let (x, y) = (x as Real + (aperture_samp.x - 0.5), y as Real + (aperture_samp.y - 0.5)); // MSAA
        let dir = vec3(
            (x / self.width - 0.5) * self.aspect,
            y / self.height - 0.5,
            self.dir_z,
        );
        (Ray::new(Point3::origin(), dir.normalize()), 1.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::macros::*;
    use crate::scene::*;
    use crate::sampler::{Fake, Sampler};
    use crate::primitive::*;
    use std::sync::Arc;

    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;

    fn setup_scene() -> Scene<impl Geometry, impl BSDF, impl Texture> {
        let mut scene = Scene::new();
        scene.push(Primitive::new_with_label(
            "sphere".into(),
            Sphere::new(2.),
            Arc::new(Material { bsdf: bsdf::Simple::default(), texture: texture::Uniform::new(Spectrum::new(1., 0., 0.)) }),
            Matrix4::from_translation(vec3(0., 0., 0.))));
        scene
    }

    fn setup_camera(fovy: impl Into<Radf>) -> Camera<Perspective> {
        let pers = Perspective::new(WIDTH, HEIGHT, fovy);
        let camera = Camera::new(
            pers,
            Matrix4::look_at(
                pt3(0., 0., -4.),
                pt3(0., 0., 0.),
                vec3(0., 1., 0.)),
        );
        camera
    }

    #[test]
    fn fov_test() {
        let scene = setup_scene();
        let camera = setup_camera(Deg(60.)); // just tangent
        let mut sampler = Fake;
        let mut hit_count = 0;
        let x = WIDTH / 2;
        for y in 0..HEIGHT {
            if let Some(its) = scene.nearest_hit(&camera.generate_ray(x, y, sampler.next2d()).0) {
                assert_eq!(its.1.label, "sphere");
                hit_count += 1;
            }
        }
        assert_eq!(hit_count, HEIGHT);

        let camera = setup_camera(Deg(61.));
        let mut hit_count = 0;
        let x = WIDTH / 2;
        for y in 0..HEIGHT {
            if let Some(its) = scene.nearest_hit(&camera.generate_ray(x, y, sampler.next2d()).0) {
                assert_eq!(its.1.label, "sphere");
                hit_count += 1;
            }
        }
        assert_lt!(hit_count, HEIGHT);
    }
}

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

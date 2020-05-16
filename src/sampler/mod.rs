use crate::core::*;

mod independent;
mod fake;

pub use independent::Independent;
pub use fake::Fake;
use std::fmt::Debug;

pub trait Sampler: Debug + Clone + Send + Sync + 'static {
    fn next(&mut self) -> Float;
    fn next2d(&mut self) -> Point2f;
}

pub fn cosine_on_hemisphere(samp: Point2f) -> Point3f {
    let Point2f { x, y } = uniform_on_disk(samp);
    pt3(x, y, (1. - x * x - y * y).max(0.).sqrt())
}

pub fn uniform_on_disk(samp: Point2f) -> Point2f {
    let (x, y) = (2. * samp.x - 1., 2. * samp.y - 1.); // [0, 1]^2 -> [-1, 1]^2
    if x == 0. && y == 0. { return pt2(0., 0.); }
    let (phi, r) = if x.abs() > y.abs() {
        (y / x * Float::FRAC_PI_4(), x)
    } else {
        (Float::FRAC_PI_2() - x / y * Float::FRAC_PI_4(), y)
    };
    pt2(r * phi.cos(), r * phi.sin())
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::macros::*;

    #[test]
    fn test_uniform_on_disk() {
        use crate::utils::ToImageBuffer;
        let mut sampler = Independent;
        let mut film = Film::new_with_color(100, 100, Spectrum::white());
        for _i in 0..10000 {
            let Point2f { x, y } = uniform_on_disk(sampler.next2d());
            assert_le!(x*x + y*y, 1.);
            let (x, y) = ((100. * (0.5 * x + 0.5)) as u32, (100. * (0.5 * y + 0.5)) as u32);
            *film.at_mut(x, y) = Spectrum::black();
        }
        film.to_image_buffer().save("test-dist.png").unwrap();
    }

    #[test]
    fn test_cosine_on_hemisphere() {
        let mut sampler = Independent;
        for _i in 0..10000 {
            let samp = cosine_on_hemisphere(sampler.next2d());
            assert_approx!(samp.to_vec().magnitude(), 1.);
            assert_ge!(samp.z, 0.);
        }
    }
}

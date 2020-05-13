use super::*;
use image::*;

pub type RGBf = Rgb<Real>;

impl From<Spectrum> for RGBf {
    fn from(c: Spectrum) -> Self {
        Self([c.x, c.y, c.z])
    }
}

pub type Film = ImageBuffer<RGBf, Vec<<RGBf as Pixel>::Subpixel>>;

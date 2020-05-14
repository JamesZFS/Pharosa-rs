use crate::core::*;
use image::*;

const GAMMA: Real = 2.2;
const INV_GAMMA: Real = 1. / GAMMA;

pub trait GammaCorrection {
    fn gamma_correction(&self) -> Self;
}

impl GammaCorrection for Real {
    fn gamma_correction(&self) -> Self {
        self.powf(INV_GAMMA)
    }
}

impl GammaCorrection for Spectrum {
    fn gamma_correction(&self) -> Self {
        Self::new(self.x.gamma_correction(), self.y.gamma_correction(), self.z.gamma_correction())
    }
}

impl From<Spectrum> for [u8; 3] {
    fn from(s: Spectrum) -> Self {
        [(s.x * 255.) as u8, (s.y * 255.) as u8, (s.z * 255.) as u8]
    }
}

pub trait ToImageBuffer {
    fn to_image_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}

impl ToImageBuffer for Film {
    fn to_image_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(self.width(), self.height(),
                             |x, y| Rgb(self.at(x, y).gamma_correction().into()))
    }
}

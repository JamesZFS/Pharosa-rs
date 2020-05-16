use crate::core::*;
use image::*;

const GAMMA: Float = 2.2;
const INV_GAMMA: Float = 1. / GAMMA;

pub trait GammaCorrection {
    fn gamma_correction(&self) -> Self;
}

impl GammaCorrection for Float {
    fn gamma_correction(&self) -> Self {
        self.powf(INV_GAMMA)
    }
}

impl GammaCorrection for Spectrum {
    fn gamma_correction(&self) -> Self {
        Self::new(self.r.gamma_correction(), self.g.gamma_correction(), self.b.gamma_correction())
    }
}

impl From<Spectrum> for [u8; 3] {
    fn from(s: Spectrum) -> Self {
        [(s.r * 255.) as u8, (s.g * 255.) as u8, (s.b * 255.) as u8]
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

pub trait ToHexColor {
    fn to_hex_color(&self) -> u32;
}

impl ToHexColor for Spectrum {
    fn to_hex_color(&self) -> u32 {
        (((self.r * 255.) as u32) << 16) | (((self.g * 255.) as u32) << 8) | ((self.b * 255.) as u32)
    }
}

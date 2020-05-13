use super::*;

#[derive(Debug, Clone)] /// Simple materials
pub enum Simple {
    Diffuse,
    Specular,
    Dielectric { n: Real },
}

impl Default for Simple {
    fn default() -> Self {
        Simple::Diffuse
    }
}

impl BSDF for Simple {
}

use super::*;

#[derive(Debug, Clone)]
pub struct PathTracing {
    pub max_depth: u32,
    pub max_depth_rr: u32,
}

#[allow(unused_variables)]
impl SampleIntegratorDelegate for PathTracing {
    fn Li<G, B, T>(&self, ray: Ray, scene: &Scene<G, B, T>, sampler: &mut impl Sampler) -> Spectrum
        where G: Geometry, B: BSDF, T: Texture {
        unimplemented!()
    }
}

impl Default for PathTracing {
    fn default() -> Self {
        unimplemented!()
    }
}

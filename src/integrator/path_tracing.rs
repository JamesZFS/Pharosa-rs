use super::*;

#[derive(Debug, Clone)]
pub struct PathTracing {
    pub max_depth: u32,
    pub max_depth_rr: u32,
}

#[allow(unused_variables)]
impl SampleIntegratorDelegate for PathTracing {
    fn Li(&self, ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, sampler: &mut impl Sampler) -> Spectrum {
        unimplemented!()
    }
}

impl Default for PathTracing {
    fn default() -> Self {
        unimplemented!()
    }
}

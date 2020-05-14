use super::*;

#[derive(Debug, Clone)]
pub struct PathTracing {
    pub max_depth: u32,
    pub max_depth_rr: u32,
}

#[allow(non_snake_case)]
impl SampleIntegratorDelegate for PathTracing {
    fn Li(&self, _ray: Ray, _scene: &Scene, _sampler: &mut impl Sampler) -> Spectrum {
        unimplemented!()
    }
}

impl Default for PathTracing {
    fn default() -> Self {
        unimplemented!()
    }
}

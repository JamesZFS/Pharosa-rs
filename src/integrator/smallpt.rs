use super::*;

#[derive(Debug, Clone)]
pub struct SmallPT {
    /// Depth for doing Russian Roulette
    pub rr_depth: u32,
}

impl SampleIntegratorDelegate for SmallPT {
    fn Li(&self, mut ray: Ray, scene: &Scene<impl Geometry, impl BSDF, impl Texture>, sampler: &mut impl Sampler) -> Spectrum {
        let mut throughput = Spectrum::white();
        let mut radiance = Spectrum::black();
        let mut depth = 0;
        loop {
            match scene.nearest_hit(&ray) {
                None => {
                    radiance += &throughput * scene.environ_map(&ray);
                    break;
                }
                Some(its) => {
                    radiance += &throughput * its.emission();
                    // do bsdf sampling:
                    let b_rec = its.sample_bsdf(sampler.next2d());
                    throughput *= b_rec.weight / b_rec.pdf;

                    let P = throughput.max();
                    if P < 1e-3 || depth >= self.rr_depth { // R.R.
                        if sampler.next() < P { // continue
                            throughput /= P;
                        } else { // terminate
                            break;
                        }
                    }

                    // forward ray to the next intersection
                    ray = Ray::new(its.pos(), b_rec.wo);
                    ray.forward(Float::epsilon());
                    depth += 1;
                },
            }
        }
        radiance
    }
}

impl Default for SmallPT {
    fn default() -> Self {
        Self { rr_depth: 4 }
    }
}

use super::*;
use derive_more::*;
use crate::sampler::cosine_on_hemisphere;

#[derive(Debug, Clone, From)]           /// Simple materials
pub enum Simple {
    Diffuse(Diffuse),
    Specular(Specular),
    Dielectric(Dielectric),
}

impl Default for Simple {
    fn default() -> Self {
        Diffuse.into()
    }
}

impl BSDF for Simple {
    fn sample(&self, its: &GeometryIntersection, samp: Point2f) -> SampleRecord {
        match self {
            Simple::Diffuse(d) => d.sample(its, samp),
            Simple::Specular(s) => s.sample(its, samp),
            Simple::Dielectric(d) => d.sample(its, samp),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Diffuse;

#[derive(Debug, Clone, Default)]
pub struct Specular;

#[derive(Debug, Clone)]
pub struct Dielectric {
    /// Refraction index
    pub n: Float
}

impl Default for Dielectric {
    fn default() -> Self {
        Self { n: 1.5 } // assuming glass
    }
}

impl BSDF for Diffuse {
    /// Sample cosine on hemisphere
    fn sample(&self, its: &GeometryIntersection, samp: Point2f) -> SampleRecord {
        let samp = cosine_on_hemisphere(samp);
        let basis = onb(its.normal);
        SampleRecord {
            wo: basis * samp.to_vec(),
            weight: Spectrum::white(), // assume no attenuation
            pdf: 1., // since we just cosine-ly sampled the diffuse surface
        }
    }
}

impl BSDF for Specular {
    /// Sample delta dist. with determined direction
    fn sample(&self, its: &GeometryIntersection, _samp: Point2f) -> SampleRecord {
        SampleRecord {
            wo: 2. * dot(its.wi, its.normal) * its.normal - its.wi,
            weight: Spectrum::white(), // assume no attenuation
            pdf: 1.,
        }
    }
}

#[allow(non_snake_case)]
impl BSDF for Dielectric {
    /// Sample delta dist. with determined direction
    fn sample(&self, its: &GeometryIntersection, samp: Point2f) -> SampleRecord {
        use Side::*;
        // reflection:
        let w_R: Vector3f = 2. * dot(its.wi, its.normal) * its.normal - its.wi;
        let nc = 1.; // todo, support more
        let nt = self.n;
        let nnt = match its.side {
            Outside => nc / nt,
            Inside => nt / nc,
        };
        let ddn: Float = -dot(its.wi, its.normal);
        let cos2t = 1. - nnt * nnt * (1. - ddn * ddn);
        if cos2t < 0. { // complete internal reflection
            SampleRecord {
                wo: w_R,
                weight: Spectrum::white(),
                pdf: 1.,
            }
        } else {  // refraction and reflection
            let w_T: Vector3f = -its.wi * nnt - its.normal * (ddn * nnt + cos2t.sqrt());
            let a = nt - nc;
            let b = nt + nc;
            let c = 1. + match its.side {
                Outside => ddn,
                Inside => dot(w_T, its.normal),
            };
            let R0 = a * a / (b * b);
            let Re = R0 + (1. - R0) * c.powi(5);
            let Tr = 1. - Re;
            let P = 0.25 + 0.5 * Re;
            // Russian roulette, de-branching
            if samp.x < P { // sample reflection
                SampleRecord {
                    wo: w_R,
                    weight: Spectrum::uniform(Re),
                    pdf: P,
                }
            } else {
                SampleRecord { // sample transmission
                    wo: w_T,
                    weight: Spectrum::uniform(Tr),
                    pdf: 1. - P,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::random;
    use crate::sampler::Independent;
    use crate::Sampler;
    use crate::macros::*;

    #[test]
    fn diffuse_sample() {
        let mut sampler = Independent;
        let its = GeometryIntersection {
            pos: pt3(1., 1., 1.),
            normal: vec3(1., 0., 0.),
            wi: vec3(random(), random(), random()),
            t: random(),
            side: Side::Outside,
            uv: pt2(random(), random()),
        };
        let diffuse = Diffuse;
        for _ in 0..10000 {
            let rc = diffuse.sample(&its, sampler.next2d());
            assert_eq!(rc.weight, Spectrum::uniform(1.));
            assert_eq!(rc.pdf, 1.);
            assert_approx!(rc.wo.magnitude(), 1.);
            assert_ge!(dot(rc.wo, its.normal), 0.);
        }
    }
}

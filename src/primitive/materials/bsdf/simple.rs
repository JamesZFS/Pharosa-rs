use super::*;
use derive_more::*;
use crate::sampler::cosine_on_hemisphere;

#[derive(Debug, Clone, From)]          /// Simple materials
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
        // todo: too slow
        let basis = Basis3::look_at(its.normal, if its.normal.x.abs() > 0.1 { Vector3::unit_y() } else { Vector3::unit_x() });
        SampleRecord {
            wo: basis.rotate_vector(samp.to_vec()),
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
}

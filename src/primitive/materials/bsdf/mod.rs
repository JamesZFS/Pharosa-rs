use super::*;

pub mod simple;

pub use simple::Simple;

pub trait BSDF: Debug + Clone + Send + Sync + 'static {
    /// Importance sample the BSDF, return the outgoing direction, weight and pdf
    fn sample(&self, its: &GeometryIntersection, samp: Point2f) -> SampleRecord;
}

#[derive(Debug, Clone)]
pub struct SampleRecord {
    /// New ray unit direction, pointing out
    pub wo: Vector3f,
    /// BSDF weight
    pub weight: Spectrum,
    /// pdf in this sample
    pub pdf: Float,
}

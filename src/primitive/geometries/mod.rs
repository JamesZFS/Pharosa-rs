use super::*;

mod sphere;
mod triangle;
mod dynamic;

pub use sphere::Sphere;
pub use triangle::Triangle;
pub use dynamic::DynamicGeometry;

pub trait Geometry: Intersect + Send + Sync + 'static {
    // Others...
}

pub trait Intersect: Debug + Clone {
    /// Local ray to local intersection
    fn intersect(&self, ray: &Ray) -> Option<GeometryIntersection>;
}


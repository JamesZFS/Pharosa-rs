use super::*;

#[derive(Debug, Clone)]
pub struct Triangle {
    // Todo
}

impl Intersect for Triangle {
    fn intersect(&self, _ray: &Ray) -> Option<GeometryIntersection> {
        unimplemented!()
    }
}

impl Geometry for Triangle {
    // todo
}

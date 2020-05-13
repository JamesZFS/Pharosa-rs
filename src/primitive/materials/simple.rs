use super::*;

#[derive(Debug, Clone)] /// Simple materials
enum Simple {
    Diffuse,
    Specular,
    Dielectric { n: Real },
}

impl Simple {
    // todo
}

impl Material for Simple {

}

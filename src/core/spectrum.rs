use super::*;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Spectrum(Vector3f);

impl Spectrum {
    pub fn new(r: Float, g: Float, b: Float) -> Self { Spectrum(vec3(r, g, b)) }
    pub fn uniform(f: Float) -> Self { Spectrum(vec3(f, f, f)) }
    pub fn black() -> Self { Spectrum(Vector3::zero()) }
    pub fn white() -> Self { Self::uniform(1.) }
    pub fn max(&self) -> Float { self.x.max(self.y).max(self.z) }
    pub fn min(&self) -> Float { self.x.min(self.y).min(self.z) }
    pub fn sum(&self) -> Float { self.x + self.y + self.z }
}

impl Default for Spectrum {
    fn default() -> Self {
        Spectrum::black()
    }
}

impl Deref for Spectrum {
    type Target = Vector3f;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Spectrum {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Add for Spectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Spectrum(self.0.add_element_wise(rhs.0)) }
}

impl AddAssign for Spectrum {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign_element_wise(rhs.0);
    }
}

impl Sub for Spectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Spectrum(self.0.sub_element_wise(rhs.0)) }
}

impl SubAssign for Spectrum {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign_element_wise(rhs.0);
    }
}

impl Mul for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { Spectrum(self.0.mul_element_wise(rhs.0)) }
}

impl MulAssign<Spectrum> for Spectrum {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign_element_wise(rhs.0);
    }
}

impl MulAssign<Float> for Spectrum {
    fn mul_assign(&mut self, rhs: Float) {
        self.0.mul_assign_element_wise(rhs)
    }
}

impl Div for Spectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output { Spectrum(self.0.div_element_wise(rhs.0)) }
}

impl DivAssign<Spectrum> for Spectrum {
    fn div_assign(&mut self, rhs: Self) {
        self.0.div_assign_element_wise(rhs.0);
    }
}

impl DivAssign<Float> for Spectrum {
    fn div_assign(&mut self, rhs: Float) {
        self.0.div_assign_element_wise(rhs)
    }
}

impl Mul<Float> for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output { Spectrum(self.0 * rhs) }
}

impl Div<Float> for Spectrum {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output { Spectrum(self.0 / rhs) }
}

impl From<Point3f> for Spectrum {
    fn from(p: Point3f) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}

impl From<Vector3f> for Spectrum {
    fn from(v: Vector3f) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ops() {
        let c1 = Spectrum::new(0., 1., 2.);
        let c2 = Spectrum::new(2., 2., 4.);
        assert_eq!(c1 / c2, Spectrum::new(0., 0.5, 0.5));
        assert_eq!(c1 * c2, Spectrum::new(0., 2., 8.));
        assert_eq!(c1 * 2., Spectrum::new(0., 2., 4.));
        assert_eq!(c1 + c2, Spectrum::new(2., 3., 6.));
        assert_eq!(c1 - c2, Spectrum::new(-2., -1., -2.));
    }
}

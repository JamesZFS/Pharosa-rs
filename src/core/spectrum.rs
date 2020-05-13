use super::*;

use std::ops::{Add, Sub, Mul, Div, Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Spectrum(Vector3f);

impl Deref for Spectrum {
    type Target = Vector3f;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Spectrum {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Spectrum {
    pub fn new(r: Real, g: Real, b: Real) -> Self { Spectrum(vec3(r, g, b)) }
}

impl Add for Spectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Spectrum(self.0.add_element_wise(rhs.0)) }
}

impl Sub for Spectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Spectrum(self.0.sub_element_wise(rhs.0)) }
}

impl Mul for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output { Spectrum(self.0.mul_element_wise(rhs.0)) }
}

impl Div for Spectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output { Spectrum(self.0.div_element_wise(rhs.0)) }
}

impl Mul<Real> for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self::Output { Spectrum(self.0 * rhs) }
}

impl Div<Real> for Spectrum {
    type Output = Self;
    fn div(self, rhs: Real) -> Self::Output { Spectrum(self.0 / rhs) }
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

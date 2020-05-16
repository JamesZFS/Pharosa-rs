use super::*;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Spectrum {
    pub r: Float,
    pub g: Float,
    pub b: Float,
}

impl Spectrum {
    pub fn new(r: Float, g: Float, b: Float) -> Self { Self { r, g, b } }
    pub fn uniform(f: Float) -> Self { Self::new(f, f, f) }
    pub fn black() -> Self { Self::uniform(0.) }
    pub fn white() -> Self { Self::uniform(1.) }
    pub fn max(&self) -> Float { self.r.max(self.g).max(self.b) }
    pub fn min(&self) -> Float { self.r.min(self.g).min(self.b) }
    pub fn sum(&self) -> Float { self.r + self.g + self.b }
}

impl From<Vector3f> for Spectrum {
    fn from(v: Vector3f) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Point3f> for Spectrum {
    fn from(p: Point3f) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}

impl Default for Spectrum {
    fn default() -> Self {
        Spectrum::black()
    }
}

macro_rules! impl_op {
    ($Ty: ty, $Op: tt, $op: ident) => {
        impl $Op<$Ty> for $Ty {  // T + T
            type Output = $Ty;
            fn $op(self, rhs: $Ty) -> Self::Output { <$Ty>::new(self.r.$op(rhs.r), self.g.$op(rhs.g), self.b.$op(rhs.b)) }
        }
        impl $Op<$Ty> for &$Ty {  // &T + T
            type Output = $Ty;
            fn $op(self, rhs: $Ty) -> Self::Output { <$Ty>::new(self.r.$op(rhs.r), self.g.$op(rhs.g), self.b.$op(rhs.b)) }
        }
        impl $Op<&$Ty> for $Ty {  // T + &T
            type Output = $Ty;
            fn $op(self, rhs: &$Ty) -> Self::Output { <$Ty>::new(self.r.$op(rhs.r), self.g.$op(rhs.g), self.b.$op(rhs.b)) }
        }
        impl $Op<&$Ty> for &$Ty {  // &T + &T
            type Output = $Ty;
            fn $op(self, rhs: &$Ty) -> Self::Output { <$Ty>::new(self.r.$op(rhs.r), self.g.$op(rhs.g), self.b.$op(rhs.b)) }
        }
    };
}

macro_rules! impl_op_float {
    ($Ty: ty, $Op: tt, $op: ident) => {
        impl $Op<Float> for $Ty {  // T * float
            type Output = $Ty;
            fn $op(self, rhs: Float) -> Self::Output { <$Ty>::new(self.r.$op(rhs), self.g.$op(rhs), self.b.$op(rhs)) }
        }
        impl $Op<Float> for &$Ty {  // &T * float
            type Output = $Ty;
            fn $op(self, rhs: Float) -> Self::Output { <$Ty>::new(self.r.$op(rhs), self.g.$op(rhs), self.b.$op(rhs)) }
        }
        impl $Op<$Ty> for Float {  // float * T
            type Output = $Ty;
            fn $op(self, rhs: $Ty) -> Self::Output { <$Ty>::new(self.$op(rhs.r), self.$op(rhs.g), self.$op(rhs.b)) }
        }
        impl $Op<&$Ty> for Float {  // float * &T
            type Output = $Ty;
            fn $op(self, rhs: &$Ty) -> Self::Output { <$Ty>::new(self.$op(rhs.r), self.$op(rhs.g), self.$op(rhs.b)) }
        }
    };
}

macro_rules! impl_op_assign {
    ($Ty: ty, $Op: tt, $op_assign: ident) => {
        impl $Op<$Ty> for $Ty {  // T += T
            fn $op_assign(&mut self, rhs: $Ty) { self.r.$op_assign(rhs.r); self.g.$op_assign(rhs.g); self.b.$op_assign(rhs.b); }
        }
        impl $Op<&$Ty> for $Ty {  // T += &T
            fn $op_assign(&mut self, rhs: &$Ty) { self.r.$op_assign(rhs.r); self.g.$op_assign(rhs.g); self.b.$op_assign(rhs.b); }
        }
    };
}

macro_rules! impl_op_float_assign {
    ($Ty: ty, $Op: tt, $op_assign: ident) => {
        impl $Op<Float> for $Ty {  // T *= float
            fn $op_assign(&mut self, rhs: Float) { self.r.$op_assign(rhs); self.g.$op_assign(rhs); self.b.$op_assign(rhs); }
        }
    };
}

impl_op!(Spectrum, Add, add);
impl_op!(Spectrum, Sub, sub);
impl_op!(Spectrum, Mul, mul);
impl_op!(Spectrum, Div, div);

impl_op_float!(Spectrum, Mul, mul);
impl_op_float!(Spectrum, Div, div);

impl_op_assign!(Spectrum, AddAssign, add_assign);
impl_op_assign!(Spectrum, SubAssign, sub_assign);
impl_op_assign!(Spectrum, MulAssign, mul_assign);
impl_op_assign!(Spectrum, DivAssign, div_assign);

impl_op_float_assign!(Spectrum, MulAssign, mul_assign);
impl_op_float_assign!(Spectrum, DivAssign, div_assign);



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ops() {
        let a = Spectrum::new(1., 1., 1.);
        let b = Spectrum::new(1., 2., 3.);
        let ans = Spectrum::new(2., 3., 4.);
        let c = a.clone() + b.clone();
        assert_eq!(c, ans);
        let c = &a + b.clone();
        assert_eq!(c, ans);
        let c = a.clone() + &b;
        assert_eq!(c, ans);
        let c = &a + &b;
        assert_eq!(c, ans);

        let ans = Spectrum::new(0., -1., -2.);
        let c = a.clone() - b.clone();
        assert_eq!(c, ans);

        let ans = Spectrum::new(1., 2., 3.);
        let c = &a * &b;
        assert_eq!(c, ans);
        let c = &a * b.clone();
        assert_eq!(c, ans);

        let ans = Spectrum::new(1., 0.5, 1. / 3.);
        let c = a / &b;
        assert_eq!(c, ans);

        let ans = Spectrum::new(2.0, 4.0, 6.0);
        assert_eq!(2. * &b, ans);
        assert_eq!(&b * 2., ans);
        let ans = Spectrum::new(6., 3., 2.);
        assert_eq!(6. / &b, ans);
        assert_eq!(b / 2., Spectrum::new(0.5, 1., 1.5));
    }

    #[test]
    fn ops_assign() {
        let mut a = Spectrum::new(1., 1., 1.);
        let b = Spectrum::new(1., 2., 3.);
        a += &b;
        assert_eq!(a, Spectrum::new(2., 3., 4.));
        a -= &b;
        assert_eq!(a, Spectrum::new(1., 1., 1.));
        a *= &b;
        assert_eq!(a, Spectrum::new(1., 2., 3.));
        a /= b;
        assert_eq!(a, Spectrum::new(1., 1., 1.));

        a *= 2.;
        assert_eq!(a, Spectrum::new(2., 2., 2.));
        a /= 2.;
        assert_eq!(a, Spectrum::new(1., 1., 1.));
    }
}

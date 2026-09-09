use crate::complex::complex::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// (a+bi) + (c+di)
impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

/// (a+bi) + x
impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

/// x + (a+bi)
impl Add<Complex> for f64 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            re: rhs.re + self,
            im: rhs.im,
        }
    }
}

/// (a+bi) - (c+di)
impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

/// (a+bi) - x
impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

/// x - (a+bi)
impl Sub<Complex> for f64 {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex {
        -rhs + self
    }
}

/// (a+bi) * (c+di)
impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

/// (a+bi) * x
impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

/// x * (a+bi)
impl Mul<Complex> for f64 {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        rhs * self
    }
}

/// (a+bi) / (c+di)
impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / rhs.magnitude_squared(),
            im: (self.im * rhs.re - self.re * rhs.im) / rhs.magnitude_squared(),
        }
    }
}

/// (a+bi) / x
impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

/// x / (a+bi)
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Complex> for f64 {
    type Output = Complex;

    fn div(self, rhs: Complex) -> Complex {
        rhs.inverse() * self
    }
}

/// -(a+bi)
impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1_f64
    }
}

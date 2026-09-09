use std::{
    f64::consts::PI,
    fmt,
    ops::{Div, Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Returns a new complex number x + iy based on cartesian coordinates (x, y)
    pub fn new_from_cartesian(real: f64, imaginary: f64) -> Self {
        Self {
            re: real,
            im: imaginary,
        }
    }

    /// Returns a new complex number of magnitude r and argument θ from polar coordinates (r, θ)
    pub fn new_from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// The complex constant 0 + 0i
    const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
    pub const ZERO: Complex = Self::zero();

    /// The complex constant 1 + 0i
    const fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }
    pub const ONE: Complex = Self::one();

    /// The complex constant i = √-1
    const fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }
    pub const I: Complex = Self::i();

    /// The magnitude of a complex number
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// The squared magnitude of a complex number
    pub fn magnitude_squared(self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// The argument of a complex number
    pub fn argument(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// The inverse of a complex number z⁻¹
    pub fn inverse(self) -> Self {
        let magnitude_squared: f64 = self.magnitude_squared();
        Self {
            re: self.re / magnitude_squared,
            im: -self.im / magnitude_squared,
        }
    }

    /// The complex conjugate of a complex number
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// The square root of a complex number
    pub fn sqrt(self) -> Self {
        let x: f64 = (self.magnitude() + self.re).div(2.0).sqrt();
        let y: f64 = (self.magnitude() - self.re).div(2.0).sqrt();
        Self { re: x, im: y }
    }

    /// Returns the complex number raised to a real exponent
    pub fn powf(self, exponent: f64) -> Self {
        let z: Complex = Complex::new_from_polar(self.magnitude(), self.argument() * exponent);
        z * self.magnitude().powf(exponent)
    }

    /// Returns the complex number raised to a complex exponent
    pub fn powc(self, exponent: Self) -> Self {
        self.ln().mul(exponent).exp()
    }

    /// Returns the exponential of a complex number
    pub fn exp(self) -> Self {
        Self {
            re: self.re.exp() * self.im.cos(),
            im: self.re.exp() * self.im.sin(),
        }
    }

    /// Returns the natural logarithm of a complex number
    pub fn ln(self) -> Self {
        Self {
            re: f64::ln(self.magnitude()),
            im: self.argument(),
        }
    }

    /// Returns the base-n logarithm of a complex number
    pub fn log(self, base: f64) -> Self {
        self.ln() / base.ln()
    }

    /// Returns the base-10 logarithm of a complex number
    pub fn log10(self) -> Self {
        self.log(10.0)
    }

    pub fn sin(self) -> Self {
        Self {
            re: self.re.sin() * self.im.cosh(),
            im: self.re.cos() * self.im.sinh(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            re: self.re.cos() * self.im.cosh(),
            im: self.re.sin().neg() * self.im.sinh(),
        }
    }

    pub fn tan(self) -> Self {
        self.sin() / self.cos()
    }

    pub fn sec(self) -> Self {
        self.cos().inverse()
    }

    pub fn csc(self) -> Self {
        self.sin().inverse()
    }

    pub fn cot(self) -> Self {
        self.tan().inverse()
    }

    pub fn asin(self) -> Self {
        let i = Complex::new_from_cartesian(0.0, 1.0);
        -i * (i * self + (self.powf(2.0).neg() + 1.0).sqrt()).ln()
    }

    pub fn acos(self) -> Self {
        self.asin().neg() + PI / 2.0
    }

    pub fn atan(self) -> Self {
        let i = Complex::new_from_cartesian(0.0, 1.0);
        (self.conjugate() / self).ln() * i / 2.0
    }

    pub fn asec(self) -> Self {
        self.inverse().acos()
    }

    pub fn acsc(self) -> Self {
        self.inverse().asin()
    }

    pub fn acot(self) -> Self {
        self.inverse().atan()
    }

    pub fn sinh(self) -> Self {
        Self {
            re: self.re.sinh() * self.im.cos(),
            im: self.re.cosh() * self.im.sin(),
        }
    }

    pub fn cosh(self) -> Self {
        Self {
            re: self.re.cosh() * self.im.cos(),
            im: self.re.sinh() * self.im.sin(),
        }
    }

    pub fn tanh(self) -> Self {
        self.sinh() / self.cosh()
    }

    pub fn sech(self) -> Self {
        self.cosh().inverse()
    }

    pub fn csch(self) -> Self {
        self.sinh().inverse()
    }

    pub fn coth(self) -> Self {
        self.tanh().inverse()
    }

    pub fn asinh(self) -> Self {
        (self + (self.powf(2.0) + 1.0).sqrt()).ln()
    }

    pub fn acosh(self) -> Self {
        (self - (self.powf(2.0) - 1.0).sqrt()).ln()
    }

    pub fn atanh(self) -> Self {
        ((self + 1.0) / (-self + 1.0)).ln() / 2.0
    }

    pub fn asech(self) -> Self {
        self.inverse().acosh()
    }

    pub fn acsch(self) -> Self {
        self.inverse().asinh()
    }

    pub fn acoth(self) -> Self {
        self.inverse().atanh()
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)?;
        Ok(())
    }
}

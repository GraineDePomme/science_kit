use crate::polynomial::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// polynomial + polynomial
impl Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    + rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial + f64
impl Add<f64> for Polynomial {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let mut result_coefficients = self.coefficients.clone();
        result_coefficients[0] += rhs;
        Self {
            coefficients: result_coefficients,
        }
    }
}

/// f64 + polynomial
impl Add<Polynomial> for f64 {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        rhs + self
    }
}

/// polynomial - polynomial
impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let n = self.coefficients.len().max(rhs.coefficients.len());

        let result_coefficients = (0..n)
            .map(|i| {
                self.coefficients.get(i).copied().unwrap_or(0.0)
                    - rhs.coefficients.get(i).copied().unwrap_or(0.0)
            })
            .collect();

        Self {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial - f64
impl Sub<f64> for Polynomial {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        let mut result_coefficients = self.coefficients.clone();
        result_coefficients[0] -= rhs;
        Self {
            coefficients: result_coefficients,
        }
    }
}

/// f64 - polynomial
impl Sub<Polynomial> for f64 {
    type Output = Polynomial;

    fn sub(self, rhs: Polynomial) -> Polynomial {
        -rhs + self
    }
}

/// polynomial * polynomial
impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result_coefficients =
            vec![0.0; self.coefficients.len() + rhs.coefficients.len() - 1];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, rhs_coeff) in rhs.coefficients.iter().enumerate() {
                result_coefficients[i + j] += self_coeff * rhs_coeff;
            }
        }

        Self {
            coefficients: result_coefficients,
        }
    }
}

/// polynomial * f64
impl Mul<f64> for Polynomial {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| x * rhs).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

/// f64 * polynomial
impl Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Polynomial {
        rhs * self
    }
}

/// polynomial / f64
impl Div<f64> for Polynomial {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| x / rhs).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

/// -polynomial
impl Neg for Polynomial {
    type Output = Self;

    fn neg(self) -> Self {
        let result_coefficients = self.coefficients.iter().map(|x| -x).collect();
        Self {
            coefficients: result_coefficients,
        }
    }
}

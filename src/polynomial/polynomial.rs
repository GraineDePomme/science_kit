use crate::complex::Complex;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<f64>,
}

impl Polynomial {
    /// Initialization of a polynomial using its coefficients.
    /// The coefficients are expected to be formatted with the lowest power of x at the beginning.
    pub fn new(coefficients: Vec<f64>) -> Self {
        let mut clean_coefficients: Vec<f64> = coefficients;

        while clean_coefficients.last() == Some(&0.0) {
            if clean_coefficients == [0.0] {
                break;
            }
            clean_coefficients.pop();
        }

        Self {
            coefficients: clean_coefficients,
        }
    }

    /// Evaluates the polynomial for a given x ∈ ℝ using Horner's method for stability
    pub fn evaluate(&self, at: f64) -> f64 {
        let x = at;
        self.coefficients
            .iter()
            .rfold(0.0, |acc, &coeff| acc * x + coeff)
    }

    /// Evaluates the polynomial for a given x ∈ ℂ
    pub fn evaluate_complex(self, at: Complex) -> Complex {
        let x = at;
        self.coefficients
            .iter()
            .rfold(Complex::ZERO, |acc, &coeff| {
                acc * x + Complex::new_from_cartesian(coeff, 0.0)
            })
    }

    pub fn degree(&self) -> Option<usize> {
        if self.coefficients.is_empty() {
            None
        } else {
            Some(self.coefficients.len() - 1)
        }
    }

    /// Transform the polynomial into its derivative
    pub fn differentiate(&mut self) {
        let n = self.coefficients.len();

        if n <= 1 {
            self.coefficients.clear();
            self.coefficients.push(0.0);
            return;
        }

        for i in 1..n {
            self.coefficients[i - 1] = self.coefficients[i] * i as f64;
        }

        self.coefficients.pop();
    }

    /// Returns the derivative of the polynomial without changing the original
    pub fn derivative(self) -> Self {
        let mut result = self;
        result.differentiate();
        result
    }

    /// Transforms the polynomial into its definite integral with a given constant
    pub fn integrate(&mut self, constant: Option<f64>) {
        let c: f64 = constant.unwrap_or(0.0);
        let n = self.coefficients.len();

        // add space for new highest power term
        self.coefficients.resize(n + 1, 0.0);

        // shift and divide backwards to avoid overwriting
        for i in (0..n).rev() {
            self.coefficients[i + 1] = self.coefficients[i] / (i as f64 + 1.0);
        }

        // set integration constant
        self.coefficients[0] = c;
    }

    /// Returns the definite integral of the polynomial with a given constant
    pub fn integral(self, constant: Option<f64>) -> Self {
        let mut result: Polynomial = self;
        result.integrate(constant);
        result
    }

    /// Returns the real roots of the polynomial
    pub fn real_roots(&self) -> Vec<f64> {
        let degree: usize = self.degree().unwrap_or(0);

        match degree {
            0 => vec![],

            // a x + b = 0
            1 => {
                let b: f64 = self.coefficients[0];
                let a: f64 = self.coefficients[1];

                if a == 0.0 { vec![] } else { vec![-b / a] }
            }

            // a x² + b x + c = 0
            2 => {
                let c: f64 = self.coefficients[0];
                let b: f64 = self.coefficients[1];
                let a: f64 = self.coefficients[2];

                if a == 0.0 {
                    return Polynomial::new(vec![c, b]).real_roots();
                }

                let disc: f64 = b * b - 4.0 * a * c;

                if disc > 0.0 {
                    let s: f64 = disc.sqrt();

                    // Numerically stable quadratic formula
                    let q: f64 = -0.5 * (b + b.signum() * s);
                    let r1: f64 = q / a;
                    let r2: f64 = c / q;

                    let mut roots: Vec<f64> = vec![r1, r2];
                    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    roots
                } else if disc.abs() <= 0.0 {
                    vec![-b / (2.0 * a)]
                } else {
                    vec![]
                }
            }

            // a x³ + b x² + c x + d = 0
            3 => {
                let d: f64 = self.coefficients[0];
                let c: f64 = self.coefficients[1];
                let b: f64 = self.coefficients[2];
                let a: f64 = self.coefficients[3];

                if a.abs() < 0.0 {
                    return Polynomial::new(vec![d, c, b]).real_roots();
                }

                // Normalize
                let a1: f64 = b / a;
                let a2: f64 = c / a;
                let a3: f64 = d / a;

                // Depressed cubic:
                // t³ + p t + q = 0
                let p: f64 = a2 - a1 * a1 / 3.0;
                let q: f64 = 2.0 * a1.powi(3) / 27.0 - a1 * a2 / 3.0 + a3;

                let disc: f64 = (q / 2.0).powi(2) + (p / 3.0).powi(3);

                if disc > 0.0 {
                    // One real root
                    let sqrt_disc: f64 = disc.sqrt();

                    let u: f64 = (-q / 2.0 + sqrt_disc).cbrt();
                    let v: f64 = (-q / 2.0 - sqrt_disc).cbrt();

                    vec![u + v - a1 / 3.0]
                } else if disc.abs() <= 0.0 {
                    // Multiple roots
                    let u: f64 = (-q / 2.0).cbrt();

                    let r1: f64 = 2.0 * u - a1 / 3.0;
                    let r2: f64 = -u - a1 / 3.0;

                    if (r1 - r2).abs() < 0.0 {
                        vec![r1]
                    } else {
                        let mut roots: Vec<f64> = vec![r1, r2];
                        roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                        roots
                    }
                } else {
                    // Three distinct real roots
                    let r: f64 = (-p / 3.0).sqrt();
                    let phi: f64 = (-q / (2.0 * r.powi(3))).acos();

                    let x1: f64 = 2.0 * r * (phi / 3.0).cos() - a1 / 3.0;

                    let x2: f64 =
                        2.0 * r * ((phi + 2.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;

                    let x3: f64 =
                        2.0 * r * ((phi + 4.0 * std::f64::consts::PI) / 3.0).cos() - a1 / 3.0;

                    let mut roots: Vec<f64> = vec![x1, x2, x3];
                    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    roots
                }
            }

            _ => panic!("degree > 3 not supported"),
        }
    }
}

// A private helper function to convert a unmber in superscript
fn superscript(n: usize) -> String {
    let mut number = n;

    const DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

    if number == 0 {
        return "⁰".to_string();
    }

    let mut result = Vec::new();

    while number > 0 {
        result.push(DIGITS[number % 10]);
        number /= 10;
    }

    result.iter().rev().collect()
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for (power, &c) in self.coefficients.iter().enumerate().rev() {
            if c == 0.0 {
                continue;
            }

            if first {
                if c < 0.0 {
                    write!(f, "-")?;
                }
                first = false;
            } else {
                write!(f, " {} ", if c < 0.0 { "-" } else { "+" })?;
            }

            let abs_c = c.abs();

            match power {
                0 => write!(f, "{abs_c}")?,
                1 => {
                    if abs_c != 1.0 {
                        write!(f, "{abs_c}x")?;
                    } else {
                        write!(f, "x")?;
                    }
                }
                _ => {
                    if abs_c != 1.0 {
                        write!(f, "{abs_c}x{}", superscript(power))?;
                    } else {
                        write!(f, "x{}", superscript(power))?;
                    }
                }
            }
        }

        if first {
            write!(f, "0")?;
        }

        Ok(())
    }
}

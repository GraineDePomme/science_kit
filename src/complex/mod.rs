pub mod complex;
pub mod operators;

pub use complex::*;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_construction_cartesian() {
        let a: Complex = Complex::new_from_cartesian(3.2, 1.7);
        assert!(a.re == 3.2 && a.im == 1.7, "Wrong initialization of a complex number using cartesian coordinates");
    }

    #[test]
    fn number_construction_polar() {
        let a: Complex = Complex::new_from_polar(1.0, std::f64::consts::FRAC_PI_2);
        assert!((a.re - 0.0).abs() <= 1.0e-15 && a.im == 1.0, "Wrong initialization of a complex number using polar coordinates");
    }

    #[test]
    fn addition_1() {
        let a: Complex = Complex::new_from_cartesian(1.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a + b == Complex { re: 4.0, im: 6.0}, "Wrong complex addition");
    }

    #[test]
    fn addition_2() {
        let a: Complex = Complex::new_from_cartesian(5.0, -3.0);
        let b: Complex = Complex::new_from_cartesian(-2.0, 7.0);
        assert!(a + b == Complex { re: 3.0, im: 4.0}, "Wrong complex addition");
    }

    #[test]
    fn plus_zero() {
        let a: Complex = Complex::new_from_cartesian(5.0, -3.0);
        assert!(a + 0.0 == a, "Complex + 0 gives wrong result!");
    }

    #[test]
    fn addition() {
        let a: Complex = Complex::new_from_cartesian(5.0, 4.0);
        let b: Complex = Complex::new_from_cartesian(2.0, 1.0);
        assert!(a - b == Complex { re: 3.0, im: 3.0 }, "Complex substraction gives wrong result!");
    }

    #[test]
    fn minus_itself() {
        let a: Complex = Complex::new_from_cartesian(5.0, -3.0);
        assert!(a - a == Complex { re: 0.0, im: 0.0 }, "Complex minus itself doesn't gives 0!");
    }

    #[test]
    fn simple_product() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        assert!(a * b == Complex { re: -5.0, im: 14.0}, "Wrong complex multiplication");
    }

    #[test]
    fn mulitplication_by_0() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        assert!(a * 0.0 == Complex { re: 0.0, im: 0.0}, "Complex multiplication by 0 doesn't give the right result!");
    }

    #[test]
    fn mulitplication_by_1() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        assert!(a * 1.0 == a, "Complex multiplication by 1 doesn't give the right result!");
    }

    #[test]
    fn simple_division() {
        let a: Complex = Complex::new_from_cartesian(1.0, 1.0);
        let b: Complex = Complex::new_from_cartesian(1.0, -1.0);
        assert!(a / b == Complex { re: 0.0, im: 1.0}, "Wrong complex division");
    }

    #[test]
    fn self_division() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        assert!(a / a == Complex { re: 1.0, im: 0.0 }, "Complex self division doesn't give the right result!");
    }

    #[test]
    fn pure_imaginary_division() {
        let a: Complex = Complex::new_from_cartesian(0.0, 1.0);
        assert!(a / a == Complex { re: 1.0, im: 0.0 }, "Complex pure imaginary division doesn't give the right result!");
    }

    #[test]
    fn conjugate() {
        let a: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a.conjugate() == Complex { re: 3.0, im: -4.0 }, "Complex conjugate gives wrong result!");
    }

    #[test]
    fn double_conjugate() {
        let a: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a.conjugate().conjugate() == a, "Complex double conjugate gives wrong result!");
    }

    #[test]
    fn simple_norm() {
        let a: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a.magnitude() == 5.0, "Complex magnitude gives wrong result!");
    }

    #[test]
    fn imaginary_norm() {
        let a: Complex = Complex::new_from_cartesian(0.0, 5.0);
        assert!(a.magnitude() == 5.0, "Complex pure imaginary magnitude gives wrong result!");
    }

    #[test]
    fn imaginary_zero() {
        let a: Complex = Complex::new_from_cartesian(0.0, 0.0);
        assert!(a.magnitude() == 0.0, "Complex zero magnitude gives wrong result!");
    }

    #[test]
    fn squared_magnitude() {
        let a: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a.magnitude_squared() == 25.0, "Complex squared magnitude gives wrong result!");
    }

    #[test]
    fn squared_magnitude_formula() {
        let a: Complex = Complex::new_from_cartesian(3.0, 4.0);
        assert!(a.magnitude_squared() * Complex::ONE == a * a.conjugate(), "Complex squared magnitude is not verified!");
    }

    #[test]
    fn argument_positive_real() {
        let a: Complex = Complex::new_from_cartesian(1.0, 0.0);
        assert!(a.argument() == 0.0, "Complex argument for positive real value gives wrong result!");
    }

    #[test]
    fn argument_positive_imaginary() {
        let a: Complex = Complex::new_from_cartesian(0.0, 1.0);
        assert!(a.argument() == std::f64::consts::FRAC_PI_2, "Complex argument for positive imaginary value gives wrong result!");
    }

    #[test]
    fn argument_negative_real() {
        let a: Complex = Complex::new_from_cartesian(-1.0, 0.0);
        assert!(a.argument() == std::f64::consts::PI, "Complex argument for negative real value gives wrong result!");
    }

    #[test]
    fn argument_negative_imaginary() {
        let a: Complex = Complex::new_from_cartesian(0.0, -1.0);
        assert!(a.argument() == -std::f64::consts::FRAC_PI_2, "Complex argument for negative imaginary value gives wrong result!");
    }

    #[test]
    fn exponential() {
        let a: Complex = Complex::I * std::f64::consts::PI;
        assert!((a.exp() + 1.0).re.abs() == 0.0, "Complex exponential gives wrong real part!");
        assert!((a.exp() + 1.0).im.abs() <= 1.0e-15, "Complex exponential gives wrong imaginary part!");
    }

    #[test]
    fn square_root() {
        let a: Complex = Complex::new_from_cartesian(-1.0, 0.0);
        assert!(a.sqrt() == Complex { re: 0.0, im: 1.0 }, "Complex square root gives wrong result!");
    }

    #[test]
    fn square_root_squared() {
        let a: Complex = Complex::new_from_cartesian(-1.0, 0.0);
        assert!(a.sqrt() * a.sqrt() == a, "Complex square root gives wrong result!");
    }

    #[test]
    fn sine() {
        let a: Complex = Complex::new_from_cartesian(0.0, 0.0);
        assert!(a.sin() == Complex { re: 0.0, im: 0.0 }, "Complex sine gives wrong result!");
    }

    #[test]
    fn cosine() {
        let a: Complex = Complex::new_from_cartesian(0.0, 0.0);
        assert!(a.cos() == Complex { re: 1.0, im: 0.0 }, "Complex cosine gives wrong result!");
    }

    #[test]
    fn trigonometric_identity() {
        let a: Complex = Complex::new_from_cartesian(0.0, 0.0);
        let result = a.sin() * a.sin() + a.cos() * a.cos();
        assert!(result == Complex { re: 1.0, im: 0.0 }, "Complex trig identity gives wrong result!");
    }

    #[test]
    fn addition_commutativity() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        assert!(a + b == b + a, "Complex addition is not commutative!");
    }

    #[test]
    fn multiplication_commutativity() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        assert!(a * b == b * a, "Complex multiplication is not commutative!");
    }

    #[test]
    fn addition_associativity() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        let c: Complex = Complex::new_from_cartesian(-1.0, 6.1);
        assert!( (a + b) + c == a + (b + c), "Complex addition is not associative!");
    }

    #[test]
    fn multiplication_associativity() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        let c: Complex = Complex::new_from_cartesian(-1.0, 6.1);
        assert!( (a * b) * c == a * (b * c), "Complex multiplication is not associative!");
    }

    #[test]
    fn distributivity() {
        let a: Complex = Complex::new_from_cartesian(3.0, 2.0);
        let b: Complex = Complex::new_from_cartesian(1.0, 4.0);
        let c: Complex = Complex::new_from_cartesian(-1.0, 6.1);
        assert!(a * (b + c) == a * b + a * c, "Complex multiplication is not distributive!");
    }

    #[test]
    fn conjugate_small_values_stability() {
        let a: Complex = Complex::new_from_cartesian(1.0e-300, 1.0e-300);
        assert!(a.conjugate() == Complex { re: 1.0e-300, im: -1.0e-300 }, "Unstable complex conjugate for small values!");
    }

    #[test]
    fn addition_small_values_stability() {
        let a: Complex = Complex::new_from_cartesian(2.0e-300, 1.0e-300);
        let b: Complex = Complex::new_from_cartesian(1.0e-300, 2.0e-300);
        assert!(a + b == Complex::new_from_cartesian(3.0e-300, 3.0e-300), "Unstable complex addition for small values!");
    }

    #[test]
    fn substraction_small_values_stability() {
        let a: Complex = Complex::new_from_cartesian(3.0e-300, 3.0e-300);
        let b: Complex = Complex::new_from_cartesian(1.0e-300, 2.0e-300);
        assert!((a - b).re.abs() <= std::f64::EPSILON, "Unstable complex substraction for small values!");
        assert!((a - b).im.abs() <= std::f64::EPSILON, "Unstable complex substraction for small values!");
    }

    #[test]
    fn conjugate_large_values_stability() {
        let a: Complex = Complex::new_from_cartesian(1.0e300, 1.0e300);
        assert!(a.conjugate() == Complex { re: 1.0e300, im: -1.0e300 }, "Unstable complex conjugate for large values!");
    }

    #[test]
    fn addition_large_values_stability() {
        let a: Complex = Complex::new_from_cartesian(2.0e300, 1.0e300);
        let b: Complex = Complex::new_from_cartesian(1.0e300, 2.0e300);
        println!("{}", a+b);
        assert!(a + b == Complex::new_from_cartesian(3.0e300, 3.0e300), "Unstable complex addition for large values!");
    }

    #[test]
    fn substraction_large_values_stability() {
        let a: Complex = Complex::new_from_cartesian(3.0e300, 3.0e300);
        let b: Complex = Complex::new_from_cartesian(1.0e300, 2.0e300);
        println!("{}", a-b);
        assert!(a - b == Complex::new_from_cartesian(2.0e300, 1.0e300), "Unstable complex substraction for large values!");
    }
}

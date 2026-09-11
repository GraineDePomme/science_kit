pub mod romberg;
pub use romberg::*;

#[cfg(test)]
mod romberg_method {

    use crate::integral;
    use crate::measure::*;

    #[test]
    fn simple_integral_1() {
        let f = |x: f64| -> f64 { x.powi(4) - 2.0 * x + 1.0 };
        let result: Measure = integral::adaptive::romberg(f, 0.0, 2.0, 10, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 4.4).abs() <= 0.0001 && (result.value - 4.4).abs() <= result.error,
            "Adaptive Romberg integral gives wrong result!"
        );
    }

    #[test]
    fn simple_integral_2() {
        let f = |x: f64| -> f64 { (100.0 * x).sqrt().sin().powi(2) };
        let result: Measure = integral::adaptive::romberg(f, 0.0, 1.0, 10, 0.0001, 0.0);
        println!("{result}");
        assert!(
            (result.value - 0.45583).abs() <= 0.0001
                && (result.value - 0.45583).abs() <= result.error,
            "Adaptive Romberg integral gives wrong result!"
        );
    }
}

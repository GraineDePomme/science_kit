#[allow(clippy::module_inception)]
pub mod polynomial;
pub use polynomial::*;

pub mod operators;

#[cfg(test)]
mod construction {
    use crate::polynomial::*;

    #[test]
    fn empty_polynomial() {
        let a: Polynomial = Polynomial::new(vec![]);
        assert!(a.degree() == None, "Wrong degree for empty polynomial");
    }

    #[test]
    fn constant_polynomial() {
        let a: Polynomial = Polynomial::new(vec![5.0]);
        assert!(
            a.degree() == Some(0),
            "Wrong degree for constant polynomial"
        );
        assert!(
            a.evaluate(0.0) == 5.0,
            "Wrong evaluate of constant polynomial!"
        );
    }

    #[test]
    fn removing_leading_zeros() {
        let a: Polynomial = Polynomial::new(vec![1.0, 2.0, 0.0, 0.0]);
        assert!(
            a.coefficients == vec![1.0, 2.0],
            "Leading zeros are not properly removed during initialization!"
        );
    }

    #[test]
    fn only_zeros_coefficients() {
        let a: Polynomial = Polynomial::new(vec![0.0, 0.0, 0.0, 0.0]);
        assert!(
            a.coefficients == vec![0.0],
            "Only zero coefficients during initialization should return [0]!"
        );
    }
}

#[cfg(test)]
mod evaluation {
    use crate::polynomial::*;

    #[test]
    fn simple_evaluation() {
        let a: Polynomial = Polynomial::new(vec![1.0, 2.0, 3.0]);
        assert!(a.evaluate(0.0) == 1.0, "Wrong evaluation!");
        assert!(a.evaluate(1.0) == 6.0, "Wrong evaluation!");
        assert!(a.evaluate(2.0) == 17.0, "Wrong evaluation!");
    }
}

#[cfg(test)]
mod addition {
    use crate::polynomial::*;

    #[test]
    fn same_degree() {
        let a: Polynomial = Polynomial::new(vec![1.0, 2.0]);
        let b: Polynomial = Polynomial::new(vec![3.0, 4.0]);
        assert!(
            (a + b).coefficients == vec![4.0, 6.0],
            "Addition of polynomials gives wrong result!"
        );
    }

    #[test]
    fn different_degree() {
        let a: Polynomial = Polynomial::new(vec![1.0, 1.0, 1.0]);
        let b: Polynomial = Polynomial::new(vec![3.0]);
        assert!(
            (a + b).coefficients == vec![4.0, 1.0, 1.0],
            "Addition of polynomials gives wrong result!"
        );
    }

    #[test]
    fn add_zero() {
        let a: Polynomial = Polynomial::new(vec![1.0, 1.0, 1.0]);
        assert!(
            (a + 0.0).coefficients == vec![1.0, 1.0, 1.0],
            "Addition of polynomials gives wrong result!"
        );
    }
}

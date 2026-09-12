pub mod vector;
pub use vector::*;

pub mod matrix;
pub use matrix::*;

pub mod matrix_vector_operations;
//pub use matrix_vector_operations::*;

pub mod lu_decomposition;
pub use lu_decomposition::*;

pub mod solve;
pub use solve::*;




#[cfg(test)]
mod tests {
    use crate::linear_algebra::{Matrix, Vector, solve};

    #[test]
    fn simple_linear_solver_test() {
        let a: Matrix = Matrix::new(vec![2.0, 1.0, 4.0, 1.0,
                                                    3.0, 4.0, -1.0, -1.0,
                                                    1.0, -4.0, 1.0, 5.0,
                                                    2.0, -2.0, 1.0, 3.0], 4, 4);

        let _lu = a.lu_decomposition().unwrap();

        let v: Vector = Vector::new(vec![-4.0, 3.0, 9.0, 7.0]);

        let result: Vector = solve(&a, &v).expect("Failed to solve the linear system");

        println!("{}", a);

        println!("{}", result);

        println!("{}", a * result);
    }
}
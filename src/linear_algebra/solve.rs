use crate::linear_algebra::{Matrix, Vector};

/// This function solves the system of linear equations Ax = b using LU decomposition
pub fn solve(a: &Matrix, v: &Vector) -> Result<Vector, String> {
    let lu = a.lu_decomposition()?;

    let mut v_with_permutation = v.clone();
    v_with_permutation.apply_permutation(lu.p.clone());

    // Forward substitution: Ly = Pb
    let mut y = Vector::new_from_value(0.0, a.rows);

    for row in 0..lu.l.rows {
        let mut sum = 0.0;

        for j in 0..row {
            sum += lu.l.get(row, j) * y.get(j);
        }

        y.set(
            row,
            (v_with_permutation.get(row) - sum) / lu.l.get(row, row),
        );
    }

    // Backward substitution: Ux = y
    let mut result = Vector::new_from_value(0.0, a.rows);

    for row in (0..lu.u.rows).rev() {
        let mut sum = 0.0;

        for j in (row + 1)..lu.u.cols {
            sum += lu.u.get(row, j) * result.get(j);
        }

        result.set(row, (y.get(row) - sum) / lu.u.get(row, row));
    }

    Ok(result)
}

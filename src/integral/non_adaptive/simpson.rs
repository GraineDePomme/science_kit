pub fn simpson<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    if a == b {
        return 0.0;
    };

    // Simpson integration requires an even number of slices so we add one if necessary
    let n: usize = if n.is_multiple_of(2) { n } else { n + 1 };

    let lower_bound: f64 = if a < b { a } else { b };
    let upper_bound: f64 = if a < b { b } else { a };

    let h: f64 = (upper_bound - lower_bound) / (n as f64);

    let mut result: f64 = f(lower_bound) + f(upper_bound);

    let mut temp: f64 = 0.0;
    for k in (1..n).step_by(2) {
        temp += f(lower_bound + (k as f64) * h);
    }
    result += 4.0 * temp;

    temp = 0.0;
    for k in (2..n - 1).step_by(2) {
        temp += f(lower_bound + (k as f64) * h);
    }
    result += 2.0 * temp;

    result * h / 3.0
}

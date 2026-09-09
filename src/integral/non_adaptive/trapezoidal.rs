pub fn trapezoidal<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    if a == b {
        return 0.0;
    };

    let lower_bound: f64 = if a < b { a } else { b };
    let upper_bound: f64 = if a < b { b } else { a };

    let h: f64 = (upper_bound - lower_bound) / (n as f64);

    let mut result: f64 = (f(lower_bound) + f(upper_bound)) / 2.0;

    for k in 1..n {
        result += f(lower_bound + (k as f64) * h);
    }

    result * h
}

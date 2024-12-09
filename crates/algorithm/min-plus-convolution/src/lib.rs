use monotone_minima::monotone_minima;

/// b[i+1] - b[i] <= b[i+2] - b[i+1]
pub fn min_plus_convolution_arbitrary_convex<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let cmp = |i: usize, j: usize, k: usize| {
        if i < k {
            false
        } else if i >= m + j {
            true
        } else {
            a[j] + b[i - j] >= a[k] + b[i - k]
        }
    };
    let c = monotone_minima(n + m - 1, n, cmp);
    (0..n + m - 1).map(|i| a[c[i]] + b[i - c[i]]).collect()
}

/// a[i+1] - a[i] <= a[i+2] - a[i+1]
pub fn min_plus_convolution_convex_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,
{
    min_plus_convolution_arbitrary_convex(b, a)
}

/// a[i+1] - a[i] <= a[i+2] - a[i+1]
/// b[i+1] - b[i] <= b[i+2] - b[i+1]
pub fn min_plus_convolution_convex_convex<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let mut c = vec![a[0] + b[0]; n + m - 1];
    let mut i = 0;
    let mut j = 0;
    for k in 1..n + m - 1 {
        if j == m - 1 || (i != n - 1 && a[i + 1] + b[j] < a[i] + b[j + 1]) {
            i += 1;
        } else {
            j += 1;
        }
        c[k] = a[i] + b[j];
    }
    c
}

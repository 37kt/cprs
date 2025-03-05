use numeric_traits::Numeric;

pub fn convolution_naive<T: Numeric>(f: &[T], g: &[T]) -> Vec<T> {
    let n = f.len();
    let m = g.len();
    if n > m {
        return convolution_naive(g, f);
    } else if n == 0 {
        return vec![];
    }

    let mut h = vec![T::zero(); n + m - 1];
    for i in 0..n {
        for j in 0..m {
            h[i + j] += f[i] * g[j];
        }
    }
    h
}

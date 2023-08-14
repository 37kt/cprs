use modint::ModInt;

pub fn convolution_naive<T: ModInt>(a: &[T], b: &[T]) -> Vec<T> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let l = n + m - 1;
    let mut c = vec![0.into(); l];
    for i in 0..n {
        for j in 0..m {
            c[i + j] += a[i] * b[j];
        }
    }
    c
}

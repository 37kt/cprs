use numeric_traits::{Integer, Signed};

use crate::{is_concave, is_convex};

pub fn minplus_convolution_convex_and_convex<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    assert!(is_convex(a));
    assert!(is_convex(b));

    let mut c = Vec::with_capacity(n + m - 1);
    c.push(a[0] + b[0]);
    let mut i = 0;
    let mut j = 0;
    for _ in 1..n + m - 1 {
        if j == m - 1 || (i < n - 1 && a[i + 1] + b[j] < a[i] + b[j + 1]) {
            i += 1;
        } else {
            j += 1;
        }
        c.push(a[i] + b[j]);
    }
    c
}

pub fn maxplus_convolution_concave_and_concave<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Signed,
{
    assert!(is_concave(a));
    assert!(is_concave(b));

    let a = a.iter().map(|x| -*x).collect::<Vec<_>>();
    let b = b.iter().map(|x| -*x).collect::<Vec<_>>();
    let mut c = minplus_convolution_convex_and_convex(&a, &b);
    c.iter_mut().for_each(|x| *x = -*x);
    c
}

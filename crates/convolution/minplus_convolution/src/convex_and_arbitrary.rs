use monotone_minima::monotone_minima;
use numeric_traits::{Integer, Signed};

use crate::{is_concave, is_convex};

pub fn minplus_convolution_arbitrary_and_convex<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    assert!(is_convex(b));

    let select = |i: usize, j: usize, k: usize| {
        if i < k {
            false
        } else if i >= m + j {
            true
        } else {
            a[j] + b[i - j] >= a[k] + b[i - k]
        }
    };
    let d = monotone_minima(n + m - 1, n, select);
    d.into_iter()
        .enumerate()
        .map(|(i, j)| a[j] + b[i - j])
        .collect()
}

pub fn minplus_convolution_convex_and_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer,
{
    minplus_convolution_arbitrary_and_convex(b, a)
}

pub fn maxplus_convolution_concave_and_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Signed,
{
    assert!(is_concave(a));
    let a = a.iter().map(|x| -*x).collect::<Vec<_>>();
    let b = b.iter().map(|x| -*x).collect::<Vec<_>>();
    let mut c = minplus_convolution_convex_and_arbitrary(&a, &b);
    c.iter_mut().for_each(|x| *x = -*x);
    c
}

pub fn maxplus_convolution_arbitrary_and_concave<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Signed,
{
    maxplus_convolution_concave_and_arbitrary(b, a)
}

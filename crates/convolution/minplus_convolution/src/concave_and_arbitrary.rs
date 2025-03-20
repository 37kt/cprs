use monotone_minima::monotone_minima;
use numeric_traits::{Inf, Integer, Signed};

use crate::{is_concave, is_convex};

pub fn minplus_convolution_concave_and_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Inf,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    assert!(is_concave(a));

    let mut c = vec![T::MAX; n + m - 1];
    divide_column(a, b, &mut c);
    c
}

pub fn minplus_convolution_arbitrary_and_concave<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Inf,
{
    minplus_convolution_concave_and_arbitrary(b, a)
}

// 長方形領域が取れるようになるまで列を分割
fn divide_column<T>(a: &[T], b: &[T], c: &mut [T])
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return;
    }

    if n + 2 <= m {
        let mm = m / 2;
        divide_column(a, &b[..mm], &mut c[..n + mm - 1]);
        divide_column(a, &b[mm..], &mut c[mm..]);
    } else {
        solve_rectangle(a, b, &mut c[m - 1..m - 1 + n]);
        divide_lower_triangle(&a[..m - 1], &b[..m - 1], &mut c[..m - 1]);
        divide_upper_triangle(&a[n - (m - 1)..], &b[1..], &mut c[n..]);
    }
}

fn solve_rectangle<T>(a: &[T], b: &[T], c: &mut [T])
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return;
    }

    let f = |i: usize, j: usize| a[i + j] + b[m - 1 - j];
    let select = |i: usize, j: usize, k: usize| f(i, j) > f(i, k);
    let argmin = monotone_minima(n - (m - 1), m, select);
    for i in 0..n - (m - 1) {
        c[i] = c[i].min(f(i, argmin[i]));
    }
}

// 上側の下三角行列を処理
fn divide_lower_triangle<T>(a: &[T], b: &[T], c: &mut [T])
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    assert_eq!(n, m);
    assert_eq!(n, c.len());
    if n == 0 {
        return;
    } else if n == 1 {
        c[0] = c[0].min(a[0] + b[0]);
        return;
    }

    let nm = n / 2;
    solve_rectangle(a, &b[..=nm], &mut c[nm..]);
    divide_lower_triangle(&a[..nm], &b[..nm], &mut c[..nm]);
    divide_lower_triangle(&a[..n - nm - 1], &b[nm + 1..], &mut c[nm + 1..]);
}

// 下側の上三角行列を処理
fn divide_upper_triangle<T>(a: &[T], b: &[T], c: &mut [T])
where
    T: Integer,
{
    let n = a.len();
    let m = b.len();
    assert_eq!(n, m);
    assert_eq!(n, c.len());
    if n == 0 || m == 0 {
        return;
    } else if n == 1 {
        c[0] = c[0].min(a[0] + b[0]);
        return;
    }

    let nm = n / 2;
    solve_rectangle(a, &b[nm..], &mut c[..=nm]);
    divide_upper_triangle(&a[n - nm..], &b[..nm], &mut c[..nm]);
    divide_upper_triangle(&a[nm + 1..], &b[nm + 1..], &mut c[nm + 1..])
}

pub fn maxplus_convolution_convex_and_arbitrary<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Signed + Inf,
{
    assert!(is_convex(a));
    let a = a.iter().map(|x| -*x).collect::<Vec<_>>();
    let b = b.iter().map(|x| -*x).collect::<Vec<_>>();
    let mut c = minplus_convolution_concave_and_arbitrary(&a, &b);
    c.iter_mut().for_each(|x| *x = -*x);
    c
}

pub fn maxplus_convolution_arbitrary_and_convex<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Integer + Signed + Inf,
{
    maxplus_convolution_convex_and_arbitrary(b, a)
}

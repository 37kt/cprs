use algebraic::{One, Zero};
use std::ops::{Add, Div, Sub};

/// 半開区間 `[l..r)` において `f(x) = false` となる最小の `x` を探索する。
///
/// # 概要
/// - 二分探索により、条件を満たす最小の値を求める
/// - `f(x)` は単調である必要がある
///
/// # 戻り値
/// - `f(x) = false` となる最小の `x`
/// - そのような `x` が存在しない場合は `r` を返す
pub fn binary_search<I>(mut l: I, mut r: I, mut f: impl FnMut(I) -> bool) -> I
where
    I: Copy + Add<Output = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,
{
    let one = I::one();
    let two = one + one;
    if !f(l) {
        return l;
    }
    while l + one < r {
        let m = (l + r) / two;
        if f(m) {
            l = m;
        } else {
            r = m;
        }
    }
    r
}

/// 区間 `(l..r)` で `f(x) = false` となる最小の `x` を返す。
///
/// # 概要
/// - 二分探索により、条件を満たす最小の値を求める
/// - `f(x)` は単調である必要がある
///
/// # 戻り値
/// - `f(x) = false` となる最小の `x`
/// - そのような `x` が存在しない場合は `r` を返す
pub fn binary_search_f64(mut l: f64, mut r: f64, mut f: impl FnMut(f64) -> bool) -> f64 {
    for _ in 0..100 {
        let m = (l + r) / 2.0;
        if f(m) {
            l = m;
        } else {
            r = m;
        }
    }
    r
}

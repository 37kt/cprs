// TODO: verify

use std::ops::RangeBounds;

use as_half_open_range::AsHalfOpenRange;
use numeric_traits::{Cast, Inf, Integer, NegInf, Signed};

/// Aliens DP  
/// 凸関数 `f` の `f(x)` を求める
///
/// - `g(p) := f(x) + px`  
/// - `p_range` の範囲を探索する
pub fn aliens_dp<T>(x: usize, p_range: impl RangeBounds<T>, mut g: impl FnMut(T) -> T) -> T
where
    T: Integer + Signed + Inf + NegInf,
    usize: Cast<T>,
{
    let one = T::one();

    let x = x.cast();
    let (mut l, mut r) = p_range.as_half_open_range(T::neg_inf(), T::inf());
    l -= one;
    r -= one;
    while l + one < r {
        let p = l + ((r - l) >> 1);
        let c = g(p + one) - g(p);
        if c <= x {
            r = p;
        } else {
            l = p;
        }
    }

    g(r) - r * x
}

/// Aliens DP (凸関数)
///
/// 凸関数 `f` の `f(x)` を求める。
///
/// # 概要
/// - `g(p) := f(x) + px` となる関数 `g` を用いて `f(x)` を計算する
/// - `p_lb ≤ p ≤ p_ub` の範囲で探索を行う
/// - 探索範囲は問題ごとに適切に設定する必要がある
pub fn aliens_dp_convex(x: usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) -> i64) -> i64 {
    let x = x as i64;
    assert!(p_lb < p_ub);
    let mut l = p_lb - 1;
    let mut r = p_ub + 1;
    while l + 1 < r {
        let p = l + (r - l) / 2;
        let c = g(p + 1) - g(p);
        if c <= x {
            r = p;
        } else {
            l = p;
        }
    }
    g(r) - r * x
}

/// Aliens DP (凹関数)
///
/// 凹関数 `f` の `f(x)` を求める。
///
/// # 概要
/// - `g(p) := f(x) - px` となる関数 `g` を用いて `f(x)` を計算する
/// - `p_lb ≤ p ≤ p_ub` の範囲で探索を行う
/// - 探索範囲は問題ごとに適切に設定する必要がある
pub fn aliens_dp_concave(x: usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) -> i64) -> i64 {
    let x = x as i64;
    assert!(p_lb < p_ub);
    let mut l = p_lb - 1;
    let mut r = p_ub + 1;
    while l + 1 < r {
        let p = l + (r - l) / 2;
        let c = g(p) - g(p + 1);
        if c <= x {
            r = p;
        } else {
            l = p;
        }
    }
    g(r) + r * x
}

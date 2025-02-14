use algebraic::{One, Zero};
use std::ops::{Add, Div, Sub};

/// 区間上の凸関数の最小値を三分探索で求める
///
/// # 概要
/// - 区間 `[l..r]` 上で下に凸な関数 `f` の最小値を求める
/// - 関数の最小値を取る位置 `x` と、その値 `f(x)` を返す
///
/// # 引数
/// - `l`: 探索区間の左端
/// - `r`: 探索区間の右端
/// - `f`: 目的関数（下に凸である必要がある）
///
/// # 型パラメータ
/// - `I`: 座標の型（整数型を想定）
/// - `T`: 関数値の型
///
/// # 戻り値
/// - `(x, f(x))`: 最小値を取る位置とその値
///
/// # 制約
/// - `l <= r` であること
/// - `f` は区間 `[l..r]` で下に凸であること
///
/// # 計算量
/// - O(log(r - l))
pub fn ternary_search<I, T>(mut l: I, mut r: I, mut f: impl FnMut(I) -> T) -> (I, T)
where
    I: Copy + Add<Output = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,
    T: Copy + PartialOrd,
{
    assert!(l <= r);
    let one = I::one();
    let two = one + one;
    let three = two + one;
    while l + two < r {
        let m1 = (l + l + r) / three;
        let m2 = (l + r + r) / three;
        if f(m1) < f(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }
    let mut mn = f(l);
    let mut i = l;
    if l + one <= r && f(l + one) < mn {
        mn = f(l + one);
        i = l + one;
    }
    if l + two <= r && f(l + two) < mn {
        mn = f(l + two);
        i = l + two;
    }
    (i, mn)
}

/// 実数区間上の凸関数の最小値を三分探索で求める
///
/// # 概要
/// - 区間 `[l..r]` で下に凸な関数 `f` の最小値を求める
/// - 関数の最小値を取る位置 `x` と、その値 `f(x)` を返す
///
/// # 引数
/// - `l`: 探索区間の左端（実数）
/// - `r`: 探索区間の右端（実数）
/// - `f`: 目的関数（下に凸である必要がある）
///
/// # 型パラメータ
/// - `T`: 関数値の型
///
/// # 戻り値
/// - `(x, f(x))`: 最小値を取る位置とその値
///
/// # 実装詳細
/// - 100回の反復で十分な精度を得られる
/// - 精度は約 2^(-100) ≈ 10^(-30)
pub fn ternary_search_f64<T>(mut l: f64, mut r: f64, mut f: impl FnMut(f64) -> T) -> (f64, T)
where
    T: Copy + PartialOrd,
{
    for _ in 0..100 {
        let m1 = (l * 2.0 + r) / 3.0;
        let m2 = (l + 2.0 * r) / 3.0;
        if f(m1) < f(m2) {
            r = m2;
        } else {
            l = m1;
        }
    }
    (l, f(l))
}

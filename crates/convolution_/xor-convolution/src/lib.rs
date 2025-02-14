use std::ops::{Add, Div, DivAssign, MulAssign, Sub};

/// 高速アダマール変換
///
/// # 概要
/// - 配列 `a` に対し、以下の式で定義される変換を計算する：
/// ```text
/// a[i] = Σ_{j=0}^{n-1} a[j] * (-1)^{popcount(i&j)}
/// ```
/// - 逆変換のあとに n で割る
///
/// # 引数
/// - `a`: 入力配列
/// - `inv`: 逆変換の場合は `true`
pub fn hadamard<T>(a: &mut [T], inv: bool)
where
    T: Clone
        + Default
        + Eq
        + Add<Output = T>
        + Sub<Output = T>
        + From<i64>
        + MulAssign
        + Div<Output = T>
        + DivAssign,
{
    let log = a.len().trailing_zeros();
    assert_eq!(1 << log, a.len());
    for i in 0..log {
        let i = 1 << i;
        for j in (0..a.len()).step_by(i << 1) {
            for k in 0..i {
                let x = a[j + k].clone();
                let y = a[i + j + k].clone();
                a[j + k] = x.clone() + y.clone();
                a[i + j + k] = x - y;
            }
        }
    }
    if inv {
        let inv_n = T::from(1) / T::from(a.len() as i64);
        if inv_n == T::default() {
            let n = T::from(a.len() as i64);
            for i in 0..a.len() {
                a[i] /= n.clone();
            }
        } else {
            for i in 0..a.len() {
                a[i] *= inv_n.clone();
            }
        }
    }
}

/// XOR 畳み込み
///
/// # 概要
/// - 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[k] = Σ_{k=i^j} (a[i] * b[j])
/// ```
/// - 計算量は O(N log N)
pub fn xor_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone
        + Default
        + Eq
        + Add<Output = T>
        + Sub<Output = T>
        + From<i64>
        + MulAssign
        + Div<Output = T>
        + DivAssign,
{
    assert_eq!(a.len(), b.len());
    hadamard(&mut a, false);
    hadamard(&mut b, false);
    for i in 0..a.len() {
        a[i] *= b[i].clone();
    }
    hadamard(&mut a, true);
    a
}

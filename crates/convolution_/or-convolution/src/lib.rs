use std::ops::{Add, Mul, Sub};

/// 高速ゼータ変換
///
/// # 概要
/// - 集合 s に含まれる下位集合 t についての f\[t\] の総和を計算
///
/// # 計算式
/// g\[s\] = Σ_{t ⊆ s} f\[t\]
///
/// # 計算量
/// - O(n 2^n)
///   - n: 集合の要素数
pub fn subset_zeta<T>(a: &mut [T])
where
    T: Clone + Add<Output = T>,
{
    let log = a.len().trailing_zeros();
    assert_eq!(1 << log, a.len());
    for i in 0..log {
        let i = 1 << i;
        for j in (0..a.len()).step_by(i << 1) {
            for k in 0..i {
                a[i + j + k] = a[i + j + k].clone() + a[j + k].clone();
            }
        }
    }
}

/// 高速メビウス変換
///
/// # 概要
/// - 高速ゼータ変換の逆変換を計算
/// - 集合 s に含まれる下位集合 t についての包除原理を適用
///
/// # 計算式
/// f\[s\] = Σ_{t ⊆ s} (-1)^{|s| - |t|} g\[t\]
///
/// # 計算量
/// - O(n 2^n)
///   - n: 集合の要素数
pub fn subset_moebius<T>(a: &mut [T])
where
    T: Clone + Sub<Output = T>,
{
    let log = a.len().trailing_zeros();
    assert_eq!(1 << log, a.len());
    for i in 0..log {
        let i = 1 << i;
        for j in (0..a.len()).step_by(i << 1) {
            for k in 0..i {
                a[i + j + k] = a[i + j + k].clone() - a[j + k].clone();
            }
        }
    }
}

/// ビット単位のOR畳み込みを計算する
///
/// # 概要
/// 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[k] = Σ_{i | j = k} (a[i] * b[j])
/// ```
///
/// # 引数
/// - `a`: 1つ目の配列（長さは2の冪乗である必要がある）
/// - `b`: 2つ目の配列（`a`と同じ長さ）
///
/// # 戻り値
/// - OR畳み込みの結果（長さは入力と同じ）
///
/// # 制約
/// - `a.len() == b.len()`
/// - 配列の長さは2の冪乗
///
/// # 計算量
/// - O(N log N)
///   - N: 配列の長さ
pub fn or_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    subset_zeta(&mut a);
    subset_zeta(&mut b);
    a = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a * b)
        .collect();
    subset_moebius(&mut a);
    a
}

use std::ops::{Add, Mul, Sub};

/// 高速ゼータ変換
///
/// # 概要
/// - 集合 s を含む上位集合 t についての f\[t\] の総和を計算
///
/// # 計算式
/// g\[s\] = Σ_{s ⊆ t} f\[t\]
///
/// # 計算量
/// - O(n 2^n)
///   - n: 集合の要素数
pub fn superset_zeta<T>(f: &mut [T])
where
    T: Clone + Add<Output = T>,
{
    let log = f.len().trailing_zeros();
    assert_eq!(1 << log, f.len());
    for i in 0..log {
        let w = 1 << i;
        for f in f.chunks_exact_mut(w << 1) {
            let (f0, f1) = f.split_at_mut(w);
            for (x, y) in f0.iter_mut().zip(f1) {
                *x = x.clone() + y.clone();
            }
        }
    }
}

/// 高速メビウス変換
///
/// # 概要
/// - 高速ゼータ変換の逆変換を計算
/// - 集合 s を含む上位集合 t についての包除原理を適用
///
/// # 計算式
/// f\[s\] = Σ_{s ⊆ t} (-1)^{|t| - |s|} g\[t\]
///
/// # 計算量
/// - O(n 2^n)
///   - n: 集合の要素数
pub fn superset_moebius<T>(g: &mut [T])
where
    T: Clone + Sub<Output = T>,
{
    let log = g.len().trailing_zeros();
    assert_eq!(1 << log, g.len());
    for i in 0..log {
        let w = 1 << i;
        for g in g.chunks_exact_mut(w << 1) {
            let (g0, g1) = g.split_at_mut(w);
            for (x, y) in g0.iter_mut().zip(g1) {
                *x = x.clone() - y.clone();
            }
        }
    }
}

/// ビット単位のAND畳み込みを計算する
///
/// # 概要
/// 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[k] = Σ_{i & j = k} (a[i] * b[j])
/// ```
///
/// # 引数
/// - `a`: 1つ目の配列（長さは2の冪乗である必要がある）
/// - `b`: 2つ目の配列（`a`と同じ長さ）
///
/// # 戻り値
/// - AND畳み込みの結果（長さは入力と同じ）
///
/// # 制約
/// - `a.len() == b.len()`
/// - 配列の長さは2の冪乗
///
/// # 計算量
/// - O(N log N)
///   - N: 配列の長さ
pub fn and_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    superset_zeta(&mut a);
    superset_zeta(&mut b);
    a = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a * b)
        .collect();
    superset_moebius(&mut a);
    a
}

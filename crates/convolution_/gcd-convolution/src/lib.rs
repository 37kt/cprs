use std::ops::{Add, Mul, Sub};

/// 倍数ゼータ変換
///
/// # 概要
/// - i の倍数 j についての f\[j\] の総和を計算
///
/// # 引数
/// - `f`: 入力配列
///
/// # 計算式
/// g\[i\] = Σ_{i | j} f\[j\]
///
/// # 計算量
/// - O(n log log n)
///   - n: 配列の長さ
pub fn multiple_zeta<T>(f: &mut [T])
where
    T: Clone + Add<Output = T>,
{
    let n = f.len() - 1;
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if is_prime[p] {
            for q in (p * 2..=n).step_by(p) {
                is_prime[q] = false;
            }
            for i in (1..=n / p).rev() {
                f[i] = f[i].clone() + f[i * p].clone();
            }
        }
    }
}

/// 倍数メビウス変換
///
/// # 概要
/// - 倍数ゼータ変換の逆変換
///
/// # 引数
/// - `g`: 入力配列
///
/// # 計算式
/// g\[i\] = Σ_{i | j} f\[j\]
///
/// # 計算量
/// - O(n log log n)
///   - n: 配列の長さ
pub fn multiple_moebius<T>(g: &mut [T])
where
    T: Clone + Sub<Output = T>,
{
    let n = g.len() - 1;
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if is_prime[p] {
            for q in (p * 2..=n).step_by(p) {
                is_prime[q] = false;
            }
            for i in 1..=n / p {
                g[i] = g[i].clone() - g[i * p].clone();
            }
        }
    }
}

/// GCD 畳み込み
///
/// # 概要
/// - 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[k] = Σ_{k | k=gcd(i, j)} (a[i] * b[j])
/// ```
///
/// # 引数
/// - `a`: 1つ目の配列
/// - `b`: 2つ目の配列
///
/// # 戻り値
/// - GCD 畳み込みの結果（長さは `a.len() + b.len() - 1`）
///
/// # 計算量
/// - O(N log log N)
///   - N: max(a.len(), b.len())
pub fn gcd_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    multiple_zeta(&mut a);
    multiple_zeta(&mut b);
    a = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x * y)
        .collect();
    multiple_moebius(&mut a);
    a
}

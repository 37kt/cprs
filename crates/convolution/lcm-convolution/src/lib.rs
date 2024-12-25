use std::ops::{Add, Mul, Sub};

/// 約数ゼータ変換
///
/// # 概要
/// - i の約数 j についての f\[j\] の総和を計算
///
/// # 引数
/// - `f`: 入力配列
///
/// # 計算式
/// g\[i\] = Σ_{j | i} f\[j\]
///
/// # 計算量
/// - O(n log log n)
///   - n: 配列の長さ
pub fn divisor_zeta<T>(f: &mut [T])
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
            for i in 1..=n / p {
                f[i * p] = f[i * p].clone() + f[i].clone();
            }
        }
    }
}

/// 約数メビウス変換
///
/// # 概要
/// - 約数ゼータ変換の逆変換
///
/// # 引数
/// - `g`: 入力配列
///
/// # 計算式
/// g\[i\] = Σ_{j | i} f\[j\]
///
/// # 計算量
/// - O(n log log n)
///   - n: 配列の長さ
pub fn divisor_moebius<T>(g: &mut [T])
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
            for i in (1..=n / p).rev() {
                g[i * p] = g[i * p].clone() - g[i].clone();
            }
        }
    }
}

/// LCM 畳み込み
///
/// # 概要
/// - 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[k] = Σ_{k | k=lcm(i, j)} (a[i] * b[j])
/// ```
///
/// # 引数
/// - `a`: 1つ目の配列
/// - `b`: 2つ目の配列
///
/// # 戻り値
/// - LCM 畳み込みの結果（長さは `a.len() + b.len() - 1`）
///
/// # 計算量
/// - O(N log log N)
///   - N: max(a.len(), b.len())
pub fn lcm_convolution<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    divisor_zeta(&mut a);
    divisor_zeta(&mut b);
    a = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| x * y)
        .collect();
    divisor_moebius(&mut a);
    a
}

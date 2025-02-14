use std::ops::{Add, Mul, Sub};

pub fn ranked_zeta<T>(a: &[T]) -> Vec<[T; 21]>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let n = a.len();
    let logn = 63 - n.leading_zeros() as usize;
    assert_eq!(1 << logn, n);
    let mut b = vec![[T::default(); 21]; n];
    for s in 0..n {
        b[s][s.count_ones() as usize] = a[s].clone();
    }
    for i in 0..logn {
        let w = 1 << i;
        for p in (0..n).step_by(w * 2) {
            for s in p..p + w {
                let t = s | w;
                for d in 0..=logn {
                    b[t][d] = b[t][d] + b[s][d];
                }
            }
        }
    }
    b
}

pub fn ranked_moebius<T>(a: &[[T; 21]]) -> Vec<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let mut a = a.to_vec();
    let n = a.len();
    let logn = 63 - n.leading_zeros() as usize;
    assert_eq!(1 << logn, n);
    for i in 0..logn {
        let w = 1 << i;
        for p in (0..n).step_by(w * 2) {
            for s in p..p + w {
                let t = s | w;
                for d in 0..=logn {
                    a[t][d] = a[t][d] - a[s][d];
                }
            }
        }
    }
    (0..n)
        .into_iter()
        .map(|s| a[s][s.count_ones() as usize])
        .collect()
}

/// Subset Convolution
///
/// # 概要
/// - 2つの配列 `a`, `b` に対し、以下の式で定義される畳み込みを計算する：
/// ```text
/// res[s] = Σ_{t⊆s} (a[t] * b[s\t])
/// ```
///
/// # 引数
/// - `a`: 1つ目の配列
/// - `b`: 2つ目の配列
///
/// # 戻り値
/// - Subset Convolution の結果
pub fn subset_convolution<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert_eq!(a.len(), b.len());
    let logn = 63 - a.len().leading_zeros() as usize;
    let mut ra = ranked_zeta(&a);
    let rb = ranked_zeta(&b);
    for (f, g) in ra.iter_mut().zip(&rb) {
        for d in (0..=logn).rev() {
            let mut x = T::default();
            for i in 0..=d {
                x = x + f[i] * g[d - i];
            }
            f[d] = x;
        }
    }
    ranked_moebius(&ra)
}

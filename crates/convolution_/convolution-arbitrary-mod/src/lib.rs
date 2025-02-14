use convolution_naive::convolution_naive;
use convolution_ntt_friendly::convolution_ntt_friendly;
use modint::{ModInt, StaticModInt};

const M1: u32 = 167_772_161;
const M2: u32 = 469_762_049;
const M3: u32 = 754_974_721;
type Fp1 = StaticModInt<M1>;
type Fp2 = StaticModInt<M2>;
type Fp3 = StaticModInt<M3>;

const fn pow(x: u32, mut n: u32, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    let mut r = 1u64;
    let mut y = (x % m) as u64;
    while n != 0 {
        if n & 1 != 0 {
            r = r * y % m as u64;
        }
        y = y * y % m as u64;
        n >>= 1;
    }
    r as u32
}

const fn inv(x: u32, m: u32) -> u32 {
    pow(x, m - 2, m)
}

const M1INV_FP2: Fp2 = Fp2::raw(inv(M1, M2));
const M1INV_FP3: Fp3 = Fp3::raw(inv(M1, M3));
const M2INV_FP3: Fp3 = Fp3::raw(inv(M2, M3));

/// 任意の法に対する畳み込みを計算する
///
/// # 概要
/// 2つの配列 `a`, `b` に対し、任意の法での畳み込みを計算する。  
/// 内部では Chinese Remainder Theorem (CRT) を用いて3つの素数法での畳み込みから復元する。
///
/// # 引数
/// - `a`: 1つ目の配列
/// - `b`: 2つ目の配列
///
/// # 戻り値
/// - 畳み込みの結果（長さは `a.len() + b.len() - 1`）
///
/// # 計算量
/// - O(N log N)
///   - N: max(a.len(), b.len())
pub fn convolution_arbitrary_mod<T: ModInt>(a: &[T], b: &[T]) -> Vec<T> {
    if a.len().min(b.len()) < 60 {
        return convolution_naive(a, b);
    }
    let a1 = a.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let a2 = a.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let a3 = a.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let b1 = b.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let b2 = b.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let b3 = b.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let a1 = convolution_ntt_friendly(a1, b1);
    let a2 = convolution_ntt_friendly(a2, b2);
    let a3 = convolution_ntt_friendly(a3, b3);
    let m1 = T::from(Fp1::modulus());
    let m1m2 = m1 * T::from(Fp2::modulus());
    a1.iter()
        .zip(a2.iter())
        .zip(a3.iter())
        .map(|((&e1, &e2), &e3)| {
            let x1 = e1;
            let x2 = (e2 - Fp2::raw(x1.val())) * M1INV_FP2;
            let x3 = ((e3 - Fp3::raw(x1.val())) * M1INV_FP3 - Fp3::raw(x2.val())) * M2INV_FP3;
            T::from(x1.val()) + T::from(x2.val()) * m1 + T::from(x3.val()) * m1m2
        })
        .collect()
}

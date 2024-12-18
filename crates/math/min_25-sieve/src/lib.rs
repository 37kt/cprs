use std::ops::{Add, Mul, Neg};

use algebraic::One;
use prime_sieve::PrimeSieve;
use quotients_array::QuotientsArray;

/// f は乗法的関数
/// sum_fp(n) = Σp∈\[1..=n | p is prime\] f(p)  
/// f(p, e) = f(p^e)  
/// return Σi∈\[1..=n\] f(i)  
pub fn min_25_sieve<T>(
    sum_fp: &QuotientsArray<T>,
    mut f: impl FnMut(usize, usize) -> T,
) -> QuotientsArray<T>
where
    T: Copy + Add<Output = T> + Neg<Output = T> + Mul<Output = T> + One,
{
    let sqrt_n = sum_fp.sqrt_n();
    let ps = PrimeSieve::new(sqrt_n).primes();
    let qs = sum_fp.quotients().to_vec();

    let mut dp = sum_fp.clone();
    for &p in ps.iter().rev() {
        for &q in &qs {
            let mut pc = p;
            if pc * p > q {
                break;
            }
            let mut c = 1;
            while q / p >= pc {
                let d = dp[q / pc] + -sum_fp[p];
                dp[q] = dp[q] + f(p, c) * d + f(p, c + 1);
                pc *= p;
                c += 1;
            }
        }
    }
    for &q in &qs {
        dp[q] = dp[q] + T::one();
    }

    dp
}

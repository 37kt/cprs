use std::ops::{Add, Neg};

use prime_sieve::PrimeSieve;
use quotients_array::QuotientsArray;

/// sum_f(n) = Σi∈\[1..=n\] f(i)  
/// mul(f(x), p) = f(px)  
/// return Σp∈\[1..=n | p is prime\] f(p)
pub fn lucy_dp<T>(
    sum_f: &QuotientsArray<T>,
    mut mul: impl FnMut(T, usize) -> T,
) -> QuotientsArray<T>
where
    T: Copy + Add<Output = T> + Neg<Output = T>,
{
    let sqrt_n = sum_f.sqrt_n();
    let qs = sum_f.quotients().to_vec();
    let ps = PrimeSieve::new(sqrt_n).primes();

    let mut dp = sum_f.clone();
    let f1 = dp[1];
    for &i in &qs {
        dp[i] = dp[i] + -f1;
    }

    for &p in &ps {
        let dpr = dp[p - 1];
        let p2 = p * p;
        for &q in &qs {
            if q < p2 {
                break;
            }
            let dpl = dp[q / p];
            dp[q] = dp[q] + -mul(dpl + -dpr, p);
        }
    }

    dp
}

use std::ops::Mul;

use dynamic_modint_64::DynamicModInt64;

mod miller_rabin;
mod pollard_rho;

use miller_rabin::miller_rabin;
use pollard_rho::pollard_rho;

pub fn is_prime(n: impl TryInto<u64, Error = impl std::fmt::Debug>) -> bool {
    let n: u64 = n.try_into().expect("failed to convert to u64");
    if n & 1 == 0 {
        n == 2
    } else if n <= 1 {
        false
    } else if n < 1 << 30 {
        miller_rabin(n, &[2, 7, 61])
    } else {
        miller_rabin(n, &[2, 325, 9375, 28178, 450775, 9780504, 1795265022])
    }
}

/// n の素因数を昇順で返す
pub fn prime_factors<T>(n: T) -> Vec<T>
where
    T: TryInto<u64> + TryFrom<u64>,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    let n: u64 = n.try_into().expect("failed to convert to u64");
    assert!(n > 0, "n must be positive");

    let mut res = vec![];
    pollard_rho(n, &mut res);
    res.sort_unstable();
    res.into_iter().map(|x| x.try_into().unwrap()).collect()
}

/// n の素因数分解を \[(素因数, 指数), ...\] の形で昇順で返す
pub fn prime_factorization<T>(n: T) -> Vec<(T, usize)>
where
    T: TryInto<u64> + TryFrom<u64> + Eq,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    let factors = prime_factors(n);
    if factors.is_empty() {
        return vec![];
    }
    let mut res = vec![];
    for p in factors {
        if matches!(res.last(), Some((last, _)) if last == &p) {
            res.last_mut().unwrap().1 += 1;
        } else {
            res.push((p, 1));
        }
    }
    res
}

pub fn divisors_unordered<T>(n: T) -> Vec<T>
where
    T: TryInto<u64> + TryFrom<u64> + Eq + Mul<Output = T> + Copy,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    let pe = prime_factorization(n);
    let mut res = vec![T::try_from(1).unwrap()];
    for (p, e) in pe {
        for i in 0..res.len() {
            let mut x = res[i];
            for _ in 0..e {
                x = x * p;
                res.push(x);
            }
        }
    }
    res
}

/// n の約数を昇順で返す
pub fn divisors<T>(n: T) -> Vec<T>
where
    T: TryInto<u64> + TryFrom<u64> + Ord + Mul<Output = T> + Copy,
    <T as TryInto<u64>>::Error: std::fmt::Debug,
    <T as TryFrom<u64>>::Error: std::fmt::Debug,
{
    let mut res = divisors_unordered(n);
    res.sort_unstable();
    res
}

enum Id {}
type Mint = DynamicModInt64<Id>;

use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    mem::swap,
};

use montgomery_modint::MontgomeryModInt;

/// 素数判定
pub fn is_prime(n: impl TryInto<u64, Error = impl Debug>) -> bool {
    let n: u64 = n.try_into().unwrap();
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

/// 素因数分解  
/// 素因数を昇順に列挙
pub fn factorize<N, E, F>(n: N) -> Vec<N>
where
    N: TryInto<u64, Error = E> + TryFrom<u64, Error = F> + Ord + Copy,
    E: Debug,
    F: Debug,
{
    let n = n.try_into().unwrap();
    let mut f = factorize_(n);
    f.sort_unstable();
    f.into_iter().map(|x| x.try_into().unwrap()).collect()
}

/// 素因数分解  
/// (素因数, 指数) のペアを列挙
pub fn factor_count<N, E, F>(n: N) -> Vec<(N, usize)>
where
    N: TryInto<u64, Error = E> + TryFrom<u64, Error = F> + Ord + Copy,
    E: Debug,
    F: Debug,
{
    let f = factorize(n);
    if f.len() == 0 {
        return vec![];
    }
    let mut r = vec![(f[0], 0)];
    for p in f {
        if r.last().unwrap().0 == p {
            r.last_mut().unwrap().1 += 1;
        } else {
            r.push((p, 1));
        }
    }
    r
}

/// 約数を昇順に列挙
pub fn divisors<N, E, F>(n: N) -> Vec<N>
where
    N: TryInto<u64, Error = E> + TryFrom<u64, Error = F> + Ord + Copy,
    E: Debug,
    F: Debug,
{
    let n = n.try_into().unwrap();
    if n == 0 {
        return vec![];
    }
    let fc = factor_count(n);
    let mut r = vec![1];
    for (p, c) in fc {
        for i in 0..r.len() {
            let mut x = r[i];
            for _ in 0..c {
                x *= p;
                r.push(x);
            }
        }
    }
    r.sort_unstable();
    r.into_iter().map(|x| x.try_into().unwrap()).collect()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

fn miller_rabin(n: u64, a: &[u64]) -> bool {
    MontgomeryModInt::set_modulus(n);
    let d = (n - 1) >> (n - 1).trailing_zeros();
    let e = MontgomeryModInt::new(1);
    let r = MontgomeryModInt::new(n - 1);
    for &a in a {
        if n <= a {
            break;
        }
        let mut t = d;
        let mut y = MontgomeryModInt::new(a).pow(t);
        while t != n - 1 && y != e && y != r {
            y *= y;
            t *= 2;
        }
        if y != r && t % 2 == 0 {
            return false;
        }
    }
    true
}

fn pollard_rho(n: u64) -> u64 {
    if n & 1 == 0 {
        return 2;
    } else if is_prime(n) {
        return n;
    }
    let m = 1 << (64 - n.leading_zeros()) / 8;
    let o = MontgomeryModInt::new(1);
    let mut c = o;
    loop {
        let f = |x: MontgomeryModInt| x * x + c;
        let mut x = o;
        let mut y = MontgomeryModInt::new(2);
        let mut ys = o;
        let mut q = o;
        let mut r = 1;
        let mut g = 1;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = f(y);
            }
            for k in (0..r).step_by(m) {
                if g != 1 {
                    break;
                }
                ys = y;
                for _ in 0..m.min(r - k) {
                    y = f(y);
                    q *= x - y;
                }
                g = gcd(q.val(), n);
            }
            r <<= 1;
        }
        if g == n {
            g = 1;
            while g == 1 {
                ys = f(ys);
                g = gcd((x - ys).val(), n);
            }
        }
        if g < n {
            return if is_prime(g) {
                g
            } else if is_prime(n / g) {
                n / g
            } else {
                pollard_rho(g)
            };
        }
        c += o;
    }
}

fn factorize_(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    };
    let p = pollard_rho(n);
    if p == n {
        return vec![p];
    }
    let mut r = factorize_(p);
    r.extend(factorize_(n / p));
    r
}

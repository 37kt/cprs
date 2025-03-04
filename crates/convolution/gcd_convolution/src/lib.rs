use algebraic_traits::{AbelianGroup, CommutativeMonoid, Magma, Ring};
use eratosthenes::Eratosthenes;

/// 約数の方向に累積和を取る
pub fn multiple_zeta_transform<M: CommutativeMonoid>(f: &mut [M::Value]) {
    let n = f.len() - 1;
    let sieve = Eratosthenes::new(n);
    for p in sieve.primes() {
        for i in (1..=n / p).rev() {
            f[i] = M::op(&f[i], &f[i * p]);
        }
    }
}

/// 約数の方向に取られた累積和から復元する
pub fn multiple_moebius_transform<G: AbelianGroup>(f: &mut [G::Value]) {
    let n = f.len() - 1;
    let sieve = Eratosthenes::new(n);
    for p in sieve.primes() {
        for i in 1..=n / p {
            f[i] = G::op(&f[i], &G::inv(&f[i * p]));
        }
    }
}

/// h\[k\] = sum_{gcd(i, j) = k} f\[i\] * g\[j\]
pub fn gcd_convolution<R>(f: &[R::Value], g: &[R::Value]) -> Vec<R::Value>
where
    R: Ring,
    R::Additive: AbelianGroup,
    R::Value: Clone,
{
    assert_eq!(f.len(), g.len(), "f and g must have the same length");
    let mut f = f.to_vec();
    let mut g = g.to_vec();
    if f.len() == 0 {
        return vec![];
    }
    multiple_zeta_transform::<R::Additive>(&mut f);
    multiple_zeta_transform::<R::Additive>(&mut g);
    for (x, y) in f.iter_mut().zip(&g).skip(1) {
        *x = R::Multiplicative::op(x, y);
    }
    multiple_moebius_transform::<R::Additive>(&mut f);
    f
}

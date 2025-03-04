use algebraic_traits::{AbelianGroup, CommutativeMonoid, Magma, Ring};
use eratosthenes::Eratosthenes;

/// 約数の方向に累積和を取る
pub fn divisor_zeta_transform<M: CommutativeMonoid>(f: &mut [M::Value]) {
    let n = f.len() - 1;
    let sieve = Eratosthenes::new(n);
    for p in sieve.primes() {
        for i in 1..=n / p {
            f[i * p] = M::op(&f[i * p], &f[i]);
        }
    }
}

/// 約数の方向に取られた累積和から復元する
pub fn divisor_moebius_transform<G: AbelianGroup>(f: &mut [G::Value]) {
    let n = f.len() - 1;
    let sieve = Eratosthenes::new(n);
    for p in sieve.primes() {
        for i in (1..=n / p).rev() {
            f[i * p] = G::op(&f[i * p], &G::inv(&f[i]));
        }
    }
}

/// h\[k\] = sum_{lcm(i, j) = k} f\[i\] * g\[j\]
pub fn lcm_convolution<R>(f: &[R::Value], g: &[R::Value]) -> Vec<R::Value>
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
    divisor_zeta_transform::<R::Additive>(&mut f);
    divisor_zeta_transform::<R::Additive>(&mut g);
    for (x, y) in f.iter_mut().zip(&g).skip(1) {
        *x = R::Multiplicative::op(x, y);
    }
    divisor_moebius_transform::<R::Additive>(&mut f);
    f
}

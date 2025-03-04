use algebraic_traits::{AbelianGroup, CommutativeMonoid, Magma, Ring};
use numeric_traits::Integer;

/// 部分集合の方向に累積和を取る
pub fn superset_zeta_transform<M: CommutativeMonoid>(f: &mut [M::Value]) {
    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "n must be a power of 2");

    for i in 0..lg {
        let w = 1 << i;
        for f in f.chunks_exact_mut(w << 1) {
            let (f0, f1) = f.split_at_mut(w);
            for (x, y) in f0.iter_mut().zip(f1) {
                *x = M::op(x, y);
            }
        }
    }
}

/// 部分集合の方向に取られた累積和から復元する
pub fn superset_moebius_transform<G: AbelianGroup>(f: &mut [G::Value]) {
    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "n must be a power of 2");

    for i in 0..lg {
        let w = 1 << i;
        for f in f.chunks_exact_mut(w << 1) {
            let (f0, f1) = f.split_at_mut(w);
            for (x, y) in f0.iter_mut().zip(f1) {
                *x = G::op(x, &G::inv(y));
            }
        }
    }
}

/// h\[k\] = sum_{i & j = k} f\[i\] * g\[j\]
pub fn bitwise_and_convolution<R>(f: &[R::Value], g: &[R::Value]) -> Vec<R::Value>
where
    R: Ring,
    R::Additive: AbelianGroup,
    R::Value: Clone,
{
    assert_eq!(f.len(), g.len(), "f and g must have the same length");
    let mut f = f.to_vec();
    let mut g = g.to_vec();
    superset_zeta_transform::<R::Additive>(&mut f);
    superset_zeta_transform::<R::Additive>(&mut g);
    for (x, y) in f.iter_mut().zip(&g) {
        *x = R::Multiplicative::op(x, y);
    }
    superset_moebius_transform::<R::Additive>(&mut f);
    f
}

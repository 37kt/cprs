use algebraic_traits::{AbelianGroup, Magma, Ring};
use numeric_traits::Integer;

pub fn hadamard_transform<G>(f: &mut [G::Value])
where
    G: AbelianGroup,
{
    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "n must be a power of 2");

    for i in 0..lg {
        let w = 1 << i;
        for f in f.chunks_exact_mut(w << 1) {
            let (f0, f1) = f.split_at_mut(w);
            for (x, y) in f0.iter_mut().zip(f1) {
                let nx = G::op(x, y);
                let ny = G::op(x, &G::inv(y));
                *x = nx;
                *y = ny;
            }
        }
    }
}

pub fn hadamard_transform_inverse<G>(
    f: &mut [G::Value],
    mut divide_by_2: impl FnMut(G::Value) -> G::Value,
) where
    G: AbelianGroup,
{
    let n = f.len();
    let lg = n.floor_log2();
    assert_eq!(n, 1 << lg, "n must be a power of 2");

    for i in 0..lg {
        let w = 1 << i;
        for f in f.chunks_exact_mut(w << 1) {
            let (f0, f1) = f.split_at_mut(w);
            for (x, y) in f0.iter_mut().zip(f1) {
                let nx = divide_by_2(G::op(x, y));
                let ny = divide_by_2(G::op(x, &G::inv(y)));
                *x = nx;
                *y = ny;
            }
        }
    }
}

pub fn bitwise_xor_convolution<R>(
    f: &[R::Value],
    g: &[R::Value],
    divide_by_2: impl FnMut(R::Value) -> R::Value,
) -> Vec<R::Value>
where
    R: Ring,
    R::Additive: AbelianGroup,
    R::Value: Clone,
{
    let mut f = f.to_vec();
    let mut g = g.to_vec();
    hadamard_transform::<R::Additive>(&mut f);
    hadamard_transform::<R::Additive>(&mut g);
    for (x, y) in f.iter_mut().zip(&g) {
        *x = R::Multiplicative::op(x, y);
    }
    hadamard_transform_inverse::<R::Additive>(&mut f, divide_by_2);
    f
}

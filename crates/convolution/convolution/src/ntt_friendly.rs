use numeric_traits::Integer;
use static_modint::StaticModInt;

use crate::{convolution_naive, ntt, ntt_inv};

const NAIVE_THRESHOLD: usize = 128;

pub fn convolution_ntt_friendly<const P: u32>(
    f: &[StaticModInt<P>],
    g: &[StaticModInt<P>],
) -> Vec<StaticModInt<P>> {
    let n = f.len();
    let m = g.len();
    if n == 0 || m == 0 {
        return vec![];
    } else if n.min(m) <= NAIVE_THRESHOLD {
        return convolution_naive(f, g);
    }

    let l = n + m - 1;
    let sz = l.ceil_pow2();
    let mut f = f.to_vec();
    let mut g = g.to_vec();
    f.resize(sz, 0.into());
    g.resize(sz, 0.into());
    ntt(&mut f);
    ntt(&mut g);
    for (x, y) in f.iter_mut().zip(g) {
        *x *= y;
    }
    ntt_inv(&mut f);
    f.truncate(l);
    f
}

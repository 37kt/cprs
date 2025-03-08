use modint::ModInt;
use numeric_traits::Numeric;
use static_modint::StaticModInt;

use crate::{convolution_naive, convolution_ntt_friendly, inv};

const NAIVE_THRESHOLD: usize = 512; // 雑

const M1: u32 = 167_772_161;
const M2: u32 = 469_762_049;
const M3: u32 = 754_974_721;
type Fp1 = StaticModInt<M1>;
type Fp2 = StaticModInt<M2>;
type Fp3 = StaticModInt<M3>;

const M1INV_FP2: Fp2 = Fp2::from_raw(inv(M1, M2));
const M1INV_FP3: Fp3 = Fp3::from_raw(inv(M1, M3));
const M2INV_FP3: Fp3 = Fp3::from_raw(inv(M2, M3));

pub fn convolution_arbitrary_mod<M: ModInt<Value = u32> + Numeric>(f: &[M], g: &[M]) -> Vec<M> {
    let n = f.len();
    let m = g.len();
    if n.min(m) <= NAIVE_THRESHOLD {
        return convolution_naive(f, g);
    }

    let f1 = f.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let f2 = f.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let f3 = f.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let g1 = g.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let g2 = g.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let g3 = g.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let fg1 = convolution_ntt_friendly(&f1, &g1);
    let fg2 = convolution_ntt_friendly(&f2, &g2);
    let fg3 = convolution_ntt_friendly(&f3, &g3);

    let m1 = M::from(M1);
    let m1m2 = m1 * M::from(M2);
    fg1.into_iter()
        .zip(fg2)
        .zip(fg3)
        .map(|((e1, e2), e3)| {
            let x1 = e1;
            let x2 = (e2 - Fp2::from_raw(x1.val())) * M1INV_FP2;
            let x3 =
                ((e3 - Fp3::from_raw(x1.val())) * M1INV_FP3 - Fp3::from_raw(x2.val())) * M2INV_FP3;
            M::from(x1.val()) + M::from(x2.val()) * m1 + M::from(x3.val()) * m1m2
        })
        .collect()
}

use static_modint::StaticModInt;

use crate::{convolution_ntt_friendly, inv};

const NAIVE_THRESHOLD: usize = 512; // 雑

// const M1: u32 = 167_772_161;
// const M2: u32 = 469_762_049;
// const M3: u32 = 754_974_721;
// const M4: u32 = 880_803_841;
// const M5: u32 = 998_244_353;
const M1: u32 = 0x6c000001;
const M2: u32 = 0x78000001;
const M3: u32 = 0x7c800001;
const M4: u32 = 0x7e000001;
const M5: u32 = 0x7f000001;
type Fp1 = StaticModInt<M1>;
type Fp2 = StaticModInt<M2>;
type Fp3 = StaticModInt<M3>;
type Fp4 = StaticModInt<M4>;
type Fp5 = StaticModInt<M5>;

const M1INV_FP2: Fp2 = Fp2::from_raw(inv(M1, M2));
const M1INV_FP3: Fp3 = Fp3::from_raw(inv(M1, M3));
const M1INV_FP4: Fp4 = Fp4::from_raw(inv(M1, M4));
const M1INV_FP5: Fp5 = Fp5::from_raw(inv(M1, M5));
const M2INV_FP3: Fp3 = Fp3::from_raw(inv(M2, M3));
const M2INV_FP4: Fp4 = Fp4::from_raw(inv(M2, M4));
const M2INV_FP5: Fp5 = Fp5::from_raw(inv(M2, M5));
const M3INV_FP4: Fp4 = Fp4::from_raw(inv(M3, M4));
const M3INV_FP5: Fp5 = Fp5::from_raw(inv(M3, M5));
const M4INV_FP5: Fp5 = Fp5::from_raw(inv(M4, M5));
const P_M1: u64 = M1 as u64;
const P_M1M2: u64 = P_M1.wrapping_mul(M2 as u64);
const P_M1M2M3: u64 = P_M1M2.wrapping_mul(M3 as u64);
const P_M1M2M3M4: u64 = P_M1M2M3.wrapping_mul(M4 as u64);

pub fn convolution_mod_2_64(f: &[u64], g: &[u64]) -> Vec<u64> {
    let n = f.len();
    let m = g.len();
    if n.min(m) <= NAIVE_THRESHOLD {
        return convolution_naive_mod_2_64(f, g);
    }

    let f1 = f.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();
    let f2 = f.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();
    let f3 = f.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();
    let f4 = f.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();
    let f5 = f.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();
    let g1 = g.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();
    let g2 = g.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();
    let g3 = g.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();
    let g4 = g.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();
    let g5 = g.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();
    let fg1 = convolution_ntt_friendly(&f1, &g1);
    let fg2 = convolution_ntt_friendly(&f2, &g2);
    let fg3 = convolution_ntt_friendly(&f3, &g3);
    let fg4 = convolution_ntt_friendly(&f4, &g4);
    let fg5 = convolution_ntt_friendly(&f5, &g5);

    fg1.into_iter()
        .zip(fg2)
        .zip(fg3)
        .zip(fg4)
        .zip(fg5)
        .map(|((((e1, e2), e3), e4), e5)| {
            let x1 = e1;
            let x2 = (e2 - Fp2::from_raw(x1.val())) * M1INV_FP2;
            let x3 =
                ((e3 - Fp3::from_raw(x1.val())) * M1INV_FP3 - Fp3::from_raw(x2.val())) * M2INV_FP3;
            let x4 = (((e4 - Fp4::from_raw(x1.val())) * M1INV_FP4 - Fp4::from_raw(x2.val()))
                * M2INV_FP4
                - Fp4::from_raw(x3.val()))
                * M3INV_FP4;
            let x5 = ((((e5 - Fp5::from_raw(x1.val())) * M1INV_FP5 - Fp5::from_raw(x2.val()))
                * M2INV_FP5
                - Fp5::from_raw(x3.val()))
                * M3INV_FP5
                - Fp5::from_raw(x4.val()))
                * M4INV_FP5;
            (x1.val() as u64)
                .wrapping_add((x2.val() as u64).wrapping_mul(P_M1))
                .wrapping_add((x3.val() as u64).wrapping_mul(P_M1M2))
                .wrapping_add((x4.val() as u64).wrapping_mul(P_M1M2M3))
                .wrapping_add((x5.val() as u64).wrapping_mul(P_M1M2M3M4))
        })
        .collect()
}

pub fn convolution_naive_mod_2_64(f: &[u64], g: &[u64]) -> Vec<u64> {
    let n = f.len();
    let m = g.len();
    if n > m {
        return convolution_naive_mod_2_64(g, f);
    } else if n == 0 {
        return vec![];
    }

    let mut h = vec![0u64; n + m - 1];
    for i in 0..n {
        for j in 0..m {
            h[i + j] = h[i + j].wrapping_add(f[i].wrapping_mul(g[j]));
        }
    }
    h
}

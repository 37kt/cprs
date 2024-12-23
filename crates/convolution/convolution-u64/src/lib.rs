use convolution_ntt_friendly::convolution_ntt_friendly;
use modint::StaticModInt;

const M1: u32 = 167_772_161;
const M2: u32 = 469_762_049;
const M3: u32 = 754_974_721;
const M4: u32 = 880_803_841;
const M5: u32 = 998_244_353;
type Fp1 = StaticModInt<M1>;
type Fp2 = StaticModInt<M2>;
type Fp3 = StaticModInt<M3>;
type Fp4 = StaticModInt<M4>;
type Fp5 = StaticModInt<M5>;

const fn pow(x: u32, mut n: u32, m: u32) -> u32 {
    if m == 1 {
        return 0;
    }
    let mut r = 1u64;
    let mut y = (x % m) as u64;
    while n != 0 {
        if n & 1 != 0 {
            r = r * y % m as u64;
        }
        y = y * y % m as u64;
        n >>= 1;
    }
    r as u32
}

const fn inv(x: u32, m: u32) -> u32 {
    pow(x, m - 2, m)
}

const M1INV_FP2: Fp2 = Fp2::raw(inv(M1, M2));
const M1INV_FP3: Fp3 = Fp3::raw(inv(M1, M3));
const M1INV_FP4: Fp4 = Fp4::raw(inv(M1, M4));
const M1INV_FP5: Fp5 = Fp5::raw(inv(M1, M5));
const M2INV_FP3: Fp3 = Fp3::raw(inv(M2, M3));
const M2INV_FP4: Fp4 = Fp4::raw(inv(M2, M4));
const M2INV_FP5: Fp5 = Fp5::raw(inv(M2, M5));
const M3INV_FP4: Fp4 = Fp4::raw(inv(M3, M4));
const M3INV_FP5: Fp5 = Fp5::raw(inv(M3, M5));
const M4INV_FP5: Fp5 = Fp5::raw(inv(M4, M5));
const P_M1: u64 = M1 as u64;
const P_M1M2: u64 = P_M1.wrapping_mul(M2 as u64);
const P_M1M2M3: u64 = P_M1M2.wrapping_mul(M3 as u64);
const P_M1M2M3M4: u64 = P_M1M2M3.wrapping_mul(M4 as u64);

pub fn convolution_u64(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let a1 = a.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();
    let a2 = a.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();
    let a3 = a.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();
    let a4 = a.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();
    let a5 = a.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();
    let b1 = b.iter().map(|&x| Fp1::new(x)).collect::<Vec<_>>();
    let b2 = b.iter().map(|&x| Fp2::new(x)).collect::<Vec<_>>();
    let b3 = b.iter().map(|&x| Fp3::new(x)).collect::<Vec<_>>();
    let b4 = b.iter().map(|&x| Fp4::new(x)).collect::<Vec<_>>();
    let b5 = b.iter().map(|&x| Fp5::new(x)).collect::<Vec<_>>();
    let a1 = convolution_ntt_friendly(a1, b1);
    let a2 = convolution_ntt_friendly(a2, b2);
    let a3 = convolution_ntt_friendly(a3, b3);
    let a4 = convolution_ntt_friendly(a4, b4);
    let a5 = convolution_ntt_friendly(a5, b5);
    a1.iter()
        .zip(a2.iter())
        .zip(a3.iter())
        .zip(a4.iter())
        .zip(a5.iter())
        .map(|((((&e1, &e2), &e3), &e4), &e5)| {
            let x1 = e1;
            let x2 = (e2 - Fp2::raw(x1.val())) * M1INV_FP2;
            let x3 = ((e3 - Fp3::raw(x1.val())) * M1INV_FP3 - Fp3::raw(x2.val())) * M2INV_FP3;
            let x4 = (((e4 - Fp4::raw(x1.val())) * M1INV_FP4 - Fp4::raw(x2.val())) * M2INV_FP4
                - Fp4::raw(x3.val()))
                * M3INV_FP4;
            let x5 = ((((e5 - Fp5::raw(x1.val())) * M1INV_FP5 - Fp5::raw(x2.val())) * M2INV_FP5
                - Fp5::raw(x3.val()))
                * M3INV_FP5
                - Fp5::raw(x4.val()))
                * M4INV_FP5;
            (x1.val() as u64)
                .wrapping_add((x2.val() as u64).wrapping_mul(P_M1))
                .wrapping_add((x3.val() as u64).wrapping_mul(P_M1M2))
                .wrapping_add((x4.val() as u64).wrapping_mul(P_M1M2M3))
                .wrapping_add((x5.val() as u64).wrapping_mul(P_M1M2M3M4))
        })
        .collect()
}

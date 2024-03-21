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
            let x2 = (e2 - Fp2::new(x1.val())) * Fp2::new(Fp1::modulus()).inv();
            let x3 = ((e3 - Fp3::new(x1.val())) * Fp3::new(Fp1::modulus()).inv()
                - Fp3::new(x2.val()))
                * Fp3::new(Fp2::modulus()).inv();
            let x4 = (((e4 - Fp4::new(x1.val())) * Fp4::new(Fp1::modulus()).inv()
                - Fp4::new(x2.val()))
                * Fp4::new(Fp2::modulus()).inv()
                - Fp4::new(x3.val()))
                * Fp4::new(Fp3::modulus()).inv();
            let x5 = ((((e5 - Fp5::new(x1.val())) * Fp5::new(Fp1::modulus()).inv()
                - Fp5::new(x2.val()))
                * Fp5::new(Fp2::modulus()).inv()
                - Fp5::new(x3.val()))
                * Fp5::new(Fp3::modulus()).inv()
                - Fp5::new(x4.val()))
                * Fp5::new(Fp4::modulus()).inv();
            (x1.val() as u64)
                .wrapping_add((x2.val() as u64).wrapping_mul(Fp1::modulus() as u64))
                .wrapping_add(
                    (x3.val() as u64)
                        .wrapping_mul((Fp1::modulus() as u64).wrapping_mul(Fp2::modulus() as u64)),
                )
                .wrapping_add(
                    (x4.val() as u64).wrapping_mul(
                        (Fp1::modulus() as u64)
                            .wrapping_mul(Fp2::modulus() as u64)
                            .wrapping_mul(Fp3::modulus() as u64),
                    ),
                )
                .wrapping_add(
                    (x5.val() as u64).wrapping_mul(
                        (Fp1::modulus() as u64)
                            .wrapping_mul(Fp2::modulus() as u64)
                            .wrapping_mul(Fp3::modulus() as u64)
                            .wrapping_mul(Fp4::modulus() as u64),
                    ),
                )
        })
        .collect()
}

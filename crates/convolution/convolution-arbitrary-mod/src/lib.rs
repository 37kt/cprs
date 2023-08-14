use convolution_naive::convolution_naive;
use convolution_ntt_friendly::convolution_ntt_friendly;
use modint::{ModInt, StaticModInt};

const M1: u32 = 167_772_161;
const M2: u32 = 469_762_049;
const M3: u32 = 754_974_721;
type Fp1 = StaticModInt<M1>;
type Fp2 = StaticModInt<M2>;
type Fp3 = StaticModInt<M3>;

pub fn convolution_arbitrary_mod<T: ModInt>(a: &[T], b: &[T]) -> Vec<T> {
    if a.len().min(b.len()) < 60 {
        return convolution_naive(a, b);
    }
    let a1 = a.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let a2 = a.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let a3 = a.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let b1 = b.iter().map(|&x| Fp1::new(x.val())).collect::<Vec<_>>();
    let b2 = b.iter().map(|&x| Fp2::new(x.val())).collect::<Vec<_>>();
    let b3 = b.iter().map(|&x| Fp3::new(x.val())).collect::<Vec<_>>();
    let a1 = convolution_ntt_friendly(a1, b1);
    let a2 = convolution_ntt_friendly(a2, b2);
    let a3 = convolution_ntt_friendly(a3, b3);
    a1.iter()
        .zip(a2.iter())
        .zip(a3.iter())
        .map(|((&e1, &e2), &e3)| {
            let x1 = e1;
            let x2 = (e2 - Fp2::new(x1.val())) * Fp2::new(Fp1::modulus()).inv();
            let x3 = ((e3 - Fp3::new(x1.val())) * Fp3::new(Fp1::modulus()).inv()
                - Fp3::new(x2.val()))
                * Fp3::new(Fp2::modulus()).inv();
            T::from(x1.val())
                + T::from(x2.val()) * T::from(Fp1::modulus())
                + T::from(x3.val()) * T::from(Fp1::modulus()) * T::from(Fp2::modulus())
        })
        .collect()
}

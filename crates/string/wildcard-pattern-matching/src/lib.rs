use convolution_ntt_friendly::convolution_ntt_friendly;
use modint::ModInt998244353 as Mint;
use random::Pcg64Fast;

pub fn wildcard_pattern_matching<T>(s: &[T], t: &[T], wildcard: T) -> Vec<bool>
where
    T: Copy + Eq + Into<Mint>,
{
    let n = s.len();
    let m = t.len();
    assert!(n >= m);
    let mut rng = Pcg64Fast::default();
    let r = Mint::new(rng.u64());
    let a1: Vec<_> = s
        .iter()
        .map(|&x| if x == wildcard { Mint::new(0) } else { r + x })
        .collect();
    let a2: Vec<_> = a1.iter().map(|&x| x * x).collect();
    let a3: Vec<_> = a1.iter().map(|&x| x * x * x).collect();
    let b1: Vec<_> = t
        .iter()
        .rev()
        .map(|&x| if x == wildcard { Mint::new(0) } else { r + x })
        .collect();
    let b2: Vec<_> = b1.iter().map(|&x| x * x).collect();
    let b3: Vec<_> = b1.iter().map(|&x| x * x * x).collect();
    let c13 = convolution_ntt_friendly(a1, b3);
    let c22 = convolution_ntt_friendly(a2, b2);
    let c31 = convolution_ntt_friendly(a3, b1);
    (0..=n - m)
        .map(|i| (c13[i + m - 1] - c22[i + m - 1] * 2 + c31[i + m - 1]).val() == 0)
        .collect()
}

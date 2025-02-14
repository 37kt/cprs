// verification-helper: PROBLEM https://judge.yosupo.jp/problem/sum_of_multiplicative_function

use lucy_dp::lucy_dp;
use min_25_sieve::min_25_sieve;
use modint::StaticModInt;
use proconio::input;
use quotients_array::QuotientsArray;

type Mint = StaticModInt<469762049>;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            a: Mint,
            b: Mint,
        }
        let inv_2 = Mint::new(2).inv();
        let sum_fa = QuotientsArray::from_fn(n, |i| Mint::new(i) * a);
        let sum_fb = QuotientsArray::from_fn(n, |i| Mint::new(i) * (i + 1) * inv_2 * b);
        let qs = sum_fa.quotients().to_vec();
        let sum_fap = lucy_dp(&sum_fa, |x, _| x);
        let sum_fbp = lucy_dp(&sum_fb, |x, p| x * p);
        let mut sum_fp = sum_fap;
        for &q in &qs {
            sum_fp[q] += sum_fbp[q];
        }
        let sum_f = min_25_sieve(&sum_fp, |p, e| a * e + b * p);
        println!("{}", sum_f[n]);
    }
}

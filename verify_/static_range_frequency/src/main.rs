// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use proconio::input;
use wavelet_matrix::WaveletMatrix;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let wm = WaveletMatrix::new(a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: usize,
        }
        println!("{}", wm.range_freq(l..r, x..=x));
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency

use proconio::fastout;
use proconio::input;
use wavelet_matrix::WaveletMatrix;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }

    let wm = WaveletMatrix::<_>::new(a);
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            x: u32,
        }
        println!("{}", wm.count(l..r, x..=x));
    }
}

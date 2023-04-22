// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

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
            k: usize,
        }
        println!("{}", wm.kth_smallest(l..r, k));
    }
}

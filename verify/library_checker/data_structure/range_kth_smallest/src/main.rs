// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest

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
            k: usize,
        }
        println!("{}", wm.kth_smallest(l..r, k).unwrap());
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_det_mod_2

use matrix_mod2::MatrixMod2;
use proconio::{input, marker::Bytes};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: [Bytes; n],
    }
    let mut a = MatrixMod2::new(n, n);
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'1' {
                a.set(i, j, true);
            }
        }
    }
    println!("{}", a.det() as u8);
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/matrix_product_mod_2

use matrix_mod2::MatrixMod2;
use proconio::{input, marker::Bytes};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: [Bytes; n],
        t: [Bytes; m],
    }
    let mut a = MatrixMod2::new(n, m);
    let mut b = MatrixMod2::new(m, k);
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == b'1' {
                a.set(i, j, true);
            }
        }
    }
    for i in 0..m {
        for j in 0..k {
            if t[i][j] == b'1' {
                b.set(i, j, true);
            }
        }
    }
    let c = &a * &b;
    print!("{}", c);
}

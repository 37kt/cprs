// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bitwise_and_convolution

use ac_library::ModInt998244353 as Mint;
use and_convolution::and_convolution;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; 1 << n],
        b: [Mint; 1 << n],
    }
    let c = and_convolution(a, b);
    for i in 0..1 << n {
        print!("{}", c[i]);
        if i < 1 << n - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}

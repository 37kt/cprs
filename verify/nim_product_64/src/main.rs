// verification-helper: PROBLEM https://judge.yosupo.jp/problem/nim_product_64

use nimber::Nimber;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            a: Nimber,
            b: Nimber,
        }
        println!("{}", a * b);
    }
}

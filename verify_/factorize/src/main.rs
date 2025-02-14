// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use fast_factorize::factorize;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            a: u64,
        }
        let x = factorize(a);
        print!("{}", x.len());
        for x in x {
            print!(" {}", x);
        }
        println!();
    }
}

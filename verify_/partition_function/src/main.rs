// verification-helper: PROBLEM https://judge.yosupo.jp/problem/partition_function

use partition::partition;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let p = partition::<998244353>(n);
    println!("{}", p);
}

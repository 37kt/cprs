// verification-helper: PROBLEM https://yukicoder.me/problems/no/1737

use prime_sieve::PrimeSieve;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let pr = PrimeSieve::new(n);
    let mut res = 0;
    for (x, k) in pr.factorize(n) {
        res += x * k;
    }
    println!("{}", res);
}

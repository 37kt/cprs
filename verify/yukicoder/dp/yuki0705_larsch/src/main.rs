// verification-helper: PROBLEM https://yukicoder.me/problems/no/705

use larsch_simple::larsch;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        x: [i64; n],
        y: [i64; n],
    }
    let cube = |x: i64| x.abs().pow(3);
    let w = |j, i| cube(a[i - 1] - x[j]) + cube(y[j]);
    let f = |i, j, &x: &i64| x + w(j, i);
    let min = larsch(n, f, 0);
    let res = min[n].0;
    println!("{}", res);
}

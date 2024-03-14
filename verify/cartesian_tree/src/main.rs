// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use cartesian_tree::cartesian_tree;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let p = cartesian_tree(&a);
    println!(
        "{}",
        p.into_iter()
            .enumerate()
            .map(|(i, x)| if x == !0 { i } else { x })
            .join(" ")
    );
}

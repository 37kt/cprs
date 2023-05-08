// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum

use algebraic::{act, algebra, monoid};
use graph::Graph;
use itertools::Itertools;
use modint::ModInt998244353 as Mint;
use proconio::input;
use re_rooting_dp::ReRootingDP;

algebra!(M, (Mint, Mint));
monoid!(M, (0.into(), 0.into()), |&(c1, s1), &(c2, s2)| (
    c1 + c2,
    s1 + s2
));

algebra!(V, Mint);
act!(V, (Mint, Mint), |&v, &(c, s)| (c + 1, s + v));

algebra!(E, (Mint, Mint));
act!(E, (Mint, Mint), |&(a, b), &(c, s)| (c, a * s + b * c));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Mint; n],
    }
    let mut g = Graph::from(a);
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
            w: (Mint, Mint),
        }
        g.add_undirected_edge(u, v, w);
    }
    let dp = ReRootingDP::build::<M, V, E>(&g);
    println!("{}", (0..n).map(|v| dp.prod(v).1).join(" "));
}

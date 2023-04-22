// verification-helper: PROBLEM https://yukicoder.me/problems/no/1333

use ac_library::ModInt1000000007 as M;
use algebraic::{act, algebra, monoid};
use graph::Graph;
use proconio::{input, marker::Usize1};
use re_rooting_dp::ReRootingDP;

algebra!(DP, (M, M, M));
monoid!(
    DP,
    (0.into(), 0.into(), 0.into()),
    |&(s1, sq1, c1), &(s2, sq2, c2)| (s1 + s2, sq1 + sq2, c1 + c2)
);

algebra!(V, ());
act!(V, (M, M, M), |_, &(s, sq, c)| (s, sq, c + 1));

algebra!(E, M);
act!(E, (M, M, M), |&w, &(s, sq, c)| (
    s + c * w,
    sq + s * w * 2 + w * w * c,
    c
));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut g = Graph::<(), M>::new(n);
    for _ in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
            w: M,
        }
        g.add_undirected_edge(u, v, w);
    }
    let dp = ReRootingDP::build::<DP, V, E>(&g);
    let mut res = M::new(0);
    for i in 0..n {
        res += dp.prod(i).1;
    }
    res /= 2;
    println!("{}", res);
}

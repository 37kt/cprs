// verification-helper: PROBLEM https://yukicoder.me/problems/no/1333

use ac_library::ModInt1000000007 as M;
use graph::Graph;
use proconio::{input, marker::Usize1};
use re_rooting_dp::ReRootingDP;

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
    let dp = ReRootingDP::build(
        &g,
        (M::new(0), M::new(0), M::new(0)),
        |&(s1, sq1, c1), &(s2, sq2, c2)| (s1 + s2, sq1 + sq2, c1 + c2),
        |&(s, sq, c), _| (s, sq, c + 1),
        |&(s, sq, c), w| (s + c * w, sq + s * w * 2 + w * w * c, c),
    );
    let mut res = M::new(0);
    for i in 0..n {
        res += dp.prod(i).1;
    }
    res /= 2;
    println!("{}", res);
}

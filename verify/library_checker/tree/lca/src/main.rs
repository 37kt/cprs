// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut p: [usize; n - 1],
    }
    p.insert(0, !0);

    let hld = HeavyLightDecomposition::from_parents(&p);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        let lca = hld.lca(u, v);
        println!("{}", lca);
    }
}

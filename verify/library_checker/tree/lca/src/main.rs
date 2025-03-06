// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n - 1],
    }
    let es = p.into_iter().zip(1..).collect::<Vec<_>>();
    let hld = HeavyLightDecomposition::new(&es, 0);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        let lca = hld.lca(u, v);
        println!("{}", lca);
    }
}

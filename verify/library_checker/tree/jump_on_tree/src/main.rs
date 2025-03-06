// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
    }
    let hld = HeavyLightDecomposition::new(&ab, 0);
    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
            d: usize,
        }
        if let Some(v) = hld.jump(s, t, d) {
            println!("{}", v);
        } else {
            println!("-1");
        }
    }
}

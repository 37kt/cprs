// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use graph::Graph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut g = Graph::<(), ()>::new(n);
    for _ in 1..n {
        input! {
            a: usize,
            b: usize,
        }
        g.add_undirected_edge(a, b, ());
    }
    let hld = HeavyLightDecomposition::new(&g);
    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
            i: usize,
        }
        let v = hld.jump(s, t, i);
        println!("{}", v as i64);
    }
}

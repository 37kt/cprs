// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use graph::Graph;
use itertools::Itertools;
use proconio::input;
use strongly_connected_components::strongly_connected_components;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut g = Graph::<(), ()>::new(n);
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        g.add_edge(a, b, ());
    }
    let h = strongly_connected_components(&g);
    println!("{}", h.size());
    for i in 0..h.size() {
        let vs = h.vertex(i);
        println!("{} {}", vs.len(), vs.iter().join(" "));
    }
}

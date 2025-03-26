// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use extended_block_cut_tree::extended_block_cut_tree;
use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let g = UndirectedGraph::from_edges(n, &ab);
    let ebct = extended_block_cut_tree(&g);
    println!("{}", ebct.len() - n);
    for g in ebct.iter().skip(n) {
        print!("{}", g.len());
        for v in g {
            print!(" {}", v);
        }
        println!();
    }
}

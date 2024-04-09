// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use extended_block_cut_tree::extended_block_cut_tree;
use graph::Graph;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let g = Graph::from_unweighted_undirected_edges(n, &ab);
    let bct = extended_block_cut_tree(&g);
    println!("{}", bct.len() - n);
    for i in n..bct.len() {
        println!(
            "{} {}",
            bct[i].len(),
            bct[i].iter().map(|&(i, _)| i).join(" ")
        );
    }
}

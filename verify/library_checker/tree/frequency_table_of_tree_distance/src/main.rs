// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance

use centroid_decomposition::CentroidDecomposition;
use convolution::convolution_mod_2_64;
use graph::{GraphBuilder, UndirectedGraph};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_edges(n, ab);
    let mut res = vec![0; n];
    let mut depth = vec![0; n];
    CentroidDecomposition::solve(&g, |tr| {
        depth[tr.root] = 0;
        let mut f = vec![vec![]; 2];
        for c in 0..2 {
            for (&v, &p) in tr.vs[c].iter().zip(tr.par[c].iter()) {
                depth[v] = depth[p] + 1;
                if f[c].len() <= depth[v] {
                    f[c].resize(depth[v] + 1, 0);
                }
                f[c][depth[v]] += 1;
            }
        }
        let g = convolution_mod_2_64(&f[0], &f[1]);
        for i in 0..g.len() {
            res[i] += g[i];
        }
    });
    res[1] += (n - 1) as u64;
    for x in &res[1..] {
        print!("{} ", x);
    }
    println!();
}

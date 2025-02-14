// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance

use centroid_decomposition::centroid_decomposition;
use convolution_u64::convolution_u64;
use graph::UndirectedGraph;
use itertools::Itertools;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_unweighted_edges(n, &uv);
    let mut res = vec![0; n];
    let mut dist = vec![0; n];
    let mut f = |idx: &[usize], par: &[usize], m: usize| {
        let n = idx.len();
        let r = par[0];
        dist[r] = 0;
        let mut mx = 0;
        for i in 0..n {
            let v = idx[i];
            let p = par[i];
            dist[v] = dist[p] + 1;
            mx = mx.max(dist[v]);
        }
        let mut a = vec![0; mx + 1];
        let mut b = vec![0; mx + 1];
        for i in 0..m {
            let v = idx[i];
            a[dist[v]] += 1;
        }
        for i in m..n {
            let v = idx[i];
            b[dist[v]] += 1;
        }
        let c = convolution_u64(&a, &b);
        for i in 0..c.len().min(res.len()) {
            res[i] += c[i];
        }
    };
    centroid_decomposition(&g, &mut f);
    res[1] += n as u64 - 1;
    println!("{}", res[1..].iter().join(" "));
}

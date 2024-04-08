// verification-helper: PROBLEM https://yukicoder.me/problems/no/1014

use chminmax::{chmax, chmin};
use graph::Graph;
use proconio::input;
use range_edge_graph::RangeEdgeGraph;

const INF: i64 = 1 << 60;

fn dfs(v: usize, g: &Graph<(), i64>, res: &mut [Option<i64>], b: &[i64]) -> i64 {
    if let Some(res) = res[v] {
        return res;
    }
    res[v] = Some(INF);
    let w = if v < b.len() { b[v] } else { 0 };
    let mut r = w;
    for &(u, _) in &g[v] {
        chmax!(r, dfs(u, g, res, b) + w);
        chmin!(r, INF);
    }
    res[v] = Some(r);
    r
}

fn main() {
    input! {
        n: usize,
        abc: [(i64, i64, i64); n],
    }
    let mut ord: Vec<usize> = (0..n).collect();
    ord.sort_by_key(|&i| abc[i].0);
    let mut rev = vec![0; n];
    let mut bs = vec![0; n];
    for i in 0..n {
        rev[ord[i]] = i;
        bs[i] = abc[ord[i]].1;
    }
    let mut g = RangeEdgeGraph::new(n);
    for i in 0..n {
        let (_, b, c) = abc[ord[i]];
        let j = ord.partition_point(|&i| abc[i].0 <= b - c);
        if j <= i {
            g.add_edge(i..=i, 0..j, 0);
        } else {
            g.add_edge(i..=i, 0..i, 0);
            g.add_edge(i..=i, i + 1..j, 0);
        }
    }
    let h = g.build();
    let mut res = vec![None; h.len()];
    for i in 0..n {
        let r = dfs(rev[i], &h, &mut res, &bs);
        if r >= INF {
            println!("BAN");
        } else {
            println!("{r}");
        }
    }
}

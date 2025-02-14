// verification-helper: PROBLEM https://judge.yosupo.jp/problem/assignment

use itertools::Itertools;
use min_cost_b_flow::MinCostBFlow;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut mcf = MinCostBFlow::new();
    let a = mcf.add_vertices(n);
    let b = mcf.add_vertices(n);
    let mut es = vec![vec![0; n]; n];
    for &v in &a {
        mcf.add_supply(v, 1);
    }
    for &v in &b {
        mcf.add_demand(v, 1);
    }
    for i in 0..n {
        for j in 0..n {
            input! {
                cost: i64,
            }
            es[i][j] = mcf.add_edge(a[i], b[j], 0, 1, cost);
        }
    }
    let value = mcf.min_cost_b_flow().unwrap();
    println!("{}", value);
    let mut res = vec![];
    for i in 0..n {
        for j in 0..n {
            if mcf.get_edge(es[i][j]).flow > 0 {
                res.push(j);
            }
        }
    }
    println!("{}", res.iter().join(" "));
}

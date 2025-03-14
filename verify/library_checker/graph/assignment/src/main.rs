// verification-helper: PROBLEM https://judge.yosupo.jp/problem/assignment

use min_cost_b_flow::MinCostBFlow;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut mcf = MinCostBFlow::new();
    let a = mcf.add_vertices(n).collect::<Vec<_>>();
    let b = mcf.add_vertices(n).collect::<Vec<_>>();
    for &v in &a {
        mcf.add_supply(v, 1);
    }
    for &v in &b {
        mcf.add_demand(v, 1);
    }

    let es = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    input! {
                        cost: i64,
                    }
                    mcf.add_edge(a[i], b[j], 0, 1, cost)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res = mcf.min_cost_b_flow().unwrap();
    println!("{}", res);

    for i in 0..n {
        for j in 0..n {
            if mcf.edge(es[i][j]).flow > 0 {
                print!("{} ", j);
            }
        }
    }
    println!();
}

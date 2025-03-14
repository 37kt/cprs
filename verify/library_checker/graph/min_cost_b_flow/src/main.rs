// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow

use min_cost_b_flow::MinCostBFlow;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut mcf = MinCostBFlow::new();
    let vs = mcf.add_vertices(n).collect::<Vec<_>>();

    for &v in &vs {
        input! {
            b: i64,
        }
        mcf.add_supply(v, b);
    }

    let es = (0..m)
        .map(|_| {
            input! {
                s: usize,
                t: usize,
                lower: i64,
                upper: i64,
                cost: i64,
            }
            mcf.add_edge(vs[s], vs[t], lower, upper, cost)
        })
        .collect::<Vec<_>>();

    if let Ok(z) = mcf.min_cost_b_flow() {
        println!("{}", z);

        let potentials = mcf.potentials();
        for v in vs {
            println!("{}", potentials[v]);
        }

        let edges = mcf.edges().collect::<Vec<_>>();
        for e in es {
            println!("{}", edges[e].flow);
        }
    } else {
        println!("infeasible");
    }
}

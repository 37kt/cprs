// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_cost_b_flow

use itertools::Itertools;
use min_cost_b_flow::MinCostBFlow;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut mcf = MinCostBFlow::new();
    let vs = mcf.add_vertices(n);
    for &v in &vs {
        input! {
            b: i64,
        }
        mcf.add_supply(v, b);
    }
    let mut es = vec![];
    for _ in 0..m {
        input! {
            s: usize,
            t: usize,
            lower: i64,
            upper: i64,
            cost: i64,
        }
        es.push(mcf.add_edge(s, t, lower, upper, cost));
    }
    if let Ok(_) = mcf.min_cost_b_flow() {
        let z = mcf.get_result_value_i128();
        let f = mcf.get_edges().iter().map(|e| e.flow).collect::<Vec<_>>();
        let p = mcf.get_potential();
        println!("{}", z);
        if p.len() > 0 {
            println!("{}", p.iter().join("\n"));
        }
        if f.len() > 0 {
            println!("{}", f.iter().join("\n"));
        }
    } else {
        println!("infeasible");
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root

use dynamic_tree_dp::DynamicTreeDP;
use dynamic_tree_dp::DynamicTreeDPOperator;
use graph::UndirectedGraph;
use modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;

#[derive(Clone, Copy)]
struct S {
    a: Mint,
    b: Mint,
    cnt: Mint,
    sum: Mint,
}

enum Op {}
impl DynamicTreeDPOperator for Op {
    type V = Mint;
    type E = (Mint, Mint);
    type X = S;

    fn e() -> Self::X {
        S {
            a: 1.into(),
            b: 0.into(),
            cnt: 0.into(),
            sum: 0.into(),
        }
    }

    fn single(&v: &Self::V, e: Option<&Self::E>) -> Self::X {
        let &(a, b) = e.unwrap_or(&(1.into(), 0.into()));
        S {
            a,
            b,
            cnt: 1.into(),
            sum: a * v + b,
        }
    }

    fn rake(&l: &Self::X, &r: &Self::X) -> Self::X {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + r.sum,
        }
    }

    fn compress(&p: &Self::X, &c: &Self::X) -> Self::X {
        S {
            a: p.a * c.a,
            b: p.a * c.b + p.b,
            cnt: p.cnt + c.cnt,
            sum: p.sum + p.a * c.sum + p.b * c.cnt,
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
        uvbc: [(usize, usize, (Mint, Mint)); n - 1],
    }
    let g = UndirectedGraph::from_vertices_and_edges(&a, &uvbc);
    let mut dp = DynamicTreeDP::<Op>::new(&g, 0);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                w: usize,
                x: Mint,
            }
            dp.set_vertex(w, x);
        } else {
            input! {
                e: usize,
                y: Mint,
                z: Mint,
            }
            let (u, v, _) = uvbc[e];
            dp.set_edge(u, v, (y, z));
        }
        let S { sum, .. } = dp.prod();
        println!("{}", sum);
    }
}

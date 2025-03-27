// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root

use dynamic_tree_dp::{DynamicTreeDp, DynamicTreeDpOperator};
use proconio::{fastout, input};
use static_modint::ModInt998244353 as Mint;

#[derive(Clone, Copy)]
struct S {
    a: Mint,
    b: Mint,
    cnt: Mint,
    sum: Mint,
}

enum Op {}
impl DynamicTreeDpOperator for Op {
    type Value = S;
    type Vertex = Mint;
    type Edge = (Mint, Mint);

    fn unit() -> Self::Value {
        S {
            a: 1.into(),
            b: 0.into(),
            cnt: 0.into(),
            sum: 0.into(),
        }
    }

    fn add_vertex(x: &Self::Value, v: &Self::Vertex) -> Self::Value {
        S {
            a: x.a,
            b: x.b,
            cnt: x.cnt + 1,
            sum: x.sum + v,
        }
    }

    fn add_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value {
        S {
            a: e.0,
            b: e.1,
            cnt: 1.into(),
            sum: e.0 * x.sum + e.1 * x.cnt,
        }
    }

    fn rake(l: &Self::Value, r: &Self::Value) -> Self::Value {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + r.sum,
        }
    }

    fn compress(p: &Self::Value, c: &Self::Value) -> Self::Value {
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
        edges: [(usize, usize, (Mint, Mint)); n - 1],
    }
    let mut dp = DynamicTreeDp::<Op>::with_vertices(&edges, &a, 0);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        match ty {
            0 => {
                input! {
                    v: usize,
                    x: Mint,
                }
                dp.set_vertex(v, x);
            }
            1 => {
                input! {
                    e: usize,
                    a: Mint,
                    b: Mint,
                }
                let (u, v, _) = edges[e];
                dp.set_edge(u, v, (a, b));
            }
            _ => unreachable!(),
        }

        println!("{}", dp.fold().sum);
    }
}

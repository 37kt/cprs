// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum

use dynamic_rerooting_tree_dp::{DynamicRerootingTreeDp, DynamicRerootingTreeDpOperator};
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
impl DynamicRerootingTreeDpOperator for Op {
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

    fn vertex(v: &Self::Vertex) -> Self::Value {
        S {
            a: 1.into(),
            b: 0.into(),
            cnt: 1.into(),
            sum: *v,
        }
    }

    fn add_up_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value {
        S {
            a: e.0,
            b: e.1,
            cnt: 1.into(),
            sum: e.0 * x.sum + e.1,
        }
    }

    fn add_down_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value {
        S {
            a: e.0,
            b: e.1,
            cnt: 1.into(),
            sum: x.sum,
        }
    }

    fn rake1(l: &Self::Value, r: &Self::Value) -> Self::Value {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + r.sum,
        }
    }

    fn rake2(l: &Self::Value, r: &Self::Value) -> Self::Value {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + l.a * r.sum + l.b * r.cnt,
        }
    }

    fn rake3(p: &Self::Value, c: &Self::Value) -> Self::Value {
        Self::rake1(p, c)
    }

    fn compress1(p: &Self::Value, c: &Self::Value) -> Self::Value {
        S {
            a: p.a * c.a,
            b: p.a * c.b + p.b,
            cnt: p.cnt + c.cnt,
            sum: p.sum + p.a * c.sum + p.b * c.cnt,
        }
    }

    fn compress2(p: &Self::Value, c: &Self::Value) -> Self::Value {
        Self::compress1(c, p)
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
    let mut dp = DynamicRerootingTreeDp::<Op>::with_vertices(&edges, &a, 0);
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

        input! {
            r: usize,
        }
        println!("{}", dp.fold(r).sum);
    }
}

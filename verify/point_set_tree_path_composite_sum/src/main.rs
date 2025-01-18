// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum

use dynamic_rerooting_tree_dp::DynamicRerootingTreeDP;
use dynamic_rerooting_tree_dp::DynamicRerootingTreeDPOperator;
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
impl DynamicRerootingTreeDPOperator for Op {
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

    fn single(&v: &Self::V, e: Option<&Self::E>) -> (Self::X, Self::X) {
        let &(a, b) = e.unwrap_or(&(1.into(), 0.into()));
        (
            S {
                a,
                b,
                cnt: 1.into(),
                sum: a * v + b,
            },
            S {
                a,
                b,
                cnt: 1.into(),
                sum: v,
            },
        )
    }

    fn rake(&l: &Self::X, &r: &Self::X) -> Self::X {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + r.sum,
        }
    }

    fn rake2(&l: &Self::X, &r: &Self::X) -> Self::X {
        S {
            a: l.a,
            b: l.b,
            cnt: l.cnt + r.cnt,
            sum: l.sum + l.a * r.sum + l.b * r.cnt,
        }
    }

    fn rake3(p: &Self::X, c: &Self::X) -> Self::X {
        Self::rake(p, c)
    }

    fn compress(&p: &Self::X, &c: &Self::X) -> Self::X {
        S {
            a: p.a * c.a,
            b: p.a * c.b + p.b,
            cnt: p.cnt + c.cnt,
            sum: p.sum + p.a * c.sum + p.b * c.cnt,
        }
    }

    fn compress2(p: &Self::X, c: &Self::X) -> Self::X {
        Self::compress(c, p)
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
    let mut dp = DynamicRerootingTreeDP::<Op>::new(&g, 0);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                v: usize,
                x: Mint,
            }
            dp.set_vertex(v, x);
        } else {
            input! {
                e: usize,
                a: Mint,
                b: Mint,
            }
            let (u, v, _) = uvbc[e];
            dp.set_edge(u, v, (a, b));
        }

        input! {
            r: usize,
        }
        let S { sum, .. } = dp.prod(r);
        println!("{}", sum);
    }
}

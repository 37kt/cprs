// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_tree_path_composite_sum_fixed_root

use graph::UndirectedGraph;
use modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;
use static_top_tree_dp::{StaticTopTreeDP, TreeDPOperator};

enum Op {}
impl TreeDPOperator for Op {
    type Path = (Mint, Mint, Mint, Mint);
    type Point = (Mint, Mint);
    type V = Mint;
    type E = (Mint, Mint);

    fn vertex(&v: &Self::V) -> Self::Path {
        (v, 1.into(), 1.into(), 0.into())
    }

    fn compress(p: &Self::Path, c: &Self::Path, e: &Self::E) -> Self::Path {
        let &(ps, pn, pa, pb) = p;
        let &(cs, cn, ca, cb) = c;
        let &(a, b) = e;
        let (cs, cn, ca, cb) = (cs * a + cn * b, cn, ca * a, cb * a + b);
        (ps + cs * pa + cn * pb, pn + cn, pa * ca, pa * cb + pb)
    }

    fn rake(l: &Self::Point, r: &Self::Point) -> Self::Point {
        let &(ls, ln) = l;
        let &(rs, rn) = r;
        (ls + rs, ln + rn)
    }

    fn add_edge(d: &Self::Path, e: &Self::E) -> Self::Point {
        let &(s, n, _, _) = d;
        let &(a, b) = e;
        (s * a + n * b, n)
    }

    fn add_vertex(d: &Self::Point, v: &Self::V) -> Self::Path {
        let &(s, n) = d;
        (s + v, n + 1, 1.into(), 0.into())
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
    let mut dp = StaticTopTreeDP::<Op>::new(&g);
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
            dp.set_edge(e, (y, z));
        }
        println!("{}", dp.prod().0);
    }
}

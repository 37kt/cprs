// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_path_sum

use algebraic::Monoid;
use algebraic::{algebra, monoid};
use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use proconio::fastout;
use proconio::input;
use segment_tree::SegmentTree;

algebra!(M, i64);
monoid!(M, 0, |x, y| x + y);

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        uv: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&a, &uv);
    let hld = HeavyLightDecomposition::new(&g, 0);
    let mut seg = SegmentTree::<M>::new(n);
    for i in 0..n {
        seg.set(hld.vertex_index(i), a[i]);
    }
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                x: i64,
            }
            let i = hld.vertex_index(p);
            let y = seg.get(i) + x;
            seg.set(i, y);
        } else {
            input! {
                u: usize,
                v: usize,
            }
            let mut s = M::e();
            hld.path_query(
                u,
                v,
                |l, r, _| {
                    s = M::op(&s, &seg.prod(l..r));
                },
                true,
            );
            println!("{}", s);
        }
    }
}

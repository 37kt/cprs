// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use algebraic::{algebra, monoid, Monoid, ReversedMonoid};
use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;
use modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;
use segment_tree::SegmentTree;

algebra!(M, (Mint, Mint));
monoid!(M, (1.into(), 0.into()), |&(a, b), &(c, d)| (
    a * c,
    b * c + d
));

type RM = ReversedMonoid<M>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(Mint, Mint); n],
        uv: [(usize, usize); n - 1],
    }
    let g = UndirectedGraph::from_vertices_and_unweighted_edges(&a, &uv);
    let mut seg = SegmentTree::<M>::new(n);
    let mut rev_seg = SegmentTree::<RM>::new(n);
    let hld = HeavyLightDecomposition::new(&g, 0);
    for i in 0..n {
        seg.set(hld.vertex_index(i), a[i]);
        rev_seg.set(hld.vertex_index(i), a[i]);
    }
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                p: usize,
                c: Mint,
                d: Mint,
            }
            seg.set(hld.vertex_index(p), (c, d));
            rev_seg.set(hld.vertex_index(p), (c, d));
        } else {
            input! {
                u: usize,
                v: usize,
                x: Mint,
            }
            let mut ab = M::e();
            hld.path_query(
                u,
                v,
                |l, r, rev| {
                    if rev {
                        ab = M::op(&ab, &rev_seg.prod(l..r));
                    } else {
                        ab = M::op(&ab, &seg.prod(l..r));
                    }
                },
                true,
            );
            let (a, b) = ab;
            println!("{}", a * x + b);
        }
    }
}

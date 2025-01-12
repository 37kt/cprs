// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use algebraic::{algebra, monoid};
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
        mut p: [usize; n - 1],
    }
    p.insert(0, !0);
    let hld = HeavyLightDecomposition::from_parents(&p);
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
                v: usize,
                x: i64,
            }
            let i = hld.vertex_index(v);
            let y = seg.get(i) + x;
            seg.set(i, y);
        } else {
            input! {
                v: usize,
            }
            let (l, r) = hld.subtree_range(v);
            let s = seg.prod(l..r);
            println!("{}", s);
        }
    }
}

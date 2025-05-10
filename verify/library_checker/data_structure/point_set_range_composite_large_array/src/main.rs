// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite_large_array

use algebraic_structure::magma::{Affine, AffineOperator};
use proconio::fastout;
use proconio::input;
use sparse_segment_tree::SparseSegmentTree;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut seg = SparseSegmentTree::<AffineOperator<Mint>>::new(n);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                c: Mint,
                d: Mint,
            }
            seg.set(p, Affine(c, d));
        } else {
            input! {
                l: usize,
                r: usize,
                x: Mint,
            }
            let f = seg.fold(l..r);
            let y = f.eval(x);
            println!("{}", y);
        }
    }
}

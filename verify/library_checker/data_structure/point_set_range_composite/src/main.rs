// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use algebraic_structure::magma::{Affine, AffineOperator};
use proconio::fastout;
use proconio::input;
use segment_tree::SegmentTree;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Mint, Mint); n],
    }
    let mut seg: SegmentTree<AffineOperator<Mint>> =
        ab.into_iter().map(|(a, b)| Affine(a, b)).collect();
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
            println!("{}", seg.fold(l..r).eval(x));
        }
    }
}

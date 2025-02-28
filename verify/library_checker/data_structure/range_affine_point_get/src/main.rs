// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use algebraic_structure::magma::{Affine, AffineOperator};
use dual_segment_tree::DualSegmentTree;
use proconio::fastout;
use proconio::input;
use static_modint::ModInt998244353 as Mint;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }
    let mut seg: DualSegmentTree<AffineOperator<Mint>> =
        a.into_iter().map(|x| Affine(0.into(), x)).collect();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                l: usize,
                r: usize,
                a: Mint,
                b: Mint,
            }
            seg.apply_range(l..r, Affine(a, b));
        } else {
            input! {
                i: usize,
            }
            println!("{}", seg.get(i).1);
        }
    }
}

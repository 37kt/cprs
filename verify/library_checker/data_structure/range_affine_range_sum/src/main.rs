// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use algebraic_structure::act::CountsumAffineOperator;
use algebraic_structure::magma::{Affine, CountSum};
use lazy_segment_tree::LazySegmentTree;
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
    let mut seg: LazySegmentTree<CountsumAffineOperator<Mint>> = a
        .into_iter()
        .map(|x| CountSum {
            count: 1.into(),
            sum: x,
        })
        .collect();
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
                l: usize,
                r: usize,
            }
            println!("{}", seg.fold(l..r).sum);
        }
    }
}

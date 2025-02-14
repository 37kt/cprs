// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use algebraic_structure::{AddOperator, Affine, CountSum, CountsumAffineOperator, MulOperator};

use algebraic_traits::define_algebra;
use algebraic_traits::Semiring;
use lazy_segment_tree::LazySegmentTree;
use modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;

define_algebra! {
    name: SR,
    element: Mint,
}
impl Semiring for SR {
    type Additive = AddOperator<Mint>;
    type Multiplicative = MulOperator<Mint>;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }

    let a: Vec<_> = a
        .into_iter()
        .map(|x| CountSum {
            count: 1.into(),
            sum: x,
        })
        .collect();
    let mut seg = LazySegmentTree::<CountsumAffineOperator<SR>>::from(a);
    for _ in 0..q {
        input! {
            ty: usize,
            l: usize,
            r: usize,
        }
        if ty == 0 {
            input! {
                b: Mint,
                c: Mint,
            }
            seg.apply_range(l..r, Affine(b, c));
        } else {
            println!("{}", seg.prod(l..r).sum);
        }
    }
}

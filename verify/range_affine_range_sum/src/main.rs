// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use algebra::{define_algebra, Affine, CntSum, Semiring};
use lazy_segment_tree::LazySegmentTree;
use modint::ModInt998244353 as Mint;
use proconio::fastout;
use proconio::input;

define_algebra! {
    name: A,
    element: Mint,
    op: |x, y| x + y,
    unit: 0.into(),
    associative,
    commutative,
}

define_algebra! {
    name: M,
    element: Mint,
    op: |x, y| x * y,
    unit: 1.into(),
    associative,
    commutative,
}

define_algebra! {
    name: SR,
    element: Mint,
}
impl Semiring for SR {
    type Additive = A;
    type Multiplicative = M;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }
    let a: Vec<_> = a.into_iter().map(|x| (1.into(), x)).collect();
    let mut seg = LazySegmentTree::<CntSum<SR>, Affine<SR>>::from(a);
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
            seg.apply_range(l..r, (b, c));
        } else {
            println!("{}", seg.prod(l..r).1);
        }
    }
}

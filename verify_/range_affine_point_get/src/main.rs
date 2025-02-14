// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get

use algebraic::{algebra, monoid};
use dual_segment_tree::DualSegmentTree;
use modint::ModInt998244353 as Mint;
use proconio::input;

algebra!(F, (Mint, Mint));
monoid!(F, (1.into(), 0.into()), |&(a, b), &(c, d)| (
    a * c,
    a * d + b
));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }
    let mut seg = DualSegmentTree::<F>::new(n);
    for i in 0..n {
        seg.apply(i, (0.into(), a[i]));
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                l: usize,
                r: usize,
                b: Mint,
                c: Mint,
            }
            seg.apply_range(l..r, (b, c));
        } else {
            input! {
                i: usize,
            }
            println!("{}", seg.get(i).1);
        }
    }
}

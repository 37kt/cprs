// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum

use ac_library::ModInt998244353 as Mint;
use algebraic::{act, algebra, monoid};
use lazy_segment_tree::LazySegmentTree;
use proconio::input;

algebra!(M, (Mint, Mint));
monoid!(M, (0.into(), 0.into()), |&(s1, c1), &(s2, c2)| (
    s1 + s2,
    c1 + c2
));

algebra!(F, (Mint, Mint));
monoid!(F, (1.into(), 0.into()), |&(a, b), &(c, d)| (
    a * c,
    b * c + d
));
act!(F, (Mint, Mint), |&(a, b), &(s, c)| (a * s + b * c, c));

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Mint; n],
    }
    let a: Vec<_> = a.into_iter().map(|x| (x, 1.into())).collect();
    let mut seg = LazySegmentTree::<M, F>::from(a);
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
            println!("{}", seg.prod(l..r).0);
        }
    }
}

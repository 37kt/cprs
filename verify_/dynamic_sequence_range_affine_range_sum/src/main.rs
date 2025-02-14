// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum

use ac_library::ModInt998244353 as Mint;
use algebraic::{act, algebra, monoid};
use proconio::input;
use splay_tree::SplayTree;

algebra!(M, (Mint, Mint));
monoid!(M, (0.into(), 0.into()), |&(s1, c1), &(s2, c2)| (
    s1 + s2,
    c1 + c2
));

algebra!(F, (Mint, Mint));
act!(F, (Mint, Mint), |&(a, b), &(s, c)| (a * s + b * c, c));
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
    let a = a
        .into_iter()
        .map(|x| (x, 1.into()))
        .collect::<Vec<(_, _)>>();
    let mut sp = SplayTree::<M, F>::from(&a[..]);
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                i: usize,
                x: Mint,
            }
            sp.insert(i, (x, 1.into()));
        } else if ty == 1 {
            input! {
                i: usize,
            }
            sp.remove(i);
        } else if ty == 2 {
            input! {
                l: usize,
                r: usize,
            }
            sp.reverse(l..r);
        } else if ty == 3 {
            input! {
                l: usize,
                r: usize,
                b: Mint,
                c: Mint,
            }
            sp.apply(l..r, (b, c));
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let (res, _) = sp.prod(l..r);
            println!("{}", res);
        }
    }
}

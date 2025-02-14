// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_reverse_range_sum

use algebraic::{act, algebra, monoid};
use proconio::input;
use splay_tree::SplayTree;

algebra!(M, i64);
monoid!(M, 0, |&s1, &s2| s1 + s2);

algebra!(F, ());
act!(F, i64, |_, &s| s);
monoid!(F, (), |_, _| ());

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut sp = SplayTree::<M, F>::from(&a[..]);
    for _ in 0..q {
        input! {
            ty: usize,
            l: usize,
            r: usize,
        }
        if ty == 0 {
            sp.reverse(l..r);
        } else {
            println!("{}", sp.prod(l..r));
        }
    }
}

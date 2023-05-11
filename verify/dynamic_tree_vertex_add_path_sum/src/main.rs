// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_tree_vertex_add_path_sum

use algebraic::{act, algebra, monoid};
use link_cut_tree::LinkCutTree;
use proconio::input;

algebra!(M, i64);
monoid!(M, 0, |&a, &b| a + b);

algebra!(F, i64);
act!(F, i64, |&a, &b| a + b);
monoid!(F, 0, |&a, &b| a + b);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut lct = LinkCutTree::<M, F>::from(&a[..]);
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize,
        }
        lct.link(u, v);
    }
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                u: usize,
                v: usize,
                w: usize,
                x: usize,
            }
            lct.cut(u, v);
            lct.link(w, x);
        } else if ty == 1 {
            input! {
                p: usize,
                x: i64,
            }
            lct.apply(p, p, x);
        } else {
            input! {
                u: usize,
                v: usize,
            }
            println!("{}", lct.prod(u, v));
        }
    }
}

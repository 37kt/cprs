// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_tree_vertex_set_path_composite

use ac_library::ModInt998244353 as Mint;
use algebraic::{act, algebra, monoid};
use link_cut_tree::LinkCutTree;
use proconio::input;

algebra!(M, (Mint, Mint));
monoid!(M, (1.into(), 0.into()), |&(a, b), &(c, d)| (
    a * c,
    a * d + b
));

algebra!(F, ());
act!(F, (Mint, Mint), |_, &a| a);
monoid!(F, (), |_, _| ());

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [(Mint, Mint); n],
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
                c: (Mint, Mint),
            }
            lct.set(p, c);
        } else {
            input! {
                u: usize,
                v: usize,
                x: Mint,
            }
            let (a, b) = lct.prod(v, u);
            println!("{}", a * x + b);
        }
    }
}

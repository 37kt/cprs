// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum

use proconio::input;
use segtree_beats::SegtreeBeats;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut seg = SegtreeBeats::from(&a[..]);
    for _ in 0..q {
        input! {
            t: usize,
            l: usize,
            r: usize,
        }
        if t == 0 {
            input! {
                b: i64,
            }
            seg.chmin(l..r, b);
        } else if t == 1 {
            input! {
                b: i64,
            }
            seg.chmax(l..r, b);
        } else if t == 2 {
            input! {
                b: i64,
            }
            seg.add(l..r, b);
        } else {
            let s = seg.sum(l..r);
            println!("{}", s);
        }
    }
}

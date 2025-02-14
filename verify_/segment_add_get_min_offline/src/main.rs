// verification-helper: PROBLEM https://judge.yosupo.jp/problem/segment_add_get_min

use li_chao_tree::LiChaoTree;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut xs = vec![];
    let mut qs = vec![];
    for _ in 0..n {
        input! {
            l: i64,
            r: i64,
            a: i64,
            b: i64,
        }
        qs.push((0, l, r, a, b));
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                l: i64,
                r: i64,
                a: i64,
                b: i64,
            }
            qs.push((0, l, r, a, b));
        } else {
            input! {
                p: i64,
            }
            xs.push(p);
            qs.push((1, p, 0, 0, 0));
        }
    }
    let mut tr = LiChaoTree::new(xs);
    for (t, l, r, a, b) in qs {
        if t == 0 {
            tr.add_segment(l..r, (a, b));
        } else if let Some(y) = tr.min(l) {
            println!("{}", y);
        } else {
            println!("INFINITY");
        }
    }
}

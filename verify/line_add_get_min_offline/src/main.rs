// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min

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
            a: i64,
            b: i64,
        }
        qs.push((0, a, b));
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                a: i64,
                b: i64,
            }
            qs.push((0, a, b));
        } else {
            input! {
                p: i64,
            }
            xs.push(p);
            qs.push((1, p, 0));
        }
    }
    let mut tr = LiChaoTree::new(xs);
    for (t, a, b) in qs {
        if t == 0 {
            tr.add_line((a, b));
        } else if let Some(y) = tr.min(a) {
            println!("{}", y);
        } else {
            println!("INFINITY");
        }
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/segment_add_get_min

use li_chao_tree_dynamic::LiChaoTree;
use proconio::input;

const MAX: i64 = 1_000_000_000;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lct = LiChaoTree::new(-MAX, MAX, false);
    for _ in 0..n {
        input! {
            l: i64,
            r: i64,
            a: i64,
            b: i64,
        }
        lct.add_segment(a, b, l, r);
    }
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                l: i64,
                r: i64,
                a: i64,
                b: i64,
            }
            lct.add_segment(a, b, l, r);
        } else {
            input! {
                p: i64,
            }
            if let Some(res) = lct.find(p) {
                println!("{}", res);
            } else {
                println!("INFINITY");
            }
        }
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min

use li_chao_tree_dynamic::LiChaoTreeDynamic;
use proconio::input;

const MAX: i64 = 1_000_000_000;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut lct = LiChaoTreeDynamic::new(-MAX, MAX, false);
    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        }
        lct.add_line(a, b);
    }
    for _ in 0..q {
        input! {
            ty: usize,
        }
        if ty == 0 {
            input! {
                a: i64,
                b: i64,
            }
            lct.add_line(a, b);
        } else {
            input! {
                p: i64,
            }
            println!("{}", lct.find(p).unwrap());
        }
    }
}

// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum

use algebraic_structure::magma::AddOperator;
use persistent_segment_tree::PersistentSegmentTree;
use proconio::fastout;
use proconio::input;

struct Add {
    x: usize,
    y: usize,
    w: i64,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut add = vec![];
    for _ in 0..n {
        input! {
            x: usize,
            y: usize,
            w: i64,
        }
        add.push(Add { x, y, w });
    }
    add.sort_by_key(|a| a.x);

    let mut segs = vec![PersistentSegmentTree::<AddOperator<i64>>::new(
        1_000_000_000,
    )];
    for i in 0..n {
        let Add { y, w, .. } = add[i];
        segs.push(segs[i].add(y, w));
    }

    for _ in 0..q {
        input! {
            xl: usize,
            yl: usize,
            xr: usize,
            yr: usize,
        }
        let xl = add.partition_point(|a| a.x < xl);
        let xr = add.partition_point(|a| a.x < xr);
        let res = segs[xr].fold(yl..yr) - segs[xl].fold(yl..yr);
        println!("{}", res);
    }
}

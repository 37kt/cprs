// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum

use algebraic::{algebra, monoid};
use proconio::input;
use range_tree::RangeTree;

algebra!(M, i64);
monoid!(M, 0, |&a, &b| a + b);

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        xyw: [(i64, i64, i64); n],
    }
    let mut xy: Vec<_> = xyw.iter().map(|&(x, y, _)| (x, y)).collect();
    let mut qs = vec![];
    for _ in 0..q {
        input! {
            ty: usize,
            a: [i64; 3 + ty],
        }
        if ty == 0 {
            xy.push((a[0], a[1]));
        }
        qs.push(a);
    }
    let mut rt = RangeTree::<_, M>::new(xy);
    for &(x, y, w) in &xyw {
        rt.add((x, y), w);
    }
    for a in qs {
        if a.len() == 3 {
            rt.add((a[0], a[1]), a[2]);
        } else {
            let res = rt.prod(a[0]..a[2], a[1]..a[3]);
            println!("{}", res);
        }
    }
}

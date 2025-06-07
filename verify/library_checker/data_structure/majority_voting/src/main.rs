// verification-helper: PROBLEM https://judge.yosupo.jp/problem/majority_voting

use algebraic_traits::define_algebra;
use fenwick_tree_01::FenwickTree01;
use proconio::fastout;
use proconio::input;
use segment_tree::SegmentTree;

const C: usize = 1_000_000_010;

define_algebra! {
    name: S,
    value: (u32, u32),
    op: |&(m1, c1): &(u32, u32), &(m2, c2): &(u32, u32)| {
        if m1 == m2 {
            (m1, c1 + c2)
        } else if c1 >= c2 {
            (m1, c1 - c2)
        } else {
            (m2, c2 - c1)
        }
    },
    unit: (0, 0),
    associative,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut queries: [(usize, usize, usize); q],
    }

    let mut z: Vec<usize> = a.iter().enumerate().map(|(i, &x)| x * C + i).collect();
    for &(t, x, y) in &queries {
        if t == 0 {
            z.push(y * C + x);
        }
    }
    z.sort_unstable();
    z.dedup();

    let idx = |x: usize| z.partition_point(|&y| y < x);
    let mut seg = SegmentTree::<S>::from_fn(n, |i| (a[i] as u32, 1));
    let mut ft = FenwickTree01::new(z.len());
    for (i, &x) in a.iter().enumerate() {
        ft.set(idx(x * C + i), 1);
    }
    for &(t, x, y) in &queries {
        if t == 0 {
            let i = idx(a[x] * C + x);
            ft.set(i, 0);
            let i = idx(y * C + x);
            ft.set(i, 1);
            a[x] = y;
            seg.set(x, (y as u32, 1));
        } else {
            let m = seg.fold(x..y).0 as usize;
            let l = idx(m * C + x);
            let r = idx(m * C + y);
            let c = ft.fold(l..r);
            if c * 2 > y - x {
                println!("{}", m);
            } else {
                println!("-1");
            }
        }
    }
}

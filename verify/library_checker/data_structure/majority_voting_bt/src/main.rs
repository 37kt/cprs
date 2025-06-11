// verification-helper: PROBLEM https://judge.yosupo.jp/problem/majority_voting

use algebraic_traits::define_algebra;
use binary_trie::BinaryTrie;
use proconio::fastout;
use proconio::input;
use segment_tree::SegmentTree;

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

    let mut bt = BinaryTrie::new();
    let mut seg = SegmentTree::<S>::from_fn(n, |i| (a[i] as u32, 1));

    for (i, &x) in a.iter().enumerate() {
        bt.insert(x << 20 | i, 1);
    }

    for &(t, x, y) in &queries {
        if t == 0 {
            bt.remove(a[x] << 20 | x, 1);
            a[x] = y;
            bt.insert(a[x] << 20 | x, 1);
            seg.set(x, (a[x] as u32, 1));
        } else {
            let m = seg.fold(x..y).0 as usize;
            let c = bt.count_range(m << 20 | x..m << 20 | y, 0);
            if c * 2 > y - x {
                println!("{}", m);
            } else {
                println!("-1");
            }
        }
    }
}

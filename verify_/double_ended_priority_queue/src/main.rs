// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue

use interval_heap::IntervalHeap;
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        s: [i64; n],
    }
    let mut pq = IntervalHeap::from(s);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                x: i64,
            }
            pq.push(x);
        } else if t == 1 {
            println!("{}", pq.pop_min().unwrap());
        } else {
            println!("{}", pq.pop_max().unwrap());
        }
    }
}

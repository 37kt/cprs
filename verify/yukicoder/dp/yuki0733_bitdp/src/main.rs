// verification-helper: PROBLEM https://yukicoder.me/problems/no/733

use bit_subset::BitSubsetExt;
use macros::chmin;
use proconio::{fastout, input};

const INF: usize = 1 << 60;

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        a: [usize; n],
    }
    let mut b = vec![0; 1 << n];
    for i in 0..n {
        for s in 0..1 << n {
            if s >> i & 1 == 0 {
                b[s | 1 << i] = b[s] + a[i];
            }
        }
    }

    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for s in 0usize..1 << n {
        for t in s.subsets() {
            if b[t] <= m {
                chmin!(dp[s], dp[s ^ t] + 1);
            }
        }
    }

    println!("{}", dp.last().unwrap());
}

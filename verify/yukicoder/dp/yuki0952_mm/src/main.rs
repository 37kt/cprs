// verification-helper: PROBLEM https://yukicoder.me/problems/no/952

use monotone_minima::monotone_minima;
use proconio::{fastout, input};

const INF: i64 = 1 << 60;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }

    let mut dp = vec![INF; n + 2];
    dp[0] = 0;

    // ループ 1 回ごとに開けないドアを 1 つ選ぶ
    let mut res = vec![INF; n + 1];
    for k in 1..=n {
        let f = |i: usize, j: usize| {
            if i > j {
                dp[j] + (b[i - 1] - b[j]).pow(2)
            } else {
                INF
            }
        };
        let select = |i, j, k| f(i, j) > f(i, k);
        let argmin = monotone_minima(n + 2, n + 2, select);
        dp = (0..n + 2).map(|i| f(i, argmin[i])).collect();
        res[n + 1 - k] = dp[n + 1];
    }

    for &x in &res[1..=n] {
        println!("{}", x);
    }
}

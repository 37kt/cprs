use aliens_dp::aliens_dp;
use proconio::{fastout, input};

const INF: i64 = 1 << 60;
const C: i64 = 1 << 40;

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
        a: [i64; n - 1],
    }

    let g = |p: i64| {
        let mut dp = [0, p];
        for &x in &a {
            let mut ndp = [INF; 2];
            #[allow(clippy::needless_range_loop)]
            for i in 0..2 {
                for j in 0..2 {
                    let mut cost = 0;
                    if i != j {
                        cost -= x;
                    }
                    if j == 1 {
                        cost += p;
                    }
                    ndp[j] = ndp[j].min(dp[i] + cost);
                }
            }
            dp = ndp;
        }
        dp[0].min(dp[1])
    };

    let res = aliens_dp(r, -C..=C, g);
    println!("{}", -res);
}

// verification-helper: PROBLEM https://yukicoder.me/problems/no/1269

use modint::ModInt1000000007 as Mint;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};
use trie::Trie;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let to_array = |mut x: usize| {
        let mut res = vec![];
        while x > 0 {
            res.push(x % 10);
            x /= 10;
        }
        res.reverse();
        res
    };

    let mut trie = Trie::<10>::new();
    let mut a0 = 1;
    let mut a1 = 1;
    while a1 <= r {
        if l <= a1 {
            trie.add(&to_array(a1));
        }
        (a0, a1) = (a1, a0 + a1);
    }
    trie.build();

    let mut dp = vec![Mint::new(0); trie.len()];
    dp[0] += 1;
    for _ in 0..n {
        let mut ndp = vec![Mint::new(0); trie.len()];
        for v in 0..trie.len() {
            for c in 0..10 {
                let u = trie.next(v, c);
                if trie.count(u) == 0 {
                    ndp[u] += dp[v];
                }
            }
        }
        dp = ndp;
    }

    let res = dp.iter().sum::<Mint>() - 1;
    println!("{}", res);
}

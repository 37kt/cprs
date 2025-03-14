// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set

use binary_trie::BinaryTrie;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let xor = 0xE869120; // 全体 xor の test
    let mut trie = BinaryTrie::new();
    for &x in &a {
        trie.insert(x ^ xor, 1);
    }
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        match t {
            0 => {
                if trie.count(x ^ xor) == 0 {
                    trie.insert(x ^ xor, 1);
                }
            }
            1 => {
                if trie.count(x ^ xor) == 1 {
                    trie.remove(x ^ xor, 1);
                }
            }
            2 => {
                println!("{}", trie.kth_smallest(x - 1, xor).map_or(-1, |x| x as i64));
            }
            3 => {
                println!("{}", trie.count_range(..=x, xor));
            }
            4 => {
                let cnt = trie.count_range(x + 1.., xor);
                println!("{}", trie.kth_largest(cnt, xor).map_or(-1, |x| x as i64));
            }
            5 => {
                let cnt = trie.count_range(..x, xor);
                println!("{}", trie.kth_smallest(cnt, xor).map_or(-1, |x| x as i64));
            }
            _ => unreachable!(),
        }
    }
}

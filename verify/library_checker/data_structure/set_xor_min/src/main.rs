// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use binary_trie::BinaryTrie;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut trie = BinaryTrie::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        match t {
            0 => {
                if trie.count(x) == 0 {
                    trie.insert(x, 1);
                }
            }
            1 => {
                if trie.count(x) == 1 {
                    trie.remove(x, 1);
                }
            }
            2 => {
                println!("{}", trie.min(x).unwrap());
            }
            _ => unreachable!(),
        }
    }
}

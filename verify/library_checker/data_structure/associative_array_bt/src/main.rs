// verification-helper: PROBLEM https://judge.yosupo.jp/problem/associative_array

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
            k: usize,
        }
        if t == 0 {
            input! {
                v: usize,
            }
            let p = trie.count(k);
            if p < v {
                trie.insert(k, v - p);
            } else {
                trie.remove(k, p - v);
            }
        } else {
            println!("{}", trie.count(k));
        }
    }
}

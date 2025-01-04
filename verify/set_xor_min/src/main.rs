// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use binary_trie::BinaryTrie;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut st = BinaryTrie::<30>::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 0 {
            if st.count(x) == 0 {
                st.insert(x, 1);
            }
        } else if t == 1 {
            if st.count(x) == 1 {
                st.remove(x, 1);
            }
        } else {
            st.operate_xor(x);
            let res = st.min().unwrap();
            st.operate_xor(x);
            println!("{}", res);
        }
    }
}

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
    let mut st = BinaryTrie::<30>::new();
    for &x in &a {
        st.insert(x, 1);
    }
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
        } else if t == 2 {
            let res = st.kth_smallest(.., x - 1).unwrap_or(!0);
            println!("{}", res as i64);
        } else if t == 3 {
            let res = st.upper_bound(x);
            println!("{}", res);
        } else if t == 4 {
            let res = st.kth_largest(..=x, 0).unwrap_or(!0);
            println!("{}", res as i64);
        } else if t == 5 {
            let res = st.kth_smallest(x.., 0).unwrap_or(!0);
            println!("{}", res as i64);
        }
    }
}

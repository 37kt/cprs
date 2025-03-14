// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::fastout;
use proconio::input;
use proconio::marker::Bytes;
use wordsize_ary_tree_set::WordsizeAryTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        t: Bytes,
    }

    let mut st = WordsizeAryTreeSet::new(n);
    for i in 0..n {
        if t[i] == b'1' {
            st.insert(i);
        }
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        match t {
            0 => {
                st.insert(x);
            }
            1 => {
                st.remove(x);
            }
            2 => {
                println!("{}", st[x] as i32);
            }
            3 => {
                println!("{}", st.next(x).map_or(-1, |x| x as isize));
            }
            4 => {
                println!("{}", st.prev(x).map_or(-1, |x| x as isize));
            }
            _ => unreachable!(),
        }
    }
}

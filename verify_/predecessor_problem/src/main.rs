// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::{input, marker::Bytes};
use w_ary_tree_set::WAryTreeSet;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: Bytes,
    }
    let mut tr = WAryTreeSet::new(10_000_000);
    for i in 0..n {
        if t[i] == b'1' {
            tr.insert(i);
        }
    }
    for _ in 0..q {
        input! {
            ty: usize,
            k: usize,
        }
        match ty {
            0 => {
                tr.insert(k);
            }
            1 => {
                tr.remove(k);
            }
            2 => {
                println!("{}", tr.contains(k) as usize);
            }
            3 => {
                if let Some(x) = tr.next(k) {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
            _ => {
                if let Some(x) = tr.prev(k) {
                    println!("{}", x);
                } else {
                    println!("-1");
                }
            }
        }
    }
}

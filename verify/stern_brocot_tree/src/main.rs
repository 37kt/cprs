// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stern_brocot_tree

use proconio::input;
use rational::Rational;
use stern_brocot_tree::SternBrocotTreeNode;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            ty: String,
        }
        match ty.as_str() {
            "ENCODE_PATH" => {
                input! {
                    a: i128,
                    b: i128,
                }
                let sbt = SternBrocotTreeNode::new(Rational::new(a, b));
                let path = sbt.path();
                print!("{}", path.len());
                for x in path {
                    if x < 0 {
                        print!(" L {}", -x);
                    } else {
                        print!(" R {}", x);
                    }
                }
                println!();
            }
            "DECODE_PATH" => {
                input! {
                    k: usize,
                }
                let mut sbt = SternBrocotTreeNode::default();
                for _ in 0..k {
                    input! {
                        c: String,
                        n: i128,
                    }
                    sbt.go_child(if c == "L" { -n } else { n });
                }
                let Rational { num, den } = sbt.val();
                println!("{} {}", num, den);
            }
            "LCA" => {
                input! {
                    a: i128,
                    b: i128,
                    c: i128,
                    d: i128,
                }
                let sbt1 = SternBrocotTreeNode::new(Rational::new(a, b));
                let sbt2 = SternBrocotTreeNode::new(Rational::new(c, d));
                let lca = sbt1.lca(&sbt2);
                let Rational { num, den } = lca.val();
                println!("{} {}", num, den);
            }
            "ANCESTOR" => {
                input! {
                    k: i128,
                    a: i128,
                    b: i128,
                }
                let mut sbt = SternBrocotTreeNode::new(Rational::new(a, b));
                let d = sbt.depth();
                if d < k {
                    println!("-1");
                } else {
                    sbt.go_parent(d - k);
                    let Rational { num, den } = sbt.val();
                    println!("{} {}", num, den);
                }
            }
            "RANGE" => {
                input! {
                    a: i128,
                    b: i128,
                }
                let sbt = SternBrocotTreeNode::new(Rational::new(a, b));
                let Rational { num: f, den: g } = sbt.lower_bound();
                let Rational { num: h, den: k } = sbt.upper_bound();
                println!("{} {} {} {}", f, g, h, k);
            }
            _ => unreachable!(),
        }
    }
}

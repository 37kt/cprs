use multivalued_optimization::MultivaluedOptimization;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    let mut opt = MultivaluedOptimization::new(vec![5; n * n]);
    let id = |i: usize, j: usize| i * n + j;
    let f = |x: usize, y: usize| Some(x.abs_diff(y).pow(2) as i64);
    #[allow(clippy::needless_range_loop)]
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != 0 {
                opt.add_unary(id(i, j), |x| (x == a[i][j] - 1).then_some(0));
            }
            if i + 1 < n {
                opt.add_binary(id(i, j), id(i + 1, j), f);
            }
            if j + 1 < n {
                opt.add_binary(id(i, j), id(i, j + 1), f);
            }
        }
    }

    let (_, res) = opt.solve();
    for i in 0..n {
        for j in 0..n {
            print!("{} ", res[id(i, j)] + 1);
        }
        println!();
    }
}

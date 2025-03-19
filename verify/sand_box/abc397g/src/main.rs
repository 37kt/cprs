use multivalued_optimization::MultivaluedOptimization;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        uv: [(Usize1, Usize1); m],
    }

    let mut opt = MultivaluedOptimization::new(vec![n; n]);
    opt.add_unary(0, |dv| (dv == 0).then_some(0));

    for &(u, v) in &uv {
        opt.add_binary(u, v, |du, dv| {
            if du >= dv {
                Some(0)
            } else if du + 1 == dv {
                Some(1)
            } else {
                None
            }
        });
    }

    let mut ok = 0;
    let mut ng = n;
    while ok + 1 < ng {
        let d = (ok + ng) / 2;
        let mut opt = opt.clone();
        opt.add_unary(n - 1, |dv| (dv == d).then_some(0));
        let (cost, _) = opt.solve();
        if cost <= k {
            ok = d;
        } else {
            ng = d;
        }
    }
    println!("{}", ok);
}

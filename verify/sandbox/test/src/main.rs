use macros::{chmax, mvec, yes};
use proconio::fastout;

#[fastout]
fn main() {
    let mut x = 10;
    chmax!(x, 20);
    println!("{}", x);

    let a = mvec![1; 10; 10];

    println!("{:?}", a);

    yes!(a[0][0] == 1);
}

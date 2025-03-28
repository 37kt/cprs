use proconio::fastout;

#[fastout]
fn main() {
    let mut v = Vec::with_capacity(10);
    for i in 0..100 {
        v.resize(i, i);
        eprintln!("{}", v.capacity());
    }
}

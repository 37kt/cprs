use modint_61::ModInt61;
use proconio::fastout;

#[fastout]
fn main() {
    let timer = std::time::Instant::now();
    let mut rng = random::Pcg64Fast::default();
    const N: usize = 1_000_000_000;
    let x = ModInt61::new(rng.u64());
    let mut s = ModInt61::new(1);
    for _ in 0..N {
        s *= x;
    }
    println!("{}", s);
    println!("{} ms", timer.elapsed().as_millis());
}

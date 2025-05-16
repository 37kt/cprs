use binomial::BinomialPrime;
use dynamic_modint::DefaultDynamicModInt;
use rand::{thread_rng, Rng};

fn main() {
    let t = 1_000_000;
    let m = 998244353;
    type Mint = DefaultDynamicModInt;
    Mint::set_modulus(m);
    let mut comb = BinomialPrime::<Mint>::new();
    let r = 10_000_000;
    let mut rng = thread_rng();
    comb.expand(r);
    for _ in 0..t {
        let n = rng.gen_range(0..r);
        let k = rng.gen_range(0..r);
        println!("{}", comb.nck(n, k));
    }
}

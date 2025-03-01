use crate::Mint;

pub(crate) fn miller_rabin(n: u64, a: &[u64]) -> bool {
    Mint::set_modulus(n);
    let d = (n - 1) >> (n - 1).trailing_zeros();
    let e = Mint::from_raw(1);
    let r = Mint::from_raw(n - 1);
    for &a in a {
        if n <= a {
            break;
        }
        let mut t = d;
        let mut y = Mint::new(a).pow(t as _);
        while t != n - 1 && y != e && y != r {
            y *= y;
            t *= 2;
        }
        if y != r && t % 2 == 0 {
            return false;
        }
    }
    true
}

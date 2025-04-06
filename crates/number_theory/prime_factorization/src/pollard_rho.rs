use numeric_traits::Integer;

use crate::{is_prime, Mint};

pub(crate) fn pollard_rho(n: u64, res: &mut Vec<u64>) {
    if n == 1 {
        return;
    } else if n & 1 == 0 {
        res.push(2);
        pollard_rho(n >> 1, res);
        return;
    } else if is_prime(n) {
        res.push(n);
        return;
    }
    let m = 1 << ((n.floor_log2() + 1) / 8);
    let mut c = Mint::from_raw(1);
    loop {
        let mut x = Mint::from_raw(1);
        let mut y = Mint::from_raw(2);
        let mut ys = Mint::from_raw(1);
        let mut q = Mint::from_raw(1);
        let mut r = 1;
        let mut g = 1;
        while g == 1 {
            x = y;
            for _ in 0..r {
                y = y * y + c;
            }
            for k in (0..r).step_by(m) {
                if g != 1 {
                    break;
                }
                ys = y;
                for _ in 0..m.min(r - k) {
                    y = y * y + c;
                    q *= x - y;
                }
                g = n.gcd(q.val());
            }
            r <<= 1;
        }
        if g == n {
            g = 1;
            while g == 1 {
                ys = ys * ys + c;
                g = n.gcd((x - ys).val());
            }
        }
        if g < n {
            pollard_rho(g, res);
            pollard_rho(n / g, res);
            return;
        }
        c += 1;
    }
}

use crate::mod_arithmetic::{inv_mod, is_prime, mul_mod, pow_mod, primitive_root};

pub struct NTTPrecalc {
    pub is_prime: bool,
    pub rank2: usize,
    pub primitive_root: u32,
    pub root: [u32; 30],
    pub iroot: [u32; 30],
    pub rate2: [u32; 30],
    pub irate2: [u32; 30],
    pub rate3: [u32; 30],
    pub irate3: [u32; 30],
}

impl NTTPrecalc {
    pub const fn new(modulus: u32) -> Self {
        let is_prime = is_prime(modulus);

        if !is_prime {
            return Self {
                is_prime,
                rank2: 0,
                primitive_root: 0,
                root: [0; 30],
                iroot: [0; 30],
                rate2: [0; 30],
                irate2: [0; 30],
                rate3: [0; 30],
                irate3: [0; 30],
            };
        }

        let rank2 = (modulus - 1).trailing_zeros() as usize;
        let primitive_root = primitive_root(modulus);
        let mut root = [0; 30];
        let mut iroot = [0; 30];
        let mut rate2 = [0; 30];
        let mut irate2 = [0; 30];
        let mut rate3 = [0; 30];
        let mut irate3 = [0; 30];

        root[rank2] = pow_mod(primitive_root, ((modulus - 1) >> rank2) as _, modulus);
        iroot[rank2] = inv_mod(root[rank2], modulus);
        let mut i = rank2;
        while i > 0 {
            i -= 1;
            root[i] = mul_mod(root[i + 1], root[i + 1], modulus);
            iroot[i] = mul_mod(iroot[i + 1], iroot[i + 1], modulus);
        }

        let mut prod = 1;
        let mut iprod = 1;
        let mut i = 0;
        while i + 2 <= rank2 {
            rate2[i] = mul_mod(root[i + 2], prod, modulus);
            irate2[i] = mul_mod(iroot[i + 2], iprod, modulus);
            prod = mul_mod(prod, iroot[i + 2], modulus);
            iprod = mul_mod(iprod, root[i + 2], modulus);
            i += 1;
        }

        let mut prod = 1;
        let mut iprod = 1;
        let mut i = 0;
        while i + 3 <= rank2 {
            rate3[i] = mul_mod(root[i + 3], prod, modulus);
            irate3[i] = mul_mod(iroot[i + 3], iprod, modulus);
            prod = mul_mod(prod, iroot[i + 3], modulus);
            iprod = mul_mod(iprod, root[i + 3], modulus);
            i += 1;
        }

        Self {
            is_prime,
            rank2,
            primitive_root,
            root,
            iroot,
            rate2,
            irate2,
            rate3,
            irate3,
        }
    }
}

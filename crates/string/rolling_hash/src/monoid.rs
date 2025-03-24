use algebraic_traits::{Algebraic, Associative, Magma, Pow, Unital};
use modint_61::ModInt61;

use crate::RollingHash;

pub enum RollingHashOperator {}

impl Algebraic for RollingHashOperator {
    type Value = RollingHash;
}

impl Magma for RollingHashOperator {
    fn op(a: &RollingHash, b: &RollingHash) -> RollingHash {
        RollingHash {
            hash: a.hash + b.hash * a.base_pow,
            base_pow: a.base_pow * b.base_pow,
        }
    }
}

impl Unital for RollingHashOperator {
    fn unit() -> RollingHash {
        RollingHash {
            hash: ModInt61::from_raw(0),
            base_pow: ModInt61::from_raw(1),
        }
    }
}

impl Associative for RollingHashOperator {}

impl Pow for RollingHashOperator {}

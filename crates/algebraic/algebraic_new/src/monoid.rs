use crate::traits::{Algebraic, Associative, Commutative, Magma, Unital};

pub enum AddMonoid {}

impl Algebraic for AddMonoid {
    type S = i32;
}

impl Magma for AddMonoid {
    fn op(x: Self::S, y: Self::S) -> Self::S {
        x + y
    }
}

impl Unital for AddMonoid {
    fn unit() -> Self::S {
        0
    }
}

impl Commutative for AddMonoid {}

impl Associative for AddMonoid {}

pub trait Algebraic {
    type S;
}

pub trait Magma: Algebraic {
    fn op(x: Self::S, y: Self::S) -> Self::S;
}

pub trait Unital: Magma {
    fn unit() -> Self::S;
}

pub trait Invertive: Magma {
    fn inv(x: Self::S) -> Self::S;
}

pub trait Commutative: Magma {}

pub trait Associative: Magma {}

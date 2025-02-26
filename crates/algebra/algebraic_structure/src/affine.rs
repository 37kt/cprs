use std::marker::PhantomData;

use algebraic_traits::{Algebraic, Associative, Invertive, Magma, Pow, Unital};
use numeric_traits::{Numeric, Recip, Signed};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine<T: Numeric>(pub T, pub T);

impl<T: Numeric> Affine<T> {
    pub fn eval(self, x: T) -> T {
        self.0 * x + self.1
    }
}

pub struct AffineOperator<T: Numeric>(PhantomData<fn() -> T>);

impl<T: Numeric> Algebraic for AffineOperator<T> {
    type Value = Affine<T>;
}

impl<T: Numeric> Magma for AffineOperator<T> {
    fn op(&Affine(a1, b1): &Affine<T>, &Affine(a2, b2): &Affine<T>) -> Affine<T> {
        Affine(a1 * a2, a2 * b1 + b2)
    }
}

impl<T: Numeric> Unital for AffineOperator<T> {
    fn unit() -> Affine<T> {
        Affine(T::one(), T::zero())
    }
}

impl<T: Numeric> Associative for AffineOperator<T> {}

impl<T: Numeric + Signed + Recip> Invertive for AffineOperator<T> {
    fn inv(&Affine(a, b): &Affine<T>) -> Affine<T> {
        let recip_a = a.recip();
        Affine(recip_a, -recip_a * b)
    }
}

impl<T: Numeric> Pow for AffineOperator<T> {}

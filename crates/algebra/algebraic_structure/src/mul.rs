use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use numeric_traits::{Numeric, Recip};

pub struct MulOperator<T: Numeric>(Infallible, PhantomData<fn() -> T>);

impl<T: Numeric> Algebraic for MulOperator<T> {
    type Value = T;
}

impl<T: Numeric> Magma for MulOperator<T> {
    fn op(a: &T, b: &T) -> T {
        *a * *b
    }
}

impl<T: Numeric> Unital for MulOperator<T> {
    fn unit() -> T {
        T::one()
    }
}

impl<T: Numeric> Associative for MulOperator<T> {}

impl<T: Numeric> Commutative for MulOperator<T> {}

impl<T: Numeric + Recip> Invertive for MulOperator<T> {
    fn inv(x: &T) -> T {
        x.recip()
    }
}

impl<T: Numeric> Pow for MulOperator<T> {}

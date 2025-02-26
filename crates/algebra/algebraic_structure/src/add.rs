use std::{convert::Infallible, marker::PhantomData};

use algebraic_traits::{Algebraic, Associative, Commutative, Invertive, Magma, Pow, Unital};
use numeric_traits::{Cast, Numeric, Signed};

pub struct AddOperator<T: Numeric>(Infallible, PhantomData<fn() -> T>);

impl<T: Numeric> Algebraic for AddOperator<T> {
    type Value = T;
}

impl<T: Numeric> Magma for AddOperator<T> {
    fn op(a: &T, b: &T) -> T {
        *a + *b
    }
}

impl<T: Numeric> Unital for AddOperator<T> {
    fn unit() -> T {
        T::zero()
    }
}

impl<T: Numeric> Associative for AddOperator<T> {}

impl<T: Numeric> Commutative for AddOperator<T> {}

impl<T: Numeric + Signed> Invertive for AddOperator<T> {
    fn inv(x: &T) -> T {
        -*x
    }
}

impl<T: Numeric> Pow for AddOperator<T>
where
    usize: Cast<T>,
{
    fn pow(x: &T, exp: usize) -> T {
        *x * exp.cast()
    }
}

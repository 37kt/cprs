use std::marker::PhantomData;

use crate::{Algebraic, Associative, Field, Invertive, Magma, Semiring, Unital};

pub struct Affine<T: Semiring> {
    _marker: PhantomData<fn() -> T>,
}

impl<T: Semiring> Algebraic for Affine<T> {
    type Element = (T::Element, T::Element);
}

impl<T: Semiring> Magma for Affine<T> {
    fn op((a, b): &Self::Element, (c, d): &Self::Element) -> Self::Element {
        (T::mul(a, c), T::add(&T::mul(b, c), d))
    }
}

impl<T: Semiring> Unital for Affine<T> {
    fn unit() -> Self::Element {
        (T::one(), T::zero())
    }
}

impl<T: Field> Invertive for Affine<T>
where
    T::Additive: Invertive,
    T::Multiplicative: Invertive,
    T::Element: Clone,
{
    fn inv((a, b): &Self::Element) -> Self::Element {
        let recip_a = T::recip(a);
        (recip_a.clone(), T::neg(&T::mul(b, &recip_a)))
    }
}

impl<T: Semiring> Associative for Affine<T> {}

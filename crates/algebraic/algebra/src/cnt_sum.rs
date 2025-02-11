use std::marker::PhantomData;

use crate::{Act, Affine, Algebraic, Associative, Commutative, Magma, Semiring, Unital};

pub struct CntSum<T: Semiring> {
    _marker: PhantomData<fn() -> T>,
}

impl<T: Semiring> Algebraic for CntSum<T> {
    type Element = (T::Element, T::Element);
}

impl<T: Semiring> Magma for CntSum<T> {
    fn op((a, b): &Self::Element, (c, d): &Self::Element) -> Self::Element {
        (T::add(a, c), T::add(b, d))
    }
}

impl<T: Semiring> Unital for CntSum<T> {
    fn unit() -> Self::Element {
        (T::zero(), T::zero())
    }
}

impl<T: Semiring> Associative for CntSum<T> {}

impl<T: Semiring> Commutative for CntSum<T> {}

impl<T: Semiring> Act<CntSum<T>> for Affine<T>
where
    T::Element: Clone,
{
    fn act(
        (a, b): &Self::Element,
        (c, s): &<CntSum<T> as Algebraic>::Element,
    ) -> <CntSum<T> as Algebraic>::Element {
        (c.clone(), T::add(&T::mul(a, s), &T::mul(b, c)))
    }
}

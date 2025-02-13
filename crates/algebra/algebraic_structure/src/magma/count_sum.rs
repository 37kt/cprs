use std::{
    convert::Infallible,
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use algebraic_traits::{Algebraic, Associative, Commutative, Magma, Semiring, Unital};

pub struct CountSumOperator<T: Semiring>(Infallible, PhantomData<fn() -> T>);

pub struct CountSum<T: Semiring> {
    pub count: T::Element,
    pub sum: T::Element,
}

impl<T: Semiring> Algebraic for CountSumOperator<T> {
    type Element = CountSum<T>;
}

impl<T: Semiring> Magma for CountSumOperator<T> {
    fn op(
        CountSum { count: c1, sum: s1 }: &CountSum<T>,
        CountSum { count: c2, sum: s2 }: &CountSum<T>,
    ) -> CountSum<T> {
        CountSum {
            count: T::add(c1, c2),
            sum: T::add(s1, s2),
        }
    }
}

impl<T: Semiring> Unital for CountSumOperator<T> {
    fn unit() -> CountSum<T> {
        CountSum {
            count: T::zero(),
            sum: T::zero(),
        }
    }
}

impl<T: Semiring> Associative for CountSumOperator<T> {}

impl<T: Semiring> Commutative for CountSumOperator<T> {}

impl<T: Semiring> Debug for CountSum<T>
where
    T::Element: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ count: {:?}, sum: {:?} }}", self.count, self.sum)
    }
}

impl<T: Semiring> Clone for CountSum<T>
where
    T::Element: Clone,
{
    fn clone(&self) -> Self {
        CountSum {
            count: self.count.clone(),
            sum: self.sum.clone(),
        }
    }
}

impl<T: Semiring> Copy for CountSum<T> where T::Element: Copy {}

impl<T: Semiring> PartialEq for CountSum<T>
where
    T::Element: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.sum == other.sum
    }
}

impl<T: Semiring> Eq for CountSum<T> where T::Element: Eq {}

impl<T: Semiring> Hash for CountSum<T>
where
    T::Element: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.count.hash(state);
        self.sum.hash(state);
    }
}

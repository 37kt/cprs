pub trait Algebraic {
    type Element;
}

pub trait Magma: Algebraic {
    fn op(x: &Self::Element, y: &Self::Element) -> Self::Element;
}

pub trait Unital: Magma {
    fn unit() -> Self::Element;
}

pub trait Invertive: Magma {
    fn inv(x: &Self::Element) -> Self::Element;
}

pub trait Associative: Magma {}

pub trait Commutative: Magma {}

pub trait Idempotent: Magma {}

pub trait Semigroup: Associative {}
impl<T: Associative> Semigroup for T {}

pub trait Monoid: Associative + Unital {}
impl<T: Associative + Unital> Monoid for T {}

pub trait CommutativeMonoid: Associative + Unital + Commutative {}
impl<T: Associative + Unital + Commutative> CommutativeMonoid for T {}

pub trait Group: Associative + Unital + Invertive {}
impl<T: Associative + Unital + Invertive> Group for T {}

pub trait AbelianGroup: Associative + Unital + Commutative + Invertive {}
impl<T: Associative + Unital + Commutative + Invertive> AbelianGroup for T {}

pub trait Band: Associative + Idempotent {}
impl<T: Associative + Idempotent> Band for T {}

pub trait Act<Operand: Algebraic>: Algebraic {
    fn act(f: &Self::Element, x: &Operand::Element) -> Operand::Element;
}

pub trait Semiring: Algebraic {
    type Additive: CommutativeMonoid<Element = Self::Element>;
    type Multiplicative: Monoid<Element = Self::Element>;

    fn zero() -> Self::Element {
        Self::Additive::unit()
    }

    fn one() -> Self::Element {
        Self::Multiplicative::unit()
    }

    fn add(x: &Self::Element, y: &Self::Element) -> Self::Element {
        Self::Additive::op(x, y)
    }

    fn mul(x: &Self::Element, y: &Self::Element) -> Self::Element {
        Self::Multiplicative::op(x, y)
    }
}

pub trait Ring: Semiring
where
    Self::Additive: Invertive,
{
    fn neg(x: &Self::Element) -> Self::Element {
        Self::Additive::inv(x)
    }

    fn sub(x: &Self::Element, y: &Self::Element) -> Self::Element {
        Self::Additive::op(x, &Self::neg(y))
    }
}
impl<T> Ring for T
where
    T: Semiring,
    T::Additive: Invertive,
{
}

pub trait Field: Ring
where
    Self::Additive: Invertive,
    Self::Multiplicative: Invertive,
{
    fn recip(x: &Self::Element) -> Self::Element {
        Self::Multiplicative::inv(x)
    }

    fn div(x: &Self::Element, y: &Self::Element) -> Self::Element {
        Self::Multiplicative::op(x, &Self::recip(y))
    }
}
impl<T> Field for T
where
    T: Ring,
    T::Additive: Invertive,
    T::Multiplicative: Invertive,
{
}

#[macro_export]
macro_rules! define_algebra {
    (name: $name:ident, element: $element:ty) => {
        enum $name {}
        impl $crate::Algebraic for $name {
            type Element = $element;
        }
    };

    ($vis:vis, name: $name:ident, element: $element:ty) => {
        $vis enum $name {}
        impl $crate::Algebraic for $name {
            type Element = $element;
        }
    };

    (name: $name:ident, element: $element:ty, $($rest:tt)*) => {
        define_algebra!(name: $name, element: $element);
        define_algebra!(@impl $name, $($rest)*);
    };

    ($vis:vis, name: $name:ident, element: $element:ty, $($rest:tt)*) => {
        define_algebra!($vis, name: $name, element: $element);
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, op: $op:expr, $($rest:tt)*) => {
        impl $crate::Magma for $name {
            fn op(x: &Self::Element, y: &Self::Element) -> Self::Element {
                $op(x, y)
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, unit: $unit:expr, $($rest:tt)*) => {
        impl $crate::Unital for $name {
            fn unit() -> Self::Element {
                $unit
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, inv: $inv:expr, $($rest:tt)*) => {
        impl $crate::Invertive for $name {
            fn inv(x: &Self::Element) -> Self::Element {
                $inv(x)
            }
        }
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, associative, $($rest:tt)*) => {
        impl $crate::Associative for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, commutative, $($rest:tt)*) => {
        impl $crate::Commutative for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident, idempotent, $($rest:tt)*) => {
        impl $crate::Idempotent for $name {}
        define_algebra!(@impl $name, $($rest)*);
    };

    (@impl $name:ident $(,)?) => {};
}

#[macro_export]
macro_rules! impl_act {
    ($operator:ty, $operand:ty, $act:expr $(,)?) => {
        impl $crate::Act for $operator {
            type Operand = $operand;
            fn act(
                f: &Self::Element,
                x: &<Self::Operand as $crate::Algebraic>::Element,
            ) -> <Self::Operand as $crate::Algebraic>::Element {
                $act(f, x)
            }
        }
    };

    ($vis:vis, $operator:ty, $operand:ty, $act:expr $(,)?) => {
        $vis impl $crate::Act for $operator {
            type Operand = $operand;
            fn act(
                f: &Self::Element,
                x: &<Self::Operand as $crate::Algebraic>::Element,
            ) -> <Self::Operand as $crate::Algebraic>::Element {
                $act(f, x)
            }
        }
    };
}

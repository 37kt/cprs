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

pub trait Pow: Monoid {
    fn pow(x: &Self::Element, mut exp: usize) -> Self::Element {
        let mut res = Self::unit();
        let mut x = Self::op(&res, x);
        while exp > 0 {
            if exp & 1 == 1 {
                res = Self::op(&res, &x);
            }
            x = Self::op(&x, &x);
            exp >>= 1;
        }
        res
    }
}

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

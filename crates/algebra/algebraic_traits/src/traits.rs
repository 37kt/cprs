pub trait Algebraic {
    type Value;
}

pub trait Magma: Algebraic {
    fn op(x: &Self::Value, y: &Self::Value) -> Self::Value;
}

pub trait Unital: Magma {
    fn unit() -> Self::Value;
}

pub trait Invertive: Magma {
    fn inv(x: &Self::Value) -> Self::Value;
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
    fn pow(x: &Self::Value, mut exp: usize) -> Self::Value {
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

pub trait Act {
    type Operand: Algebraic;
    type Operator: Algebraic;

    fn act(
        x: &<Self::Operand as Algebraic>::Value,
        f: &<Self::Operator as Algebraic>::Value,
    ) -> <Self::Operand as Algebraic>::Value;
}

pub trait Semiring: Algebraic {
    type Additive: CommutativeMonoid<Value = Self::Value>;
    type Multiplicative: Monoid<Value = Self::Value>;

    fn zero() -> Self::Value {
        Self::Additive::unit()
    }

    fn one() -> Self::Value {
        Self::Multiplicative::unit()
    }

    fn add(x: &Self::Value, y: &Self::Value) -> Self::Value {
        Self::Additive::op(x, y)
    }

    fn mul(x: &Self::Value, y: &Self::Value) -> Self::Value {
        Self::Multiplicative::op(x, y)
    }
}

pub trait Ring: Semiring
where
    Self::Additive: Invertive,
{
    fn neg(x: &Self::Value) -> Self::Value {
        Self::Additive::inv(x)
    }

    fn sub(x: &Self::Value, y: &Self::Value) -> Self::Value {
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
    fn recip(x: &Self::Value) -> Self::Value {
        Self::Multiplicative::inv(x)
    }

    fn div(x: &Self::Value, y: &Self::Value) -> Self::Value {
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

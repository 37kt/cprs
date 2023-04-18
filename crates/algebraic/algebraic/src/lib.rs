pub trait Monoid {
    type S;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub trait ActMonoid {
    type M: Monoid;
    type F;
    fn id() -> Self::F;
    fn comp(a: &Self::F, b: &Self::F) -> Self::F;
    fn act(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S;
}

// pub trait Group: Monoid {
//     fn inv(a: &Self::S) -> Self::S;
// }

// pub trait SemiRing {
//     type S;
//     type Add: Monoid<S = Self::S>;
//     type Mul: Monoid<S = Self::S>;
//     fn zero() -> Self::S {
//         <Self::Add as Monoid>::e()
//     }
//     fn one() -> Self::S {
//         <Self::Mul as Monoid>::e()
//     }
//     fn add(a: &Self::S, b: &Self::S) -> Self::S {
//         <Self::Add as Monoid>::op(a, b)
//     }
//     fn mul(a: &Self::S, b: &Self::S) -> Self::S {
//         <Self::Mul as Monoid>::op(a, b)
//     }
// }

// pub trait Ring: SemiRing
// where
//     Self::Add: Group,
// {
//     fn neg(a: &Self::S) -> Self::S {
//         <Self::Add as Group>::inv(a)
//     }
// }

// pub trait Field: Ring
// where
//     Self::Add: Group,
//     Self::Mul: Group,
// {
//     fn recip(a: &Self::S) -> Self::S {
//         <Self::Mul as Group>::inv(a)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        enum Min {}
        impl Monoid for Min {
            type S = i64;
            fn e() -> Self::S {
                1 << 60
            }
            fn op(a: &Self::S, b: &Self::S) -> Self::S {
                *a.min(b)
            }
        }
        enum Add {}
        impl Monoid for Add {
            type S = i64;
            fn e() -> Self::S {
                0
            }
            fn op(a: &Self::S, b: &Self::S) -> Self::S {
                a + b
            }
        }
        // enum MinAdd {}
        // impl SemiRing for MinAdd {
        //     type S = i64;
        //     type Add = Min;
        //     type Mul = Add;
        // }
    }
}

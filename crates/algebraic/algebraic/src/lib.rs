pub trait Monoid {
    type S;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub trait ActMonoid {
    type F;
    type X;
    fn e() -> Self::F;
    fn op(a: &Self::F, b: &Self::F) -> Self::F;
    fn act(f: &Self::F, x: &Self::X) -> Self::X;
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
        enum A {}
        impl ActMonoid for A {
            type F = i64;
            type X = i64;
            fn e() -> Self::F {
                0
            }
            fn op(a: &Self::F, b: &Self::F) -> Self::F {
                a + b
            }
            fn act(f: &Self::F, x: &Self::X) -> Self::X {
                f + x
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

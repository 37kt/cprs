mod add;
mod affine;
mod count_sum;
mod mul;
mod trivial_group;
pub mod magma {
    pub use crate::add::*;
    pub use crate::affine::*;
    pub use crate::count_sum::*;
    pub use crate::mul::*;
    pub use crate::trivial_group::*;
}

mod countsum_affine;
pub mod act {
    pub use crate::countsum_affine::*;
}

pub mod semiring;
pub use semiring::*;

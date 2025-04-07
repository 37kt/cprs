mod add;
mod affine;
mod count_sum;
mod max;
mod min;
mod mul;
mod trivial_group;
mod xor;
pub mod magma {
    pub use crate::add::*;
    pub use crate::affine::*;
    pub use crate::count_sum::*;
    pub use crate::max::*;
    pub use crate::min::*;
    pub use crate::mul::*;
    pub use crate::trivial_group::*;
    pub use crate::xor::*;
}

mod countsum_affine;
pub mod act {
    pub use crate::countsum_affine::*;
}

pub mod semiring;
pub use semiring::*;

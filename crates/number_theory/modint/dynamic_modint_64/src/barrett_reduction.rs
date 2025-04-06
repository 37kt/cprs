use std::cell::Cell;

use barrett_reduction::BarrettReduction64;

#[allow(clippy::extra_unused_type_parameters)]
pub(crate) fn barrett_reduction<Id, Ret>(f: impl FnOnce(&Cell<BarrettReduction64>) -> Ret) -> Ret {
    thread_local! {
        static BARRETT_REDUCTION: Cell<BarrettReduction64> = Cell::new(BarrettReduction64::new(1_000_000_009));
    }

    BARRETT_REDUCTION.with(|br| f(br))
}

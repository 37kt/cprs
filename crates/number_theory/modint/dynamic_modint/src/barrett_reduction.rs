use std::cell::Cell;

use barrett_reduction::BarrettReduction32;

pub(crate) fn barrett_reduction<Id, Ret>(f: impl FnOnce(&Cell<BarrettReduction32>) -> Ret) -> Ret {
    thread_local! {
        static BARRETT_REDUCTION: Cell<BarrettReduction32> = Cell::new(BarrettReduction32::new(1_000_000_009));
    }

    BARRETT_REDUCTION.with(|br| f(br))
}

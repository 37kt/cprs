use std::{cell::RefCell, ptr::NonNull};

use simple_arena::Arena;

thread_local! {
    static ARENA: RefCell<Arena> = RefCell::new(Arena::new(1024 * 1024 * 1024));
}

pub(crate) fn new_ptr<T>(val: T) -> NonNull<T> {
    ARENA.with(|arena| NonNull::new(arena.borrow_mut().alloc(val)).unwrap())
}

use std::{cell::UnsafeCell, marker::PhantomData};

pub struct RecurseImpl<'a, Arg, Ret>(
    UnsafeCell<&'a mut dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret>,
);

impl<'a, Arg, Ret> RecurseImpl<'a, Arg, Ret> {
    pub fn call(&mut self, arg: Arg) -> Ret {
        let f = unsafe { &mut *self.0.get() };
        f(self, arg)
    }
}

/// クロージャで再帰する  
/// 危険！ 絶対使うな！！！
pub struct Recurse<'a, Arg, Ret, F>(
    UnsafeCell<F>,
    PhantomData<&'a ()>,
    PhantomData<Arg>,
    PhantomData<Ret>,
)
where
    F: FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret;

impl<'a, Arg, Ret, F> Recurse<'a, Arg, Ret, F>
where
    F: FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret,
{
    pub fn new(f: F) -> Self {
        Self(UnsafeCell::new(f), PhantomData, PhantomData, PhantomData)
    }
}

impl<'a, Arg: 'a, Ret: 'a, F> Recurse<'a, Arg, Ret, F>
where
    F: FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret + 'a,
{
    pub fn call(&mut self, arg: Arg) -> Ret {
        let f = unsafe { &mut *self.0.get() };
        let g = unsafe { &mut *self.0.get() };
        f(&mut RecurseImpl(UnsafeCell::new(g)), arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut s = 0;
        let mut f = Recurse::new(|f, n| {
            if n < 0 {
            } else if n <= 1 {
                s += 1;
            } else {
                f.call(n - 1);
                f.call(n - 2);
            }
        });
        f.call(10);
        assert_eq!(s, 89);
    }
}

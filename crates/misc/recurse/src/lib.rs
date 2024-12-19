pub struct RecurseImpl<'a, Arg, Ret>(&'a mut dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret);

impl<'a, Arg, Ret> RecurseImpl<'a, Arg, Ret> {
    pub fn call(&mut self, arg: Arg) -> Ret {
        let f: &mut dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret =
            unsafe { &mut *(&mut *self.0 as *mut _) };
        f(self, arg)
    }
}

pub struct Recurse<'a, 'b, Arg, Ret>(
    Box<dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret + 'b>,
);

impl<'a, 'b: 'a, Arg, Ret> Recurse<'a, 'b, Arg, Ret> {
    pub fn new(f: impl FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret + 'b) -> Self {
        Self(Box::new(f))
    }

    pub fn call(&mut self, arg: Arg) -> Ret {
        let f = unsafe { &mut *(&mut *self.0 as *mut _) };
        RecurseImpl(f).call(arg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut f = Recurse::new(|f, n| if n == 0 { 1 } else { n * f.call(n - 1) });
        assert_eq!(f.call(10), 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);
    }
}

---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cell::UnsafeCell, marker::PhantomData};\n\npub struct RecurseImpl<'a,\
    \ Arg, Ret>(\n    UnsafeCell<&'a mut dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>,\
    \ Arg) -> Ret>,\n);\n\nimpl<'a, Arg, Ret> RecurseImpl<'a, Arg, Ret> {\n    pub\
    \ fn call(&mut self, arg: Arg) -> Ret {\n        let f = unsafe { &mut *self.0.get()\
    \ };\n        f(self, arg)\n    }\n}\n\npub struct Recurse<'a, Arg, Ret, F>(\n\
    \    UnsafeCell<F>,\n    PhantomData<&'a ()>,\n    PhantomData<Arg>,\n    PhantomData<Ret>,\n\
    )\nwhere\n    F: FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret;\n\nimpl<'a,\
    \ Arg, Ret, F> Recurse<'a, Arg, Ret, F>\nwhere\n    F: FnMut(&mut RecurseImpl<'a,\
    \ Arg, Ret>, Arg) -> Ret,\n{\n    pub fn new(f: F) -> Self {\n        Self(UnsafeCell::new(f),\
    \ PhantomData, PhantomData, PhantomData)\n    }\n}\n\nimpl<'a, Arg: 'a, Ret: 'a,\
    \ F> Recurse<'a, Arg, Ret, F>\nwhere\n    F: FnMut(&mut RecurseImpl<'a, Arg, Ret>,\
    \ Arg) -> Ret + 'a,\n{\n    pub fn call(&mut self, arg: Arg) -> Ret {\n      \
    \  let f = unsafe { &mut *self.0.get() };\n        let g = unsafe { &mut *self.0.get()\
    \ };\n        f(&mut RecurseImpl(UnsafeCell::new(g)), arg)\n    }\n}\n\n#[cfg(test)]\n\
    mod tests {\n    use super::*;\n\n    #[test]\n    fn test() {\n        let mut\
    \ s = 0;\n        let mut f = Recurse::new(|f, n| {\n            if n < 0 {\n\
    \            } else if n <= 1 {\n                s += 1;\n            } else {\n\
    \                f.call(n - 1);\n                f.call(n - 2);\n            }\n\
    \        });\n        f.call(10);\n        assert_eq!(s, 89);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/recurse/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-21 11:33:20+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/recurse/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/recurse/src/lib.rs
- /library/crates/misc/recurse/src/lib.rs.html
title: crates/misc/recurse/src/lib.rs
---

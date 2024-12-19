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
  code: "pub struct RecurseImpl<'a, Arg, Ret>(&'a mut dyn FnMut(&mut RecurseImpl<'a,\
    \ Arg, Ret>, Arg) -> Ret);\n\nimpl<'a, Arg, Ret> RecurseImpl<'a, Arg, Ret> {\n\
    \    pub fn call(&mut self, arg: Arg) -> Ret {\n        let f: &mut dyn FnMut(&mut\
    \ RecurseImpl<'a, Arg, Ret>, Arg) -> Ret =\n            unsafe { &mut *(&mut *self.0\
    \ as *mut _) };\n        f(self, arg)\n    }\n}\n\npub struct Recurse<'a, 'b,\
    \ Arg, Ret>(\n    Box<dyn FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret +\
    \ 'b>,\n);\n\nimpl<'a, 'b: 'a, Arg, Ret> Recurse<'a, 'b, Arg, Ret> {\n    pub\
    \ fn new(f: impl FnMut(&mut RecurseImpl<'a, Arg, Ret>, Arg) -> Ret + 'b) -> Self\
    \ {\n        Self(Box::new(f))\n    }\n\n    pub fn call(&mut self, arg: Arg)\
    \ -> Ret {\n        let f = unsafe { &mut *(&mut *self.0 as *mut _) };\n     \
    \   RecurseImpl(f).call(arg)\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\
    \n    #[test]\n    fn test() {\n        let mut f = Recurse::new(|f, n| if n ==\
    \ 0 { 1 } else { n * f.call(n - 1) });\n        assert_eq!(f.call(10), 10 * 9\
    \ * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/recurse/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-19 09:16:31+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/recurse/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/recurse/src/lib.rs
- /library/crates/misc/recurse/src/lib.rs.html
title: crates/misc/recurse/src/lib.rs
---

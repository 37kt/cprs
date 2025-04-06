---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/data_structure/persistent_array/src/lib.rs
    title: crates/data_structure/persistent_array/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub struct Arena {\n    pool: Vec<u8>,\n    offset: usize,\n}\n\nimpl Arena\
    \ {\n    #[allow(clippy::uninit_vec)]\n    pub fn new(capacity: usize) -> Self\
    \ {\n        let mut pool = Vec::with_capacity(capacity);\n        unsafe { pool.set_len(capacity)\
    \ };\n        Self { pool, offset: 0 }\n    }\n\n    pub fn alloc<T>(&mut self,\
    \ value: T) -> *mut T {\n        let align = std::mem::align_of::<T>();\n    \
    \    let size = std::mem::size_of::<T>();\n\n        let pool_addr = self.pool.as_ptr()\
    \ as usize;\n        let start = (pool_addr + self.offset + align - 1) & !(align\
    \ - 1);\n        let end = start + size;\n        assert!(\n            end <=\
    \ pool_addr + self.pool.len(),\n            \"simple_arena: out of memory\"\n\
    \        );\n\n        let ptr = start as *mut T;\n        unsafe {\n        \
    \    std::ptr::write(ptr, value);\n        }\n        self.offset = end - pool_addr;\n\
    \        ptr\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/simple_arena/src/lib.rs
  requiredBy:
  - crates/data_structure/persistent_array/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/simple_arena/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/simple_arena/src/lib.rs
- /library/crates/misc/simple_arena/src/lib.rs.html
title: crates/misc/simple_arena/src/lib.rs
---

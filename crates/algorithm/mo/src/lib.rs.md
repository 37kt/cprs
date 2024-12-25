---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/static_range_inversions_query/src/main.rs
    title: verify/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/static_range_sum/src/main.rs
    title: verify/static_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// Mo's algorithm \u306E\u5B9F\u88C5\u306B\u5FC5\u8981\u306A\u64CD\u4F5C\
    \u3092\u5B9A\u7FA9\u3059\u308B\u30C8\u30EC\u30A4\u30C8\n///\n/// # \u6982\u8981\
    \n/// - \u533A\u9593\u30AF\u30A8\u30EA\u3092\u52B9\u7387\u7684\u306B\u51E6\u7406\
    \u3059\u308B\u305F\u3081\u306E\u30A2\u30EB\u30B4\u30EA\u30BA\u30E0\u3092\u5B9F\
    \u88C5\n/// - \u30AF\u30A8\u30EA\u3092\u9069\u5207\u306A\u9806\u5E8F\u3067\u51E6\
    \u7406\u3059\u308B\u3053\u3068\u3067\u8A08\u7B97\u91CF\u3092\u524A\u6E1B\n///\n\
    /// # \u30C8\u30EC\u30A4\u30C8\u8981\u4EF6\n/// - `Output`: \u30AF\u30A8\u30EA\
    \u306E\u7D50\u679C\u578B\uFF08`Default + Clone` \u3092\u5B9F\u88C5\u3059\u308B\
    \u5FC5\u8981\u3042\u308A\uFF09\n///\n/// # \u5B9F\u88C5\u65B9\u6CD5\n/// \u4EE5\
    \u4E0B\u306E\u30E1\u30BD\u30C3\u30C9\u306E\u3046\u3061\u3001\u5FC5\u8981\u306A\
    \u3082\u306E\u3092\u5B9F\u88C5\u3059\u308B\uFF1A\n///\n/// - \u57FA\u672C\u64CD\
    \u4F5C\uFF08\u3069\u3061\u3089\u304B\u3092\u9078\u629E\uFF09\n///   - `add`/`remove`:\
    \ \u65B9\u5411\u306B\u4F9D\u5B58\u3057\u306A\u3044\u5834\u5408\n///   - `add_left`/`add_right`/`remove_left`/`remove_right`:\
    \ \u65B9\u5411\u306B\u4F9D\u5B58\u3059\u308B\u5834\u5408\n/// - `query`: \u73FE\
    \u5728\u306E\u72B6\u614B\u3067\u306E\u7D50\u679C\u3092\u8A08\u7B97\n/// - `initial_position`:\
    \ \u5FC5\u8981\u306B\u5FDC\u3058\u3066\u521D\u671F\u4F4D\u7F6E\u3092\u8A2D\u5B9A\
    \uFF08\u30C7\u30D5\u30A9\u30EB\u30C8\u306F`(0, 0)`\uFF09\n///\n/// # \u8A08\u7B97\
    \u91CF\n/// - \u79FB\u52D51\u56DE\u3042\u305F\u308A\u306E\u8A08\u7B97\u91CF\u3092\
    \ O(\u03B1) \u3068\u3057\u3066\u3001\u5168\u4F53\u3067 O(\u03B1N\u221AQ)\n///\
    \   - N: \u914D\u5217\u306E\u9577\u3055\n///   - Q: \u30AF\u30A8\u30EA\u306E\u6570\
    \npub trait Mo {\n    type Output: Default + Clone;\n\n    /// \u8981\u7D20\u3092\
    \u8FFD\u52A0\u3059\u308B\n    ///\n    /// # \u5F15\u6570\n    /// - `i`: \u8FFD\
    \u52A0\u3059\u308B\u8981\u7D20\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\n   \
    \ ///\n    /// # \u6CE8\u610F\n    /// \u5DE6\u53F3\u306E\u65B9\u5411\u306B\u4F9D\
    \u5B58\u3059\u308B\u5834\u5408\u306F\u3001\u4EE3\u308F\u308A\u306B `add_left`/`add_right`\
    \ \u3092\u5B9F\u88C5\u3059\u308B\u3053\u3068\n    #[allow(unused_variables)]\n\
    \    fn add(&mut self, i: usize) {\n        unimplemented!()\n    }\n\n    ///\
    \ \u8981\u7D20\u3092\u524A\u9664\u3059\u308B\n    ///\n    /// # \u5F15\u6570\n\
    \    /// - `i`: \u524A\u9664\u3059\u308B\u8981\u7D20\u306E\u30A4\u30F3\u30C7\u30C3\
    \u30AF\u30B9\n    ///\n    /// # \u6CE8\u610F\n    /// \u5DE6\u53F3\u306E\u65B9\
    \u5411\u306B\u4F9D\u5B58\u3059\u308B\u5834\u5408\u306F\u3001\u4EE3\u308F\u308A\
    \u306B `remove_left`/`remove_right` \u3092\u5B9F\u88C5\u3059\u308B\u3053\u3068\
    \n    #[allow(unused_variables)]\n    fn remove(&mut self, i: usize) {\n     \
    \   unimplemented!()\n    }\n\n    /// \u5DE6\u65B9\u5411\u306B\u8981\u7D20\u3092\
    \u8FFD\u52A0\u3059\u308B\n    ///\n    /// # \u52D5\u4F5C\n    /// - \u8981\u7D20\
    \ `a[i]` \u3092\u8FFD\u52A0\n    /// - \u307E\u305F\u306F\u533A\u9593\u3092 `(i+1,\
    \ j)` \u304B\u3089 `(i, j)` \u306B\u79FB\u52D5\n    fn add_left(&mut self, i:\
    \ usize) {\n        self.add(i);\n    }\n\n    /// \u53F3\u65B9\u5411\u306B\u8981\
    \u7D20\u3092\u8FFD\u52A0\u3059\u308B\n    ///\n    /// # \u52D5\u4F5C\n    ///\
    \ - \u8981\u7D20 `a[i]` \u3092\u8FFD\u52A0\n    /// - \u307E\u305F\u306F\u533A\
    \u9593\u3092 `(j, i)` \u304B\u3089 `(j, i+1)` \u306B\u79FB\u52D5\n    fn add_right(&mut\
    \ self, i: usize) {\n        self.add(i);\n    }\n\n    /// \u5DE6\u65B9\u5411\
    \u306B\u8981\u7D20\u3092\u524A\u9664\u3059\u308B\n    ///\n    /// # \u52D5\u4F5C\
    \n    /// - \u8981\u7D20 `a[i]` \u3092\u524A\u9664\n    /// - \u307E\u305F\u306F\
    \u533A\u9593\u3092 `(i, j)` \u304B\u3089 `(i+1, j)` \u306B\u79FB\u52D5\n    fn\
    \ remove_left(&mut self, i: usize) {\n        self.remove(i);\n    }\n\n    ///\
    \ \u53F3\u65B9\u5411\u306B\u8981\u7D20\u3092\u524A\u9664\u3059\u308B\n    ///\n\
    \    /// # \u52D5\u4F5C\n    /// - \u8981\u7D20 `a[i]` \u3092\u524A\u9664\n  \
    \  /// - \u307E\u305F\u306F\u533A\u9593\u3092 `(j, i+1)` \u304B\u3089 `(j, i)`\
    \ \u306B\u79FB\u52D5\n    fn remove_right(&mut self, i: usize) {\n        self.remove(i);\n\
    \    }\n\n    /// \u73FE\u5728\u306E\u533A\u9593\u306B\u5BFE\u3059\u308B\u30AF\
    \u30A8\u30EA\u3092\u5B9F\u884C\u3059\u308B\n    ///\n    /// # \u623B\u308A\u5024\
    \n    /// \u73FE\u5728\u306E\u72B6\u614B\u306B\u304A\u3051\u308B\u8A08\u7B97\u7D50\
    \u679C\n    fn query(&self) -> Self::Output;\n\n    /// \u521D\u671F\u4F4D\u7F6E\
    \u3092\u8A2D\u5B9A\u3059\u308B\n    ///\n    /// # \u623B\u308A\u5024\n    ///\
    \ - \u30C7\u30D5\u30A9\u30EB\u30C8\u3067\u306F `(0, 0)` \u3092\u8FD4\u3059\n \
    \   /// - \u5FC5\u8981\u306B\u5FDC\u3058\u3066\u30AA\u30FC\u30D0\u30FC\u30E9\u30A4\
    \u30C9\u3059\u308B\u3053\u3068\n    fn initial_position(&self) -> (usize, usize)\
    \ {\n        (0, 0)\n    }\n\n    /// \u3059\u3079\u3066\u306E\u30AF\u30A8\u30EA\
    \u3092\u51E6\u7406\u3059\u308B\n    ///\n    /// # \u5F15\u6570\n    /// - `qs`:\
    \ `(left, right)` \u306E\u5F62\u5F0F\u306E\u30AF\u30A8\u30EA\u914D\u5217\n   \
    \ ///\n    /// # \u8A08\u7B97\u91CF\n    /// - \u79FB\u52D51\u56DE\u3042\u305F\
    \u308A\u306E\u8A08\u7B97\u91CF\u3092 O(\u03B1) \u3068\u3057\u3066 O(\u03B1N\u221A\
    Q)\n    ///   - N: \u914D\u5217\u306E\u9577\u3055\n    ///   - Q: \u30AF\u30A8\
    \u30EA\u306E\u6570\n    fn solve(&mut self, qs: &[(usize, usize)]) -> Vec<Self::Output>\
    \ {\n        let n = qs.iter().map(|&(l, r)| l.max(r)).max().unwrap();\n     \
    \   let q = qs.len();\n        let w = 1.max((n as f64 / 1.0f64.max((q as f64\
    \ * 2.0 / 3.0).sqrt())).round() as usize);\n        let mut ord = (0..q).collect::<Vec<_>>();\n\
    \        ord.sort_unstable_by_key(|&i| {\n            let (l, r) = qs[i];\n  \
    \          (l / w, if (l / w) & 1 == 0 { r } else { !r })\n        });\n     \
    \   let (mut l, mut r) = self.initial_position();\n        let mut res = vec![Default::default();\
    \ q];\n        for i in ord {\n            let (ll, rr) = qs[i];\n           \
    \ while l > ll {\n                l -= 1;\n                self.add_left(l);\n\
    \            }\n            while r < rr {\n                self.add_right(r);\n\
    \                r += 1;\n            }\n            while l < ll {\n        \
    \        self.remove_left(l);\n                l += 1;\n            }\n      \
    \      while r > rr {\n                r -= 1;\n                self.remove_right(r);\n\
    \            }\n            res[i] = self.query();\n        }\n        res\n \
    \   }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/mo/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/static_range_sum/src/main.rs
  - verify/static_range_inversions_query/src/main.rs
documentation_of: crates/algorithm/mo/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/mo/src/lib.rs
- /library/crates/algorithm/mo/src/lib.rs.html
title: crates/algorithm/mo/src/lib.rs
---

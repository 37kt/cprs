---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-arbitrary-mod/src/lib.rs
    title: crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/bostan-mori/src/lib.rs
    title: crates/polynomial/bostan-mori/src/lib.rs
  - icon: ':x:'
    path: crates/polynomial/polynomial-interpolation/src/lib.rs
    title: crates/polynomial/polynomial-interpolation/src/lib.rs
  - icon: ':x:'
    path: crates/polynomial/shift-of-sampling-points/src/lib.rs
    title: crates/polynomial/shift-of-sampling-points/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/exp_of_formal_power_series/src/main.rs
    title: verify/exp_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/inv_of_formal_power_series/src/main.rs
    title: verify/inv_of_formal_power_series/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
    title: verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/log_of_formal_power_series/src/main.rs
    title: verify/log_of_formal_power_series/src/main.rs
  - icon: ':x:'
    path: verify/multipoint_evaluation/src/main.rs
    title: verify/multipoint_evaluation/src/main.rs
  - icon: ':x:'
    path: verify/polynomial_interpolation/src/main.rs
    title: verify/polynomial_interpolation/src/main.rs
  - icon: ':x:'
    path: verify/polynomial_taylor_shift/src/main.rs
    title: verify/polynomial_taylor_shift/src/main.rs
  - icon: ':x:'
    path: verify/pow_of_formal_power_series/src/main.rs
    title: verify/pow_of_formal_power_series/src/main.rs
  - icon: ':x:'
    path: verify/shift_of_sampling_points_of_polynomial/src/main.rs
    title: verify/shift_of_sampling_points_of_polynomial/src/main.rs
  - icon: ':x:'
    path: verify/sqrt_of_formal_power_series/src/main.rs
    title: verify/sqrt_of_formal_power_series/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_arbitrary_mod::convolution_arbitrary_mod;\nuse convolution_ntt_friendly::{convolution_ntt_friendly,\
    \ ntt, ntt_inv};\nuse modint::StaticModInt;\nuse std::{\n    fmt::{Debug, Display},\n\
    \    iter::repeat,\n    mem::swap,\n    ops::{\n        Add, AddAssign, Deref,\
    \ DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Shl,\n      \
    \  ShlAssign, Shr, ShrAssign, Sub, SubAssign,\n    },\n};\n\n#[derive(Default,\
    \ Clone, PartialEq, Eq)]\n#[repr(transparent)]\npub struct FormalPowerSeries<const\
    \ P: u32>(pub Vec<StaticModInt<P>>);\n\n// #[derive(Default, Clone, PartialEq,\
    \ Eq)]\n// #[repr(transparent)]\n// pub struct SparseFormalPowerSeries<const P:\
    \ u32>(pub Vec<(usize, StaticModInt<P>)>);\n\npub type FormalPowerSeries998244353\
    \ = FormalPowerSeries<998_244_353>;\npub type FormalPowerSeries1000000007 = FormalPowerSeries<1_000_000_007>;\n\
    // pub type SparseFormalPowerSeries998244353 = SparseFormalPowerSeries<998_244_353>;\n\
    // pub type SparseFormalPowerSeries1000000007 = SparseFormalPowerSeries<1_000_000_007>;\n\
    \n#[macro_export]\nmacro_rules! fps {\n    ($($x:expr), *) => (\n        $crate::FormalPowerSeries(vec![$(modint::StaticModInt::from($x)),\
    \ *])\n    );\n    ($x:expr; $n:expr) => (\n        $crate::FormalPowerSeries(vec![modint::StaticModInt::from($x);\
    \ $n])\n    );\n}\n\n// #[macro_export]\n// macro_rules! sfps {\n//     ($(($d:expr,\
    \ $x:expr)), *) => (\n//         $crate::SparseFormalPowerSeries(vec![$(($d, modint::StaticModInt::from($x))),\
    \ *])\n//     );\n// }\n\nimpl<const P: u32> FormalPowerSeries<P> {\n    pub fn\
    \ shrink(&mut self) {\n        while self.last() == Some(&0.into()) {\n      \
    \      self.pop();\n        }\n    }\n\n    pub fn pre(&self, d: usize) -> Self\
    \ {\n        Self(self.0[0..d.min(self.len())].to_vec())\n    }\n\n    pub fn\
    \ eval(&self, x: StaticModInt<P>) -> StaticModInt<P> {\n        let mut r = 0.into();\n\
    \        let mut w = StaticModInt::new(1);\n        for &v in &self.0 {\n    \
    \        r += w * v;\n            w *= x;\n        }\n        r\n    }\n\n   \
    \ pub fn differential(&self) -> Self {\n        Self(\n            self.iter()\n\
    \                .enumerate()\n                .skip(1)\n                .map(|(i,\
    \ v)| v * i)\n                .collect(),\n        )\n    }\n\n    pub fn integral(&self)\
    \ -> Self {\n        let n = self.len();\n        let mut res = fps![0; n + 1];\n\
    \        if n > 0 {\n            res[1] = 1.into();\n        }\n        let m\
    \ = StaticModInt::<P>::modulus() as usize;\n        for i in 2..=n {\n       \
    \     res[i] = -res[m % i] * (m / i);\n        }\n        for i in 0..n {\n  \
    \          res[i + 1] *= self[i];\n        }\n        res\n    }\n\n    pub fn\
    \ inv(&self, d: usize) -> Self {\n        assert_ne!(self[0].val(), 0);\n    \
    \    if StaticModInt::<P>::IS_NTT_FRIENDLY {\n            let mut res = fps![0;\
    \ d];\n            res[0] = self[0].inv();\n            for k in 0.. {\n     \
    \           let k = 1 << k;\n                if k >= d {\n                   \
    \ break;\n                }\n                let mut f = Self(self.iter().take(k\
    \ * 2).map(|&x| x).collect());\n                f.resize(k * 2, 0.into());\n \
    \               let mut g = Self(res.iter().take(k).map(|&x| x).collect());\n\
    \                g.resize(k * 2, 0.into());\n                ntt(&mut f);\n  \
    \              ntt(&mut g);\n                for (a, b) in f.iter_mut().zip(g.iter())\
    \ {\n                    *a *= b;\n                }\n                ntt_inv(&mut\
    \ f);\n                for a in f.iter_mut().take(k) {\n                    *a\
    \ = 0.into();\n                }\n                ntt(&mut f);\n             \
    \   for (a, b) in f.iter_mut().zip(g.iter()) {\n                    *a *= b;\n\
    \                }\n                ntt_inv(&mut f);\n                for (a,\
    \ b) in res.iter_mut().zip(f.iter()).skip(k) {\n                    *a = -b;\n\
    \                }\n            }\n            res.truncate(d);\n            res\n\
    \        } else {\n            let mut res = fps![self[0].inv()];\n          \
    \  for k in 0.. {\n                let k = 1 << k;\n                if k >= d\
    \ {\n                    break;\n                }\n                res = (&res\
    \ + &res - &res * &res * self.pre(k * 2)).pre(k * 2);\n            }\n       \
    \     res.truncate(d);\n            res\n        }\n    }\n\n    pub fn log(&self,\
    \ d: usize) -> Self {\n        assert!(self[0].val() == 1);\n        (self.differential()\
    \ * self.inv(d)).pre(d - 1).integral()\n    }\n\n    pub fn exp(&self, d: usize)\
    \ -> Self {\n        assert!(self.len() == 0 || self[0].val() == 0);\n       \
    \ if StaticModInt::<P>::IS_NTT_FRIENDLY {\n            let mut b = fps![1, if\
    \ self.len() > 1 { self[1] } else { 0.into() }];\n            let mut c = fps![1];\n\
    \            let mut z1;\n            let mut z2 = fps![1, 1];\n            for\
    \ m in 1.. {\n                let m = 1 << m;\n                if m >= d {\n \
    \                   break;\n                }\n                let mut y = b.clone();\n\
    \                y.resize(m * 2, 0.into());\n                ntt(&mut y);\n  \
    \              z1 = z2;\n                let mut z = Self((0..m).map(|i| y[i]\
    \ * z1[i]).collect());\n                ntt_inv(&mut z);\n                for\
    \ v in z.iter_mut().take(m / 2) {\n                    *v = 0.into();\n      \
    \          }\n                ntt(&mut z);\n                for i in 0..m {\n\
    \                    z[i] *= -z1[i];\n                }\n                ntt_inv(&mut\
    \ z);\n                c.append(&mut z.drain(m / 2..).collect());\n          \
    \      z2 = c.clone();\n                z2.resize(m * 2, 0.into());\n        \
    \        ntt(&mut z2);\n                let mut x: Self = self.clone().pre(m);\n\
    \                x.resize(m, 0.into());\n                x = x.differential();\n\
    \                x.push(0.into());\n                ntt(&mut x);\n           \
    \     for i in 0..m {\n                    x[i] *= y[i];\n                }\n\
    \                ntt_inv(&mut x);\n                x -= b.differential();\n  \
    \              x.resize(m * 2, 0.into());\n                for i in 0..m - 1 {\n\
    \                    x[m + i] = x[i];\n                    x[i] = 0.into();\n\
    \                }\n                ntt(&mut x);\n                for i in 0..m\
    \ * 2 {\n                    x[i] *= z2[i];\n                }\n             \
    \   ntt_inv(&mut x);\n                x.pop();\n                x = x.integral();\n\
    \                for i in m..self.len().min(m * 2) {\n                    x[i]\
    \ += self[i];\n                }\n                for v in x.iter_mut().take(m)\
    \ {\n                    *v = 0.into();\n                }\n                ntt(&mut\
    \ x);\n                for i in 0..m * 2 {\n                    x[i] *= y[i];\n\
    \                }\n                ntt_inv(&mut x);\n                b.append(&mut\
    \ x.drain(m..).collect());\n            }\n            b.pre(d)\n        } else\
    \ {\n            let mut res = fps![1];\n            for i in 0.. {\n        \
    \        let i = 1 << i;\n                if i >= d {\n                    break;\n\
    \                }\n                let mut t = self.clone().pre(i << 1);\n  \
    \              t[0] += 1;\n                t -= res.log(i << 1);\n           \
    \     res = (res * t).pre(i << 1);\n            }\n            res.pre(d)\n  \
    \      }\n    }\n\n    pub fn pow(&self, k: usize, d: usize) -> FormalPowerSeries<P>\
    \ {\n        let n = self.len();\n        if k == 0 {\n            let mut res\
    \ = fps![0; d];\n            if d > 0 {\n                res[0] = 1.into();\n\
    \            }\n            return res;\n        }\n        for i in 0..n {\n\
    \            if self[i].val() != 0 {\n                let c = self[i].inv();\n\
    \                let mut res = (((self * c) >> i).log(d) * StaticModInt::new(k)).exp(d);\n\
    \                res *= self[i].pow(k);\n                res = (res << i * k).pre(d);\n\
    \                if res.len() < d {\n                    res.resize(d, 0.into());\n\
    \                }\n                return res;\n            }\n            if\
    \ i + 1 >= d / k {\n                return fps![0; d];\n            }\n      \
    \  }\n        fps![0; d]\n    }\n\n    pub fn sqrt(&self, d: usize) -> Option<FormalPowerSeries<P>>\
    \ {\n        if self.len() == 0 {\n            return Some(fps![0; d]);\n    \
    \    }\n        if self[0].val() == 0 {\n            if let Some(i) = self.iter().position(|&x|\
    \ x.val() != 0) {\n                if i & 1 != 0 {\n                    return\
    \ None;\n                } else if d <= i / 2 {\n                    return Some(fps![0;\
    \ d]);\n                }\n                let mut res = (self >> i).sqrt(d -\
    \ i / 2)?;\n                res <<= i / 2;\n                if res.len() < d {\n\
    \                    res.resize(d, 0.into());\n                }\n           \
    \     return Some(res);\n            }\n            return Some(fps![0; d]);\n\
    \        }\n\n        let r = self[0].sqrt()?;\n        assert_eq!(r * r, self[0]);\n\
    \        let mut res = fps![r];\n        let inv2 = StaticModInt::new(2).inv();\n\
    \        for i in 0.. {\n            let i = 1 << i;\n            if i >= d {\n\
    \                break;\n            }\n            res = (&res + self.clone().pre(i\
    \ << 1) * res.inv(i << 1)) * inv2;\n        }\n        Some(res.pre(d))\n    }\n\
    \n    pub fn multipoint_evaluate(&self, xs: &[StaticModInt<P>]) -> Vec<StaticModInt<P>>\
    \ {\n        let m = xs.len();\n        if m == 0 {\n            return vec![];\n\
    \        }\n        let m2 = 1 << 64 - (m - 1).leading_zeros();\n        let mut\
    \ g = vec![fps![1]; m2 + m2];\n        for i in 0..m {\n            g[m2 + i]\
    \ = fps![-xs[i], 1];\n        }\n        for i in (1..m2).rev() {\n          \
    \  g[i] = &g[i << 1 | 0] * &g[i << 1 | 1];\n        }\n        g[1] = self % &g[1];\n\
    \        for i in 2..m2 + m {\n            g[i] = &g[i >> 1] % &g[i];\n      \
    \  }\n        (m2..m2 + m)\n            .map(|i| if g[i].len() == 0 { 0.into()\
    \ } else { g[i][0] })\n            .collect()\n    }\n\n    /// f(x + c)\n   \
    \ pub fn taylor_shift(mut self, c: StaticModInt<P>) -> Self {\n        if self.len()\
    \ == 0 {\n            return self;\n        }\n        let n = self.len();\n \
    \       let mut fact = vec![StaticModInt::new(1); n];\n        let mut inv = vec![StaticModInt::new(1);\
    \ n];\n        let mut fact_inv = vec![StaticModInt::new(1); n];\n        for\
    \ i in 1..n {\n            fact[i] = fact[i - 1] * i;\n        }\n        fact_inv[n\
    \ - 1] = fact[n - 1].inv();\n        for i in (1..n).rev() {\n            inv[i]\
    \ = fact_inv[i] * fact[i - 1];\n            fact_inv[i - 1] = fact_inv[i] * i;\n\
    \        }\n        for i in 0..n {\n            self[i] *= fact[i];\n       \
    \ }\n        self.reverse();\n        let mut g = fps![1; n];\n        for i in\
    \ 1..n {\n            g[i] = g[i - 1] * c * inv[i];\n        }\n        self =\
    \ (self * g).pre(n);\n        self.reverse();\n        for i in 0..n {\n     \
    \       self[i] *= fact_inv[i];\n        }\n        self\n    }\n}\n\n// impl<const\
    \ P: u32> SparseFormalPowerSeries<P> {\n//     pub fn normalize(&mut self) {\n\
    //         if self.len() == 0 {\n//             return;\n//         }\n//    \
    \     self.0.sort_by_key(|&(i, _)| i);\n//         let mut res = Self(vec![(self[0].0,\
    \ StaticModInt::new(0))]);\n//         for &(i, v) in &self.0 {\n//          \
    \   if res.len() == 0 || res.last().unwrap().0 != i {\n//                 res.push((i,\
    \ v));\n//             } else {\n//                 res.last_mut().unwrap().1\
    \ += v;\n//             }\n//         }\n//         if res.len() != 0 && res.last().unwrap().1.val()\
    \ == 0 {\n//             res.pop();\n//         }\n//         *self = res;\n//\
    \     }\n\n//     pub fn differential(&self) -> Self {\n//         Self(\n// \
    \            self.iter()\n//                 .filter_map(|&(i, v)| (i > 0).then(||\
    \ (i - 1, v * i)))\n//                 .collect(),\n//         )\n//     }\n\n\
    //     pub fn integral(&self) -> Self {\n//         Self(self.iter().map(|&(i,\
    \ v)| (i + 1, v / (i + 1))).collect())\n//     }\n\n//     pub fn inv(self, d:\
    \ usize) -> FormalPowerSeries<P> {\n//         let mut f = fps![0; d];\n//   \
    \      f[0] += 1;\n//         f /= self;\n//         f\n//     }\n\n//     pub\
    \ fn log(self, d: usize) -> FormalPowerSeries<P> {\n//         assert!(self[0].0\
    \ == 0 && self[0].1.val() == 1);\n//         let f = self.differential();\n//\
    \         let mut res = (self.inv(d) * f).pre(d - 1).integral();\n//         res.resize(d,\
    \ 0.into());\n//         res\n//     }\n\n//     pub fn exp(&self, d: usize) ->\
    \ FormalPowerSeries<P> {\n//         if self.len() == 0 {\n//             let\
    \ mut res = fps![0; d];\n//             if d > 0 {\n//                 res[0]\
    \ = 1.into();\n//             }\n//             return res;\n//         }\n//\
    \         assert_ne!(self[0].0, 0);\n//         let mut res = fps![0; d];\n//\
    \         if d == 0 {\n//             return res;\n//         }\n//         let\
    \ mut a = self.differential();\n//         for (d, _) in a.iter_mut() {\n//  \
    \           *d += 1;\n//         }\n//         res[0] = 1.into();\n//        \
    \ let mut inv = vec![StaticModInt::<P>::new(1); d];\n//         let m = StaticModInt::<P>::modulus()\
    \ as usize;\n//         for i in 1..d {\n//             if i > 1 {\n//       \
    \          inv[i] = -inv[m % i] * (m / i);\n//             }\n//             res[i]\
    \ = a\n//                 .iter()\n//                 .filter_map(|&(j, v)| (i\
    \ >= j).then(|| v * res[i - j]))\n//                 .sum::<StaticModInt<P>>()\n\
    //                 * inv[i];\n//         }\n//         res\n//     }\n\n//   \
    \  pub fn pow(&self, k: usize, d: usize) -> FormalPowerSeries<P> {\n//       \
    \  let offset = self.iter().position(|&(_, v)| v.val() != 0);\n//         let\
    \ mut res = fps![0; d];\n//         if offset.is_none() {\n//             if k\
    \ == 0 {\n//                 res[0] += 1;\n//             }\n//             return\
    \ res;\n//         }\n//         let offset = offset.unwrap();\n//         if\
    \ self[offset].0 > 0 {\n//             let deg = self[offset].0;\n//         \
    \    if k > (d - 1) / deg {\n//                 return res;\n//             }\n\
    //             let g = Self(\n//                 self.iter()\n//             \
    \        .filter_map(|&(i, v)| (i >= deg).then(|| (i - deg, v)))\n//         \
    \            .collect(),\n//             );\n//             let t = g.pow(k, d\
    \ - k * deg);\n//             for i in 0..d - k * deg {\n//                 res[k\
    \ * deg + i] = t[i];\n//             }\n//             return res;\n//       \
    \  }\n//         let mut inv = vec![StaticModInt::<P>::new(1); d + 1];\n//   \
    \      let m = P as usize;\n//         for i in 2..=d {\n//             inv[i]\
    \ = -inv[m % i] * (m / i);\n//         }\n//         res[0] = self[0].1.pow(k);\n\
    //         let c = self[0].1.inv();\n//         for i in 1..d {\n//          \
    \   for &(j, v) in self.iter().skip(1).filter(|&&(j, _)| i >= j) {\n//       \
    \          res[i] = res[i] + v * res[i - j] * (StaticModInt::<P>::new(k) * j -\
    \ (i - j));\n//             }\n//             res[i] *= inv[i] * c;\n//      \
    \   }\n//         res\n//     }\n\n//     pub fn sqrt(&self, d: usize) -> Option<FormalPowerSeries<P>>\
    \ {\n//         if self.len() == 0 {\n//             return Some(fps![0; d]);\n\
    //         }\n//         let p = self[0].0;\n//         if p & 1 != 0 {\n//  \
    \           return None;\n//         } else if p / 2 >= d {\n//             return\
    \ Some(fps![0; d]);\n//         }\n//         let inv_f0 = self[0].1.inv();\n\
    //         let lz = p / 2;\n//         let mut g = fps![0; d];\n//         g[lz]\
    \ = self[0].1.sqrt()?;\n//         let k = StaticModInt::new(2).inv();\n//   \
    \      let mut inv = vec![StaticModInt::new(1); d];\n//         let m = P as usize;\n\
    //         for i in 2..d {\n//             inv[i] = -inv[m % i] * (m / i);\n//\
    \         }\n//         for i in 1..d - lz {\n//             g[lz + i] = self\n\
    //                 .iter()\n//                 .skip(1)\n//                 .filter_map(|&(j,\
    \ v)| (j - p <= i).then(|| (j - p, v)))\n//                 .map(|(j, v)| v *\
    \ g[lz + i - j] * (k * j - (i - j)))\n//                 .sum::<StaticModInt<P>>()\n\
    //                 * inv[i]\n//                 * inv_f0;\n//         }\n//  \
    \       Some(g)\n//     }\n// }\n\nimpl<const P: u32> Debug for FormalPowerSeries<P>\
    \ {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n\
    \        f.write_fmt(format_args!(\"{:?}\", &self.0))\n    }\n}\n\nimpl<const\
    \ P: u32> Display for FormalPowerSeries<P> {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)\
    \ -> std::fmt::Result {\n        if self.len() != 0 {\n            f.write_fmt(format_args!(\"\
    {}\", self[0]))?;\n        }\n        for v in self.iter().skip(1) {\n       \
    \     f.write_fmt(format_args!(\" {}\", v))?;\n        }\n        Ok(())\n   \
    \ }\n}\n\n// impl<const P: u32> Debug for SparseFormalPowerSeries<P> {\n//   \
    \  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n//  \
    \       f.write_fmt(format_args!(\"{:?}\", &self.0))\n//     }\n// }\n\nimpl<const\
    \ P: u32> Deref for FormalPowerSeries<P> {\n    type Target = Vec<StaticModInt<P>>;\n\
    \    fn deref(&self) -> &Self::Target {\n        &self.0\n    }\n}\n\n// impl<const\
    \ P: u32> Deref for SparseFormalPowerSeries<P> {\n//     type Target = Vec<(usize,\
    \ StaticModInt<P>)>;\n//     fn deref(&self) -> &Self::Target {\n//         &self.0\n\
    //     }\n// }\n\nimpl<const P: u32> DerefMut for FormalPowerSeries<P> {\n   \
    \ fn deref_mut(&mut self) -> &mut Self::Target {\n        &mut self.0\n    }\n\
    }\n\n// impl<const P: u32> DerefMut for SparseFormalPowerSeries<P> {\n//     fn\
    \ deref_mut(&mut self) -> &mut Self::Target {\n//         &mut self.0\n//    \
    \ }\n// }\n\nimpl<const P: u32> From<Vec<StaticModInt<P>>> for FormalPowerSeries<P>\
    \ {\n    fn from(v: Vec<StaticModInt<P>>) -> Self {\n        Self(v)\n    }\n\
    }\n\n// impl<const P: u32> From<Vec<(usize, StaticModInt<P>)>> for SparseFormalPowerSeries<P>\
    \ {\n//     fn from(v: Vec<(usize, StaticModInt<P>)>) -> Self {\n//         Self(v)\n\
    //     }\n// }\n\n// impl<const P: u32> From<SparseFormalPowerSeries<P>> for FormalPowerSeries<P>\
    \ {\n//     fn from(f: SparseFormalPowerSeries<P>) -> Self {\n//         if f.len()\
    \ == 0 {\n//             return fps![];\n//         }\n//         let mut g =\
    \ FormalPowerSeries(vec![0.into(); f.last().unwrap().0 + 1]);\n//         for\
    \ (i, v) in f.0 {\n//             g[i] += v;\n//         }\n//         g\n// \
    \    }\n// }\n\n// impl<const P: u32> From<FormalPowerSeries<P>> for SparseFormalPowerSeries<P>\
    \ {\n//     fn from(f: FormalPowerSeries<P>) -> Self {\n//         Self(f.0.into_iter().enumerate().collect())\n\
    //     }\n// }\n\nimpl<const P: u32> Neg for FormalPowerSeries<P> {\n    type\
    \ Output = Self;\n    fn neg(mut self) -> Self::Output {\n        for v in self.iter_mut()\
    \ {\n            *v = -*v;\n        }\n        self\n    }\n}\n\n// impl<const\
    \ P: u32> Neg for SparseFormalPowerSeries<P> {\n//     type Output = Self;\n//\
    \     fn neg(mut self) -> Self::Output {\n//         for (_, v) in self.iter_mut()\
    \ {\n//             *v = -*v;\n//         }\n//         self\n//     }\n// }\n\
    \nimpl<const P: u32> Neg for &FormalPowerSeries<P> {\n    type Output = FormalPowerSeries<P>;\n\
    \    fn neg(self) -> Self::Output {\n        -self.clone()\n    }\n}\n\n// impl<const\
    \ P: u32> Neg for &SparseFormalPowerSeries<P> {\n//     type Output = SparseFormalPowerSeries<P>;\n\
    //     fn neg(self) -> Self::Output {\n//         -self.clone()\n//     }\n//\
    \ }\n\nimpl<const P: u32> MulAssign<StaticModInt<P>> for FormalPowerSeries<P>\
    \ {\n    fn mul_assign(&mut self, rhs: StaticModInt<P>) {\n        for v in self.iter_mut()\
    \ {\n            *v *= rhs;\n        }\n    }\n}\n\n// impl<const P: u32> MulAssign<StaticModInt<P>>\
    \ for SparseFormalPowerSeries<P> {\n//     fn mul_assign(&mut self, rhs: StaticModInt<P>)\
    \ {\n//         for (_, v) in self.iter_mut() {\n//             *v *= rhs;\n//\
    \         }\n//     }\n// }\n\nimpl<const P: u32> DivAssign<StaticModInt<P>> for\
    \ FormalPowerSeries<P> {\n    fn div_assign(&mut self, rhs: StaticModInt<P>) {\n\
    \        *self *= rhs.inv();\n    }\n}\n\n// impl<const P: u32> DivAssign<StaticModInt<P>>\
    \ for SparseFormalPowerSeries<P> {\n//     fn div_assign(&mut self, rhs: StaticModInt<P>)\
    \ {\n//         *self *= rhs.inv();\n//     }\n// }\n\nimpl<const P: u32> AddAssign<Self>\
    \ for FormalPowerSeries<P> {\n    fn add_assign(&mut self, rhs: Self) {\n    \
    \    if self.len() < rhs.len() {\n            self.resize(rhs.len(), 0.into());\n\
    \        }\n        self.iter_mut().zip(rhs.iter()).for_each(|(a, b)| *a += b);\n\
    \    }\n}\n\n// impl<const P: u32> AddAssign<Self> for SparseFormalPowerSeries<P>\
    \ {\n//     fn add_assign(&mut self, rhs: Self) {\n//         let mut res = sfps![];\n\
    //         let n = self.len();\n//         let m = rhs.len();\n//         let\
    \ mut i = 0;\n//         let mut j = 0;\n//         while i < n && j < m {\n//\
    \             if j == m {\n//                 res.push(self[i]);\n//         \
    \        i += 1;\n//             } else if i == n {\n//                 res.push(rhs[j]);\n\
    //                 j += 1;\n//             } else if self[i].0 == rhs[j].0 {\n\
    //                 res.push((self[i].0, self[i].1 + rhs[j].1));\n//          \
    \       i += 1;\n//                 j += 1;\n//             } else if self[i].0\
    \ < rhs[j].0 {\n//                 res.push(self[i]);\n//                 i +=\
    \ 1;\n//             } else {\n//                 res.push(rhs[j]);\n//      \
    \           j += 1;\n//             }\n//         }\n//         *self = res;\n\
    //     }\n// }\n\n// impl<const P: u32> AddAssign<SparseFormalPowerSeries<P>>\
    \ for FormalPowerSeries<P> {\n//     fn add_assign(&mut self, rhs: SparseFormalPowerSeries<P>)\
    \ {\n//         if rhs.len() == 0 {\n//             return;\n//         }\n//\
    \         let m = rhs.last().unwrap().0 + 1;\n//         if self.len() < m {\n\
    //             self.resize(m, 0.into());\n//         }\n//         for (i, v)\
    \ in rhs.0 {\n//             self[i] += v;\n//         }\n//     }\n// }\n\nimpl<const\
    \ P: u32> SubAssign<Self> for FormalPowerSeries<P> {\n    fn sub_assign(&mut self,\
    \ rhs: Self) {\n        if self.len() < rhs.len() {\n            self.resize(rhs.len(),\
    \ 0.into());\n        }\n        self.iter_mut().zip(rhs.iter()).for_each(|(a,\
    \ b)| *a -= b);\n    }\n}\n\n// impl<const P: u32> SubAssign<Self> for SparseFormalPowerSeries<P>\
    \ {\n//     fn sub_assign(&mut self, rhs: Self) {\n//         *self += -rhs;\n\
    //     }\n// }\n\n// impl<const P: u32> SubAssign<SparseFormalPowerSeries<P>>\
    \ for FormalPowerSeries<P> {\n//     fn sub_assign(&mut self, rhs: SparseFormalPowerSeries<P>)\
    \ {\n//         *self += -rhs;\n//     }\n// }\n\nimpl<const P: u32> MulAssign<Self>\
    \ for FormalPowerSeries<P> {\n    fn mul_assign(&mut self, rhs: Self) {\n    \
    \    if StaticModInt::<P>::IS_NTT_FRIENDLY {\n            let mut a = vec![];\n\
    \            swap(&mut a, &mut self.0);\n            self.0 = convolution_ntt_friendly(a,\
    \ rhs.0);\n        } else {\n            self.0 = convolution_arbitrary_mod(&self.0,\
    \ &rhs.0);\n        }\n    }\n}\n\n// impl<const P: u32> MulAssign<Self> for SparseFormalPowerSeries<P>\
    \ {\n//     fn mul_assign(&mut self, rhs: Self) {\n//         self.0 = self\n\
    //             .0\n//             .drain(..)\n//             .map(|(i, x)| rhs.iter().map(move\
    \ |(j, y)| (i + j, x * y)))\n//             .flatten()\n//             .collect::<Vec<_>>();\n\
    //         self.normalize();\n//     }\n// }\n\n// impl<const P: u32> MulAssign<SparseFormalPowerSeries<P>>\
    \ for FormalPowerSeries<P> {\n//     fn mul_assign(&mut self, rhs: SparseFormalPowerSeries<P>)\
    \ {\n//         if self.len() == 0 || rhs.len() == 0 {\n//             self.clear();\n\
    //             return;\n//         }\n//         let n = self.len();\n//     \
    \    self.resize(n + rhs.last().unwrap().0, 0.into());\n//         let c = if\
    \ rhs[0].0 == 0 { rhs[0].1 } else { 0.into() };\n//         for i in (0..n).rev()\
    \ {\n//             for &(j, v) in rhs.iter().rev() {\n//                 if j\
    \ == 0 {\n//                     continue;\n//                 }\n//         \
    \        self[i + j] = self[i + j] + self[i] * v;\n//             }\n//      \
    \       self[i] *= c;\n//         }\n//     }\n// }\n\nimpl<const P: u32> DivAssign<Self>\
    \ for FormalPowerSeries<P> {\n    fn div_assign(&mut self, mut g: Self) {\n  \
    \      if self.len() < g.len() {\n            self.clear();\n            return;\n\
    \        }\n        let n = self.len() + 1 - g.len();\n        if g.len() <= 64\
    \ {\n            let mut f = FormalPowerSeries(vec![]);\n            swap(&mut\
    \ f.0, &mut self.0);\n            g.shrink();\n            let coef = g.last().unwrap().inv();\n\
    \            for x in g.iter_mut() {\n                *x *= coef;\n          \
    \  }\n            let d = f.len() + 1 - g.len();\n            let m = g.len();\n\
    \            let mut quo = FormalPowerSeries(vec![StaticModInt::new(0); d]);\n\
    \            for i in (0..d).rev() {\n                quo[i] = f[i + m - 1];\n\
    \                for j in 0..m {\n                    f[i + j] -= quo[i] * g[j];\n\
    \                }\n            }\n            *self = quo * coef;\n         \
    \   self.resize(n, 0.into());\n        } else {\n            self.reverse();\n\
    \            self.truncate(n);\n            g.reverse();\n            *self *=\
    \ g.inv(n);\n            self.truncate(n);\n            self.reverse();\n    \
    \    }\n    }\n}\n\n// impl<const P: u32> DivAssign<SparseFormalPowerSeries<P>>\
    \ for FormalPowerSeries<P> {\n//     fn div_assign(&mut self, mut rhs: SparseFormalPowerSeries<P>)\
    \ {\n//         assert_eq!(rhs[0].0, 0);\n//         let c = rhs[0].1.inv();\n\
    //         self.iter_mut().for_each(|v| *v *= c);\n//         rhs.iter_mut().for_each(|(_,\
    \ v)| *v *= c);\n//         for i in 0..self.len() {\n//             for &(j,\
    \ v) in rhs.iter().filter(|&&(j, _)| j > 0 && i >= j) {\n//                 self[i]\
    \ = self[i] - self[i - j] * v;\n//             }\n//         }\n//     }\n// }\n\
    \nimpl<const P: u32> RemAssign<Self> for FormalPowerSeries<P> {\n    fn rem_assign(&mut\
    \ self, rhs: Self) {\n        *self -= self.clone() / &rhs * &rhs;\n        self.shrink();\n\
    \    }\n}\n\nimpl<const P: u32> ShlAssign<usize> for FormalPowerSeries<P> {\n\
    \    fn shl_assign(&mut self, rhs: usize) {\n        self.0 = repeat(0.into()).take(rhs).chain(self.0.drain(..)).collect();\n\
    \    }\n}\n\n// impl<const P: u32> ShlAssign<usize> for SparseFormalPowerSeries<P>\
    \ {\n//     fn shl_assign(&mut self, rhs: usize) {\n//         self.iter_mut().for_each(|(i,\
    \ _)| *i += rhs);\n//     }\n// }\n\nimpl<const P: u32> ShrAssign<usize> for FormalPowerSeries<P>\
    \ {\n    fn shr_assign(&mut self, rhs: usize) {\n        self.0 = self.0.drain(rhs..).collect();\n\
    \    }\n}\n\n// impl<const P: u32> ShrAssign<usize> for SparseFormalPowerSeries<P>\
    \ {\n//     fn shr_assign(&mut self, rhs: usize) {\n//         self.0 = self\n\
    //             .0\n//             .drain(..)\n//             .filter_map(|(i,\
    \ v)| (i >= rhs).then(|| (i - rhs, v)))\n//             .collect();\n//     }\n\
    // }\n\nmacro_rules! impl_ops {\n    ($(\n        $ty_l:ty,\n        $ty_r:ty,\n\
    \        $trait:ident,\n        $trait_assign:ident,\n        $fn:ident,\n   \
    \     $fn_assign:ident,\n    )*) => {$(\n        impl<const P: u32> $trait_assign<&$ty_r>\
    \ for $ty_l {\n            fn $fn_assign(&mut self, rhs: &$ty_r) {\n         \
    \       self.$fn_assign(rhs.clone());\n            }\n        }\n        impl<const\
    \ P: u32> $trait<$ty_r> for $ty_l {\n            type Output = $ty_l;\n      \
    \      fn $fn(mut self, rhs: $ty_r) -> $ty_l {\n                self.$fn_assign(rhs);\n\
    \                self\n            }\n        }\n        impl<const P: u32> $trait<$ty_r>\
    \ for &$ty_l {\n            type Output = $ty_l;\n            fn $fn(self, rhs:\
    \ $ty_r) -> $ty_l {\n                let mut r = self.clone();\n             \
    \   r.$fn_assign(rhs);\n                r\n            }\n        }\n        impl<const\
    \ P: u32> $trait<&$ty_r> for $ty_l {\n            type Output = $ty_l;\n     \
    \       fn $fn(mut self, rhs: &$ty_r) -> $ty_l {\n                self.$fn_assign(rhs.clone());\n\
    \                self\n            }\n        }\n        impl<const P: u32> $trait<&$ty_r>\
    \ for &$ty_l {\n            type Output = $ty_l;\n            fn $fn(self, rhs:\
    \ &$ty_r) -> $ty_l {\n                let mut r = self.clone();\n            \
    \    r.$fn_assign(rhs.clone());\n                r\n            }\n        }\n\
    \    )*};\n}\n\nimpl_ops! {\n    FormalPowerSeries<P>, StaticModInt<P>, Mul, MulAssign,\
    \ mul, mul_assign,\n    FormalPowerSeries<P>, StaticModInt<P>, Div, DivAssign,\
    \ div, div_assign,\n    FormalPowerSeries<P>, FormalPowerSeries<P>, Add, AddAssign,\
    \ add, add_assign,\n    FormalPowerSeries<P>, FormalPowerSeries<P>, Sub, SubAssign,\
    \ sub, sub_assign,\n    FormalPowerSeries<P>, FormalPowerSeries<P>, Mul, MulAssign,\
    \ mul, mul_assign,\n    FormalPowerSeries<P>, FormalPowerSeries<P>, Div, DivAssign,\
    \ div, div_assign,\n    FormalPowerSeries<P>, FormalPowerSeries<P>, Rem, RemAssign,\
    \ rem, rem_assign,\n    FormalPowerSeries<P>, usize, Shl, ShlAssign, shl, shl_assign,\n\
    \    FormalPowerSeries<P>, usize, Shr, ShrAssign, shr, shr_assign,\n    // SparseFormalPowerSeries<P>,\
    \ StaticModInt<P>, Mul, MulAssign, mul, mul_assign,\n    // SparseFormalPowerSeries<P>,\
    \ StaticModInt<P>, Div, DivAssign, div, div_assign,\n    // SparseFormalPowerSeries<P>,\
    \ SparseFormalPowerSeries<P>, Add, AddAssign, add, add_assign,\n    // SparseFormalPowerSeries<P>,\
    \ SparseFormalPowerSeries<P>, Sub, SubAssign, sub, sub_assign,\n    // SparseFormalPowerSeries<P>,\
    \ SparseFormalPowerSeries<P>, Mul, MulAssign, mul, mul_assign,\n    // SparseFormalPowerSeries<P>,\
    \ usize, Shl, ShlAssign, shl, shl_assign,\n    // SparseFormalPowerSeries<P>,\
    \ usize, Shr, ShrAssign, shr, shr_assign,\n    // FormalPowerSeries<P>, SparseFormalPowerSeries<P>,\
    \ Add, AddAssign, add, add_assign,\n    // FormalPowerSeries<P>, SparseFormalPowerSeries<P>,\
    \ Sub, SubAssign, sub, sub_assign,\n    // FormalPowerSeries<P>, SparseFormalPowerSeries<P>,\
    \ Mul, MulAssign, mul, mul_assign,\n    // FormalPowerSeries<P>, SparseFormalPowerSeries<P>,\
    \ Div, DivAssign, div, div_assign,\n}\n"
  dependsOn:
  - crates/convolution/convolution-arbitrary-mod/src/lib.rs
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/formal-power-series/src/lib.rs
  requiredBy:
  - crates/polynomial/polynomial-interpolation/src/lib.rs
  - crates/polynomial/shift-of-sampling-points/src/lib.rs
  - crates/polynomial/bostan-mori/src/lib.rs
  timestamp: '2023-09-16 20:40:18+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/inv_of_formal_power_series/src/main.rs
  - verify/multipoint_evaluation/src/main.rs
  - verify/log_of_formal_power_series/src/main.rs
  - verify/polynomial_taylor_shift/src/main.rs
  - verify/kth_term_of_linearly_recurrent_sequence/src/main.rs
  - verify/exp_of_formal_power_series/src/main.rs
  - verify/shift_of_sampling_points_of_polynomial/src/main.rs
  - verify/pow_of_formal_power_series/src/main.rs
  - verify/sqrt_of_formal_power_series/src/main.rs
  - verify/polynomial_interpolation/src/main.rs
documentation_of: crates/polynomial/formal-power-series/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/formal-power-series/src/lib.rs
- /library/crates/polynomial/formal-power-series/src/lib.rs.html
title: crates/polynomial/formal-power-series/src/lib.rs
---

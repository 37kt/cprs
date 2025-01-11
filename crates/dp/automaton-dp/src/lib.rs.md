---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
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
  code: "use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};\n\n\
    use algebraic::{One, Zero};\n\npub trait DeterministicFiniteAutoMaton {\n    type\
    \ Symbol;\n    type State;\n\n    fn initial_state(&self) -> Self::State;\n  \
    \  fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>;\n\
    \    fn is_accepting(&self, state: &Self::State) -> bool;\n\n    #[allow(unused_variables)]\n\
    \    fn state_to_index(&self, state: &Self::State) -> usize {\n        unimplemented!()\n\
    \    }\n\n    fn state_num(&self) -> usize {\n        unimplemented!()\n    }\n\
    \n    fn count_accepting_hashmap<T>(\n        &self,\n        alphabet: impl Iterator<Item\
    \ = Self::Symbol> + Clone,\n        len: usize,\n    ) -> T\n    where\n     \
    \   Self::State: Eq + std::hash::Hash,\n        T: Zero + One + std::ops::Add<Output\
    \ = T> + Clone,\n    {\n        self.dp_hashmap(\n            alphabet,\n    \
    \        len,\n            |_, x| x.clone(),\n            |x, y| x.clone() + y.clone(),\n\
    \            T::zero(),\n            T::one(),\n        )\n    }\n\n    fn count_accepting_vec<T>(\n\
    \        &self,\n        alphabet: impl Iterator<Item = Self::Symbol> + Clone,\n\
    \        len: usize,\n    ) -> T\n    where\n        T: Zero + One + std::ops::Add<Output\
    \ = T> + Clone,\n    {\n        self.dp_vec(\n            alphabet,\n        \
    \    len,\n            |_, x| x.clone(),\n            |x, y| x.clone() + y.clone(),\n\
    \            T::zero(),\n            T::one(),\n        )\n    }\n\n    fn dp_hashmap<T>(\n\
    \        &self,\n        alphabet: impl Iterator<Item = Self::Symbol> + Clone,\n\
    \        len: usize,\n        act: impl Fn(&Self::Symbol, &T) -> T,\n        op:\
    \ impl Fn(&T, &T) -> T,\n        e: T,\n        dp_initial: T,\n    ) -> T\n \
    \   where\n        Self::State: Eq + std::hash::Hash,\n        T: Clone,\n   \
    \ {\n        let mut dp = HashMap::<Self::State, T>::new();\n        let mut ndp\
    \ = HashMap::<Self::State, T>::new();\n        dp.insert(self.initial_state(),\
    \ dp_initial);\n\n        for _ in 0..len {\n            for (st, val) in dp.drain()\
    \ {\n                for c in alphabet.clone() {\n                    let Some(nst)\
    \ = self.transition(&st, &c) else { continue; };\n                    let nval\
    \ = act(&c, &val);\n                    ndp.entry(nst)\n                     \
    \   .and_modify(|sum| *sum = op(sum, &nval))\n                        .or_insert(nval);\n\
    \                }\n            }\n            std::mem::swap(&mut dp, &mut ndp);\n\
    \        }\n\n        dp.into_iter()\n            .filter(|(st, _)| self.is_accepting(st))\n\
    \            .fold(e, |acc, (_, val)| op(&acc, &val))\n    }\n\n    fn dp_vec<T>(\n\
    \        &self,\n        alphabet: impl Iterator<Item = Self::Symbol> + Clone,\n\
    \        len: usize,\n        act: impl Fn(&Self::Symbol, &T) -> T,\n        op:\
    \ impl Fn(&T, &T) -> T,\n        e: T,\n        dp_initial: T,\n    ) -> T\n \
    \   where\n        T: Clone,\n    {\n        let mut dp = (0..self.state_num()).map(|_|\
    \ None).collect::<Vec<_>>();\n        dp[self.state_to_index(&self.initial_state())]\
    \ = Some((self.initial_state(), dp_initial));\n\n        for _ in 0..len {\n \
    \           let mut ndp = (0..self.state_num()).map(|_| None).collect::<Vec<_>>();\n\
    \            for (st, val) in dp.into_iter().flatten() {\n                for\
    \ c in alphabet.clone() {\n                    let Some(nst) = self.transition(&st,\
    \ &c) else { continue; };\n                    let nval = act(&c, &val);\n   \
    \                 let i = self.state_to_index(&nst);\n                    if let\
    \ Some((_, sum)) = ndp[i].as_mut() {\n                        *sum = op(sum, &nval);\n\
    \                    } else {\n                        ndp[i] = Some((nst, nval));\n\
    \                    }\n                }\n            }\n            dp = ndp;\n\
    \        }\n\n        dp.into_iter()\n            .flatten()\n            .filter(|(st,\
    \ _)| self.is_accepting(st))\n            .fold(e, |acc, (_, val)| op(&acc, &val))\n\
    \    }\n}\n\n#[derive(Clone, Copy)]\npub struct NotDFA<A>\nwhere\n    A: DeterministicFiniteAutoMaton,\n\
    {\n    dfa: A,\n}\n\nimpl<A> DeterministicFiniteAutoMaton for NotDFA<A>\nwhere\n\
    \    A: DeterministicFiniteAutoMaton,\n{\n    type Symbol = A::Symbol;\n    type\
    \ State = A::State;\n\n    fn initial_state(&self) -> Self::State {\n        self.dfa.initial_state()\n\
    \    }\n\n    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        self.dfa.transition(state, c)\n    }\n\n    fn is_accepting(&self,\
    \ state: &Self::State) -> bool {\n        !self.dfa.is_accepting(state)\n    }\n\
    \n    fn state_to_index(&self, state: &Self::State) -> usize {\n        self.dfa.state_to_index(state)\n\
    \    }\n\n    fn state_num(&self) -> usize {\n        self.dfa.state_num()\n \
    \   }\n}\n\n#[derive(Clone, Copy)]\npub struct OperationDFA<A0, A1, Op>\nwhere\n\
    \    A0: DeterministicFiniteAutoMaton,\n    A1: DeterministicFiniteAutoMaton<Symbol\
    \ = A0::Symbol>,\n    Op: Fn(bool, bool) -> bool,\n{\n    dfa0: A0,\n    dfa1:\
    \ A1,\n    op: Op,\n}\n\nimpl<A0, A1, Op> DeterministicFiniteAutoMaton for OperationDFA<A0,\
    \ A1, Op>\nwhere\n    A0: DeterministicFiniteAutoMaton,\n    A1: DeterministicFiniteAutoMaton<Symbol\
    \ = A0::Symbol>,\n    Op: Fn(bool, bool) -> bool,\n{\n    type Symbol = A0::Symbol;\n\
    \    type State = (A0::State, A1::State);\n\n    fn initial_state(&self) -> Self::State\
    \ {\n        (self.dfa0.initial_state(), self.dfa1.initial_state())\n    }\n\n\
    \    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        let (st0, st1) = state;\n        let st0 = self.dfa0.transition(st0,\
    \ c)?;\n        let st1 = self.dfa1.transition(st1, c)?;\n        Some((st0, st1))\n\
    \    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n        let\
    \ (st0, st1) = state;\n        (self.op)(self.dfa0.is_accepting(st0), self.dfa1.is_accepting(st1))\n\
    \    }\n\n    fn state_to_index(&self, state: &Self::State) -> usize {\n     \
    \   let (st0, st1) = state;\n        self.dfa0.state_to_index(st0) * self.dfa1.state_num()\
    \ + self.dfa1.state_to_index(st1)\n    }\n\n    fn state_num(&self) -> usize {\n\
    \        self.dfa0.state_num() * self.dfa1.state_num()\n    }\n}\n\nimpl<A0, A1,\
    \ Op> OperationDFA<A0, A1, Op>\nwhere\n    A0: DeterministicFiniteAutoMaton,\n\
    \    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,\n    Op: Fn(bool,\
    \ bool) -> bool,\n{\n    pub fn new(dfa0: A0, dfa1: A1, op: Op) -> Self {\n  \
    \      Self { dfa0, dfa1, op }\n    }\n}\n\nimpl<A0, A1> OperationDFA<A0, A1,\
    \ fn(bool, bool) -> bool>\nwhere\n    A0: DeterministicFiniteAutoMaton,\n    A1:\
    \ DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,\n{\n    pub fn or(dfa0: A0,\
    \ dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| a || b,\n        }\n    }\n\n    pub fn and(dfa0: A0,\
    \ dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| a && b,\n        }\n    }\n\n    pub fn xor(dfa0: A0,\
    \ dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| a ^ b,\n        }\n    }\n\n    pub fn nand(dfa0: A0,\
    \ dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| !(a && b),\n        }\n    }\n\n    pub fn nor(dfa0: A0,\
    \ dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| !(a || b),\n        }\n    }\n\n    pub fn xnor(dfa0:\
    \ A0, dfa1: A1) -> Self {\n        Self {\n            dfa0,\n            dfa1,\n\
    \            op: |a, b| !(a ^ b),\n        }\n    }\n}\n\n#[derive(Clone, Copy)]\n\
    pub struct LexicographicalDFA<'a, T> {\n    seq: &'a [T],\n    ord: Ordering,\n\
    \    eq: bool,\n}\n\nimpl<'a, T> DeterministicFiniteAutoMaton for LexicographicalDFA<'a,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = (usize, bool);\n\
    \n    fn initial_state(&self) -> Self::State {\n        (0, true)\n    }\n\n \
    \   fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        let &(i, tight) = state;\n        let x = self.seq.get(i)?;\n   \
    \     match (tight, c.cmp(x)) {\n            (true, Ordering::Equal) => Some((i\
    \ + 1, true)),\n            (true, ord) if ord == self.ord.reverse() => None,\n\
    \            _ => Some((i + 1, false)),\n        }\n    }\n\n    fn is_accepting(&self,\
    \ state: &Self::State) -> bool {\n        self.eq || !state.1\n    }\n\n    fn\
    \ state_to_index(&self, state: &Self::State) -> usize {\n        state.1 as usize\n\
    \    }\n\n    fn state_num(&self) -> usize {\n        2\n    }\n}\n\nimpl<'a,\
    \ T> LexicographicalDFA<'a, T>\nwhere\n    T: Ord,\n{\n    pub fn less_than(seq:\
    \ &'a [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Less,\n\
    \            eq: false,\n        }\n    }\n\n    pub fn less_than_equal(seq: &'a\
    \ [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Less,\n\
    \            eq: true,\n        }\n    }\n\n    pub fn greater_than(seq: &'a [T])\
    \ -> Self {\n        Self {\n            seq,\n            ord: Ordering::Greater,\n\
    \            eq: false,\n        }\n    }\n\n    pub fn greater_than_equal(seq:\
    \ &'a [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Greater,\n\
    \            eq: true,\n        }\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct\
    \ ReversedLexicographicalDFA<'a, T> {\n    seq: &'a [T],\n    ord: Ordering,\n\
    \    eq: bool,\n}\n\nimpl<'a, T> DeterministicFiniteAutoMaton for ReversedLexicographicalDFA<'a,\
    \ T>\nwhere\n    T: Ord,\n{\n    type Symbol = T;\n    type State = (usize, bool);\n\
    \n    fn initial_state(&self) -> Self::State {\n        (self.seq.len(), self.eq)\n\
    \    }\n\n    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        let &(i, accepting) = state;\n        let i = i.checked_sub(1)?;\n\
    \        let x = self.seq.get(i)?;\n        match c.cmp(x) {\n            Ordering::Equal\
    \ => Some((i, accepting)),\n            ord => Some((i, ord == self.ord)),\n \
    \       }\n    }\n\n    fn is_accepting(&self, state: &Self::State) -> bool {\n\
    \        state.1\n    }\n\n    fn state_to_index(&self, state: &Self::State) ->\
    \ usize {\n        state.1 as usize\n    }\n\n    fn state_num(&self) -> usize\
    \ {\n        2\n    }\n}\n\nimpl<'a, T> ReversedLexicographicalDFA<'a, T>\nwhere\n\
    \    T: Ord,\n{\n    pub fn less_than(seq: &'a [T]) -> Self {\n        Self {\n\
    \            seq,\n            ord: Ordering::Less,\n            eq: false,\n\
    \        }\n    }\n\n    pub fn less_than_equal(seq: &'a [T]) -> Self {\n    \
    \    Self {\n            seq,\n            ord: Ordering::Less,\n            eq:\
    \ true,\n        }\n    }\n\n    pub fn greater_than(seq: &'a [T]) -> Self {\n\
    \        Self {\n            seq,\n            ord: Ordering::Greater,\n     \
    \       eq: false,\n        }\n    }\n\n    pub fn greater_than_equal(seq: &'a\
    \ [T]) -> Self {\n        Self {\n            seq,\n            ord: Ordering::Greater,\n\
    \            eq: true,\n        }\n    }\n}\n\n#[derive(Clone, Copy)]\npub struct\
    \ SymbolMapDFA<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton,\n    F: Fn(&S)\
    \ -> A::Symbol,\n{\n    dfa: A,\n    map: F,\n    _marker: PhantomData<fn() ->\
    \ S>,\n}\n\nimpl<A, S, F> SymbolMapDFA<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton<Symbol\
    \ = S>,\n    F: Fn(&S) -> A::Symbol,\n{\n    pub fn new(dfa: A, map: F) -> Self\
    \ {\n        Self {\n            dfa,\n            map,\n            _marker:\
    \ PhantomData,\n        }\n    }\n}\n\nimpl<A, S, F> DeterministicFiniteAutoMaton\
    \ for SymbolMapDFA<A, S, F>\nwhere\n    A: DeterministicFiniteAutoMaton,\n   \
    \ F: Fn(&S) -> A::Symbol,\n{\n    type Symbol = S;\n    type State = A::State;\n\
    \n    fn initial_state(&self) -> Self::State {\n        self.dfa.initial_state()\n\
    \    }\n\n    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>\
    \ {\n        self.dfa.transition(state, &(self.map)(c))\n    }\n\n    fn is_accepting(&self,\
    \ state: &Self::State) -> bool {\n        self.dfa.is_accepting(state)\n    }\n\
    \n    fn state_to_index(&self, state: &Self::State) -> usize {\n        self.dfa.state_to_index(state)\n\
    \    }\n\n    fn state_num(&self) -> usize {\n        self.dfa.state_num()\n \
    \   }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/dp/automaton-dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/dp/automaton-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/automaton-dp/src/lib.rs
- /library/crates/dp/automaton-dp/src/lib.rs.html
title: crates/dp/automaton-dp/src/lib.rs
---

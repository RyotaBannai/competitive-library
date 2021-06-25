use super::Monoid;
use std::{cmp::Ordering, collections::HashMap, hash::Hash, marker::PhantomData};

pub trait Automaton {
    type Alphabet;
    type State;
    type Effect;

    fn initial(&self) -> Self::State;
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)>;
    fn accept(&self, state: &Self::State) -> bool;
}

pub fn automaton_dp<M, A, I, F>(dfa: A, sigma: I, len: usize, mul: F, init: M::T) -> M::T
where
    A: Automaton,
    A::State: Eq + Hash,
    M: Monoid,
    I: Iterator<Item = A::Alphabet> + Clone,
    F: Fn(&M::T, &A::Effect) -> M::T,
{
    let mut dp = HashMap::new();
    let mut ndp = HashMap::new();
    dp.insert(dfa.initial(), init);
    for _ in 0..len {
        for (state, value) in dp.drain() {
            for alph in sigma.clone() {
                if let Some((nstate, eff)) = dfa.next(&state, &alph) {
                    let nvalue = mul(&value, &eff);
                    ndp.entry(nstate)
                        .and_modify(|acc| *acc = M::operate(acc, &nvalue))
                        .or_insert(nvalue);
                }
            }
        }
        std::mem::swap(&mut dp, &mut ndp);
        ndp.clear();
    }
    let mut acc = M::unit();
    for (state, value) in dp.into_iter() {
        if dfa.accept(&state) {
            acc = M::operate(&acc, &value);
        }
    }
    acc
}

pub struct IntersectionAutomaton<X: Automaton, Y: Automaton>(X, Y);
impl<A, X, Y> Automaton for IntersectionAutomaton<X, Y>
where
    X: Automaton<Alphabet = A>,
    Y: Automaton<Alphabet = A>,
{
    type Alphabet = A;
    type State = (X::State, Y::State);
    type Effect = (X::Effect, Y::Effect);
    fn initial(&self) -> Self::State {
        (self.0.initial(), self.1.initial())
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        match (self.0.next(&state.0, alph), self.1.next(&state.1, alph)) {
            (Some((s0, e0)), Some((s1, e1))) => Some(((s0, s1), (e0, e1))),
            _ => None,
        }
    }
    fn accept(&self, state: &Self::State) -> bool {
        self.0.accept(&state.0) && self.1.accept(&state.1)
    }
}

pub struct UnionAutomaton<X: Automaton, Y: Automaton>(X, Y);
impl<A, X, Y> Automaton for UnionAutomaton<X, Y>
where
    X: Automaton<Alphabet = A>,
    Y: Automaton<Alphabet = A>,
{
    type Alphabet = A;
    type State = (X::State, Y::State);
    type Effect = (X::Effect, Y::Effect);
    fn initial(&self) -> Self::State {
        (self.0.initial(), self.1.initial())
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        match (self.0.next(&state.0, alph), self.1.next(&state.1, alph)) {
            (Some((s0, e0)), Some((s1, e1))) => Some(((s0, s1), (e0, e1))),
            _ => None,
        }
    }
    fn accept(&self, state: &Self::State) -> bool {
        self.0.accept(&state.0) || self.1.accept(&state.1)
    }
}

pub struct ProductAutomaton<X: Automaton, Y: Automaton>(X, Y);
impl<X: Automaton, Y: Automaton> Automaton for ProductAutomaton<X, Y> {
    type Alphabet = (X::Alphabet, Y::Alphabet);
    type State = (X::State, Y::State);
    type Effect = (X::Effect, Y::Effect);
    fn initial(&self) -> Self::State {
        (self.0.initial(), self.1.initial())
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        match (
            self.0.next(&state.0, &alph.0),
            self.1.next(&state.1, &alph.1),
        ) {
            (Some((s0, e0)), Some((s1, e1))) => Some(((s0, s1), (e0, e1))),
            _ => None,
        }
    }
    fn accept(&self, state: &Self::State) -> bool {
        self.0.accept(&state.0) && self.1.accept(&state.1)
    }
}

pub struct LessThanAutomaton<'a, T: Ord> {
    buf: &'a [T],
    eq: bool,
}
impl<'a, T: Ord> LessThanAutomaton<'a, T> {
    pub fn new(buf: &'a [T], eq: bool) -> Self {
        Self { buf, eq }
    }
}
impl<T: Ord> Automaton for LessThanAutomaton<'_, T> {
    type Alphabet = T;
    type State = (usize, bool);
    type Effect = ();
    fn initial(&self) -> Self::State {
        (0, true)
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        self.buf
            .get(state.0)
            .and_then(|c| match (state.1, c.cmp(alph)) {
                (true, Ordering::Equal) => Some(((state.0 + 1, true), ())),
                (true, Ordering::Less) => None,
                _ => Some(((state.0 + 1, false), ())),
            })
    }
    fn accept(&self, state: &Self::State) -> bool {
        self.eq || !state.1
    }
}

pub struct GreaterThanAutomaton<'a, T: Ord> {
    buf: &'a [T],
    eq: bool,
}
impl<'a, T: Ord> GreaterThanAutomaton<'a, T> {
    pub fn new(buf: &'a [T], eq: bool) -> Self {
        Self { buf, eq }
    }
}
impl<T: Ord> Automaton for GreaterThanAutomaton<'_, T> {
    type Alphabet = T;
    type State = (usize, bool);
    type Effect = ();
    fn initial(&self) -> Self::State {
        (0, true)
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        self.buf
            .get(state.0)
            .and_then(|c| match (state.1, c.cmp(alph)) {
                (true, Ordering::Equal) => Some(((state.0 + 1, true), ())),
                (true, Ordering::Greater) => None,
                _ => Some(((state.0 + 1, false), ())),
            })
    }
    fn accept(&self, state: &Self::State) -> bool {
        self.eq || !state.1
    }
}

pub struct ContainAutomaton<'a, T: Eq>(&'a T);
impl<'a, T: Eq> Automaton for ContainAutomaton<'a, T> {
    type Alphabet = T;
    type State = bool;
    type Effect = bool;
    fn initial(&self) -> Self::State {
        false
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        Some((*state || self.0 == alph, *state ^ (self.0 == alph)))
    }
    fn accept(&self, state: &Self::State) -> bool {
        *state
    }
}

pub struct ContainCounterAutomaton<'a, T: Eq>(&'a T);
impl<'a, T: Eq> Automaton for ContainCounterAutomaton<'a, T> {
    type Alphabet = T;
    type State = usize;
    type Effect = usize;
    fn initial(&self) -> Self::State {
        0
    }
    fn next(
        &self,
        state: &Self::State,
        alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        let nstate = *state + (self.0 == alph) as usize;
        Some((nstate, nstate))
    }
    fn accept(&self, state: &Self::State) -> bool {
        *state > 0
    }
}

#[derive(Debug, Clone)]
pub struct AlwaysAcceptingAutomaton<A>(PhantomData<fn() -> A>);
impl<A> AlwaysAcceptingAutomaton<A> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<A> Default for AlwaysAcceptingAutomaton<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
impl<A> Automaton for AlwaysAcceptingAutomaton<A> {
    type Alphabet = A;
    type State = ();
    type Effect = ();
    fn initial(&self) -> Self::State {}
    fn next(
        &self,
        _state: &Self::State,
        _alph: &Self::Alphabet,
    ) -> Option<(Self::State, Self::Effect)> {
        Some(((), ()))
    }
    fn accept(&self, _state: &Self::State) -> bool {
        true
    }
}

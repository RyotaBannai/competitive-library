use super::*;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

pub trait SemiRing {
    type T: Clone;
    type Additive: AbelianMonoid<T = Self::T>;
    type Multiplicative: Monoid<T = Self::T>;
    /// additive identity: $0$
    fn zero() -> Self::T {
        <Self::Additive as Unital>::unit()
    }
    /// multiplicative identity: $1$
    fn one() -> Self::T {
        <Self::Multiplicative as Unital>::unit()
    }
    /// additive operaion: $+$
    fn add(x: &Self::T, y: &Self::T) -> Self::T {
        <Self::Additive as Magma>::operate(x, y)
    }
    /// multiplicative operaion: $+$
    fn mul(x: &Self::T, y: &Self::T) -> Self::T {
        <Self::Multiplicative as Magma>::operate(x, y)
    }
}

pub trait Ring: SemiRing
where
    Self::Additive: Invertible,
{
    /// additive inverse: $-$
    fn neg(x: &Self::T) -> Self::T {
        <Self::Additive as Invertible>::inverse(x)
    }
    /// additive right inversed operaion: $-$
    fn sub(x: &Self::T, y: &Self::T) -> Self::T {
        Self::add(x, &Self::neg(y))
    }
}

impl<R> Ring for R
where
    R: SemiRing,
    R::Additive: Invertible,
{
}

pub trait Field: Ring
where
    Self::Additive: Invertible,
    Self::Multiplicative: Invertible,
{
    /// multiplicative inverse: $-$
    fn inv(x: &Self::T) -> Self::T {
        <Self::Additive as Invertible>::inverse(x)
    }
    /// multiplicative right inversed operaion: $-$
    fn div(x: &Self::T, y: &Self::T) -> Self::T {
        Self::mul(x, &Self::inv(y))
    }
}

impl<F> Field for F
where
    F: Ring,
    F::Additive: Invertible,
    F::Multiplicative: Invertible,
{
}

/// $+,\times$
pub struct AddMulOperation<T>
where
    T: Clone + Zero + One + Add<Output = T> + Mul<Output = T>,
{
    _marker: PhantomData<fn() -> T>,
}
impl<T> SemiRing for AddMulOperation<T>
where
    T: Clone + Zero + One + Add<Output = T> + Mul<Output = T>,
{
    type T = T;
    type Additive = AdditiveOperation<T>;
    type Multiplicative = MultiplicativeOperation<T>;
}

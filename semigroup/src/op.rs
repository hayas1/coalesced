use std::ops::{Deref, DerefMut};

use crate::{Annotate, Annotated, AnnotatedSemigroup, Semigroup};

pub mod coalesce;
pub mod concat;
pub mod overwrite;

#[cfg(feature = "monoid")]
pub mod gcd;
#[cfg(feature = "monoid")]
pub mod lcm;
pub mod max;
pub mod min;
pub mod prod;
pub mod sum;
pub mod xor;

#[cfg(feature = "histogram")]
pub mod hdr_histogram;

pub trait Construction<T>: Semigroup + Sized + From<T> + Deref<Target = T> + DerefMut {
    fn into_inner(self) -> T;
    fn lift_op(base: T, other: T) -> T {
        Semigroup::op(Self::from(base), Self::from(other)).into_inner()
    }
}

pub trait ConstructionAnnotated<T, A>:
    Construction<T> + AnnotatedSemigroup<A> + Annotate<A>
{
    fn lift_annotated_op(base: Annotated<T, A>, other: Annotated<T, A>) -> Annotated<T, A> {
        AnnotatedSemigroup::annotated_op(base.map(Self::from), other.map(Self::from))
            .map(Self::into_inner)
    }
}

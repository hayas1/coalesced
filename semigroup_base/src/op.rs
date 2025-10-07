use std::ops::{Deref, DerefMut};

use crate::{
    annotate::{Annotate, Annotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

pub mod annotation;
pub mod semigroup;
pub mod statistics;

pub trait Construction<T>: Semigroup + Sized + From<T> + Deref<Target = T> + DerefMut {
    fn into_inner(self) -> T;
    fn lift_op(base: T, other: T) -> T {
        Semigroup::semigroup_op(Self::from(base), Self::from(other)).into_inner()
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

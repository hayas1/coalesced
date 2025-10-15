use semigroup_derive::ConstructionUse;

use crate::{op::Construction, Annotate, Annotated, AnnotatedSemigroup, Semigroup};

pub trait Monoid: Semigroup {
    fn unit() -> Self;
}
pub trait AnnotatedMonoid<A>: Sized + Monoid + AnnotatedSemigroup<A> {
    fn annotated_unit() -> Annotated<Self, A>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
pub struct OptionMonoid<T: Semigroup>(pub Option<T>);
impl<T: Semigroup> From<T> for OptionMonoid<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}
impl<T: Semigroup> Monoid for OptionMonoid<T> {
    fn unit() -> Self {
        Self(None)
    }
}
impl<T: AnnotatedSemigroup<A>, A> AnnotatedMonoid<Option<A>> for OptionMonoid<T> {
    fn annotated_unit() -> Annotated<Self, Option<A>> {
        Annotated::new(Self::unit(), None)
    }
}
impl<T: Semigroup> Semigroup for OptionMonoid<T> {
    fn op(base: Self, other: Self) -> Self {
        match (base, other) {
            (Self(Some(b)), Self(Some(o))) => Self(Some(T::op(b, o))),
            (b, Self(None)) => b,
            (Self(None), o) => o,
        }
    }
}
impl<T: AnnotatedSemigroup<A>, A> AnnotatedSemigroup<Option<A>> for OptionMonoid<T> {
    fn annotated_op(
        base: Annotated<Self, Option<A>>,
        other: Annotated<Self, Option<A>>,
    ) -> Annotated<Self, Option<A>> {
        let (Self(base_value), base_annotation) = base.into_parts();
        let (Self(other_value), other_annotation) = other.into_parts();
        match (base_value, base_annotation, other_value, other_annotation) {
            (Some(bv), Some(ba), Some(ov), Some(oa)) => {
                T::annotated_op(Annotated::new(bv, ba), Annotated::new(ov, oa))
                    .map_parts(Self::from, Some)
            }
            (b, ba, None, None) => Annotated::new(Self(b), ba),
            (None, None, o, oa) => Annotated::new(Self(o), oa),
            _ => unreachable!(), // TODO safety annotation
        }
    }
}
impl<T: AnnotatedSemigroup<A> + Annotate<A>, A> Annotate<Option<A>> for OptionMonoid<T> {
    type Annotation = T::Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, Option<A>> {
        match self {
            Self(None) => Self::annotated_unit(),
            Self(Some(semigroup)) => semigroup.annotated(annotation).map_parts(Self::from, Some),
        }
    }
}

#[cfg(any(test, feature = "test"))]
pub mod test_monoid {
    use std::fmt::Debug;

    use crate::semigroup::test_semigroup::{assert_associative_law, assert_semigroup_impl};

    use super::*;

    #[macro_export]
    macro_rules! assert_monoid {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::test_monoid::assert_monoid!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_monoid::assert_monoid_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }
    pub use assert_monoid;

    pub fn assert_monoid_impl<T: Monoid + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_semigroup_impl(a.clone(), b.clone(), c.clone());
        assert_monoid_unit_associative_law(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_option_monoid<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_monoid_impl(
            OptionMonoid::<T>::from(a.clone()),
            OptionMonoid::<T>::from(b.clone()),
            OptionMonoid::<T>::from(c.clone()),
        );
    }
    pub fn assert_monoid_unit_associative_law<T: Monoid + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        assert_eq!(T::unit(), T::op(T::unit(), T::unit()));
        assert_eq!(a.clone(), T::op(a.clone(), T::unit()));
        assert_eq!(a.clone(), T::op(T::unit(), a.clone()));
        assert_eq!(b.clone(), T::op(b.clone(), T::unit()));
        assert_eq!(b.clone(), T::op(T::unit(), b.clone()));
        assert_eq!(c.clone(), T::op(c.clone(), T::unit()));
        assert_eq!(c.clone(), T::op(T::unit(), c.clone()));

        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_associative_law(T::unit(), b.clone(), c.clone());
        assert_associative_law(a.clone(), T::unit(), c.clone());
        assert_associative_law(a.clone(), b.clone(), T::unit());
    }
}

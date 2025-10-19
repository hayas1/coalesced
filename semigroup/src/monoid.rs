use semigroup_derive::ConstructionPriv;

use crate::{Annotate, Annotated, AnnotatedSemigroup, Semigroup};

/// [`Monoid`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
/// 3. Existence of *identity element*: `op(unit(), a) = a = op(a, unit())`
///
/// # Deriving
/// [`Monoid`] can be derived like [`Semigroup`], use `monoid` attribute.
/// ```
/// use semigroup::{Semigroup, Monoid};
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(monoid, with = "semigroup::op::coalesce::Coalesce")]
/// pub struct ExampleStruct<'a> {
///     pub str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::sum::Sum")]
///     pub sum: u32,
/// }
///
/// let a = ExampleStruct::unit();
/// let b = ExampleStruct { str: Some("ten"), sum: 10 };
/// let c = ExampleStruct { str: None, sum: 100 };
///
/// // #[test]
/// semigroup::assert_monoid!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { str: Some("ten"), sum: 110 });
/// ```
///
/// # Construction
/// [`Monoid`] can be constructed by [`crate::ConstructionMonoid`] like [`Semigroup`], use `monoid` attribute.
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{Construction, Semigroup, Monoid};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
/// #[construction(monoid, commutative, unit = Self(0))]
/// pub struct Sum(u64);
/// impl Semigroup for Sum {
///     fn op(base: Self, other: Self) -> Self {
///         Self(base.0 + other.0)
///     }
/// }
/// let (a, b, c) = (Sum::unit(), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_monoid!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(5));
/// ```
///
/// # Optional [`Semigroup`]
/// [`Option<Semigroup>`] can behave like [`Monoid`]. In such case, [`OptionMonoid`] is useful.
///
/// # Testing
/// Use [`crate::assert_monoid!`] macro.
///
/// The *closure* and *associativity* properties are same as [`Semigroup`],
/// so they are guaranteed by [`crate::assert_semigroup!`].
/// However, existence of *identity element* is not guaranteed the macro,
/// so it must be verified manually using [`crate::assert_monoid!`].
pub trait Monoid: Semigroup {
    fn unit() -> Self;
}
pub trait AnnotatedMonoid<A>: Sized + Monoid + AnnotatedSemigroup<A> {
    fn annotated_unit() -> Annotated<Self, A>;
}

/// Construct [`Monoid`] from optional [`Semigroup`].
/// Some [`Semigroup`] lack a suitable *identity element* for extension to a [`Monoid`].
///
/// # Examples
/// In [`Semigroup`] operations of [`crate::op::min::Min`] and [`crate::op::max::Max`], [`std::time::Instant`] does not have a suitable *identity element* for extension to a [`Monoid`].
/// ```compile_fail
/// use std::time::{Duration, Instant};
/// use semigroup::{Semigroup, Monoid, OptionMonoid};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(monoid, commutative)]
/// pub struct BoundingDuration {
///     #[semigroup(with = "semigroup::op::min::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::max::Max")]
///     end: Instant,
/// }
/// impl BoundingDuration {
///      pub fn duration(&self) -> Duration {
///         self.end - self.start
///     }
/// }
/// ```
///
/// In such case, [`OptionMonoid`] is useful.
/// ```
/// use std::time::{Duration, Instant};
/// use semigroup::{Semigroup, Monoid, OptionMonoid};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct BoundingDuration {
///     #[semigroup(with = "semigroup::op::min::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::max::Max")]
///     end: Instant,
/// }
/// impl BoundingDuration {
///      pub fn duration(&self) -> Duration {
///         self.end - self.start
///     }
/// }
///
/// let (now, mut bd) = (Instant::now(), OptionMonoid::unit());
/// let v = vec! [
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(50), end: now + Duration::from_millis(100) }),
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(100), end: now + Duration::from_millis(200) }),
///     OptionMonoid::from(BoundingDuration { start: now + Duration::from_millis(150), end: now + Duration::from_millis(300) }),
/// ];
/// for vi in v {
///     bd = bd.semigroup(vi);
/// }
/// assert_eq!(bd.as_ref().unwrap().duration(), Duration::from_millis(250));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[construction(monoid, unit = Self(None))]
pub struct OptionMonoid<T: Semigroup>(pub Option<T>);
impl<T: Semigroup> From<T> for OptionMonoid<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
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

    /// Assert that the given type satisfies the *monoid* property.
    ///
    /// # Usage
    /// Same to [`crate::assert_semigroup!`].
    #[macro_export]
    macro_rules! assert_monoid {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_monoid!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_monoid::assert_monoid_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

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

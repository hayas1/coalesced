use std::ops::{Deref, DerefMut};

use crate::{Annotate, Annotated, AnnotatedSemigroup, Semigroup};

/// [`Construction`] represents [`crate::Semigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
///
/// # Examples
/// Simple example see [`crate::Semigroup#construction`].
/// TODO more derive details
pub trait Construction<T>: Semigroup + Sized + From<T> + Deref<Target = T> + DerefMut {
    /// Convert into inner type of [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html).
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, Semigroup};
    ///
    /// #[derive(Construction)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Semigroup for Coalesce<T> {
    ///     fn op(base: Self, other: Self) -> Self {
    ///         Self(base.0.or(other.0))
    ///     }
    /// }
    ///
    /// let a = Coalesce(Some(1));
    /// assert_eq!(a.into_inner(), Some(1));
    /// ```
    fn into_inner(self) -> T;

    /// Semigroup operation between `base` and `other` with constructed type.
    /// When `T` does not implement [`crate::Semigroup`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, Semigroup};
    ///
    /// #[derive(Construction)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Semigroup for Coalesce<T> {
    ///     fn op(base: Self, other: Self) -> Self {
    ///         Self(base.0.or(other.0))
    ///     }
    /// }
    ///
    /// let a = None;
    /// let b = Some(2);
    /// assert_eq!(Coalesce::lift_op(a, b), Some(2));
    /// ```
    fn lift_op(base: T, other: T) -> T {
        Semigroup::op(Self::from(base), Self::from(other)).into_inner()
    }
}

/// [`ConstructionAnnotated`] represents [`crate::AnnotatedSemigroup`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) like [`Construction`].
///
/// # Examples
/// TODO more derive details
pub trait ConstructionAnnotated<T, A>:
    Construction<T> + AnnotatedSemigroup<A> + Annotate<A>
{
    /// Semigroup operation between `base` and `other` with constructed type.
    /// When `T` does not implement [`crate::AnnotatedSemigroup`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{AnnotatedSemigroup, Annotated, Construction, ConstructionAnnotated, Semigroup};
    ///
    /// #[derive(Construction)]
    /// #[construction(annotated)]
    /// struct Coalesce<T>(Option<T>);
    /// impl<A, T> AnnotatedSemigroup<A> for Coalesce<T> {
    ///     fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
    ///         match (&base.value().0, &other.value().0) {
    ///             (Some(_), _) | (None, None) => base,
    ///             (None, Some(_)) => other,
    ///         }
    ///     }
    /// }
    ///
    /// let a = Annotated::new(None, "first");
    /// let b = Annotated::new(Some(2), "second");
    /// assert_eq!(Coalesce::lift_annotated_op(a, b), b);
    /// ```
    fn lift_annotated_op(base: Annotated<T, A>, other: Annotated<T, A>) -> Annotated<T, A> {
        AnnotatedSemigroup::annotated_op(base.map(Self::from), other.map(Self::from))
            .map(Self::into_inner)
    }
}

/// [`ConstructionMonoid`] represents [`crate::Monoid`] as a [new type struct](https://doc.rust-lang.org/rust-by-example/generics/new_types.html). like [`Construction`].
///
/// # Examples
/// Simple example see [`crate::Monoid#construction`].
/// TODO more derive details
#[cfg(feature = "monoid")]
pub trait ConstructionMonoid<T>: Construction<T> + crate::Monoid {
    /// Get monoid *identity element* with constructed type.
    /// When `T` does not implement [`crate::Monoid`], this function can be used.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{Construction, ConstructionMonoid, Semigroup};
    ///
    /// #[derive(Construction)]
    /// #[construction(monoid, identity = Self(None))]
    /// struct Coalesce<T>(Option<T>);
    /// impl<T> Semigroup for Coalesce<T> {
    ///     fn op(base: Self, other: Self) -> Self {
    ///         Self(base.0.or(other.0))
    ///     }
    /// }
    ///
    /// let a: Option<u32> = Coalesce::lift_identity();
    /// assert_eq!(a, None);
    /// ```
    fn lift_identity() -> T {
        Self::identity().into_inner()
    }
}

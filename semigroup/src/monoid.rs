use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::Semigroup;

/// [`Monoid`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
/// 3. Existence of *identity element*: `op(identity(), a) = a = op(a, identity())`
///
/// # Examples
/// ## Deriving
/// [`Monoid`] can be derived like [`Semigroup`], use `monoid` attribute.
/// ```
/// use semigroup::{Semigroup, Monoid};
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(monoid, with = "semigroup::op::Coalesce")]
/// pub struct ExampleStruct<'a> {
///     pub str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
/// }
///
/// let a = ExampleStruct::identity();
/// let b = ExampleStruct { str: Some("ten"), sum: 10 };
/// let c = ExampleStruct { str: None, sum: 100 };
///
/// // #[test]
/// semigroup::assert_monoid!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { str: Some("ten"), sum: 110 });
/// ```
///
/// ## Construction
/// [`Monoid`] can be constructed by [`crate::ConstructionMonoid`] like [`Semigroup`], use `monoid` attribute.
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{Construction, Semigroup, Monoid};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
/// #[construction(monoid, identity = Self(0))]
/// pub struct Sum(u64);
/// impl Semigroup for Sum {
///     fn op(base: Self, other: Self) -> Self {
///         Self(base.0 + other.0)
///     }
/// }
/// let (a, b, c) = (Sum::identity(), Sum(2), Sum(3));
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
    fn identity() -> Self;
}

/// Construct [`Monoid`] from optional [`Semigroup`].
/// Some [`Semigroup`] lack a suitable *identity element* for extension to a [`Monoid`].
///
/// # Examples
/// In [`Semigroup`] operations of [`crate::op::Min`] and [`crate::op::Max`], [`std::time::Instant`] does not have a suitable *identity element* for extension to a [`Monoid`].
/// ```compile_fail
/// use std::time::{Duration, Instant};
/// use semigroup::{Semigroup, Monoid, OptionMonoid};
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(monoid, commutative)]
/// pub struct BoundingDuration {
///     #[semigroup(with = "semigroup::op::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::Max")]
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
///     #[semigroup(with = "semigroup::op::Min")]
///     start: Instant,
///     #[semigroup(with = "semigroup::op::Max")]
///     end: Instant,
/// }
/// impl BoundingDuration {
///      pub fn duration(&self) -> Duration {
///         self.end - self.start
///     }
/// }
///
/// let (now, mut bd) = (Instant::now(), OptionMonoid::identity());
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
#[construction(monoid, commutative, identity = Self(None), commutative_where = "T: crate::Commutative")]
#[properties_priv(monoid, commutative, commutative_where = "T: crate::Commutative")]
pub struct OptionMonoid<T: Semigroup>(pub Option<T>);
impl<T: Semigroup> From<T> for OptionMonoid<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}
impl<T: Semigroup> Semigroup for OptionMonoid<T> {
    fn op(base: Self, other: Self) -> Self {
        match (base, other) {
            (Self(Some(b)), Self(Some(o))) => Self(Some(Semigroup::op(b, o))),
            (b, Self(None)) => b,
            (Self(None), o) => o,
        }
    }
}

#[cfg(feature = "test")]
pub mod test_monoid {
    use std::fmt::Debug;

    use crate::{
        combine::test_combine::assert_combine_iter_monoid,
        semigroup::test_semigroup::assert_associative_law,
    };

    use super::*;

    /// Assert that the given type satisfies the *monoid* property.
    ///
    /// # Usage
    /// Same to [`crate::assert_semigroup!`].
    ///
    /// # Examples
    /// ```
    /// use semigroup::{assert_monoid, op::Coalesce};
    ///
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// let c = Coalesce(Some(3));
    /// assert_monoid!(a, b, c);
    ///
    /// let v = vec![a, b, c];
    /// assert_monoid!(&v);
    /// ```
    ///
    /// # Panics
    /// - If the given function does not satisfy the *monoid* property.
    /// ```should_panic
    /// use semigroup::{assert_monoid, Construction, Semigroup};
    /// #[derive(Debug, Clone, PartialEq, Construction)]
    /// #[construction(monoid, identity = Self(0))]
    /// pub struct Sub(i32);
    /// impl Semigroup for Sub {
    ///     fn op(base: Self, other: Self) -> Self {
    ///         Self(base.0 - other.0)
    ///     }
    /// }
    /// let a = Sub(1);
    /// let b = Sub(2);
    /// let c = Sub(3);
    /// assert_monoid!(a, b, c);
    /// ```
    ///
    /// - The input iterator has less than 3 items.
    /// ```compile_fail
    /// use semigroup::{assert_monoid, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_monoid!(a, b);
    /// ```
    /// ```should_panic
    /// use semigroup::{assert_monoid, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_monoid!(&vec![a, b]);
    /// ```
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
        assert_monoid_identity_associative_law(a.clone(), b.clone(), c.clone());
        assert_combine_iter_monoid(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_option_monoid<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_monoid_impl(
            OptionMonoid::<T>::from(a.clone()),
            OptionMonoid::<T>::from(b.clone()),
            OptionMonoid::<T>::from(c.clone()),
        );
    }
    pub fn assert_monoid_identity_associative_law<T: Monoid + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        assert_eq!(T::identity(), Semigroup::op(T::identity(), T::identity()));
        assert_eq!(a.clone(), Semigroup::op(a.clone(), Monoid::identity()));
        assert_eq!(a.clone(), Semigroup::op(Monoid::identity(), a.clone()));
        assert_eq!(b.clone(), Semigroup::op(b.clone(), Monoid::identity()));
        assert_eq!(b.clone(), Semigroup::op(Monoid::identity(), b.clone()));
        assert_eq!(c.clone(), Semigroup::op(c.clone(), Monoid::identity()));
        assert_eq!(c.clone(), Semigroup::op(Monoid::identity(), c.clone()));

        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_associative_law(Monoid::identity(), b.clone(), c.clone());
        assert_associative_law(a.clone(), Monoid::identity(), c.clone());
        assert_associative_law(a.clone(), b.clone(), Monoid::identity());
    }
}

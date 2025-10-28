use crate::Annotated;

/// [`Semigroup`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
///
/// # Examples
/// ## Deriving
/// When fields do not implement [`Semigroup`], use `with` attribute.
/// ```
/// use semigroup::Semigroup;
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(with = "semigroup::op::Coalesce")]
/// pub struct ExampleStruct<'a> {
///     pub str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::Overwrite")]
///     pub boolean: bool,
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
/// }
///
/// let a = ExampleStruct { str: None, boolean: true, sum: 1 };
/// let b = ExampleStruct { str: Some("ten"), boolean: false, sum: 10 };
/// let c = ExampleStruct { str: None, boolean: false, sum: 100 };
///
/// // #[test]
/// semigroup::assert_semigroup!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { str: Some("ten"), boolean: false, sum: 111 });
/// ```
///
/// ## Construction
/// [`Semigroup`] can be constructed by [`crate::Construction`].
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{Construction, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
/// pub struct Sum(u64);
/// impl Semigroup for Sum {
///     fn op(base: Self, other: Self) -> Self {
///         Self(base.0 + other.0)
///     }
/// }
///
/// let (a, b, c) = (Sum(1), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_semigroup!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(6));
/// ```
///
/// # Testing
/// Use [`crate::assert_semigroup!`] macro.
///
/// The *closure* property is guaranteed by Rust’s type system,
/// but *associativity* must be verified manually using [`crate::assert_semigroup!`].
pub trait Semigroup {
    fn op(base: Self, other: Self) -> Self;
    fn semigroup(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Semigroup::op(self, other)
    }
}

/// [`AnnotatedSemigroup`] is a [`Semigroup`] that has an annotation, such as [`crate::Annotate`].
pub trait AnnotatedSemigroup<A>: Sized + Semigroup {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}

#[cfg(feature = "test")]
pub mod test_semigroup {
    use std::fmt::Debug;

    use rand::seq::IndexedRandom;

    use crate::{
        combine::test_combine::{assert_combine_iter, assert_lazy},
        commutative::test_commutative::{assert_reverse_associative_law, assert_reverse_reverse},
    };

    use super::*;

    /// Assert that the given type satisfies the *semigroup* property.
    ///
    /// # Usage
    /// - 1 argument: iterator of more than 3 items that implements [`Semigroup`].
    /// - More than 3 arguments: items that implements [`Semigroup`].
    ///
    /// # Examples
    /// ```
    /// use semigroup::{assert_semigroup, op::Coalesce};
    ///
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// let c = Coalesce(Some(3));
    /// assert_semigroup!(a, b, c);
    ///
    /// let v = vec![a, b, c];
    /// assert_semigroup!(&v);
    /// ```
    ///
    /// # Panics
    /// - If the given function does not satisfy the *semigroup* property.
    /// - The input iterator has less than 3 items.
    ///
    /// ```compile_fail
    /// use semigroup::{assert_semigroup, op::Coalesce};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(None);
    /// assert_semigroup!(a, b);
    /// ```
    #[macro_export]
    macro_rules! assert_semigroup {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_semigroup!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_semigroup::assert_semigroup_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn pick3<T: Clone>(data: &[T]) -> (T, T, T) {
        data.choose_multiple_array(&mut rand::rng())
            .map(|[a, b, c]| (a, b, c))
            .expect("failed to pick 3 items")
    }

    pub fn assert_semigroup_impl<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reverse_reverse(a.clone(), b.clone(), c.clone());
        assert_reverse_associative_law(a.clone(), b.clone(), c.clone());
        assert_combine_iter(a.clone(), b.clone(), c.clone());
        assert_lazy(a.clone(), b.clone(), c.clone());
        #[cfg(feature = "monoid")]
        crate::test_monoid::assert_option_monoid(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone());
        let a_bc = Semigroup::op(a.clone(), Semigroup::op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }
}

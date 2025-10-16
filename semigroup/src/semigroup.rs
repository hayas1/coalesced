use crate::Annotated;

/// [`Semigroup`] represents a binary operation that satisfies the following properties
/// 1. *Closure*: `op: T × T → T`
/// 2. *Associativity*: `op(op(a, b), c) = op(a, op(b, c))`
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

/// [`AnnotatedSemigroup`] is a [`Semigroup`] that has an annotation.
pub trait AnnotatedSemigroup<A>: Sized + Semigroup {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}

#[cfg(any(test, feature = "test"))]
pub mod test_semigroup {
    use std::fmt::Debug;

    use rand::seq::IndexedRandom;

    use crate::{
        commutative::test_commutative::{assert_reverse_associative_law, assert_reverse_reverse},
        iter::test_iter::assert_lazy_evaluation_iter,
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
    /// use semigroup::{assert_semigroup, op::coalesce::Coalesce};
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
    /// use semigroup::{assert_semigroup, op::coalesce::Coalesce};
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
                $crate::test_monoid::assert_option_monoid(a.clone(), b.clone(), c.clone());
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
        assert_lazy_evaluation_iter(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = T::op(T::op(a.clone(), b.clone()), c.clone());
        let a_bc = T::op(a.clone(), T::op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }
}

use std::future::Future;

use futures::{Stream, StreamExt};

use crate::Semigroup;

/// [`Commutative`] represents a binary operation that satisfies the following property
/// 1. *Commutativity*: `op(a, b) = op(b, a)`
///
/// The [*semigroup*](crate::Semigroup) set that satisfies the *commutativity* property is often called *commutative semigroup*.
///
/// And the [*monoid*](crate::Monoid) set that satisfies the *commutativity* property is often called *commutative monoid*.
///
/// # Examples
/// ## Deriving
/// [`Commutative`] can be derived like [`Semigroup`], use `commutative` attribute.
/// ```
/// use semigroup::Semigroup;
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct ExampleStruct {
///     #[semigroup(with = "semigroup::op::Sum")]
///     pub sum: u32,
///     #[semigroup(with = "semigroup::op::Min")]
///     pub min: u32,
/// }
///
/// let a = ExampleStruct { sum: 1, min: 1 };
/// let b = ExampleStruct { sum: 10, min: 10 };
/// let c = ExampleStruct { sum: 100, min: 100 };
///
/// // #[test]
/// semigroup::assert_commutative!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { sum: 111, min: 1 });
/// ```
///
/// ## Construction
/// [`Commutative`] can be constructed like [`Semigroup`], use `commutative` attribute.
/// ```
/// use semigroup::{Construction, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
/// #[construction(commutative)]
/// pub struct Sum(u64);
/// impl Semigroup for Sum {
///     fn op(base: Self, other: Self) -> Self {
///         Self(base.0 + other.0)
///     }
/// }
///
/// let (a, b, c) = (Sum(1), Sum(2), Sum(3));
/// // #[test]
/// semigroup::assert_commutative!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), Sum(6));
/// ```
///
/// # Testing
/// Use [`crate::assert_commutative!`] macro.
///
/// The *commutativity* property is not guaranteed by Rust’s type system,
/// so it must be verified manually using [`crate::assert_commutative!`].
pub trait Commutative: Semigroup {
    /// Used by [`CombineStream::fold_semigroup`].
    fn fold_stream(stream: impl Stream<Item = Self>, init: Self) -> impl Future<Output = Self>
    where
        Self: Sized + Send,
    {
        async { stream.fold(init, Semigroup::op_async).await }
    }
    /// Used by [`CombineStream::reduce_semigroup`].
    fn reduce_stream(
        mut stream: impl Stream<Item = Self> + Unpin,
    ) -> impl Future<Output = Option<Self>>
    where
        Self: Sized + Send,
    {
        async {
            let init = stream.next().await?;
            Some(stream.fold(init, Semigroup::op_async).await)
        }
    }
    /// Used by [`CombineStream::combine_monoid`].
    #[cfg(feature = "monoid")]
    fn combine_stream(stream: impl Stream<Item = Self>) -> impl Future<Output = Self>
    where
        Self: Sized + Send + crate::Monoid,
    {
        async { stream.fold(Self::identity(), Semigroup::op_async).await }
    }
}

pub trait CombineStream: Sized + Stream {
    /// This method like [`crate::CombineIterator::fold_final`], but stream.
    ///
    /// # Examples
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).fold_semigroup(Sum(0));
    /// assert_eq!(sum.await, Sum(45));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).fold_semigroup(Sum(0));
    /// assert_eq!(empty.await, Sum(0))
    /// # });
    /// ```
    ///
    /// # Panics
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).fold_semigroup(Coalesce(None));
    /// # });
    /// ```
    fn fold_semigroup(self, init: Self::Item) -> impl Future<Output = Self::Item>
    where
        Self::Item: Commutative + Send,
    {
        Self::Item::fold_stream(self, init)
    }

    /// This method like [`crate::CombineIterator::lreduce`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).reduce_semigroup();
    /// assert_eq!(sum.await, Some(Sum(45)));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).reduce_semigroup();
    /// assert_eq!(empty.await, None)
    /// # });
    /// ```
    ///
    /// # Panics
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).reduce_semigroup();
    /// # });
    /// ```
    fn reduce_semigroup(self) -> impl Future<Output = Option<Self::Item>>
    where
        Self: Unpin,
        Self::Item: Commutative + Send,
    {
        Self::Item::reduce_stream(self)
    }

    /// This method like [`crate::CombineIterator::combine`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).combine_monoid();
    /// assert_eq!(sum.await, Sum(45));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).combine_monoid();
    /// assert_eq!(empty.await, Sum(0))
    /// # });
    /// ```
    ///
    /// # Panics
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).combine_monoid();
    /// # });
    /// ```
    #[cfg(feature = "monoid")]
    fn combine_monoid(self) -> impl Future<Output = Self::Item>
    where
        Self::Item: Commutative + crate::Monoid + Send,
    {
        Self::Item::combine_stream(self)
    }
}
impl<T: Stream> CombineStream for T {}

#[cfg(feature = "test")]
pub mod test_commutative {
    use std::fmt::Debug;

    use crate::Reverse;

    use super::*;

    /// Assert that the given type satisfies the *commutative* property.
    ///
    /// # Usage
    /// Same to [`crate::assert_semigroup!`].
    #[macro_export]
    macro_rules! assert_commutative {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_commutative!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_commutative::assert_commutative_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn assert_commutative_impl<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_commutative_law(a.clone(), b.clone(), c.clone());
        assert_commutative_reverse(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_commutative_law<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone());
        let bca = Semigroup::op(Semigroup::op(b.clone(), c.clone()), a.clone());
        let cba = Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone());
        let acb = Semigroup::op(Semigroup::op(a.clone(), c.clone()), b.clone());
        let bac = Semigroup::op(Semigroup::op(b.clone(), a.clone()), c.clone());
        let cab = Semigroup::op(Semigroup::op(c.clone(), a.clone()), b.clone());

        assert_eq!(abc, bca);
        assert_eq!(bca, cba);
        assert_eq!(cba, acb);
        assert_eq!(acb, bac);
        assert_eq!(bac, cab);
        assert_eq!(cab, abc);
    }

    pub fn assert_commutative_reverse<T: Commutative + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Semigroup::op(ra.clone(), rb.clone()).0
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Semigroup::op(rb.clone(), rc.clone()).0
        );
        assert_eq!(
            Semigroup::op(c.clone(), a.clone()),
            Semigroup::op(rc.clone(), ra.clone()).0
        );
    }
}

use std::future::Future;

use futures::{Stream, StreamExt};

use crate::{Commutative, Semigroup};

pub trait AsyncSemigroup: Semigroup {
    fn async_op(base: Self, other: Self) -> impl std::future::Future<Output = Self>
    where
        Self: Sized + Send,
    {
        async { Semigroup::op(base, other) }
    }
}
impl<T: Semigroup> AsyncSemigroup for T {}

pub trait AsyncCommutative: AsyncSemigroup + Commutative {
    /// Used by [`CombineStream::fold_semigroup`].
    fn fold_stream(stream: impl Stream<Item = Self>, init: Self) -> impl Future<Output = Self>
    where
        Self: Sized + Send,
    {
        async { stream.fold(init, AsyncSemigroup::async_op).await }
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
            Some(stream.fold(init, AsyncSemigroup::async_op).await)
        }
    }
    /// Used by [`CombineStream::combine_monoid`].
    #[cfg(feature = "monoid")]
    fn combine_stream(stream: impl Stream<Item = Self>) -> impl Future<Output = Self>
    where
        Self: Sized + Send + crate::Monoid,
    {
        async {
            stream
                .fold(Self::identity(), AsyncSemigroup::async_op)
                .await
        }
    }
}
impl<T: Commutative> AsyncCommutative for T {}

/// Extensions for [`Stream`]s that items implement [`AsyncCommutative`]. Like [`crate::CombineIterator`].
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
        Self::Item: AsyncCommutative + Send,
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
        Self::Item: AsyncCommutative + Send,
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
        Self::Item: AsyncCommutative + crate::Monoid + Send,
    {
        Self::Item::combine_stream(self)
    }
}
impl<T: Stream> CombineStream for T {}

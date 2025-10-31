use crate::{Construction, Lazy, Reverse, Semigroup};

/// Extensions for [`Iterator`]s that items implement [`Semigroup`].
pub trait CombineIterator: Sized + Iterator {
    /// Folds every [`Semigroup`] element. Given argument is the final value.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().fold_final(Coalesce(Some(4))), Coalesce(Some(2)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().fold_final(Coalesce(Some(4))), Coalesce(Some(4)));
    /// ```
    fn fold_final(mut self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next() {
            self.chain(Some(fin)).fold(init, Semigroup::op)
        } else {
            fin
        }
    }

    /// Folds every [`Semigroup`] element in reverse order using [`Reverse`]. Given argument is the final value.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rfold_final(Coalesce(Some(3))), Coalesce(Some(3)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rfold_final(Coalesce(Some(4))), Coalesce(Some(4)));
    /// ```
    fn rfold_final(self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        self.map(Reverse)
            .fold(Reverse(fin), Semigroup::op)
            .into_inner()
    }

    /// This method like [`CombineIterator::fold_final`], but no argument is required and return [`Option`].
    ///
    /// # Example
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().lreduce(), Some(Coalesce(Some(2))));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().lreduce(), Some(Coalesce(None)));
    ///
    /// let v3 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v3.into_iter().lreduce(), None)
    /// ```
    fn lreduce(self) -> Option<Self::Item>
    where
        Self::Item: Semigroup,
    {
        self.reduce(Semigroup::op)
    }

    /// This method like [`CombineIterator::rfold_final`], but no argument is required and return [`Option`].
    ///
    /// # Example
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rreduce(), Some(Coalesce(Some(3))));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rreduce(), Some(Coalesce(None)));
    ///
    /// let v3 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v3.into_iter().rreduce(), None)
    /// ```
    fn rreduce(self) -> Option<Self::Item>
    where
        Self::Item: Semigroup,
    {
        self.map(Reverse).reduce(Semigroup::op).map(|Reverse(x)| x)
    }

    /// This method like [`CombineIterator::fold_final`], but no argument is required.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().combine(), Coalesce(Some(2)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().combine(), Coalesce(None));
    /// ```
    #[cfg(feature = "monoid")]
    fn combine(self) -> Self::Item
    where
        Self::Item: crate::Monoid,
    {
        self.fold_final(crate::Monoid::identity())
    }

    /// This method like [`CombineIterator::rfold_final`], but no argument is required.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup};
    /// let v1 = vec![Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(v1.into_iter().rcombine(), Coalesce(Some(3)));
    ///
    /// let v2 = vec![Coalesce::<u32>(None), Coalesce(None), Coalesce(None)];
    /// assert_eq!(v2.into_iter().rcombine(), Coalesce(None));
    /// ```
    #[cfg(feature = "monoid")]
    fn rcombine(self) -> Self::Item
    where
        Self::Item: crate::Monoid,
    {
        self.rfold_final(crate::Monoid::identity())
    }

    /// Collect into [`Lazy`]. If the iterator is empty, returns `None`.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, CombineIterator, Semigroup, Lazy};
    /// let v1 = vec![Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3))];
    /// assert_eq!(
    ///     v1.into_iter().collect_lazy(),
    ///     Some(Lazy::from(Coalesce(Some(1))).semigroup(Coalesce(Some(2)).into()).semigroup(Coalesce(Some(3)).into()))
    /// );
    ///
    /// let v2 = Vec::<Coalesce<u32>>::new();
    /// assert_eq!(v2.into_iter().collect_lazy(), None);
    /// ```
    fn collect_lazy(self) -> Option<Lazy<Self::Item>>
    where
        Self::Item: Semigroup,
    {
        Lazy::from_iterator(self)
    }
}
impl<I: Iterator> CombineIterator for I {}

#[cfg(feature = "test")]
pub mod test_combine {
    use std::fmt::Debug;

    use super::*;

    pub fn assert_combine_iter<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab = vec![a.clone(), b.clone()];
        assert_eq!(
            ab.into_iter().fold_final(c.clone()),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );

        let bc = vec![b.clone(), c.clone()];
        assert_eq!(
            bc.into_iter().rfold_final(a.clone()),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
    }

    #[cfg(feature = "monoid")]
    pub fn assert_combine_iter_monoid<T: crate::Monoid + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let abc = vec![a.clone(), b.clone(), c.clone()];
        assert_eq!(
            abc.clone().into_iter().combine(),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            abc.clone().into_iter().rcombine(),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
    }
}

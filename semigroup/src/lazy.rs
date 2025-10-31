use std::{ops::Index, slice::SliceIndex};

use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Annotated, Semigroup};

/// A lazy evaluated [`Semigroup`] with nonempty buffer that is implemented by [`Vec`].
///
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Coalesce, Lazy, Semigroup};
///
/// let a = Coalesce(Some(1));
/// let b = Coalesce(Some(2));
/// let c = Coalesce(Some(3));
///
/// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
///
/// assert_eq!(lazy.first(), &Coalesce(Some(1)));
/// assert_eq!(lazy.last(), &Coalesce(Some(3)));
/// assert_eq!(lazy.combine(), Coalesce(Some(1)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ConstructionPriv)]
#[construction(
    commutative,
    commutative_where = "T: crate::Commutative",
    without_construction
)]
#[properties_priv(commutative, commutative_where = "T: crate::Commutative")]
pub struct Lazy<T>(Vec<T>);
impl<T> Semigroup for Lazy<T> {
    fn op(mut base: Self, other: Self) -> Self {
        base.extend(other);
        base
    }
}
impl<T: Semigroup> Lazy<T> {
    /// Evaluates [`Lazy`] buffer.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Lazy, Semigroup};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(Some(2));
    /// let c = Coalesce(Some(3));
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)));
    /// ```
    pub fn combine(self) -> T {
        let (first, tail) = self.split_off_first();
        tail.into_iter().fold(first, Semigroup::op)
    }
    /// Evaluates [`Lazy`] buffer like [`Lazy::combine`] by cloning each element.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Lazy, Semigroup};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(Some(2));
    /// let c = Coalesce(Some(3));
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    /// assert_eq!(lazy.combine_cloned(), Coalesce(Some(1)));
    /// ```
    pub fn combine_cloned(&self) -> T
    where
        T: Clone,
    {
        let (first, tail) = self.split_first();
        tail.iter().cloned().fold(first.clone(), Semigroup::op)
    }

    /// Evaluates [`Lazy`] buffer in reverse order.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Lazy, Semigroup};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(Some(2));
    /// let c = Coalesce(Some(3));
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    /// assert_eq!(lazy.combine_rev(), Coalesce(Some(3)));
    /// ```
    pub fn combine_rev(self) -> T {
        let (last, head) = self.split_off_last();
        head.into_iter().rfold(last, Semigroup::op)
    }

    /// Evaluates [`Lazy`] buffer in reverse order like [`Lazy::combine_rev`] by cloning each element.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Lazy, Semigroup};
    /// let a = Coalesce(Some(1));
    /// let b = Coalesce(Some(2));
    /// let c = Coalesce(Some(3));
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    /// assert_eq!(lazy.combine_rev_cloned(), Coalesce(Some(3)));
    /// ```
    pub fn combine_rev_cloned(&self) -> T
    where
        T: Clone,
    {
        let (last, head) = self.split_last();
        head.iter().cloned().rfold(last.clone(), Semigroup::op)
    }
}
impl<T> From<T> for Lazy<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}
impl<T> Lazy<T> {
    /// Create [`Lazy`] from iterator like [`CombineIterator::collect_lazy`].
    pub fn from_iterator<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        // compile error: type parameter `T` must be used as the type parameter for some local type
        // impl<T> FromIterator<T> for Option<Lazy<T>> {
        //     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        //         todo!()
        //     }
        // }
        let mut iterator = iter.into_iter();
        iterator
            .next()
            .map(|head| Self(Some(head).into_iter().chain(iterator).collect()))
    }
    /// Returns `true` if the [`Lazy`] buffer contains no elements. It is nonempty, so always returns `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns `true` if the [`Lazy`] buffer contains exactly one element.
    pub fn is_single(&self) -> bool {
        self.0.len() == 1
    }
    /// Returns the number of elements in the [`Lazy`] buffer. It is nonempty, so always returns `1` or more.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Returns a reference to the first element of the [`Lazy`] buffer.
    pub fn first(&self) -> &T {
        self.0.first().unwrap_or_else(|| unreachable!())
    }
    /// Returns the reference to the first element and all the rest elements of the [`Lazy`] buffer.
    pub fn split_first(&self) -> (&T, &[T]) {
        self.0.split_first().unwrap_or_else(|| unreachable!())
    }
    /// Returns the first element and all the rest elements of the [`Lazy`] buffer.
    pub fn split_off_first(mut self) -> (T, Vec<T>) {
        let tail = self.0.split_off(1);
        (self.0.pop().unwrap_or_else(|| unreachable!()), tail)
    }
    /// Returns a reference to the last element of the [`Lazy`] buffer.
    pub fn last(&self) -> &T {
        self.0.last().unwrap_or_else(|| unreachable!())
    }
    /// Returns the reference to the last element and all the rest elements of the [`Lazy`] buffer.
    pub fn split_last(&self) -> (&T, &[T]) {
        self.0.split_last().unwrap_or_else(|| unreachable!())
    }
    /// Returns the last element and all the rest elements of the [`Lazy`] buffer.
    pub fn split_off_last(mut self) -> (T, Vec<T>) {
        let mut tail = self.0.split_off(self.0.len() - 1);
        (tail.pop().unwrap_or_else(|| unreachable!()), self.0)
    }
    /// Returns an element or slice like [`Vec`].
    pub fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.0.get(index)
    }
    /// Returns an iterator over of the [`Lazy`] buffer.
    pub fn iter(&self) -> <&[T] as IntoIterator>::IntoIter {
        self.0.iter()
    }
    /// Maps each element of the [`Lazy`] buffer with a function, and returns a new [`Lazy`] buffer.
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Lazy<U> {
        Lazy(self.0.into_iter().map(f).collect())
    }
}
impl<T, A: PartialEq> Lazy<Annotated<T, A>> {
    /// **O(n)**, searches for a value that has the given annotation.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.find_annotated(&"edge"), Some(&a));
    /// assert_eq!(lazy.find_annotated(&"middle"), Some(&b));
    /// assert_eq!(lazy.find_annotated(&"where"), None);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn find_annotated(&self, annotation: &A) -> Option<&Annotated<T, A>> {
        self.0
            .iter()
            .find(|annotated| annotated.annotation() == annotation)
    }
    /// **O(n)**, searches for a value's index that has the given annotation.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.position_annotated(&"edge"), Some(0));
    /// assert_eq!(lazy.position_annotated(&"middle"), Some(1));
    /// assert_eq!(lazy.position_annotated(&"where"), None);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn position_annotated(&self, annotation: &A) -> Option<usize> {
        self.0
            .iter()
            .position(|annotated| annotated.annotation() == annotation)
    }

    /// **O(n)**, searches for a value that has the given annotation from the end.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.rfind_annotated(&"edge"), Some(&c));
    /// assert_eq!(lazy.rfind_annotated(&"middle"), Some(&b));
    /// assert_eq!(lazy.rfind_annotated(&"where"), None);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn rfind_annotated(&self, annotation: &A) -> Option<&Annotated<T, A>> {
        self.0
            .iter()
            .rfind(|annotated| annotated.annotation() == annotation)
    }
    /// **O(n)**, searches for a value's index that has the given annotation from the end.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.rposition_annotated(&"edge"), Some(2));
    /// assert_eq!(lazy.rposition_annotated(&"middle"), Some(1));
    /// assert_eq!(lazy.rposition_annotated(&"where"), None);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn rposition_annotated(&self, annotation: &A) -> Option<usize> {
        self.0
            .iter()
            .rposition(|annotated| annotated.annotation() == annotation)
    }

    /// **O(n)**, searches for all values that have the given annotation.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.find_annotated_all(&"edge").collect::<Vec<_>>(), vec![&a, &c]);
    /// assert_eq!(lazy.find_annotated_all(&"middle").collect::<Vec<_>>(), vec![&b]);
    /// assert_eq!(lazy.find_annotated_all(&"where").collect::<Vec<_>>(), vec![&a; 0]);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn find_annotated_all<'a>(
        &'a self,
        annotation: &'a A,
    ) -> impl 'a + Iterator<Item = &'a Annotated<T, A>> {
        self.0
            .iter()
            .filter(move |annotated| annotated.annotation() == annotation)
    }

    /// **O(n)**, searches for all values' indices that have the given annotation.
    ///
    /// # Examples
    /// ```
    /// use semigroup::{op::Coalesce, Annotate, Lazy, Semigroup};
    ///
    /// let a = Coalesce(Some(1)).annotated("edge");
    /// let b = Coalesce(Some(2)).annotated("middle");
    /// let c = Coalesce(Some(3)).annotated("edge");
    ///
    /// let lazy = Lazy::from(a).semigroup(b.into()).semigroup(c.into());
    ///
    /// assert_eq!(lazy.position_annotated_all(&"edge").collect::<Vec<_>>(), vec![0, 2]);
    /// assert_eq!(lazy.position_annotated_all(&"middle").collect::<Vec<_>>(), vec![1]);
    /// assert_eq!(lazy.position_annotated_all(&"where").collect::<Vec<_>>(), vec![0; 0]);
    /// assert_eq!(lazy.combine(), Coalesce(Some(1)).annotated("edge"));
    /// ```
    pub fn position_annotated_all<'a>(
        &'a self,
        annotation: &'a A,
    ) -> impl 'a + Iterator<Item = usize> {
        self.0
            .iter()
            .enumerate()
            .filter(move |(_, annotated)| annotated.annotation() == annotation)
            .map(move |(i, _)| i)
    }
}
impl<T> IntoIterator for Lazy<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T> Extend<T> for Lazy<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}
impl<T, I: SliceIndex<[T]>> Index<I> for Lazy<T> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(feature = "test")]
pub mod test_lazy {
    use std::fmt::Debug;

    use super::*;

    pub fn assert_lazy<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let lazy = Lazy::from(a.clone());
        assert_eq!(lazy.combine_cloned(), a.clone());
        assert_eq!(lazy.combine_rev_cloned(), a.clone());

        let lazy = lazy.semigroup(b.clone().into()).semigroup(c.clone().into());
        assert_eq!(
            lazy.clone().combine(),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            lazy.combine_cloned(),
            Semigroup::op(Semigroup::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            lazy.clone().combine_rev(),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
        assert_eq!(
            lazy.combine_rev_cloned(),
            Semigroup::op(Semigroup::op(c.clone(), b.clone()), a.clone())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_nonempty() {
        let l = Lazy::from_iterator(Vec::<u32>::new());
        assert!(l.is_none());

        let l = Lazy::from_iterator(vec![1]);
        assert!(l.is_some());
    }

    #[test]
    fn test_lazy_small() {
        let l = Lazy::from(1);
        assert!(!l.is_empty());
        assert!(l.is_single());
        assert_eq!(l.first(), &1);
        assert_eq!(l.last(), &1);
        assert_eq!(l, Lazy::from_iterator(vec![1]).unwrap());

        let ll = Lazy::from_iterator(vec![1, 2]).unwrap();
        assert!(!ll.is_empty());
        assert!(!ll.is_single());
        assert_eq!(ll.first(), &1);
        assert_eq!(ll.last(), &2);
    }

    #[test]
    fn test_lazy_split() {
        let l = Lazy::from(1);
        assert_eq!(l.split_first(), (&1, &[][..]));
        assert_eq!(l.split_last(), (&1, &[][..]));
        assert_eq!(l.clone().split_off_first(), (1, vec![]));
        assert_eq!(l.clone().split_off_last(), (1, vec![]));

        let ll = Lazy::from_iterator(vec![1, 2, 3, 4, 5]).unwrap();
        assert_eq!(ll.split_first(), (&1, &[2, 3, 4, 5][..]));
        assert_eq!(ll.split_last(), (&5, &[1, 2, 3, 4][..]));
        assert_eq!(ll.clone().split_off_first(), (1, vec![2, 3, 4, 5]));
        assert_eq!(ll.clone().split_off_last(), (5, vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_lazy_index() {
        let l = Lazy::from(1);
        assert_eq!(l[0], 1);
        assert_eq!(l[..], [1]);
        assert_eq!(l[..1], [1]);
        assert_eq!(l.get(0), Some(&1));
        assert_eq!(l.get(1), None);
        assert_eq!(l.get(..), Some(&[1][..]));
        assert_eq!(l.get(..1), Some(&[1][..]));

        let ll = Lazy::from_iterator(vec![1, 2]).unwrap();
        assert_eq!(ll[0], 1);
        assert_eq!(ll[1], 2);
        assert_eq!(ll[..], [1, 2]);
        assert_eq!(ll[..1], [1]);
        assert_eq!(ll.get(0), Some(&1));
        assert_eq!(ll.get(1), Some(&2));
        assert_eq!(ll.get(2), None);
        assert_eq!(ll.get(..), Some(&[1, 2][..]));
        assert_eq!(ll.get(..1), Some(&[1][..]));
    }

    #[test]
    fn test_lazy_iter() {
        let l = Lazy::from(1);
        assert_eq!(l.iter().collect::<Vec<_>>(), vec![&1]);
        assert_eq!(l.clone().into_iter().collect::<Vec<_>>(), vec![1]);

        let ll = Lazy::from_iterator(0..100000).unwrap();
        assert_eq!(
            ll.iter().cloned().collect::<Vec<_>>(),
            (0..100000).collect::<Vec<_>>()
        );
        assert_eq!(
            ll.clone().into_iter().collect::<Vec<_>>(),
            (0..100000).collect::<Vec<_>>()
        );
    }
}

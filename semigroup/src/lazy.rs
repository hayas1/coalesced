use std::{ops::Index, slice::SliceIndex};

use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Construction, Reverse, Semigroup};

pub trait CombineIterator: Sized + Iterator {
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
    fn rfold_final(self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        self.map(Reverse)
            .fold(Reverse(fin), Semigroup::op)
            .into_inner()
    }
}
impl<I: Iterator> CombineIterator for I {}

/// A lazy [`Semigroup`] that is implemented as a nonempty [`Vec`].
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
    pub fn combine(self) -> T {
        let (first, tail) = self.split_off_first();
        tail.into_iter().fold(first, Semigroup::op)
    }
    pub fn combine_cloned(&self) -> T
    where
        T: Clone,
    {
        let (first, tail) = self.split_first();
        tail.iter().cloned().fold(first.clone(), Semigroup::op)
    }
    pub fn combine_rev(self) -> T {
        let (last, head) = self.split_off_last();
        head.into_iter().rfold(last, Semigroup::op)
    }
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
        Self::new(value)
    }
}

impl<T> Lazy<T> {
    pub fn new(value: T) -> Self {
        Self(vec![value])
    }
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
    /// Returns `true` if the collection contains no elements. [`Lazy`] is nonempty, so always returns `false`.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn is_single(&self) -> bool {
        self.0.len() == 1
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn first(&self) -> &T {
        self.0.first().unwrap_or_else(|| unreachable!())
    }
    pub fn split_first(&self) -> (&T, &[T]) {
        self.0.split_first().unwrap_or_else(|| unreachable!())
    }
    pub fn split_off_first(mut self) -> (T, Vec<T>) {
        let tail = self.0.split_off(1);
        (self.0.pop().unwrap_or_else(|| unreachable!()), tail)
    }
    pub fn last(&self) -> &T {
        self.0.last().unwrap_or_else(|| unreachable!())
    }
    pub fn split_last(&self) -> (&T, &[T]) {
        self.0.split_last().unwrap_or_else(|| unreachable!())
    }
    pub fn split_off_last(mut self) -> (T, Vec<T>) {
        let mut tail = self.0.split_off(self.0.len() - 1);
        (tail.pop().unwrap_or_else(|| unreachable!()), self.0)
    }
    pub fn get<I: SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.0.get(index)
    }
    pub fn iter(&self) -> <&[T] as IntoIterator>::IntoIter {
        self.0.iter()
    }
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Lazy<U> {
        Lazy(self.0.into_iter().map(f).collect())
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

    pub fn assert_combine_iter<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab = vec![a.clone(), b.clone()];
        assert_eq!(
            ab.into_iter().fold_final(c.clone()),
            T::op(T::op(a.clone(), b.clone()), c.clone())
        );

        let bc = vec![b.clone(), c.clone()];
        assert_eq!(
            bc.into_iter().rfold_final(a.clone()),
            T::op(T::op(c.clone(), b.clone()), a.clone())
        );
    }

    pub fn assert_lazy<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let lazy = Lazy::from(a.clone());
        assert!(!lazy.is_empty());
        assert!(lazy.is_single());
        assert_eq!(lazy.first(), &a);
        assert_eq!(lazy.last(), &a);
        assert_eq!(lazy.get(0), Some(&a));
        assert_eq!(lazy.get(1), None);
        assert_eq!(lazy.get(..), Some(&[a.clone()][..]));
        assert_eq!(lazy.clone().split_off_first(), (a.clone(), vec![]));
        assert_eq!(lazy.clone().split_off_last(), (a.clone(), vec![]));
        assert_eq!(lazy.combine_cloned(), a.clone());
        assert_eq!(lazy.combine_rev_cloned(), a.clone());

        let lazy = lazy.semigroup(b.clone().into()).semigroup(c.clone().into());
        assert!(!lazy.is_empty());
        assert!(!lazy.is_single());
        assert_eq!(lazy.first(), &a);
        assert_eq!(lazy.last(), &c);
        assert_eq!(lazy.get(0), Some(&a));
        assert_eq!(lazy.get(1), Some(&b));
        assert_eq!(lazy.get(2), Some(&c));
        assert_eq!(lazy.get(3), None);
        assert_eq!(lazy.get(..), Some(&[a.clone(), b.clone(), c.clone()][..]));
        assert_eq!(
            lazy.clone().split_off_first(),
            (a.clone(), vec![b.clone(), c.clone()])
        );
        assert_eq!(
            lazy.clone().split_off_last(),
            (c.clone(), vec![a.clone(), b.clone()])
        );

        assert_eq!(
            lazy.clone().combine(),
            T::op(T::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            lazy.combine_cloned(),
            T::op(T::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            lazy.clone().combine_rev(),
            T::op(T::op(c.clone(), b.clone()), a.clone())
        );
        assert_eq!(
            lazy.combine_rev_cloned(),
            T::op(T::op(c.clone(), b.clone()), a.clone())
        );
    }
}

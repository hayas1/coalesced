use std::{ops::Index, slice::SliceIndex};

use crate::{Construction, Reverse, Semigroup};

pub trait CombineIterator: Sized + Iterator {
    fn fold_final(self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        let iter = self.chain(Some(fin));
        iter.reduce(Semigroup::op).unwrap_or_else(|| unreachable!())
    }
    fn rfold_final(self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        let iter = Some(fin).into_iter().chain(self);
        iter.map(Reverse)
            .reduce(Semigroup::op)
            .unwrap_or_else(|| unreachable!())
            .into_inner()
    }
}
impl<I: Iterator> CombineIterator for I {}

/// A lazy [`Semigroup`] that is implemented as a nonempty [`Vec`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lazy<T>(Vec<T>);
impl<T> Semigroup for Lazy<T> {
    fn op(mut base: Self, other: Self) -> Self {
        base.extend(other);
        base
    }
}
impl<T: Semigroup> Lazy<T> {
    pub fn combine(self) -> T {
        let (head, tail) = self.split_off_first();
        tail.into_iter().fold(head, Semigroup::op)
    }
    pub fn combine_cloned(&self) -> T
    where
        T: Clone,
    {
        let (head, tail) = self.split_first();
        tail.iter().cloned().fold(head.clone(), Semigroup::op)
    }
    pub fn combine_rev(self) -> T {
        let (head, tail) = self.split_off_last();
        tail.into_iter().rfold(head, Semigroup::op)
    }
    pub fn combine_rev_cloned(&self) -> T
    where
        T: Clone,
    {
        let (head, tail) = self.split_last();
        tail.iter().cloned().rfold(head.clone(), Semigroup::op)
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
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() // must be false
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
        let head = self.0.split_off(self.0.len() - 1);
        (self.0.pop().unwrap_or_else(|| unreachable!()), head)
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

use crate::semigroup::Semigroup;

pub trait SemigroupIterator: Sized + Iterator {
    fn fold_final(mut self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next() {
            self.chain(Some(fin))
                .fold(init, |acc, item| acc.semigroup(item))
        } else {
            fin
        }
    }
}
impl<I: Iterator> SemigroupIterator for I {}

pub trait SemigroupDoubleEndedIterator: Sized + DoubleEndedIterator {
    fn rfold_final(mut self, fin: Self::Item) -> Self::Item
    where
        Self::Item: Semigroup,
    {
        if let Some(init) = self.next_back() {
            Some(fin)
                .into_iter()
                .chain(self)
                .rfold(init, |acc, item| acc.semigroup(item))
        } else {
            fin
        }
    }
}
impl<I: DoubleEndedIterator> SemigroupDoubleEndedIterator for I {}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use super::*;

    pub fn assert_lazy_evaluation_iter<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let empty = vec![];
        assert_eq!(
            empty.iter().cloned().reduce(|acc, item| T::op(acc, item)),
            None
        );
        assert_eq!(
            empty.into_iter().rev().reduce(|acc, item| T::op(item, acc)),
            None
        );

        let lazy = vec![a.clone(), b.clone(), c.clone()];
        assert_eq!(
            lazy.iter().cloned().reduce(|acc, item| T::op(acc, item)),
            Some(T::op(T::op(a.clone(), b.clone()), c.clone()))
        );
        assert_eq!(
            lazy.into_iter().rev().reduce(|acc, item| T::op(item, acc)),
            Some(T::op(a.clone(), T::op(b.clone(), c.clone())))
        );

        let ab = vec![a.clone(), b.clone()];
        assert_eq!(
            ab.iter()
                .cloned()
                .rfold(c.clone(), |acc, item| T::op(item, acc)),
            T::op(a.clone(), T::op(b.clone(), c.clone()))
        );
        assert_eq!(
            ab.into_iter().fold_final(c.clone()),
            T::op(T::op(a.clone(), b.clone()), c.clone())
        );

        let bc = vec![b.clone(), c.clone()];
        assert_eq!(
            bc.iter()
                .cloned()
                .fold(a.clone(), |acc, item| T::op(acc, item)),
            T::op(T::op(a.clone(), b.clone()), c.clone())
        );
        assert_eq!(
            bc.into_iter().rfold_final(a.clone()),
            T::op(T::op(c.clone(), b.clone()), a.clone())
        );
    }
}

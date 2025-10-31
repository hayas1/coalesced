use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Construction, Lazy, Semigroup};

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

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that flips the order of operands:
/// `op(Reverse(a), Reverse(b)) = Reverse(op(b, a))`.
///
/// If `T` is [`Commutative`], then `op(a, b) = op(b, a)`, and thus [`Reverse`] is meaningless.
///
/// ## Calculate right fold by left fold algorithm
/// By using [`Reverse`], a right fold can be computed using a left fold algorithm.
/// - Let the underlying operation be `a ⊙ b := op(a, b)`, therefore `Reverse(a) ⊙ Reverse(b) := Reverse(b ⊙ a)`
///     - `op` is [`Semigroup`], so that has associativity property: `a ⊙ b ⊙ c = a ⊙ (b ⊙ c) = (a ⊙ b) ⊙ c`
/// - A left fold evaluates as: `v1 ⊙ v2 ... ⊙ vn`
/// - A right fold evaluates as: `vn ⊙ vn-1 ... ⊙ v1`
/// Now, the left fold of [`Reverse`] is `Reverse(v1) ⊙ Reverse(v2) ... ⊙ Reverse(vn)`.
/// ```text
/// Reverse(v1) ⊙ Reverse(v2) ⊙ Reverse(v3) ⊙ ... ⊙ Reverse(vn-1) ⊙ Reverse(vn)
/// = Reverse(v2 ⊙ v1) ⊙ Reverse(v3) ⊙ ... ⊙ Reverse(vn-1) ⊙ Reverse(vn)
/// = Reverse(v3 ⊙ v2 ⊙ v1) ⊙ ... ⊙ Reverse(vn-1) ⊙ Reverse(vn)
/// ...
/// = Reverse(vn-1 ⊙ ... ⊙ v3 ⊙ v2 ⊙ v1) ⊙ Reverse(vn)
/// = Reverse(vn ⊙ vn-1 ⊙ ... ⊙ v3 ⊙ v2 ⊙ v1)
/// ```
/// The inner expression `vn ⊙ vn-1 ⊙ ... ⊙ v3 ⊙ v2 ⊙ v1` is exactly the right fold of original semigroup.
///
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ## Simple reverse two elements
/// ```
/// use semigroup::{op::Coalesce, Reverse, Construction, Semigroup};
///
/// let a = Coalesce(Some(1));
/// let b = Coalesce(Some(2));
///
/// assert_eq!(a.semigroup(b), Coalesce(Some(1)));
///
/// let ra = Reverse(a);
/// let rb = Reverse(b);
///
/// assert_eq!(ra.semigroup(rb).into_inner(), Coalesce(Some(2)));
/// ```
///
/// ## Calculate right fold by left fold algorithm
/// ```
/// # #[cfg(feature="monoid")]
/// # {
/// use semigroup::{op::Coalesce, Reverse, Construction, Semigroup, Monoid};
///
/// let v = (1..100).map(Some).map(Coalesce).collect::<Vec<_>>();
///
/// assert_eq!(v.iter().cloned().fold(Monoid::identity(), Semigroup::op), Coalesce(Some(1)));
/// assert_eq!(v.iter().cloned().map(Reverse).fold(Monoid::identity(), Semigroup::op).into_inner(), Coalesce(Some(99)));
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[construction(monoid, commutative, identity = Self(T::identity()), monoid_where = "T: crate::Monoid", commutative_where = "T: crate::Commutative")]
#[properties_priv(
    monoid,
    commutative,
    monoid_where = "T: crate::Monoid",
    commutative_where = "T: crate::Commutative"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T: Semigroup>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(Semigroup::op(other.0, base.0))
    }
}

#[cfg(feature = "test")]
pub mod test_combine {
    use std::fmt::Debug;

    use crate::semigroup::test_semigroup::assert_associative_law;

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

    pub fn assert_semigroup_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_reverse_reverse(a.clone(), b.clone(), c.clone());
        assert_reverse_associative_law(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_reverse_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            Semigroup::op(a.clone(), b.clone()),
            Semigroup::op(rb.clone(), ra.clone()).0
        );
        assert_eq!(
            Semigroup::op(b.clone(), c.clone()),
            Semigroup::op(rc.clone(), rb.clone()).0
        );
        assert_eq!(
            Semigroup::op(a.clone(), c.clone()),
            Semigroup::op(rc.clone(), ra.clone()).0
        );
    }
    pub fn assert_reverse_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a), Reverse(b), Reverse(c));
        assert_associative_law(ra, rb, rc);
    }
}

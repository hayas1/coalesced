use semigroup_derive::{properties, ConstructionUse};

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

/// A semigroup construction that returns the maximum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{semigroup::Semigroup, op::{Construction, max::Max}};
///
/// let a = Max(1);
/// let b = Max(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
#[properties(monoid, commutative)]
pub struct Max<T: Ord>(pub T);
impl<T: Ord> Semigroup for Max<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(std::cmp::max(base.0, other.0))
    }
}
#[cfg(feature = "monoid")]
impl<T: Ord + num::Bounded> crate::monoid::Monoid for Max<T> {
    fn unit() -> Self {
        Self(T::min_value())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup};

    use super::*;

    #[test]
    fn test_max_as_semigroup() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_max_as_monoid() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_max_commutative() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_max() {
        let (a, b) = (Max(1), Max(2));
        assert_eq!(a.semigroup(b).into_inner(), 2);
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }
}

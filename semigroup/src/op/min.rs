use semigroup_derive::{properties, ConstructionPriv};

use crate::Semigroup;

/// A semigroup construction that returns the minimum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::min::Min, Construction, Semigroup};
///
/// let a = Min(1);
/// let b = Min(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative)]
#[properties(monoid, commutative)]
pub struct Min<T: Ord>(pub T);
impl<T: Ord> Semigroup for Min<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(std::cmp::min(base.0, other.0))
    }
}
#[cfg(feature = "monoid")]
impl<T: Ord + num::Bounded> Default for Min<T> {
    fn default() -> Self {
        Self(T::max_value())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_min_as_semigroup() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_min_as_monoid() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_min_commutative() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_min() {
        let (a, b) = (Min(1), Min(2));
        assert_eq!(a.semigroup(b).into_inner(), 1);
        assert_eq!(b.semigroup(a).into_inner(), 1);
    }
}

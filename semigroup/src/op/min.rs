use semigroup_derive::{properties, ConstructionPriv};

use crate::{Annotated, AnnotatedSemigroup};

/// A semigroup construction that returns the minimum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Min, Construction, Semigroup};
///
/// let a = Min(1);
/// let b = Min(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, commutative, unit = Self(T::max_value()), unit_where = "T: num::Bounded")]
#[properties(annotated, monoid, commutative)]
pub struct Min<T: Ord>(pub T);
impl<A, T: Ord> AnnotatedSemigroup<A> for Min<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        std::cmp::min_by(base, other, |a, b| a.value().cmp(b.value()))
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

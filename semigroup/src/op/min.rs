use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Annotated, AnnotatedSemigroup};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the minimum value.
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
#[construction(annotated, monoid, commutative, identity = Self(T::max_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(annotated, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Min<T: Ord>(pub T);
impl<A, T: Ord> AnnotatedSemigroup<A> for Min<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        std::cmp::min_by(base, other, |a, b| a.value().cmp(b.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Construction, Semigroup};

    use super::*;

    #[test]
    fn test_min_semigroup() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_min_monoid() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    #[cfg(feature = "commutative")]
    fn test_min_commutative() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        crate::assert_commutative!(a, b, c);
    }

    #[test]
    fn test_min() {
        let (a, b) = (Min(1), Min(2));
        assert_eq!(a.semigroup(b).into_inner(), 1);
        assert_eq!(b.semigroup(a).into_inner(), 1);
    }
}

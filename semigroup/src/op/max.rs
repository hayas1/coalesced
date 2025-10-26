use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Annotated, AnnotatedSemigroup};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the maximum value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Max, Construction, Semigroup};
///
/// let a = Max(1);
/// let b = Max(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, commutative, unit = Self(T::min_value()), monoid_where = "T: num::Bounded")]
#[properties_priv(annotated, monoid, commutative, monoid_where = "T: num::Bounded")]
pub struct Max<T: Ord>(pub T);
impl<A, T: Ord> AnnotatedSemigroup<A> for Max<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        std::cmp::max_by(base, other, |a, b| a.value().cmp(b.value()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_max_semigroup() {
        let (a, b, c) = (Max(1), Max(2), Max(3));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_max_monoid() {
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

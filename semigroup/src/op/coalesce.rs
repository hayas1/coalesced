use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Annotated, AnnotatedSemigroup};

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the first non-`None` value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Coalesce, Construction, Semigroup};
///
/// let a = Coalesce(None);
/// let b = Coalesce(Some(2));
///
/// assert_eq!(a.semigroup(b).into_inner(), Some(2));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated, monoid, identity = Self(None))]
#[properties_priv(annotated, monoid)]
pub struct Coalesce<T>(pub Option<T>);
impl<T, A> AnnotatedSemigroup<A> for Coalesce<T> {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        match (&base.value().0, &other.value().0) {
            (Some(_), _) | (None, None) => base,
            (None, Some(_)) => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_coalesce_semigroup() {
        let (a, b, c) = (Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(None));
        assert_semigroup!(a, b, c);
        let (a, b, c) = (Coalesce::<u32>(None), Coalesce(None), Coalesce(None));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_coalesce_monoid() {
        let (a, b, c) = (Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(None));
        assert_monoid!(a, b, c);
        let (a, b, c) = (Coalesce::<u32>(None), Coalesce(None), Coalesce(None));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_coalesce() {
        let (a, b) = (Coalesce(None), Coalesce(Some("value")));
        assert_eq!(a.semigroup(b).into_inner(), Some("value"));
        assert_eq!(b.semigroup(a).into_inner(), Some("value"));

        let (a, b) = (Coalesce(Some(1)), Coalesce(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(1));
        assert_eq!(b.semigroup(a).into_inner(), Some(2));
    }
}

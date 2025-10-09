use semigroup_derive::ConstructionUse;

use crate::{
    annotate::{Annotate, Annotated},
    op::{Construction, ConstructionAnnotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated)]
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
    use crate::{reverse::Reverse, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_coalesce_as_semigroup_op() {
        let (a, b, c) = (Coalesce(Some(1)), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_semigroup_op!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(Some(3)));
        assert_semigroup_op!(a, b, c);
        let (a, b, c) = (Coalesce(None), Coalesce(Some(2)), Coalesce(None));
        assert_semigroup_op!(a, b, c);
        let (a, b, c) = (Coalesce::<u32>(None), Coalesce(None), Coalesce(None));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_coalesce() {
        let (a, b) = (Coalesce(None), Coalesce(Some("value")));
        assert_eq!(a.semigroup(b).into_inner(), Some("value"));
        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.semigroup(rb).0.into_inner(), Some("value"));

        let (a, b) = (Coalesce(Some(1)), Coalesce(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(1));
        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.semigroup(rb).0.into_inner(), Some(2));
    }
}

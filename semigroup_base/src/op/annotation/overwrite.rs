use semigroup_derive::ConstructionUse;

use crate::{
    annotate::{Annotate, Annotated},
    op::{Construction, ConstructionAnnotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated)]
pub struct Overwrite<T>(pub T);
impl<T, A> AnnotatedSemigroup<A> for Overwrite<T> {
    fn annotated_op(_base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        other
    }
}

#[cfg(test)]
mod tests {
    use crate::{reverse::Reverse, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_overwrite_as_semigroup_op() {
        let (a, b, c) = (Overwrite(1), Overwrite(2), Overwrite(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_overwrite() {
        let (a, b) = (Overwrite(1), Overwrite(2));
        assert_eq!(a.semigroup(b).into_inner(), 2);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.semigroup(rb).0.into_inner(), 1);
    }
}

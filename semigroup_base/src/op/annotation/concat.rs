use semigroup_derive::ConstructionUse;

use crate::{
    annotate::Annotated,
    op::{Construction, ConstructionAnnotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(
    annotated,
    annotation_type_param = "A: IntoIterator + FromIterator<A::Item>",
    annotation_where = "A::Item: Clone",
    unit = "vec![(); 0]",
    without_annotate_impl
)]
pub struct Concat<T: IntoIterator + FromIterator<T::Item>>(pub T);
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    AnnotatedSemigroup<A> for Concat<T>
{
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        let (base_value, base_annotation) = base.into_parts();
        let (other_value, other_annotation) = other.into_parts();

        Annotated::new(
            Concat(base_value.0.into_iter().chain(other_value.0).collect()),
            base_annotation
                .into_iter()
                .chain(other_annotation)
                .collect(),
        )
    }
}
impl<T: IntoIterator + FromIterator<T::Item>, A: IntoIterator + FromIterator<A::Item>>
    crate::annotate::Annotate<A> for Concat<T>
where
    A::Item: Clone,
{
    type Annotation = A::Item;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A> {
        let iter = self.0.into_iter();
        let (len, _) = iter.size_hint(); // TODO use exact size
        Annotated::new(
            Self(iter.collect()),
            std::iter::repeat_n(annotation, len).collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{commutative::Reverse, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_concat_as_semigroup_op() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concat(vec![1]), Concat(vec![2]));
        assert_eq!(a.clone().semigroup(b.clone()).into_inner(), vec![1, 2]);

        let (ra, rb) = (Reverse(a.clone()), Reverse(b.clone()));
        assert_eq!(ra.semigroup(rb).0.into_inner(), vec![2, 1]);
    }
}

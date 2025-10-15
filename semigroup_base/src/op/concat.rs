use semigroup_derive::{properties, ConstructionUse};

use crate::{
    annotate::Annotated,
    op::{Construction, ConstructionAnnotated},
    semigroup::{AnnotatedSemigroup, Semigroup},
};

/// A semigroup construction that concatenates two values.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup_base::{semigroup::Semigroup, op::{Construction, concat::Concat}};
///
/// let a = Concat(vec![1, 2]);
/// let b = Concat(vec![3, 4]);
///
/// assert_eq!(a.semigroup(b).into_inner(), vec![1, 2, 3, 4]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(
    annotated,
    annotation_type_param = "A: IntoIterator + FromIterator<A::Item>",
    annotation_where = "A::Item: Clone",
    unit = "vec![(); 0]",
    without_annotate_impl
)]
#[properties(annotated, monoid)]
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
        let (lower, upper) = iter.size_hint();
        match upper.filter(|&u| u == lower) {
            Some(len) => Annotated::new(
                Self(iter.collect()),
                std::iter::repeat_n(annotation, len).collect(),
            ),
            None => {
                let (value, annotation): (Vec<_>, Vec<_>) =
                    iter.map(|v| (v, annotation.clone())).collect();
                Annotated::new(
                    Self(value.into_iter().collect()),
                    annotation.into_iter().collect(),
                )
            }
        }
    }
}
#[cfg(feature = "monoid")]
impl<T: IntoIterator + FromIterator<T::Item>> crate::monoid::Monoid for Concat<T> {
    fn unit() -> Self {
        Concat(std::iter::empty().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_monoid, semigroup::tests::assert_semigroup};

    use super::*;

    #[test]
    fn test_concat_as_semigroup() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_concat_as_monoid() {
        let (a, b, c) = (Concat(vec![1]), Concat(vec![2]), Concat(vec![3]));
        assert_monoid!(a, b, c)
    }

    #[test]
    fn test_concat() {
        let (a, b) = (Concat(vec![1]), Concat(vec![2]));
        assert_eq!(a.clone().semigroup(b.clone()).into_inner(), vec![1, 2]);
        assert_eq!(b.semigroup(a).into_inner(), vec![2, 1]);
    }
}

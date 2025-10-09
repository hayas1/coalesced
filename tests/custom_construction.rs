use semigroup::{
    assert_semigroup_op, op::Construction, Annotated, AnnotatedSemigroup, Construction, Semigroup,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
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
    let (some_value1, some_value2, none) = (
        Coalesce(Some("value1")),
        Coalesce(Some("value2")),
        Coalesce(None),
    );
    assert_eq!(none.semigroup(none).into_inner(), None);
    assert_eq!(some_value1.semigroup(none).into_inner(), Some("value1"));
    assert_eq!(none.semigroup(some_value1).into_inner(), Some("value1"));
    assert_eq!(
        some_value1.semigroup(some_value2).into_inner(),
        Some("value1")
    );
    assert_eq!(
        some_value2.semigroup(some_value1).into_inner(),
        Some("value2")
    );
}

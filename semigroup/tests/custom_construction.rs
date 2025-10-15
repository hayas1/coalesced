#![cfg(all(feature = "monoid", feature = "test"))]
use semigroup::{
    assert_monoid, assert_semigroup, monoid::Monoid, op::Construction, properties, Construction,
    Semigroup,
};

/// A semigroup construction that join two [`String`]s into a [`String`].
/// # Properties
/// <!-- properties -->
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
#[construction()]
#[properties(monoid)]
pub struct Join(pub String);
impl Semigroup for Join {
    fn op(mut base: Self, other: Self) -> Self {
        base.0.push_str(&other.0);
        base
    }
}
impl Monoid for Join {
    fn unit() -> Self {
        Self(String::new())
    }
}

#[test]
fn test_join_as_semigroup() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_semigroup!(a, b, c);
}

#[test]
fn test_join_as_monoid() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_monoid!(a, b, c)
}

#[test]
fn test_join() {
    let (a, b, c) = (
        Join("a".to_string()),
        Join("b".to_string()),
        Join("c".to_string()),
    );
    assert_eq!(a.semigroup(b).semigroup(c).into_inner(), "abc");
}

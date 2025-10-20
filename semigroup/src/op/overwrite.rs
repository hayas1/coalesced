use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::{Annotated, AnnotatedSemigroup};

/// A semigroup construction that returns the second value.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Overwrite, Construction, Semigroup};
///
/// let a = Overwrite(1);
/// let b = Overwrite(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(annotated)]
#[properties_priv(annotated)]
pub struct Overwrite<T>(pub T);
impl<T, A> AnnotatedSemigroup<A> for Overwrite<T> {
    fn annotated_op(_base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
        other
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_overwrite_as_semigroup() {
        let (a, b, c) = (Overwrite(1), Overwrite(2), Overwrite(3));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_overwrite() {
        let (a, b) = (Overwrite(Some(1)), Overwrite(Some(2)));
        assert_eq!(a.semigroup(b).into_inner(), Some(2));
        assert_eq!(b.semigroup(a).into_inner(), Some(1));
    }
}

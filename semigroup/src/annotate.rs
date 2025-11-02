use std::ops::{Deref, DerefMut};

use semigroup_derive::{ConstructionPriv, properties_priv};

use crate::{AnnotatedSemigroup, Semigroup};

/// Some [`Semigroup`] such as [`crate::op::Coalesce`] can have an annotation.
/// The annotated operation is represented by [`AnnotatedSemigroup`], and the annotated value is represented by a type [`Annotated`].
///
/// # Examples
/// ## Deriving
/// [`Annotate`] can be derived like [`Semigroup`], use `annotated` attribute.
/// ```
/// use semigroup::{op::Coalesce, Annotate, Annotated, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Semigroup)]
/// #[semigroup(annotated, with = "semigroup::op::Coalesce")]
/// struct ExampleStruct<'a> {
///     num: Option<u32>,
///     str: Option<&'a str>,
///     #[semigroup(with = "semigroup::op::Overwrite")]
///     boolean: bool,
/// }
///
/// let a = ExampleStruct { num: Some(1), str: None, boolean: true }.annotated("first");
/// let b = ExampleStruct { num: None, str: Some("ten"), boolean: false }.annotated("second");
/// let c = ExampleStruct { num: Some(100), str: None, boolean: false }.annotated("third");
///
/// let ab = a.semigroup(b);
/// assert_eq!(ab.value(), &ExampleStruct { num: Some(1), str: Some("ten"), boolean: false });
/// assert_eq!(ab.annotation().num, "first");
/// assert_eq!(ab.annotation().str, "second");
/// assert_eq!(ab.annotation().boolean, "second");
/// assert_eq!(ab.annotation(), &ExampleStructAnnotation{ num: "first", str: "second", boolean: "second" });
///
/// let bc = b.semigroup(c);
/// assert_eq!(bc.value(), &ExampleStruct { num: Some(100), str: Some("ten"), boolean: false });
/// assert_eq!(bc.annotation().num, "third");
/// assert_eq!(bc.annotation().str, "second");
/// assert_eq!(bc.annotation().boolean, "third");
/// assert_eq!(bc.annotation(), &ExampleStructAnnotation{ num: "third", str: "second", boolean: "third" });
///
/// let ca = c.semigroup(a);
/// assert_eq!(ca.value(), &ExampleStruct { num: Some(100), str: None, boolean: true });
/// assert_eq!(ca.annotation().num, "third");
/// assert_eq!(ca.annotation().str, "third");
/// assert_eq!(ca.annotation().boolean, "first");
/// assert_eq!(ca.annotation(), &ExampleStructAnnotation{ num: "third", str: "third", boolean: "first" });
/// ```
///
/// ## Construction
/// [`Annotate`] can be constructed by [`crate::ConstructionAnnotated`] like [`Semigroup`], use `annotated` attribute.
/// In this case, the [`Semigroup`] annotated operation is represented by [`AnnotatedSemigroup`].
///
/// Some operations are already provided by [`crate::op`].
/// ```
/// use semigroup::{AnnotatedSemigroup, Annotate, Annotated, Construction, ConstructionAnnotated, Semigroup};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction)]
/// #[construction(annotated)]
/// struct Coalesce<T>(Option<T>);
/// impl<A, T> AnnotatedSemigroup<A> for Coalesce<T> {
///     fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A> {
///         match (&base.value().0, &other.value().0) {
///             (Some(_), _) | (None, None) => base,
///             (None, Some(_)) => other,
///         }
///     }
/// }
///
/// let a = Coalesce(Some(1)).annotated("first");
/// let b = Coalesce(None).annotated("second");
/// let c = Coalesce(Some(3)).annotated("third");
///
/// let ab = a.semigroup(b);
/// assert_eq!(ab.value(), &Coalesce(Some(1)));
/// assert_eq!(ab.annotation(), &"first");
///
/// let bc = b.semigroup(c);
/// assert_eq!(bc.value(), &Coalesce(Some(3)));
/// assert_eq!(bc.annotation(), &"third");
///
/// let ca = c.semigroup(a);
/// assert_eq!(ca.value(), &Coalesce(Some(3)));
/// assert_eq!(ca.annotation(), &"third");
/// ```
pub trait Annotate<A>: Sized {
    type Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A>;
}

/// [`Annotated`] represents a [`Semigroup`] value with an annotation.
/// The value will be annotated by [`Annotate`] trait.
///
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Coalesce, Annotate, Annotated};
///
/// let annotated = Coalesce(Some(1)).annotated("first");
///
/// assert_eq!(annotated.value(), &Coalesce(Some(1)));
/// assert_eq!(annotated.annotation(), &"first");
/// assert_eq!(annotated, Annotated::new(Coalesce(Some(1)), "first"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[construction(
    commutative,
    commutative_where = "T: AnnotatedSemigroup<A> + crate::Commutative",
    without_construction
)]
#[properties_priv(
    commutative,
    commutative_where = "T: AnnotatedSemigroup<A> + crate::Commutative"
)]
pub struct Annotated<T, A> {
    value: T,
    annotation: A,
}
impl<T: AnnotatedSemigroup<A>, A> Semigroup for Annotated<T, A> {
    fn op(base: Self, other: Self) -> Self {
        AnnotatedSemigroup::annotated_op(base, other)
    }
}
impl<T: AnnotatedSemigroup<A>, A> Annotated<T, A> {
    pub fn lift_unit_annotated_op((base, unit1): (T, A), (other, unit2): (T, A)) -> T {
        let (b, o) = (Self::new(base, unit1), Self::new(other, unit2));
        AnnotatedSemigroup::annotated_op(b, o).into_value()
    }
}

impl<T, A> Annotated<T, A> {
    /// [`Annotated`] has `new` method, but [`Annotated`] should be created by [`Annotate`] trait.
    pub fn new(value: T, annotation: A) -> Self {
        Self { value, annotation }
    }
    pub fn into_parts(self) -> (T, A) {
        (self.value, self.annotation)
    }
    pub fn parts(&self) -> (&T, &A) {
        (&self.value, &self.annotation)
    }
    pub fn into_value(self) -> T {
        self.value
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
    pub fn into_annotation(self) -> A {
        self.annotation
    }
    pub fn annotation(&self) -> &A {
        &self.annotation
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Annotated<U, A> {
        Annotated {
            value: f(self.value),
            annotation: self.annotation,
        }
    }
    pub fn map_annotation<B>(self, f: impl FnOnce(A) -> B) -> Annotated<T, B> {
        Annotated {
            value: self.value,
            annotation: f(self.annotation),
        }
    }
    pub fn map_parts<U, B>(
        self,
        fv: impl FnOnce(T) -> U,
        fa: impl FnOnce(A) -> B,
    ) -> Annotated<U, B> {
        Annotated {
            value: fv(self.value),
            annotation: fa(self.annotation),
        }
    }

    pub fn as_ref(&self) -> Annotated<&T, &A> {
        Annotated {
            value: &self.value,
            annotation: &self.annotation,
        }
    }
    pub fn as_ref_mut(&mut self) -> Annotated<&mut T, &A> {
        Annotated {
            value: &mut self.value,
            annotation: &self.annotation,
        }
    }
    pub fn as_deref(&self) -> Annotated<&T::Target, &A>
    where
        T: Deref,
    {
        self.as_ref().map(|v| v.deref())
    }
    pub fn as_deref_mut(&mut self) -> Annotated<&mut T::Target, &A>
    where
        T: DerefMut,
    {
        self.as_ref_mut().map(|v| v.deref_mut())
    }
}
impl<T, A> Annotated<&T, &A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(Clone::clone, Clone::clone)
    }
}
impl<T, A> Annotated<&mut T, &A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(|v| v.clone(), Clone::clone)
    }
}
impl<T, A> Annotated<&T, &mut A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(Clone::clone, |a| a.clone())
    }
}
impl<T, A> Annotated<&mut T, &mut A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(|v| v.clone(), |a| a.clone())
    }
}
impl<T, A> Annotated<&T, A> {
    pub fn value_cloned(self) -> Annotated<T, A>
    where
        T: Clone,
    {
        self.map(Clone::clone)
    }
}
impl<T, A> Annotated<&mut T, A> {
    pub fn value_cloned(self) -> Annotated<T, A>
    where
        T: Clone,
    {
        self.map(|v| v.clone())
    }
}
impl<T, A> Annotated<T, &A> {
    pub fn annotation_cloned(self) -> Annotated<T, A>
    where
        A: Clone,
    {
        self.map_annotation(Clone::clone)
    }
}
impl<T, A> Annotated<T, &mut A> {
    pub fn annotation_cloned(self) -> Annotated<T, A>
    where
        A: Clone,
    {
        self.map_annotation(|a| a.clone())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let one = Annotated::new(1, "first");
        let sone = one.map(|i| i.to_string());
        assert_eq!(
            sone.as_deref().annotation_cloned(),
            Annotated::new("1", "first")
        );
    }

    #[test]
    fn test_cloned() {
        let mut one = 1;
        let mut first = "first";
        let mut annotated_one = Annotated::new(1, "first");

        let one_value_ref = Annotated::new(&1, "first");
        assert_eq!(one_value_ref.value_cloned(), annotated_one);
        let one_value_ref_mut = Annotated::new(&mut one, "first");
        assert_eq!(one_value_ref_mut.value_cloned(), annotated_one);

        let one_annotation_ref = Annotated::new(1, &"first");
        assert_eq!(one_annotation_ref.annotation_cloned(), annotated_one);
        let one_annotation_ref_mut = Annotated::new(1, &mut first);
        assert_eq!(one_annotation_ref_mut.annotation_cloned(), annotated_one);

        let one_value_ref_annotation_ref = Annotated::new(&1, &"first");
        assert_eq!(one_value_ref_annotation_ref, annotated_one.as_ref());
        assert_eq!(one_value_ref_annotation_ref.cloned(), annotated_one);

        let one_value_ref_mut_annotation_ref = Annotated::new(&mut one, &first);
        assert_eq!(one_value_ref_mut_annotation_ref.cloned(), annotated_one);

        let one_value_ref_annotation_ref_mut = Annotated::new(&1, &mut first);
        assert_eq!(one_value_ref_annotation_ref_mut.cloned(), annotated_one);

        let one_value_ref_mut_annotation_ref_mut = Annotated::new(&mut one, &first);
        assert_eq!(
            one_value_ref_mut_annotation_ref_mut,
            annotated_one.as_ref_mut()
        );
        assert_eq!(one_value_ref_mut_annotation_ref_mut.cloned(), annotated_one);
    }
}

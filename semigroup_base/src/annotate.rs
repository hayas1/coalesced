use std::ops::{Deref, DerefMut};

use crate::semigroup::{AnnotatedSemigroup, Semigroup};

pub trait Annotate<A>: Sized {
    type Annotation;
    fn annotated(self, annotation: Self::Annotation) -> Annotated<Self, A>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Annotated<T, A> {
    value: T,
    annotation: A,
}
impl<T: AnnotatedSemigroup<A>, A> Semigroup for Annotated<T, A> {
    fn semigroup_op(base: Self, other: Self) -> Self {
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
    pub fn new(value: T, annotation: A) -> Self {
        Self { value, annotation }
    }
    pub fn into_parts(self) -> (T, A) {
        (self.value, self.annotation)
    }
    pub fn parts(&self) -> (&T, &A) {
        (&self.value, &self.annotation)
    }
    pub fn parts_mut(&mut self) -> (&mut T, &mut A) {
        (&mut self.value, &mut self.annotation)
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
    pub fn annotation_mut(&mut self) -> &mut A {
        &mut self.annotation
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
    pub fn map_parts<U, B>(self, f: impl FnOnce(T, A) -> (U, B)) -> Annotated<U, B> {
        let (value, annotation) = f(self.value, self.annotation);
        Annotated { value, annotation }
    }

    pub fn as_ref(&self) -> Annotated<&T, &A> {
        Annotated {
            value: &self.value,
            annotation: &self.annotation,
        }
    }
    pub fn as_ref_mut(&mut self) -> Annotated<&mut T, &mut A> {
        Annotated {
            value: &mut self.value,
            annotation: &mut self.annotation,
        }
    }
    pub fn as_deref(&self) -> Annotated<&T::Target, &A>
    where
        T: Deref,
    {
        self.as_ref().map(|v| v.deref())
    }
    pub fn as_deref_mut(&mut self) -> Annotated<&mut T::Target, &mut A>
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
        self.map_parts(|v, a| (v.clone(), a.clone()))
    }
}
impl<T, A> Annotated<&mut T, &A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(|v, a| (v.clone(), a.clone()))
    }
}
impl<T, A> Annotated<&T, &mut A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(|v, a| (v.clone(), a.clone()))
    }
}
impl<T, A> Annotated<&mut T, &mut A> {
    pub fn cloned(self) -> Annotated<T, A>
    where
        T: Clone,
        A: Clone,
    {
        self.map_parts(|v, a| (v.clone(), a.clone()))
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

        let one_value_ref_mut_annotation_ref_mut = Annotated::new(&mut one, &mut first);
        assert_eq!(
            one_value_ref_mut_annotation_ref_mut,
            annotated_one.as_ref_mut()
        );
        assert_eq!(one_value_ref_mut_annotation_ref_mut.cloned(), annotated_one);
    }
}

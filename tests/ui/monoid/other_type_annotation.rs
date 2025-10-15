use semigroup::{
    op::{coalesce::Coalesce, Construction},
    Annotate, Semigroup,
};

fn main() {
    let a = Coalesce(Some(1)).annotated("a");
    let b = Coalesce(Some(2)).annotated("b".to_string());

    let c = a.semigroup(b);
    assert_eq!(c.into_value().into_inner(), Some(2));
    assert_eq!(c.into_annotation(), "b".to_string());
}

use semigroup::{Annotate, Annotated, Semigroup};

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
pub struct NamedStruct {
    #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
    pub name: String,
    pub value: Option<u32>,
}

#[test]
fn test_named_struct_semigroup_op() {
    let a = NamedStruct {
        name: "A".to_string(),
        value: Some(10),
    }
    .annotated("First");

    let b = NamedStruct {
        name: "B".to_string(),
        value: None,
    }
    .annotated("Second");

    let ab = Semigroup::op(a.clone(), b.clone());
    assert_eq!(ab.value().name, "B");
    assert_eq!(ab.annotation().name, "Second");
    assert_eq!(ab.value().value, Some(10));
    assert_eq!(ab.annotation().value, "First");
    assert_eq!(
        ab,
        Annotated::new(
            NamedStruct {
                name: "B".to_string(),
                value: Some(10),
            },
            NamedStructAnnotation {
                name: "Second",
                value: "First",
            },
        ),
    );
    let ba = Semigroup::op(b.clone(), a.clone());
    assert_eq!(ba.value().name, "A");
    assert_eq!(ba.annotation().name, "First");
    assert_eq!(ba.value().value, Some(10));
    assert_eq!(ba.annotation().value, "First");
    assert_eq!(
        ba,
        Annotated::new(
            NamedStruct {
                name: "A".to_string(),
                value: Some(10),
            },
            NamedStructAnnotation {
                name: "First",
                value: "First",
            },
        ),
    );
}

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
pub struct UnnamedStruct(
    #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")] String,
    Option<u32>,
);

#[test]
fn test_unnamed_struct_semigroup_op() {
    let a = UnnamedStruct("A".to_string(), Some(10)).annotated(1.0);
    let b = UnnamedStruct("B".to_string(), None).annotated(2.0);

    let ab = Semigroup::op(a.clone(), b.clone());
    assert_eq!(ab.value().0, "B");
    assert_eq!(ab.annotation().0, 2.0);
    assert_eq!(ab.value().1, Some(10));
    assert_eq!(ab.annotation().1, 1.0);
    assert_eq!(
        ab,
        Annotated::new(
            UnnamedStruct("B".to_string(), Some(10)),
            UnnamedStructAnnotation(2.0, 1.0),
        ),
    );

    let ba = Semigroup::op(b.clone(), a.clone());
    assert_eq!(ba.value().0, "A");
    assert_eq!(ba.annotation().0, 1.0);
    assert_eq!(ba.value().1, Some(10));
    assert_eq!(ba.annotation().1, 1.0);
    assert_eq!(
        ba,
        Annotated::new(
            UnnamedStruct("A".to_string(), Some(10)),
            UnnamedStructAnnotation(1.0, 1.0),
        ),
    );
}

// #[derive(Debug, Clone, PartialEq, Semigroup)]
// pub struct UnitStruct;
// #[test]
// fn test_unit_struct_semigroup_op() {
//     let a = UnitStruct;
//     let b = UnitStruct;
//     assert_eq!(UnitStruct::semigroup_op(a.clone(), b.clone()), UnitStruct);
//     assert_eq!(UnitStruct::semigroup_op(b.clone(), a.clone()), UnitStruct);
// }

use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
pub struct NamedStruct<'a> {
    pub num: Option<u32>,
    #[semigroup(with = "semigroup::op::coalesce::Coalesce")]
    pub str: Option<&'a str>,
}

fn main() {
    let a = NamedStruct {
        num: Some(1),
        str: None,
    };
    let b = NamedStruct {
        num: None,
        str: Some("ten"),
    };

    let c = a.semigroup(b);

    assert_eq!(
        c,
        NamedStruct {
            num: Some(1),
            str: Some("ten"),
        }
    );
}

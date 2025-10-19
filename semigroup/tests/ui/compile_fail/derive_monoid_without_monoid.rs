use semigroup::Semigroup;

#[derive(Debug, Clone, PartialEq, Semigroup)]
#[semigroup(monoid, with = "semigroup::op::Coalesce")]
pub struct NamedStruct {
    pub num: Option<u32>,
    #[semigroup(with = "semigroup::op::Overwrite")]
    pub boolean: bool,
}

fn main() {}

use darling::FromMeta;

use crate::name::{var_name, Name};

#[derive(Debug, Clone, PartialEq, FromMeta)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,

    #[darling(default)]
    monoid: bool,

    #[darling(default)]
    commutative: bool,
}
impl ContainerAttr {
    pub fn validate(self) -> darling::Result<Self> {
        Ok(self)
    }
    pub fn attributes(&self) -> [&str; 3] {
        let Self {
            annotated,
            monoid,
            commutative,
        } = self;
        [
            var_name!(annotated),
            var_name!(monoid),
            var_name!(commutative),
        ]
        .map(|Name(name)| name)
    }
    pub fn properties(&self) -> [bool; 3] {
        [self.is_annotated(), self.is_monoid(), self.is_commutative()]
    }
    pub fn is_annotated(&self) -> bool {
        self.annotated
    }
    pub fn is_monoid(&self) -> bool {
        self.monoid
    }
    pub fn is_commutative(&self) -> bool {
        self.commutative
    }
}

use darling::FromMeta;

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

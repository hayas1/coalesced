use darling::FromMeta;

#[derive(Debug, Clone, PartialEq, FromMeta)]
#[darling(derive_syn_parse, and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,
    annotation_where: Option<String>, // TODO Vec

    #[darling(default)]
    monoid: bool,
    unit_where: Option<String>, // TODO Vec

    #[darling(default)]
    commutative: bool,
    commutative_where: Option<String>, // TODO Vec
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

    pub fn annotation_where(&self) -> Option<&str> {
        self.annotation_where.as_deref()
    }
    pub fn unit_where(&self) -> Option<&str> {
        self.unit_where.as_deref()
    }
    pub fn commutative_where(&self) -> Option<&str> {
        self.commutative_where.as_deref()
    }
}

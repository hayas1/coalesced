use syn::{parse_quote, Attribute, Path, TypeParam};

pub const DERIVE_CONSTRUCTION: &str = "Construction";
pub const DERIVE_SEMIGROUP: &str = "Semigroup";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub path_annotated_semigroup: Path,
    pub path_annotated: Path,
    pub path_annotate: Path,
    pub path_monoid: Path,
    pub path_commutative: Path,
    pub path_construction_trait: Path,
    pub path_construction_annotated: Path,
    pub path_construction_monoid: Path,
    pub default_type_param: TypeParam,
    pub attr_feature_monoid: Option<Attribute>,
    pub attr_feature_async: Option<Attribute>,
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum External {}
impl ConstantExt for External {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {::semigroup::Semigroup},
            path_annotated_semigroup: parse_quote! {::semigroup::AnnotatedSemigroup},
            path_annotated: parse_quote! {::semigroup::Annotated},
            path_annotate: parse_quote! {::semigroup::Annotate},
            path_monoid: parse_quote! {::semigroup::Monoid},
            path_commutative: parse_quote! {::semigroup::Commutative},
            path_construction_trait: parse_quote! {::semigroup::Construction},
            path_construction_annotated: parse_quote! {::semigroup::ConstructionAnnotated},
            path_construction_monoid: parse_quote! {::semigroup::ConstructionMonoid},
            default_type_param: parse_quote! { A },
            attr_feature_monoid: None,
            attr_feature_async: None,
        }
    }
}
pub enum Internal {}
impl ConstantExt for Internal {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {crate::Semigroup},
            path_annotated_semigroup: parse_quote! {crate::AnnotatedSemigroup},
            path_annotated: parse_quote! {crate::Annotated},
            path_annotate: parse_quote! {crate::Annotate},
            path_monoid: parse_quote! {crate::Monoid},
            path_commutative: parse_quote! {crate::Commutative},
            path_construction_trait: parse_quote! {crate::Construction},
            path_construction_annotated: parse_quote! {crate::ConstructionAnnotated},
            path_construction_monoid: parse_quote! {crate::ConstructionMonoid},
            default_type_param: parse_quote! { A },
            attr_feature_monoid: Some(parse_quote! {#[cfg(feature = "monoid")]}),
            attr_feature_async: Some(parse_quote! {#[cfg(feature = "async")]}),
        }
    }
}

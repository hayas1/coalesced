use syn::{parse_quote, Path, TypeParam};

pub const DERIVE_CONSTRUCTION: &str = "Construction";
pub const DERIVE_SEMIGROUP: &str = "Semigroup";

#[derive(Debug, Clone)]
pub struct Constant {
    pub path_semigroup: Path,
    pub path_annotated_semigroup: Path,
    pub path_annotated: Path,
    pub path_annotate: Path,
    pub path_commutative: Path,
    pub path_construction_trait: Path,
    pub path_construction_annotated: Path,
    pub default_type_param: TypeParam,
}
pub trait ConstantExt {
    fn constant() -> Constant;
}
pub enum Absolute {}
impl ConstantExt for Absolute {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {::semigroup::Semigroup},
            path_annotated_semigroup: parse_quote! {::semigroup::AnnotatedSemigroup},
            path_annotated: parse_quote! {::semigroup::Annotated},
            path_annotate: parse_quote! {::semigroup::Annotate},
            path_commutative: parse_quote! {::semigroup::Commutative},
            path_construction_trait: parse_quote! {::semigroup::op::Construction},
            path_construction_annotated: parse_quote! {::semigroup::op::ConstructionAnnotated},
            default_type_param: parse_quote! { A },
        }
    }
}
#[cfg(feature = "use_scope")]
pub enum Use {}
#[cfg(feature = "use_scope")]
impl ConstantExt for Use {
    fn constant() -> Constant {
        Constant {
            path_semigroup: parse_quote! {Semigroup},
            path_annotated_semigroup: parse_quote! {AnnotatedSemigroup},
            path_annotated: parse_quote! {Annotated},
            path_annotate: parse_quote! {Annotate},
            path_commutative: parse_quote! {Commutative},
            path_construction_trait: parse_quote! {Construction},
            path_construction_annotated: parse_quote! {ConstructionAnnotated},
            default_type_param: parse_quote! { A },
        }
    }
}

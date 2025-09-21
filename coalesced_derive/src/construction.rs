use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    construction::{ast::Construction, attr::ContainerAttr},
};

mod ast;
mod attr;

pub fn impl_construction<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let construction = Construction::new(&constant, derive, &attr)?;
    Ok(construction.into_token_stream())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::constant::{Absolute, Use};

    use super::*;

    #[rstest]
    #[case::construction_annotated(
        "construction_annotated",
        impl_construction::<Absolute>,
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(annotated, op_trait = CoalesceExt)]
            pub struct Coalesced<T>(pub Option<T>);
        },
    )]
    #[case::construction_not_annotated(
        "construction_not_annotated",
        impl_construction::<Use>,
        syn::parse_quote! {
            #[derive(ConstructionUse)]
            #[construction(op_trait = CoalesceExt)]
            pub struct Coalesced<T>(pub Option<T>);
        },
    )]
    #[case::construction_custom_annotation(
        "construction_custom_annotation",
        impl_construction::<Absolute>,
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(
                op_trait = CoalesceExt,
                annotated,
                annotation_type_param = "X: IntoIterator + FromIterator<X::Item>",
                annotation_where = "X::Item: Clone",
                unit = "vec![(); 0]"
            )]
            pub struct Concatenated<T: IntoIterator + FromIterator<T::Item>>(pub T);
        },
    )]
    fn test_derive_construction_snapshot(
        #[case] case: &str,
        #[case] f: impl Fn(&DeriveInput) -> syn::Result<TokenStream>,
        #[case] input: DeriveInput,
    ) {
        let generated = f(&input).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }
}

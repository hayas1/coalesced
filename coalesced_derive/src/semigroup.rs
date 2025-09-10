use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    constant::ConstantExt,
    semigroup::{ast::Semigroup, attr::ContainerAttr},
};

mod ast;
mod attr;

pub fn impl_semigroup<C: ConstantExt>(derive: &DeriveInput) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let attr = ContainerAttr::new(derive)?;
    let semigroup = Semigroup::new(&constant, derive, &attr)?;
    Ok(semigroup.into_token_stream())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::constant::{Absolute, Use};

    use super::*;

    #[rstest]
    #[case::semigroup_annotated(
        "semigroup_annotated",
        impl_semigroup::<Absolute>,
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated)]
            pub struct NamedStruct {
                #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: coalesced::op::annotation::replace::Replaced<bool>,
            }
        },
    )]
    #[case::semigroup_not_annotated(
        "semigroup_not_annotated",
        impl_semigroup::<Use>,
        syn::parse_quote! {
            #[derive(SemigroupUse)]
            #[semigroup(with = "coalesced::op::annotation::replace::Replaced")]
            pub struct UnnamedStruct<T: std::ops::Add> (
                #[semigroup(with = "coalesced::op::semigroup::add::Added")]
                T,
                u64
            );
        },
    )]
    #[case::semigroup_custom_annotation(
        "semigroup_custom_annotation",
        impl_semigroup::<Absolute>,
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated, annotation_param = X, with = "coalesced::op::annotation::replace::Replaced")]
            pub struct NamedStruct{
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: bool,
            }
        },
    )]
    fn test_derive_semigroup_snapshot(
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

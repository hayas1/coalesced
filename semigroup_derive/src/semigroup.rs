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

    use crate::constant::{External, Internal};

    use super::*;

    #[rstest]
    #[case::semigroup_annotated(
        "semigroup_annotated",
        impl_semigroup::<External>,
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated)]
            pub struct NamedStruct {
                #[semigroup(with = "semigroup::op::Overwrite")]
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: semigroup::op::Overwrite<bool>,
            }
        },
    )]
    #[case::semigroup_not_annotated(
        "semigroup_not_annotated",
        impl_semigroup::<Internal>,
        syn::parse_quote! {
            #[derive(SemigroupPriv)]
            #[semigroup(with = "semigroup::op::Overwrite")]
            pub struct UnnamedStruct<T: std::ops::Add> (
                #[semigroup(with = "semigroup::op::Added")]
                T,
                u64
            );
        },
    )]
    #[case::semigroup_custom_annotation(
        "semigroup_custom_annotation",
        impl_semigroup::<External>,
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated, annotation_param = X, with = "semigroup::op::Overwrite")]
            pub struct NamedStruct{
                pub foo: String,
                pub bar: Option<u32>,
                pub baz: bool,
            }
        },
    )]
    #[case::semigroup_monoid(
        "semigroup_monoid",
        impl_semigroup::<External>,
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(monoid, commutative)]
            pub struct MonoidStruct{
                #[semigroup(with = "semigroup::op::Sum")]
                pub sum: u32,
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

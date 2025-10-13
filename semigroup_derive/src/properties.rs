use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemStruct;

use crate::{
    constant::ConstantExt,
    properties::{attr::ContainerAttr, documented::Documented},
};

mod attr;
mod documented;

pub fn impl_properties<C: ConstantExt>(
    attr: &ContainerAttr,
    item: &ItemStruct,
) -> syn::Result<TokenStream> {
    let constant = C::constant();
    let documented = Documented::new(&constant, attr, item)?;
    Ok(documented.to_token_stream())
}

#[cfg(test)]
mod tests {
    // use rstest::rstest;

    // use crate::constant::{Absolute, Use};

    // use super::*;

    // #[rstest]
    // #[case::properties_annotated(
    //     "properties_annotated",
    //     impl_properties::<Absolute>,
    //     syn::parse_quote! {
    //         #[derive(Semigroup)]
    //         #[semigroup(annotated)]
    //         pub struct NamedStruct {
    //             #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
    //             pub foo: String,
    //             pub bar: Option<u32>,
    //             pub baz: semigroup::op::annotation::overwrite::Overwrite<bool>,
    //         }
    //     },
    // )]
    // #[case::properties_not_annotated(
    //     "properties_not_annotated",
    //     impl_properties::<Use>,
    //     syn::parse_quote! {
    //         #[derive(SemigroupUse)]
    //         #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
    //         pub struct UnnamedStruct<T: std::ops::Add> (
    //             #[semigroup(with = "semigroup::op::semigroup::add::Added")]
    //             T,
    //             u64
    //         );
    //     },
    // )]
    // #[case::properties_custom_annotation(
    //     "properties_custom_annotation",
    //     impl_properties::<Absolute>,
    //     syn::parse_quote! {
    //         #[derive(Semigroup)]
    //         #[semigroup(annotated, annotation_param = X, with = "semigroup::op::annotation::overwrite::Overwrite")]
    //         pub struct NamedStruct{
    //             pub foo: String,
    //             pub bar: Option<u32>,
    //             pub baz: bool,
    //         }
    //     },
    // )]
    // fn test_derive_properties_snapshot(
    //     #[case] case: &str,
    //     #[case] f: impl Fn(&ContainerAttr, &ItemStruct) -> syn::Result<TokenStream>,
    //     #[case] (attr, input): (ContainerAttr, ItemStruct),
    // ) {
    //     let generated = f(&attr, &input).unwrap();
    //     let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
    //     insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
    //         insta::assert_snapshot!(case, formatted);
    //     });
    // }
}

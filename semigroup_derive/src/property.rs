use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemStruct;

use crate::{
    constant::ConstantExt,
    property::{attr::ContainerAttr, documented::Documented},
};

mod attr;
mod documented;

pub fn impl_property<C: ConstantExt>(
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
    // #[case::property_annotated(
    //     "property_annotated",
    //     impl_property::<Absolute>,
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
    // #[case::property_not_annotated(
    //     "property_not_annotated",
    //     impl_property::<Use>,
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
    // #[case::property_custom_annotation(
    //     "property_custom_annotation",
    //     impl_property::<Absolute>,
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
    // fn test_derive_property_snapshot(
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

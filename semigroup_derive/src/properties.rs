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
    use rstest::rstest;

    use crate::constant::{External, Internal};

    use super::*;

    #[rstest]
    #[case::properties_annotated(
        "properties_annotated",
        impl_properties::<External>,
        (
            syn::parse_quote! {
                annotated, monoid
            },
            syn::parse_quote! {
                /// A semigroup construction that returns the first non-`None` value.
                /// # Properties
                /// <!-- properties -->
                #[derive(Construction)]
                #[construction(annotated, monoid)]
                pub struct Coalesce<T>(pub Option<T>);
            },
        ),
    )]
    #[case::properties_not_annotated(
        "properties_not_annotated",
        impl_properties::<Internal>,
        (
            syn::parse_quote! {

            },
            syn::parse_quote! {
                /// A semigroup struct that returns the sum and overwrite
                /// # Properties
                /// <!-- properties -->
                #[derive(SemigroupInternal)]
                #[semigroup(with = "semigroup::op::Overwrite")]
                pub struct UnnamedStruct<T: std::ops::Add> (
                    #[semigroup(with = "semigroup::op::Added")]
                    T,
                    u64
                );
            },
        )
    )]
    fn test_derive_properties_snapshot(
        #[case] case: &str,
        #[case] f: impl Fn(&ContainerAttr, &ItemStruct) -> syn::Result<TokenStream>,
        #[case] (attr, input): (ContainerAttr, ItemStruct),
    ) {
        let generated = f(&attr, &input).unwrap();
        let formatted = prettyplease::unparse(&syn::parse2(generated).unwrap());
        insta::with_settings!({ snapshot_path => "../tests/snapshots", prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(case, formatted);
        });
    }
}

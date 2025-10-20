mod annotation;
mod constant;
mod construction;
mod error;
mod name;
mod properties;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::impl_construction::<constant::External>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(ConstructionPriv, attributes(construction))]
pub fn derive_construction_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::impl_construction::<constant::Internal>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Semigroup, attributes(semigroup))]
pub fn derive_semigroup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::External>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(SemigroupInternal, attributes(semigroup))]
pub fn derive_semigroup_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::Internal>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn properties(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let meta = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::External>(&meta, &item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn properties_priv(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let meta = syn::parse_macro_input!(attr);
    let item_struct = syn::parse_macro_input!(item);
    properties::impl_properties::<constant::Internal>(&meta, &item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

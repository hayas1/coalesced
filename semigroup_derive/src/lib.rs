mod annotation;
mod constant;
mod construction;
mod error;
mod property;
mod semigroup;

#[proc_macro_derive(Construction, attributes(construction))]
pub fn derive_construction(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::impl_construction::<constant::Absolute>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "use_scope")]
#[proc_macro_derive(ConstructionUse, attributes(construction))]
pub fn derive_construction_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    construction::impl_construction::<constant::Use>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Semigroup, attributes(semigroup))]
pub fn derive_semigroup(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::Absolute>(&derive)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "use_scope")]
#[proc_macro_derive(SemigroupUse, attributes(semigroup))]
pub fn derive_semigroup_use(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive = syn::parse_macro_input!(input);
    semigroup::impl_semigroup::<constant::Use>(&derive)
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
    property::impl_property::<constant::Absolute>(&meta, &item_struct)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

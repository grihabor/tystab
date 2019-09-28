extern crate proc_macro;
use quote::{quote};
use proc_macro::{TokenStream};
use syn::parse::{ParseStream, Parse, Result};
use syn::{parse_macro_input, Token, Ident, Type};
use syn::punctuated::Punctuated;


struct Column (syn::Ident, syn::Type);

struct Table (Vec<Column>);

impl Parse for Column {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Column(name, ty))
    }
}

impl Parse for Table {
    fn parse(input: ParseStream) -> Result<Self> {
        let columns = Punctuated::<Column, Token![,]>::parse_terminated(input)?;
        Ok(Table(columns.into_iter().collect()))
    }
}

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Table(columns) = syn::parse_macro_input!(item as Table);
    let idents: Vec<Ident> = columns
        .iter()
        .map(|Column(ident, _)| ident.clone())
        .collect();
    let i = (0..columns.len()).map(syn::Index::from);
    let struct_def = quote! {
        struct Table<#( #idents[#i], )*>
    };
    let i = (0..columns.len()).map(syn::Index::from);
    let columns_def = quote!{{
        #( #idents[#i]: #idents[#i], )*
    }};
    let result = quote! {
        #struct_def
        #columns_def
    };
    result.into()
}

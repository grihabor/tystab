extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::{ Ident};

#[macro_use]
mod parse {

    use syn::{parse_macro_input, Ident, Type, Token};
    use syn::parse::{Parse, ParseStream, Result};
    use syn::punctuated::Punctuated;

    pub struct Column(pub syn::Ident, pub syn::Type);

    pub struct Table(pub Vec<Column>);

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
}

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parse::Table(columns) = syn::parse_macro_input!(item as parse::Table);
    let idents: Vec<Ident> = columns
        .iter()
        .map(|parse::Column(ident, _)| ident.clone())
        .collect();
    let i = (0..columns.len()).map(syn::Index::from);
    let struct_def = quote! {
        struct Table<#( #idents[#i], )*>
    };
    let i = (0..columns.len()).map(syn::Index::from);
    let columns_def = quote! {{
        #( #idents[#i]: #idents[#i], )*
    }};
    let result = quote! {
        #struct_def
        #columns_def
    };
    result.into()
}

use std::ops::Add;

pub struct Table0;
pub struct Table1<A>(Column<A>);
pub struct Table2<A, B>(Column<A>, Column<B>);
pub struct Table3<A, B, C>(Column<A>, Column<B>, Column<C>);

trait WithColumn<C, R> {
    fn column(self, _: Column<C>) -> R;
}

impl<A> WithColumn<A, Table1<A>> for Table0 {
    fn column(self, col: Column<A>) -> Table1<A> {
        Table1(col)
    }
}

impl<A, B> WithColumn<B, Table2<A, B>> for Table1<A> {
    fn column(self, col: Column<B>) -> Table2<A, B> {
        Table2(self.0, col)
    }
}

impl<A, B, C> WithColumn<C, Table3<A, B, C>> for Table2<A, B> {
    fn column(self, col: Column<C>) -> Table3<A, B, C> {
        Table3(self.0, self.1, col)
    }
}

pub type Table = Table0;

impl Table {
    fn new() -> Self {
        Table {}
    }
}

#[derive(Debug)]
pub struct Column<T>(T);

impl<T> From<T> for Column<T> {
    fn from(values: T) -> Self {
        Column(values)
    }
}

impl<T: Add<Output = T> + Copy> Add for &Column<Vec<T>> {
    type Output = Column<Vec<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        Column(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(a, b)| *a + *b)
                .collect(),
        )
    }
}


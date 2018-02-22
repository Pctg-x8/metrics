//! Common derive implementation for metrics coordinates

#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Coordinate2)]
pub fn c2_implementor(input: TokenStream) -> TokenStream
{
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let ref name = ast.ident;
    if let syn::Body::Struct(syn::VariantData::Tuple(ref fields)) = ast.body
    {
        assert!(fields.len() == 2);
        let ref ety = fields[0].ty;
        let gen = quote!{
            impl Coordinate2 for #name
            {
                type Element = #ety;
                fn x(&self) -> #ety { self.0 }
                fn y(&self) -> #ety { self.1 }
                fn new(x: #ety, y: #ety) -> Self { #name(x, y) }
            }
            impl Mul<#ety> for #name
            {
                type Output = Self;
                fn mul(self, other: #ety) -> Self { #name(self.0 * other, self.1 * other) }
            }
            impl Div<#ety> for #name
            {
                type Output = Self;
                fn div(self, other: #ety) -> Self { #name(self.0 / other, self.1 / other) }
            }
            impl<T: Coordinate2> Add<T> for #name where T::Element: ScalarConvertible<#ety>
            {
                type Output = Self;
                fn add(self, other: T) -> Self { #name(self.0 + other.x()._as(), self.1 + other.y()._as()) }
            }
            impl<T: Coordinate2> Sub<T> for #name where T::Element: ScalarConvertible<#ety>
            {
                type Output = Self;
                fn sub(self, other: T) -> Self { #name(self.0 - other.x()._as(), self.1 - other.y()._as()) }
            }
            impl #name { pub const ZERO: #name = #name(0 as #ety, 0 as #ety); }
        };
        gen.parse().unwrap()
    }
    else
    {
        panic!("The `Coordinate2` trait can only derive from tuple structs");
    }
}

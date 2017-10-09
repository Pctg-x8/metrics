/// Common derive implementation for metrics coordinates

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
                fn x(self) -> #ety { self.0 }
                fn y(self) -> #ety { self.1 }
                fn new(x: #ety, y: #ety) -> Self { #name(x, y) }
            }
        };
        gen.parse().unwrap()
    }
    else
    {
        panic!("The `Coordinate2` trait can only derive from tuple structs");
    }
}

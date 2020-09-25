#![allow(unused_imports)]

extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::ItemFn;

use lovm2_error::*;

#[proc_macro_attribute]
pub fn lovm2_builtin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree = syn::parse::<ItemFn>(item).unwrap();

    let mut ident = format!("{}", tree.sig.ident);
    let first_char = ident.remove(0);
    let struct_name = format_ident!("{}{}Builtin", first_char.to_uppercase().to_string(), ident);

    let block = tree.block;

    let result = quote! {
        #[derive(Debug)]
        struct #struct_name;

        impl #struct_name {
            pub fn instantiate() -> Rc<Self> {
                Rc::new(Self {})
            }
        }

        impl CallProtocol for #struct_name {
            fn run(&self, ctx: &mut Context) -> Lovm2Result<()> {
                #block
            }
        }
    };

    result.into()
}

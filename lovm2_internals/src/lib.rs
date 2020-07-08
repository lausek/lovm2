extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{ItemFn, ItemMod};

use std::collections::HashMap;

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
            fn run(&self, ctx: &mut Context) -> Result<(), String> {
                #block
            }
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn lovm2_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    /*
    let tree = syn::parse::<ItemMod>(item).unwrap();
    let items = tree.content
        .unwrap().1
        .into_iter()
        .map(|item| match item {
            syn::Item::Fn(item_fn) => item_fn,
            _ => panic!("anything except function not expected inside lovm2_module."),
        });

    let result = quote! {
        pub extern fn lovm2_module_slots(slots: &mut HashMap<Variable, CodeObjectRef>) {
            #(  slots.insert(#items.sig.ident); )*
        }
    };
    
    result.into()
    */
    item.into()
}

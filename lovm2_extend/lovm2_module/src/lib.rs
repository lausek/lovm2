extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, ItemMod, export::Span};

use lovm2::module::shared::EXTERN_LOVM2_INITIALIZER;

#[proc_macro_attribute]
pub fn lovm2_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree = syn::parse::<ItemMod>(item).unwrap();
    let items = tree.content
        .unwrap().1
        .into_iter()
        .map(|item| match item {
            syn::Item::Fn(item_fn) => item_fn,
            _ => panic!("anything except function not expected inside lovm2_module."),
        });
    let lovm2_initializer = Ident::new(EXTERN_LOVM2_INITIALIZER, Span::call_site());

    let result = quote! {
        pub extern fn #lovm2_initializer(lib: Rc<Library>, slots: &mut HashMap<Variable, CodeObjectRef>) {
            // #(  slots.insert(#items.sig.ident); )*
        }

        #( pub extern "C" #items )*
    };
    
    result.into()
}

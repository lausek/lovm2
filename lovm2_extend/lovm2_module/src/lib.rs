extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, ItemFn, ItemMod, export::Span};

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
        })
        .collect::<Vec<ItemFn>>();
    let names = items.iter().map(|item_fn| item_fn.sig.ident.to_string());
    let lovm2_initializer = Ident::new(EXTERN_LOVM2_INITIALIZER, Span::call_site());

    let result = quote! {
        pub extern fn #lovm2_initializer(lib: Rc<Library>, slots: &mut HashMap<Variable, CodeObjectRef>) {
            #(
                slots.insert(
                    Variable::from(#names),
                    Rc::new(SharedObjectSlot::new(lib.clone(), #names.to_string()))
                );
            )*
        }

        #( pub extern "C" #items )*
    };
    
    result.into()
}

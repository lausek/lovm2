#![feature(box_patterns)]

extern crate syn;
#[macro_use]
extern crate quote;

mod args;
mod func;
mod obj;
mod ret;

use lazy_static::lazy_static;
use proc_macro::{TokenStream, TokenTree};
use std::collections::HashSet;
use std::sync::Mutex;
use syn::{Block, Ident, ItemFn, ReturnType};

// TODO: move this somewhere else
const LV2_EXTERN_INITIALIZER: &str = "lovm2_module_initialize";

use self::args::*;
use self::func::*;
use self::obj::*;
use self::ret::*;

type GenResult<T> = Result<T, String>;

lazy_static! {
    static ref FUNCS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

/// Generates the module initializer (always required).
#[proc_macro]
pub fn lv2_module_init(_args: TokenStream) -> TokenStream {
    let initfn = Ident::new(LV2_EXTERN_INITIALIZER, proc_macro2::Span::call_site());
    let funcs = FUNCS.lock().unwrap();
    let names = funcs.iter();

    let result = quote! {
        #[doc(hidden)]
        #[no_mangle]
        pub extern fn #initfn(lib: std::rc::Rc<Library>, slots: &mut std::collections::HashMap<lovm2_core::LV2Variable, lovm2_core::code::LV2CallableRef>) {
            #(
                let slot = lovm2_core::module::LV2SharedObjectSlot::new(lib.clone(), #names).expect("name not found");
                slots.insert(
                    lovm2_core::LV2Variable::from(#names),
                    std::rc::Rc::new(slot)
                );
            )*
        }
    };

    result.into()
}

/// Makes the function available inside the module.
#[proc_macro_attribute]
pub fn lv2_function(attrs: TokenStream, item: TokenStream) -> TokenStream {
    use crate::quote::ToTokens;

    let is_extern = match attrs.into_iter().next() {
        Some(TokenTree::Ident(ident)) => ident.to_string() == "extern",
        _ => false,
    };

    let item_fn = syn::parse::<syn::ItemFn>(item).unwrap();
    let mut function = Function::from(item_fn).unwrap();

    if is_extern {
        function.set_extern(true);
        FUNCS.lock().unwrap().insert(function.name.to_string());
    }

    function.generate_rust_function().into_token_stream().into()
}

/// Makes the structure available inside the module.
#[proc_macro_attribute]
pub fn lv2_object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use crate::quote::ToTokens;

    let obj = Object::from(item);

    obj.generate_rust_structure().into_token_stream().into()
}

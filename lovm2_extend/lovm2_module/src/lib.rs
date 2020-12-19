#![feature(box_patterns)]

extern crate syn;
#[macro_use]
extern crate quote;

mod args;
mod func;
mod ret;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::collections::HashSet;
use std::sync::Mutex;
use syn::{export::Span, Block, Ident, ItemFn, ReturnType, Type};

use lovm2::module::EXTERN_LOVM2_INITIALIZER;

use self::args::*;
use self::func::*;
use self::ret::*;

type GenResult<T> = Result<T, String>;

lazy_static! {
    static ref FUNCS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[proc_macro]
pub fn lovm2_module_init(_: TokenStream) -> TokenStream {
    let initfn = Ident::new(EXTERN_LOVM2_INITIALIZER, Span::call_site());
    let funcs = FUNCS.lock().unwrap();
    let names = funcs.iter();

    let result = quote! {
        #[no_mangle]
        pub extern fn #initfn(lib: Rc<Library>, slots: &mut HashMap<Variable, CallableRef>) {
            #(
                let slot = SharedObjectSlot::new(lib.clone(), #names).expect("name not found");
                slots.insert(
                    Variable::from(#names),
                    Rc::new(slot)
                );
            )*
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn lovm2_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use crate::quote::ToTokens;

    let item_fn = syn::parse::<syn::ItemFn>(item).unwrap();
    let function = Function::from(item_fn).unwrap();

    FUNCS.lock().unwrap().insert(function.name.to_string());

    function.generate_rust_function().into_token_stream().into()
}

#[proc_macro_attribute]
pub fn lovm2_object(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = &syn::parse::<syn::ItemStruct>(item).unwrap();
    let name = &item_fn.ident;
    let result = quote! {
        #item_fn

        impl Into<Value> for #name {
            fn into(self) -> Value {
                Value::create_any(self)
            }
        }
    };
    result.into()
}

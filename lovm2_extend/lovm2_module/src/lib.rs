extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{export::Span, Ident, ItemMod};

use lovm2::module::shared::EXTERN_LOVM2_INITIALIZER;

// TODO: wrapped functions either return:
// () -> no errors expected, Lovm2Result == Ok(_)
// Value -> no errors expected, push Value on stack
// Option<Value> -> no errors expected, push Value or Nil on stack
// Lovm2CResult -> convert errors to Lovm2Result or push Value on stack
// Option<Lovm2CResult> -> convert errors to Lovm2Result or push Nil
// 
// + check if the expected arguments in `argn` have been taken from stack

#[proc_macro_attribute]
pub fn lovm2_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree = syn::parse::<ItemMod>(item).unwrap();

    let (mut idents, mut blocks) = (vec![], vec![]);

    for func in tree.content.unwrap().1.into_iter() {
        match func {
            syn::Item::Fn(item_fn) => {
                idents.push(item_fn.sig.ident);
                blocks.push(item_fn.block);
            }
            _ => panic!("anything except function not expected inside lovm2_module."),
        }
    }

    let names = idents
        .iter()
        .map(|ident| ident.to_string())
        .collect::<Vec<String>>();

    let lovm2_initializer = Ident::new(EXTERN_LOVM2_INITIALIZER, Span::call_site());

    let result = quote! {
        #[no_mangle]
        pub extern fn #lovm2_initializer(lib: Rc<Library>, slots: &mut HashMap<Variable, CodeObjectRef>) {
            #(
                slots.insert(
                    Variable::from(#names),
                    Rc::new(SharedObjectSlot::new(lib.clone(), #names.to_string()))
                );
            )*
        }

        #(
            #[no_mangle]
            pub extern fn #idents(ctx: &mut Context) -> lovm2_extend::prelude::Lovm2Result<()> {
                let result: Option<Lovm2CError> = { #blocks };

                match result {
                    Some(Lovm2CError { ty }) => Err(match ty {
                        lovm2_extend::prelude::BASIC => lovm2_extend::prelude::Lovm2ErrorTy::Basic,
                        lovm2_extend::prelude::FRAME_STACK_EMPTY => lovm2_extend::prelude::Lovm2ErrorTy::FrameStackEmpty,
                        lovm2_extend::prelude::IMPORT_CONFLICT => lovm2_extend::prelude::Lovm2ErrorTy::ImportConflict,
                        lovm2_extend::prelude::KEY_NOT_FOUND => lovm2_extend::prelude::Lovm2ErrorTy::KeyNotFound,
                        lovm2_extend::prelude::LOOKUP_FAILED => lovm2_extend::prelude::Lovm2ErrorTy::LookupFailed,
                        lovm2_extend::prelude::MODULE_NOT_FOUND => lovm2_extend::prelude::Lovm2ErrorTy::ModuleNotFound,
                        lovm2_extend::prelude::OPERATION_NOT_SUPPORTED => lovm2_extend::prelude::Lovm2ErrorTy::OperationNotSupported,
                        lovm2_extend::prelude::VALUE_STACK_EMPTY => lovm2_extend::prelude::Lovm2ErrorTy::ValueStackEmpty,
                        n => lovm2_extend::prelude::Lovm2ErrorTy::Custom(n.to_string()),
                    }.into()),
                    None => Ok(()),
                }
            }
        )*
    };

    result.into()
}

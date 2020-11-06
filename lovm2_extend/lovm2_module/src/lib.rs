#![feature(box_patterns)]

extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{export::Span, Ident, ItemFn, ItemMod};

use lovm2::module::shared::EXTERN_LOVM2_INITIALIZER;

// TODO: wrapped functions either return:
// () -> no errors expected, Lovm2Result == Ok(_)
// Lovm2CResult -> convert errors to Lovm2Result or push Value on stack
// Value -> no errors expected, push Value on stack
// Option<Lovm2CResult> -> convert errors to Lovm2Result or push Nil
// Option<Value> -> no errors expected, push Value or Nil on stack
// 
// + check if the expected arguments in `argn` have been taken from stack

fn accept_type(ty: &syn::Type) -> &syn::Type {
    match ty {
        syn::Type::Path(ty_path) => if let Some(ident) = ty_path.path.get_ident() {
            let ident_name = ident.to_string();
            match ident_name.as_ref() {
                "bool" | "f64" | "i64" | "String" | "Context" => ty,
                _ => panic!("unexpected type"),
            }
        } else {
            panic!("unexpected type")
        }
        syn::Type::Reference(ref_type) => {
            assert!(ref_type.mutability.is_some());
            accept_type(&ref_type.elem);
            ty
        }
        _ => panic!("unexpected type"),
    }
}

fn generate_prelude(item_fn: &ItemFn) -> impl quote::ToTokens {
    let mut names: Vec<syn::Ident> = vec![];
    let mut tys: Vec<syn::Type> = vec![];
    let mut ctx: Vec<syn::Ident> = vec![];

    let it = item_fn.sig.inputs.iter();
    for item in it.rev() {
        match item {
            syn::FnArg::Typed(syn::PatType { box pat, box ty, ..}) => {
                let ty = accept_type(ty);

                let name = if let syn::Pat::Ident(pat_ident) = pat {
                    &pat_ident.ident
                } else {
                    panic!("identifier needed")
                };

                if let syn::Type::Reference(_) = ty {
                    assert!(ctx.is_empty());
                    ctx.push(name.clone());
                } else {
                    names.push(name.clone());
                    tys.push(ty.clone());
                }
            }
            _ => panic!("unexpected argument type"),
        }
    }

    quote! {
        #(
            let #names: #tys = ctx.pop_value()?.into();
        )*
        #(
            let #ctx: &mut Context = ctx;
        )*
    }
}

fn generate_postlude(item_fn: &ItemFn) -> impl quote::ToTokens {
    match &item_fn.sig.output {
        syn::ReturnType::Default => quote! {
            ctx.push_value(Value::Nil);
            Ok(())
        },
        syn::ReturnType::Type(_, box ret) => {
            accept_type(&ret);
            let ident = format_ident!("_lv2_return_value");
            quote! {
                ctx.push_value(Value::from(#ident));
                Ok(())
            }
        }
    }
}

fn generate_body(item_fn: &ItemFn) -> impl quote::ToTokens {
    let prelude = generate_prelude(&item_fn);
    let postlude = generate_postlude(&item_fn);
    let body = &item_fn.block;

    quote! {
        let _lv2_return_value = {
            #prelude
            { #body }
        };
        #postlude
    }
}

#[proc_macro_attribute]
pub fn lovm2_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tree = syn::parse::<ItemMod>(item).unwrap();

    let (mut idents, mut bodies) = (vec![], vec![]);

    for func in tree.content.unwrap().1.into_iter() {
        match func {
            syn::Item::Fn(item_fn) => {
                let body = generate_body(&item_fn);
                
                idents.push(item_fn.sig.ident);
                bodies.push(body);
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
                #bodies
                /*
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
                */
            }
        )*
    };

    result.into()
}

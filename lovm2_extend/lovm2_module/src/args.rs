use super::*;
use syn::{punctuated::Punctuated, token::Comma, FnArg};

pub struct FunctionArgs {
    vm: Option<Ident>,
    simple: Vec<FunctionArg>,
}

impl FunctionArgs {
    pub fn from(inputs: Punctuated<FnArg, Comma>) -> GenResult<Self> {
        let mut vm = None;
        let mut simple = vec![];

        for item in inputs.into_iter() {
            match item {
                syn::FnArg::Typed(syn::PatType {
                    box pat, box ty, ..
                }) => {
                    let ty = crate::func::accept_type(&ty)?;

                    let name = if let syn::Pat::Ident(pat_ident) = pat {
                        pat_ident.ident.clone()
                    } else {
                        return Err(format!("pattern {:?} not allowed", 2));
                    };

                    match ty {
                        syn::Type::Reference(syn::TypeReference {
                            box elem,
                            mutability,
                            ..
                        }) => match elem {
                            syn::Type::Path(tp) => {
                                let ty_name = tp.path.get_ident().unwrap();
                                if "Vm" == ty_name.to_string() {
                                    if vm.is_some() {
                                        return Err(format!("vm reference declared twice."));
                                    }
                                    vm = Some(name);
                                } else {
                                    simple.push(FunctionArg {
                                        name,
                                        ty_name: ty_name.clone(),
                                        is_ref: true,
                                        is_mut: mutability.is_some(),
                                    })
                                }
                            }
                            _ => {}
                        },
                        syn::Type::Path(tp) => {
                            let ty_name = tp.path.get_ident().unwrap().clone();
                            simple.push(FunctionArg::new(name, ty_name));
                        }
                        _ => return Err(format!("this type is not allowed in argument position")),
                    }
                }
                _ => return Err(format!("{:?} not allowed as argument", 1)),
            }
        }
        Ok(Self { vm, simple })
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        let mut stackops = vec![];

        // call convention requires reverse popping
        for arg in self.simple.iter().rev() {
            let FunctionArg {
                name,
                ty_name,
                is_ref,
                is_mut,
            } = arg;

            let code = if *is_ref {
                quote! {
                    let #name = vm.ctx.pop_value()?.as_any_ref()?;
                    let mut #name = (*#name).borrow_mut();
                    let #name = (*#name).0.downcast_mut::<#ty_name>()
                                .ok_or_else(|| (Lovm2ErrorTy::OperationNotSupported, "downcast"))?;
                }
            } else {
                quote! { let #name: #ty_name = vm.ctx.pop_value()?.into(); }
            };
            stackops.push(code);
        }

        let vm = if let Some(name) = &self.vm {
            quote! { let #name: &mut Vm = vm; }
        } else {
            quote! { let _: &mut Vm = vm; }
        };

        quote! {
            use std::borrow::BorrowMut;
            #( #stackops )*
            #vm
        }
    }
}

struct FunctionArg {
    name: Ident,
    ty_name: Ident,
    is_ref: bool,
    is_mut: bool,
}

impl FunctionArg {
    pub fn new(name: Ident, ty_name: Ident) -> Self {
        Self {
            name,
            ty_name,
            is_ref: false,
            is_mut: false,
        }
    }
}

use syn::{punctuated::Punctuated, token::Comma, FnArg};

use super::*;

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
                    #[allow(unused_assignments)]
                    let mut is_mut = false;

                    let name = if let syn::Pat::Ident(pat_ident) = pat {
                        is_mut = pat_ident.mutability.is_some();
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
                            simple.push(FunctionArg {
                                name,
                                ty_name,
                                is_ref: false,
                                is_mut,
                            });
                        }
                        _ => return Err(format!("this type is not allowed in argument position")),
                    }
                }
                _ => return Err(format!("{:?} not allowed as argument", 1)),
            }
        }
        Ok(Self { vm, simple })
    }

    pub fn as_tokens(&self) -> impl quote::ToTokens {
        use crate::quote::ToTokens;
        let mut parts = vec![];

        if let Some(vm) = &self.vm {
            parts.push(quote! { #vm: &mut Vm });
        }

        for arg in self.simple.iter() {
            parts.push(arg.as_tokens().to_token_stream());
        }

        quote! { #( #parts, )* }
    }

    pub fn as_tokens_call_position(&self) -> impl quote::ToTokens {
        let mut parts = vec![];

        if let Some(vm) = &self.vm {
            parts.push(vm);
        }

        for arg in self.simple.iter() {
            parts.push(&arg.name);
        }

        quote! { #( #parts, )* }
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
                // if a immutable reference was requested, drop mutability
                let downcast_method = if *is_mut {
                    quote! { downcast_mut }
                } else {
                    quote! { downcast_ref }
                };

                quote! {
                    let #name = vm.context_mut().pop_value()?.as_any_ref()?;
                    let mut #name = (*#name).borrow_mut();
                    let #name = (*#name).0.#downcast_method::<#ty_name>()
                                .ok_or_else(|| (Lovm2ErrorTy::OperationNotSupported, "downcast"))?;
                }
            } else {
                let mutability = if *is_mut {
                    quote! { mut }
                } else {
                    quote! {}
                };
                quote! { let #mutability #name: #ty_name = vm.context_mut().pop_value()?.into(); }
            };
            stackops.push(code);
        }

        let vm = if let Some(name) = &self.vm {
            quote! { let #name: &mut Vm = vm; }
        } else {
            quote! {}
        };

        quote! {
            use std::borrow::BorrowMut;
            #( #stackops )*
            #vm
        }
    }
}

impl std::fmt::Display for FunctionArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut parts = vec![];

        if let Some(vm) = &self.vm {
            parts.push(format!("{}: &mut Vm", vm));
        }

        for arg in self.simple.iter() {
            parts.push(format!("{}", arg));
        }

        write!(f, "{}", parts.join(", "))
    }
}

struct FunctionArg {
    pub(self) name: Ident,
    ty_name: Ident,
    is_ref: bool,
    is_mut: bool,
}

impl FunctionArg {
    pub fn as_tokens(&self) -> impl quote::ToTokens {
        let (name, ty) = (&self.name, &self.ty_name);

        if self.is_mut && self.is_ref {
            quote! { #name: &mut #ty }
        } else if self.is_ref {
            quote! { #name: &#ty }
        } else if self.is_mut {
            quote! { mut #name: #ty }
        } else {
            quote! { #name: #ty }
        }
    }
}

impl std::fmt::Display for FunctionArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::quote::ToTokens;
        write!(f, "{}", self.as_tokens().to_token_stream())
    }
}

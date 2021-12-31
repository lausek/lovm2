use syn::{punctuated::Punctuated, token::Comma, FnArg};

use super::*;

pub struct FunctionArgs {
    vm: Option<Ident>,
    pub(crate) simple: Vec<FunctionArg>,
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

                                if stringify!(LV2Vm) == ty_name.to_string() {
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
            parts.push(quote! { #vm: &mut LV2Vm });
        }

        for arg in self.simple.iter() {
            parts.push(arg.as_tokens().to_token_stream());
        }

        quote! { #( #parts, )* }
    }

    pub fn as_tokens_call_position(&self, pass_as_reference: bool) -> impl quote::ToTokens {
        use crate::quote::ToTokens;

        let mut parts = vec![];

        if let Some(vm) = &self.vm {
            parts.push(quote! { #vm });
        }

        for arg in self.simple.iter() {
            parts.push(
                arg.as_tokens_call_position(pass_as_reference)
                    .to_token_stream(),
            );
        }

        quote! { #( #parts, )* }
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        let mut stackops = vec![];

        // call convention requires reverse popping
        for arg in self.simple.iter().rev() {
            let is_custom_ty = arg.is_custom_ty();
            let FunctionArg {
                name,
                ty_name,
                is_mut,
                ..
            } = arg;

            let code = if is_custom_ty {
                // if an immutable reference was requested, drop mutability
                let downcast_method = if *is_mut {
                    quote! { downcast_mut }
                } else {
                    quote! { downcast_ref }
                };

                quote! {
                    let #name = vm.context_mut().pop_value()?.as_any_inner()?;
                    let mut #name = (*#name).borrow_mut();
                    let #name = (*#name).0.#downcast_method::<#ty_name>()
                                .ok_or_else(|| (LV2ErrorTy::OperationNotSupported, "downcast"))?;
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
            quote! { let #name: &mut LV2Vm = vm; }
        } else {
            quote! {}
        };

        quote! {
            use std::borrow::BorrowMut;
            #( #stackops )*
            #vm
        }
    }

    pub fn generate_call(&self) -> impl quote::ToTokens {
        let argscall = self.as_tokens_call_position(false);

        match self.simple.first() {
            Some(first) if !first.is_custom_ty() && first.is_ref => {
                let first_name = &first.name;
                let argscall_first_normalized = self.as_tokens_call_position(true);

                let mut_kw = if first.is_mut {
                    quote! { mut }
                } else {
                    quote! {}
                };

                let borrow = if first.is_mut {
                    quote! { borrow_mut }
                } else {
                    quote! {borrow}
                };

                let deref = if first.is_mut {
                    quote! { deref_mut }
                } else {
                    quote! {deref}
                };

                quote! {
                    if let lovm2_extend::prelude::LV2Value::Ref(r) = #first_name {
                        use std::ops::{Deref, DerefMut};
                        let #first_name = r.unref_to_value()?;
                        let #mut_kw #first_name = (*#first_name).#borrow();
                        let #first_name = #first_name.#deref();
                        _lv2_wrapper(#argscall)
                    } else {
                        _lv2_wrapper(#argscall_first_normalized)
                    }
                }
            }
            _ => quote! { _lv2_wrapper(#argscall) },
        }
    }
}

impl std::fmt::Display for FunctionArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut parts = vec![];

        if let Some(vm) = &self.vm {
            parts.push(format!("{}: &mut {}", vm, stringify!(LV2Vm)));
        }

        for arg in self.simple.iter() {
            parts.push(format!("{}", arg));
        }

        write!(f, "{}", parts.join(", "))
    }
}

pub struct FunctionArg {
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

    pub fn as_tokens_call_position(&self, pass_as_reference: bool) -> impl quote::ToTokens {
        let name = &self.name;

        match (self.is_mut, self.is_ref) {
            (true, true) if pass_as_reference => quote! { &mut #name },
            (_, true) if pass_as_reference => quote! { &#name },
            _ => quote! { #name },
        }
    }

    pub fn is_custom_ty(&self) -> bool {
        match self.ty_name.to_string().as_str() {
            stringify!(LV2Value)
            | stringify!(bool)
            | stringify!(i64)
            | stringify!(f64)
            | stringify!(String) => false,
            _ => true,
        }
    }
}

impl std::fmt::Display for FunctionArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_mut && self.is_ref {
            write!(f, "{}: &mut {}", self.name, self.ty_name)
        } else if self.is_ref {
            write!(f, "{}: &{}", self.name, self.ty_name)
        } else if self.is_mut {
            write!(f, "mut {}: {}", self.name, self.ty_name)
        } else {
            write!(f, "{}: {}", self.name, self.ty_name)
        }
    }
}

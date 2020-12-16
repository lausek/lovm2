use super::*;
use syn::{punctuated::Punctuated, token::Comma, FnArg};

struct FunctionArg {
    name: Ident,
    ty: Type,
}

pub struct FunctionArgs {
    vm: Option<Ident>,
    simple: Vec<FunctionArg>,
    // TODO: handles
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

                    if let syn::Type::Reference(_) = ty {
                        if vm.is_some() {
                            return Err(format!("vm reference declared twice."));
                        }
                        vm = Some(name);
                    } else {
                        simple.push(FunctionArg {
                            name,
                            ty: ty.clone(),
                        });
                    }
                }
                _ => return Err(format!("{:?} not allowed as argument", 1)),
            }
        }
        Ok(Self { vm, simple })
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        // call convention requires reverse popping
        let names = self.simple.iter().map(|a| &a.name).rev();
        let tys = self.simple.iter().map(|a| &a.ty).rev();

        let vm = if let Some(name) = &self.vm {
            quote! { let #name: &mut Vm = vm; }
        } else {
            quote! { let _: &mut Vm = vm; }
        };

        quote! {
            #(
                let #names: #tys = vm.ctx.pop_value()?.into();
            )*
            #vm
        }
    }
}

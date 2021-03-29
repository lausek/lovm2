use super::*;

// TODO: wrapped functions either return:
// () -> no errors expected, Lovm2Result == Ok(_)
// Lovm2CResult -> convert errors to Lovm2Result or push Value on stack
// Value -> no errors expected, push Value on stack
// Option<Lovm2CResult> -> convert errors to Lovm2Result or push Nil
// Option<Value> -> no errors expected, push Value or Nil on stack
//
// + check if the expected arguments in `argn` have been taken from stack

#[derive(Debug)]
pub enum FunctionRet {
    None,
    Ident(Ident),
    Maybe(Box<FunctionRet>),
    Result(Box<FunctionRet>),
}

impl FunctionRet {
    pub fn from(ret: ReturnType) -> GenResult<Self> {
        let mut ty = Self::None;

        match &ret {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, box ret) => {
                ty = accept_type(&ret)?;
            }
        }

        Ok(ty)
    }

    pub fn as_tokens(&self) -> impl quote::ToTokens {
        match &self {
            Self::None => quote! { () },
            Self::Ident(name) => quote! { #name },
            Self::Maybe(ty) => {
                let ty = ty.as_tokens();

                quote! { Option<#ty> }
            }
            Self::Result(ty) => {
                let ty = ty.as_tokens();

                quote! { Lovm2Result<#ty> }
            }
        }
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        match self {
            Self::Ident(_) | Self::Result(_) => {
                let ident = format_ident!("_lv2_return_value");

                let raise_error = if matches!(self, Self::Result(_)) {
                    quote! { let #ident = #ident?; }
                } else {
                    quote! {}
                };

                quote! {
                    #raise_error
                    let val: Value = #ident.into();
                    vm.context_mut().push_value(val);
                    Ok(())
                }
            }
            Self::Maybe(_) => {
                let ident = format_ident!("_lv2_return_value");

                quote! {
                    if let Some(val) = #ident {
                        let val: Value = val.into();
                        vm.context_mut().push_value(val);
                    } else {
                        vm.context_mut().push_value(Value::Nil);
                    }
                    Ok(())
                }
            }
            Self::None => {
                quote! {
                    vm.context_mut().push_value(Value::Nil);
                    Ok(())
                }
            }
        }
    }
}

impl std::fmt::Display for FunctionRet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "()"),
            Self::Ident(name) => write!(f, "{}", name),
            Self::Maybe(ty) => write!(f, "Option<{}>", *ty),
            Self::Result(ty) => write!(f, "Lovm2Result<{}>", *ty),
        }
    }
}

pub(crate) fn accept_type(ty: &syn::Type) -> GenResult<FunctionRet> {
    use syn::{AngleBracketedGenericArguments, GenericArgument, PathArguments};

    match ty {
        syn::Type::Path(ty_path) => {
            if let Some(segment) = ty_path.path.segments.first() {
                let ty_arg =
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &segment.arguments
                    {
                        match args.first() {
                            Some(GenericArgument::Type(ty)) => Some(accept_type(&ty)?),
                            _ => None,
                        }
                    } else {
                        None
                    };

                let rt = match segment.ident.to_string().as_ref() {
                    "Option" => FunctionRet::Maybe(Box::new(ty_arg.unwrap())),
                    "Lovm2Result" => FunctionRet::Result(Box::new(ty_arg.unwrap())),
                    _ => FunctionRet::Ident(segment.ident.clone()),
                };

                return Ok(rt);
            }
        }
        syn::Type::Tuple(tuple) if tuple.elems.is_empty() => {
            return Ok(FunctionRet::None);
        }
        _ => {}
    }

    Err(format!("unexpected type {:?}", ty))
}

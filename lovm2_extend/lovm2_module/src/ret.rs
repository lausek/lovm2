use super::*;

// TODO: wrapped functions either return:
// () -> no errors expected, Lovm2Result == Ok(_)
// Lovm2CResult -> convert errors to Lovm2Result or push Value on stack
// Value -> no errors expected, push Value on stack
// Option<Lovm2CResult> -> convert errors to Lovm2Result or push Nil
// Option<Value> -> no errors expected, push Value or Nil on stack
//
// + check if the expected arguments in `argn` have been taken from stack

pub struct FunctionRet {
    ty: RetType,
}

impl FunctionRet {
    pub fn from(ret: ReturnType) -> GenResult<Self> {
        let mut ty = RetType::None;

        match &ret {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, box ret) => {
                ty = accept_type(&ret)?;
            }
        }

        Ok(Self { ty })
    }

    pub fn as_tokens(&self) -> impl quote::ToTokens {
        match &self.ty {
            RetType::None => quote! { () },
            RetType::Ident(ty) | RetType::Maybe(ty) | RetType::Result(ty) => quote! { #ty },
        }
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        match self.ty {
            RetType::Ident(_) | RetType::Result(_) => {
                let ident = format_ident!("_lv2_return_value");

                let raise_error = if let RetType::Result(_) = self.ty {
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
            RetType::Maybe(_) => {
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
            RetType::None => {
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
        use crate::quote::ToTokens;
        write!(f, "{}", self.as_tokens().to_token_stream())
    }
}

pub(crate) fn accept_type(ty: &syn::Type) -> GenResult<RetType> {
    match ty {
        syn::Type::Path(ty_path) => {
            if let Some(segment) = ty_path.path.segments.first() {
                let ty = ty.clone();
                let rt = match segment.ident.to_string().as_ref() {
                    "Option" => RetType::Maybe(ty),
                    "Lovm2Result" => RetType::Result(ty),
                    _ => RetType::Ident(ty),
                };
                return Ok(rt);
            }
        }
        _ => {}
    }
    Err(format!("unexpected type {:?}", ty))
}

pub enum RetType {
    None,
    Ident(Type),
    Maybe(Type),
    Result(Type),
}

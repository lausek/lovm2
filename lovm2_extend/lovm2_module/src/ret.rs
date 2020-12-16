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

    pub fn generate(&self) -> impl quote::ToTokens {
        match self.ty {
            RetType::Ident(_) => {
                let ident = format_ident!("_lv2_return_value");
                quote! {
                    let val: Value = #ident.into();
                    vm.ctx.push_value(val);
                    Ok(())
                }
            }
            RetType::Maybe(_) => {
                let ident = format_ident!("_lv2_return_value");
                quote! {
                    if let Some(val) = #ident {
                        let val: Value = val.into();
                        vm.ctx.push_value(val);
                    } else {
                        vm.ctx.push_value(Value::Nil);
                    }
                    Ok(())
                }
            }
            RetType::None => {
                quote! {
                    vm.ctx.push_value(Value::Nil);
                    Ok(())
                }
            }
        }
    }
}

pub(crate) fn accept_type(ty: &syn::Type) -> GenResult<RetType> {
    match ty {
        syn::Type::Path(ty_path) => {
            if let Some(segment) = ty_path.path.segments.first() {
                let rt = if "Option" == segment.ident.to_string() {
                    RetType::Maybe(ty.clone())
                } else {
                    RetType::Ident(ty.clone())
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
}

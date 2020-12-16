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
    ty: Option<Type>,
}

impl FunctionRet {
    pub fn from(ret: ReturnType) -> GenResult<Self> {
        let mut ty = None;

        match &ret {
            syn::ReturnType::Default => {}
            syn::ReturnType::Type(_, box ret) => {
                ty = Some(crate::func::accept_type(&ret)?.clone());
            }
        }

        Ok(Self { ty })
    }

    pub fn generate(&self) -> impl quote::ToTokens {
        if let Some(_ty) = &self.ty {
            let ident = format_ident!("_lv2_return_value");
            quote! {
                vm.ctx.push_value(Value::from(#ident));
                Ok(())
            }
        } else {
            quote! {
                vm.ctx.push_value(Value::Nil);
                Ok(())
            }
        }
    }
}

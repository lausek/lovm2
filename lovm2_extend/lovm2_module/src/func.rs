use super::*;

pub struct Function {
    pub(crate) name: Ident,
    args: FunctionArgs,
    block: Box<Block>,
    output: FunctionRet,
}

impl Function {
    pub fn from(item_fn: ItemFn) -> GenResult<Self> {
        let args = FunctionArgs::from(item_fn.sig.inputs)?;
        /*
        let mut args = vec![];

        for item in item_fn.sig.inputs.into_iter() {
            match item {
                syn::FnArg::Typed(pty) => {
                    args.push(FunctionArg::from(pty)?);
                }
                _ => Err(format!("{:?} not allowed as argument", 1)),
            }
        }
        */

        Ok(Self {
            name: item_fn.sig.ident,
            args,
            block: item_fn.block,
            output: FunctionRet::from(item_fn.sig.output)?,
        })
    }

    pub fn generate_rust_function(&self) -> impl quote::ToTokens {
        let ident = &self.name;
        let body = self.generate_body();
        let code = quote! {
            #[no_mangle]
            pub extern fn #ident(vm: &mut Vm) -> lovm2_extend::prelude::Lovm2Result<()> {
                #body
            }
        };
        code
    }

    fn generate_body(&self) -> impl quote::ToTokens {
        let prelude = self.generate_prelude();
        let postlude = self.generate_postlude();
        let block = &self.block;

        quote! {
            let _lv2_return_value = {
                #prelude
                { #block }
            };
            #postlude
        }
    }

    fn generate_postlude(&self) -> impl quote::ToTokens {
        self.output.generate()
    }

    fn generate_prelude(&self) -> impl quote::ToTokens {
        self.args.generate()
    }
}

pub(crate) fn accept_type(ty: &syn::Type) -> GenResult<&syn::Type> {
    match ty {
        syn::Type::Path(ty_path) => {
            if let Some(ident) = ty_path.path.get_ident() {
                let ident_name = ident.to_string();
                match ident_name.as_ref() {
                    "bool" | "f64" | "i64" | "String" | "Vm" => return Ok(ty),
                    _ => {}
                }
            }
        }
        syn::Type::Reference(ref_type) => {
            //assert!(ref_type.mutability.is_some());
            accept_type(&ref_type.elem)?;
            return Ok(ty);
        }
        _ => {}
    }
    Err(format!("unexpected type"))
}

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
            if let Some(_ident) = ty_path.path.get_ident() {
                return Ok(ty);
            }
        }
        syn::Type::Reference(ref_type) => {
            accept_type(&ref_type.elem)?;
            return Ok(ty);
        }
        _ => {}
    }
    Err(format!("unexpected type {:?}", ty))
}
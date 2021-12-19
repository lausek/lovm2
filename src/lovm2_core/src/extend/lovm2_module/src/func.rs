use super::*;

pub struct Function {
    pub(crate) name: Ident,
    args: FunctionArgs,
    block: Box<Block>,
    output: FunctionRet,
    is_extern: bool,
}

impl Function {
    pub fn from(item_fn: ItemFn) -> GenResult<Self> {
        let args = FunctionArgs::from(item_fn.sig.inputs)?;

        Ok(Self {
            name: item_fn.sig.ident,
            args,
            block: item_fn.block,
            output: FunctionRet::from(item_fn.sig.output)?,
            is_extern: false,
        })
    }

    pub fn set_extern(&mut self, is_extern: bool) {
        self.is_extern = is_extern;
    }

    pub fn generate_rust_function(&self) -> impl quote::ToTokens {
        let ident = &self.name;
        let body = self.generate_body();
        let docstring = format!("`{}({}) -> {}`", ident, self.args, self.output);
        let code = if self.is_extern {
            quote! {
                #[doc = #docstring]
                #[no_mangle]
                pub extern fn #ident(vm: &mut lovm2_core::vm::LV2Vm) -> lovm2_core::prelude::LV2Result<()> {
                    #body
                }
            }
        } else {
            quote! {
                #[doc = #docstring]
                pub fn #ident(vm: &mut lovm2_core::vm::LV2Vm) -> lovm2_core::prelude::LV2Result<()> {
                    #body
                }
            }
        };

        code
    }

    fn generate_body(&self) -> impl quote::ToTokens {
        let prelude = self.generate_prelude();
        let postlude = self.generate_postlude();
        let block = &self.block;
        let output = &self.output.as_tokens();

        let args = self.args.as_tokens();
        let argscall = self.args.generate_call();

        quote! {
            #[inline]
            fn _lv2_wrapper(#args) -> #output #block
            #prelude
            let _lv2_return_value: #output = #argscall;
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

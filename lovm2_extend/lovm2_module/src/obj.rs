use syn::ItemStruct;

use super::*;

pub struct Object {
    ty: ItemStruct,
}

impl Object {
    pub fn from(item: TokenStream) -> Self {
        let ty = syn::parse::<syn::ItemStruct>(item).unwrap();
        Self {
            ty,
        }
    }

    pub fn generate_rust_structure(&self) -> impl quote::ToTokens {
        let name = &self.ty.ident;
        let ty = &self.ty;
        let result = quote! {
            #ty

            impl Into<Value> for #name {
                fn into(self) -> Value {
                    Value::create_any(self)
                }
            }
        };
        result
    }
}

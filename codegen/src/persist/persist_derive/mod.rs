use proc_macro::TokenStream;
use quote::quote;
use syn;

pub(crate) fn r#impl(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_persist(&ast)
}


pub fn impl_persist(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Persist for #name {
            fn save(&self) {
                // filename is the name of the struct plus the .bin extension
                let filename = format!("{}.bin", stringify!(#name));
                let mut file = std::io::BufWriter::new(std::fs::File::create(filename).unwrap());
                bincode::serialize_into(&mut file, &self).unwrap();
            }

            fn load(&self) -> Self {
                // filename is the name of the struct + .bin
                let filename = format!("{}.bin", stringify!(#name));
                let mut file = std::fs::File::open(filename).unwrap();
                return bincode::deserialize_from(&mut file).unwrap()
            }
        }
    };

    gen.into()
}

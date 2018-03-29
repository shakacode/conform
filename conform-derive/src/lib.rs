#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate conform;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_derive(Conform, attributes(conform))]
pub fn derive_conform(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_conform(&ast).into()
}

fn impl_conform(ast: &syn::DeriveInput) -> quote::Tokens {
  let fields: Vec<syn::Field> = match ast.data {
    syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
      if fields.iter().any(|field| field.ident.is_none()) {
        panic!("struct has unnamed fields");
      }
      fields.iter().cloned().collect()
    }
    _ => panic!("#[derive(Conform)] is only defined for structs"),
  };

  let mut tokens = vec![];

  for field in &fields {
    for attr in &field.attrs {
      if attr.path != parse_quote!(conform) {
        continue;
      }

      let field_ident = field.ident.clone().unwrap();

      match attr.interpret_meta() {
        Some(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
          let meta_items: Vec<&syn::NestedMeta> = nested.iter().collect();
          for meta_item in meta_items {
            match *meta_item {
              syn::NestedMeta::Meta(ref item) => match *item {
                syn::Meta::Word(ref name) => match name.to_string().as_ref() {
                  "trim" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::trim),
                  )),
                  "trim_left" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::trim_left),
                  )),
                  "trim_right" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::trim_right),
                  )),
                  "lower" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::lower),
                  )),
                  "upper" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::upper),
                  )),
                  "sentence" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::sentence),
                  )),
                  "title" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::title),
                  )),
                  "camel" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::camel),
                  )),
                  "pascal" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::pascal),
                  )),
                  "kebab" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::kebab),
                  )),
                  "train" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::train),
                  )),
                  "snake" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::snake),
                  )),
                  "constant" => tokens.push(field_tokens(
                    field,
                    field_ident,
                    quote!(conform::make::constant),
                  )),
                  _ => panic!("Unexpected conform argument: {}", name),
                },
                _ => unreachable!("Found a non Word while looking for conform arguments"),
              },
              _ => unreachable!("Found a non Meta while looking for conform arguments"),
            }
          }
        }
        _ => unreachable!(
          "Got something other than a list of attributes while checking field `{}`",
          field_ident
        ),
      }
    }
  }

  let ident = &ast.ident;
  let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

  quote! {
    impl #impl_generics Conform for #ident #ty_generics #where_clause {
      fn conform(&mut self) -> &mut Self {
        use conform;
        #(#tokens)*
        self
      }
    }
  }
}

fn field_tokens(
  field: &syn::Field,
  field_ident: syn::Ident,
  transform: quote::Tokens,
) -> quote::Tokens {
  match field.ty {
    syn::Type::Path(syn::TypePath { ref path, .. }) => {
      let mut tokens = quote::Tokens::new();
      path.to_tokens(&mut tokens);
      match tokens.to_string().replace(' ', "").as_ref() {
        "String" => {
          quote! {
            self.#field_ident = #transform(&self.#field_ident);
          }
        }
        "Option<String>" => {
          quote! {
            self.#field_ident = match self.#field_ident {
              Some(ref value) => Some(#transform(value)),
              None => None
            };
          }
        }
        _ => panic!(
          "Field `{}`: only `String` & `Option<String>` types are supported",
          field_ident
        ),
      }
    }
    _ => panic!(
      "Field `{}`: only `String` & `Option<String>` types are supported",
      field_ident
    ),
  }
}

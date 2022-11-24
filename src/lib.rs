use proc_macro::TokenStream;
use quote::quote;
extern crate proc_macro;
use syn::{
    parse_macro_input, AttributeArgs, Data, DataStruct, Field, Fields, Ident, NestedMeta,
    NestedMeta::Meta,
};

#[proc_macro_attribute]
pub fn export_ts(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(metadata as AttributeArgs);
    let output_type = output_type(&attr_args).unwrap();

    let input_ast = syn::parse(input.clone()).unwrap();

    let mut combined = TokenStream::new();
    // re-add the original struct
    combined.extend(input);
    // add typescript things
    combined.extend(generate_ts_declarations(&input_ast, &output_type));
    combined
}

fn output_type(attr_args: &[NestedMeta]) -> Option<Ident> {
    let arg = attr_args.first()?;
    match arg {
        Meta(syn::Meta::Path(p)) => p.segments.last().map(|s| s.ident.clone()),
        _ => None,
    }
}

fn to_ts(field: &Field) -> Option<String> {
    let name = field.ident.clone()?.to_string();
    let type_ = match &field.ty {
        syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.clone()),
        _ => None,
    }?;

    // just 2 types for now
    let ts_type = match type_.to_string().as_ref() {
        "u64" => Some("number".to_owned()),
        "String" => Some("string".to_owned()),
        _ => None,
    }?;

    Some(format!("{}: {}", name, ts_type))
}

fn generate_ts_type_declaration(name: String, fields: String) -> String {
    format!(
        r#"
  interface {name} {{
    {fields}
  }}
"#
    )
}

fn generate_ts_declarations(ast: &syn::DeriveInput, output_type: &syn::Ident) -> TokenStream {
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let fields_ts: String = fields
        .iter()
        .filter_map(to_ts)
        .collect::<Vec<String>>()
        .join("\n");

    let ts_type = generate_ts_type_declaration(output_type.to_string(), fields_ts);

    TokenStream::from(quote! {
      #[wasm_bindgen(typescript_custom_section)]
      const #output_type: &'static str = #ts_type;

      #[wasm_bindgen]
      extern "C" {
        #[wasm_bindgen(typescript_type = #output_type)]
        pub type #output_type;
      }

    })
}

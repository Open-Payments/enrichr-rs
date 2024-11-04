use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Enrichable)]
pub fn derive_enrichable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract field names
    let fields = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    fields_named.named.iter()
                        .map(|f| f.ident.as_ref().unwrap())
                        .collect::<Vec<_>>()
                },
                _ => panic!("Only named fields are supported"),
            }
        },
        _ => panic!("Only structs are supported"),
    };

    let field_matches = fields.iter().map(|field| {
        let field_str = field.to_string();
        let field_ident = field;
        quote! {
            #field_str => {
                let value: Result<_, serde_json::Error> = serde_json::from_value(value_to_set.clone());
                match value {
                    Ok(v) => {
                        self.#field_ident = v;
                    },
                    Err(e) => return Err(EnrichmentError::DataTypeError(e.to_string())),
                }
            }
        }
    }).collect::<Vec<_>>();

    let gen = quote! {
        impl Enrichable for #name {
            fn enrich(&mut self, data: &HashMap<String, serde_json::Value>, rules: &[MappingRule]) -> Result<(), EnrichmentError> {
                for rule in rules {
                    // Extract source value(s)
                    let source_value = match &rule.source {
                        JsonPath::Single(path) => {
                            JsonPathExtractor::new().extract_value(data, path)?
                        },
                        JsonPath::Multiple(paths) => {
                            let values = JsonPathExtractor::new().extract_values(data, paths)?;
                            serde_json::Value::Array(values)
                        }
                    };

                    // Apply transformation if specified
                    let transformed_value = if let Some(transform) = &rule.transform {
                        match transform.transform_type {
                            TransformType::Split => {
                                let delimiter = transform.params.as_ref()
                                    .and_then(|p| p.get("delimiter"))
                                    .ok_or_else(|| EnrichmentError::TransformError("Missing delimiter parameter".into()))?;
                                
                                match &source_value {
                                    serde_json::Value::String(s) => {
                                        let parts: Vec<serde_json::Value> = s.split(delimiter)
                                            .map(|s| serde_json::Value::String(s.to_string()))
                                            .collect();
                                        serde_json::Value::Array(parts)
                                    },
                                    _ => return Err(EnrichmentError::TransformError("Expected string value for split".into())),
                                }
                            },
                            TransformType::Template => {
                                let template = transform.params.as_ref()
                                    .and_then(|p| p.get("template"))
                                    .ok_or_else(|| EnrichmentError::TransformError("Missing template parameter".into()))?;
                                
                                match &source_value {
                                    serde_json::Value::Array(values) => {
                                        let mut result = template.clone();
                                        for (i, val) in values.iter().enumerate() {
                                            let placeholder = format!("{{{}}}", i);
                                            let value_str = val.as_str()
                                                .ok_or_else(|| EnrichmentError::TransformError("Array value is not a string".into()))?;
                                            result = result.replace(&placeholder, value_str);
                                        }
                                        serde_json::Value::String(result)
                                    },
                                    _ => return Err(EnrichmentError::TransformError("Expected array value for template".into())),
                                }
                            },
                            TransformType::ToUpperCase => {
                                match &source_value {
                                    serde_json::Value::String(s) => serde_json::Value::String(s.to_uppercase()),
                                    _ => return Err(EnrichmentError::TransformError("Expected string value for toUpperCase".into())),
                                }
                            },
                            TransformType::ToLowerCase => {
                                match &source_value {
                                    serde_json::Value::String(s) => serde_json::Value::String(s.to_lowercase()),
                                    _ => return Err(EnrichmentError::TransformError("Expected string value for toLowerCase".into())),
                                }
                            },
                            _ => return Err(EnrichmentError::TransformError("Transformation not implemented".into())),
                        }
                    } else {
                        source_value.clone()
                    };

                    // Update target field(s)
                    match &rule.target {
                        Target::Single(path) => {
                            let field_name = path.split('.').last()
                                .ok_or_else(|| EnrichmentError::SpecError("Invalid target path".into()))?
                                .trim_start_matches('$')
                                .trim_start_matches('.');

                            let value_to_set = transformed_value.clone();
                            match field_name {
                                #(#field_matches)*
                                _ => return Err(EnrichmentError::SpecError(format!("Field {} not found", field_name))),
                            }
                        },
                        Target::Multiple(paths) => {
                            if let serde_json::Value::Array(values) = &transformed_value {
                                for (path, value) in paths.iter().zip(values.iter()) {
                                    let field_name = path.split('.').last()
                                        .ok_or_else(|| EnrichmentError::SpecError("Invalid target path".into()))?
                                        .trim_start_matches('$')
                                        .trim_start_matches('.');

                                    let value_to_set = value.clone();
                                    match field_name {
                                        #(#field_matches)*
                                        _ => return Err(EnrichmentError::SpecError(format!("Field {} not found", field_name))),
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            }
        }
    };

    gen.into()
}
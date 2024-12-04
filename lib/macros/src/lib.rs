// Copyright (C) 2024 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![feature(let_chains)]

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, ExprPath, ItemFn};

#[proc_macro_attribute]
pub fn service_main(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let output = {
        let attrs = input.attrs;
        let sig = input.sig;
        let block = input.block;
        let config = match attr.to_string().to_lowercase().as_str() {
            "cluster" => quote!(toolkit::once_cell::sync::Lazy::force(&toolkit::config::CLUSTER_CONF);),
            "realm" => quote!(toolkit::once_cell::sync::Lazy::force(&toolkit::config::REALM_CONF);),
            _ => quote!(),
        };

        quote! {
            #[tokio::main]
            #(#attrs)*
            #sig {
                extern crate toolkit;

                let _ = toolkit::dotenvy::dotenv();
                toolkit::env_logger::Builder::from_env(
                    toolkit::env_logger::Env::default()
                    .default_filter_or("info")
                ).init();

                #config

                #block
            }
        }
    };

    proc_macro::TokenStream::from(output)
}

#[derive(FromDeriveInput)]
#[darling(attributes(graphql_crud))]
struct GraphqlCrudStructOps {
    name: String,
}

#[derive(FromField)]
#[darling(attributes(graphql_crud))]
struct GraphqlCrudFieldOps {
    #[darling(default)]
    skip: bool,
    #[darling(default)]
    serialize_as: Option<ExprPath>,
    #[darling(default)]
    filter: bool,
}

#[proc_macro_derive(GraphqlCrud, attributes(graphql_crud))]
pub fn graphql_crud_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let opts = match GraphqlCrudStructOps::from_derive_input(&ast) {
        Ok(opts) => opts,
        Err(e) => {
            return syn::Error::new_spanned(ast, e).to_compile_error().into();
        }
    };

    let struct_name = ast.ident.clone();

    let schema_query_root = format_ident!("{}QueryRoot", ast.ident);
    let schema_mutation_root = format_ident!("{}MutationRoot", ast.ident);
    
    let record_input_ident = format_ident!("{}Input", ast.ident);
    let read_single_ident = format_ident!("{}", opts.name);
    let read_multiple_ident = format_ident!("{}s", opts.name);
    let read_multiple_filter_ident = format_ident!("{}Filter", ast.ident);
    let delete_single_ident = format_ident!("delete_{}", opts.name);
    let create_single_ident = format_ident!("create_{}", opts.name);
    let update_single_ident = format_ident!("update_{}", opts.name);
    let create_multiple_ident = format_ident!("batch_create_{}s", opts.name);

    let fields = if let syn::Data::Struct(data) = &ast.data {
        if let syn::Fields::Named(fields) = &data.fields {
            &fields.named
        } else {
            panic!("Expected named fields in struct");
        }
    } else {
        panic!("GraphqlCrud only supports structs");
    };

    let schema_fields: Vec<_> = fields.iter().filter_map(|field| {
        let field_opts = match GraphqlCrudFieldOps::from_field(field) {
            Ok(opts) => opts,
            Err(e) => {
                return Some(syn::Error::new_spanned(field, e).to_compile_error())
            },
        };

        let ident = &field.ident;
        let vis = &field.vis;
        let ty = &field.ty;

        if field_opts.skip {
            None
        } else if let Some(serialize_as) = &field_opts.serialize_as {
            Some(quote!{ #vis #ident: #serialize_as })
        } else {
            Some(quote!{ #vis #ident: #ty })
        }
    }).collect();

    let filters: Vec<_> = fields.iter().filter_map(|field| {
        let field_opts = match GraphqlCrudFieldOps::from_field(field) {
            Ok(opts) => opts,
            Err(e) => {
                return Some(syn::Error::new_spanned(field, e).to_compile_error())
            },
        };

        if field_opts.filter {
            let ident = &field.ident;
            let ty = &field.ty;

            if let Some(serialize_as) = &field_opts.serialize_as {
                Some(quote!{ pub #ident: Option<#serialize_as> })
            } else {
                Some(quote!{ pub #ident: Option<#ty> })
            }
        } else {
            None
        }
    }).collect();

    let filter_expressions: Vec<_> = fields.iter().filter_map(|field| {
        let field_opts = match GraphqlCrudFieldOps::from_field(field) {
            Ok(opts) => opts,
            Err(e) => {
                return Some(syn::Error::new_spanned(field, e).to_compile_error())
            },
        };

        if field_opts.filter {
            let ident = &field.ident;
            let field_name = field.ident.as_ref().unwrap().to_string();
            Some(quote!{ 
                if let Some(val) = &self.#ident {
                    expressions.push(doc!{ #field_name: val }); 
                }
            })
        } else {
            None
        }
    }).collect();

    let (filter_input, filter_param, filter_pass) = if filters.is_empty() {
        (quote!(), quote!(), quote!(None))
    } else {
        (
            quote!{
                #[derive(async_graphql::InputObject)]
                pub struct #read_multiple_filter_ident {
                    #(#filters),*
                }

                impl #read_multiple_filter_ident {
                    fn into_query(self) -> Document {
                        let mut expressions: Vec<Document> = vec![];

                        #(#filter_expressions)*

                        doc!{ "$and": expressions }
                    }
                }
            }, 
            quote!(filter: Option<#read_multiple_filter_ident>,),
            quote!(filter.map(#read_multiple_filter_ident::into_query))
        )
    };

    let field_serializer: Vec<_> = fields.iter().filter_map(|field| {
        let field_name = &field.ident;

        let field_opts = match GraphqlCrudFieldOps::from_field(field) {
            Ok(opts) => opts,
            Err(e) => {
                return Some(syn::Error::new_spanned(field, e).to_compile_error())
            },
        };

        if field_opts.skip {
            None
        } else if field_opts.serialize_as.is_some() {
            Some(quote!{ #field_name: value.#field_name.try_into()? })
        } else {
            Some(quote!{ #field_name: value.#field_name })
        }
    }).collect();

    let field_deserializer: Vec<_> = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_ty = &field.ty;

        let field_opts = match GraphqlCrudFieldOps::from_field(field) {
            Ok(opts) => opts,
            Err(e) => {
                return syn::Error::new_spanned(field, e).to_compile_error()
            },
        };

        if field_opts.skip {
            quote!{ #field_name: <#field_ty as Default>::default() }
        } else if field_opts.serialize_as.is_some() {
            quote!{ #field_name: value.#field_name.try_into()? }
        } else {
            quote!{ #field_name: value.#field_name }
        }
    }).collect();

    let output: TokenStream = quote! {
        mod schema {
            extern crate toolkit;

            use super::*;
            use database::DatabaseRecord;
            use toolkit::bson::{Document, doc};
            use toolkit::record_pagination::*;

            #[derive(Default)]
            pub struct #schema_query_root;

            #[derive(Default)]
            pub struct #schema_mutation_root;

            #[derive(async_graphql::SimpleObject)]
            pub struct #struct_name {
                #(#schema_fields),*
            }

            #[derive(async_graphql::InputObject)]
            pub struct #record_input_ident {
                #(#schema_fields),*
            }

            #filter_input

            impl std::convert::TryFrom<super::#struct_name> for #struct_name {
                type Error = toolkit::anyhow::Error;

                fn try_from(value: super::#struct_name) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#field_serializer),*
                    })
                }
            }

            impl std::convert::TryFrom<#record_input_ident> for super::#struct_name {
                type Error = toolkit::anyhow::Error;

                fn try_from(value: #record_input_ident) -> Result<Self, Self::Error> {
                    Ok(Self {
                        #(#field_deserializer),*
                    })
                }
            }

            #[async_graphql::Object]
            impl #schema_query_root {
                async fn #read_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    id: <super::#struct_name as DatabaseRecord>::PrimaryKey
                ) -> Result<Option<#struct_name>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    Ok(
                        match super::#struct_name::get(&db, &id).await? {
                            Some(record) => Some(record.try_into()?),
                            None => None,
                        }
                    )
                }

                async fn #read_multiple_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    #filter_param
                    after: Option<String>,
                    before: Option<String>,
                    first: Option<i32>,
                    last: Option<i32>
                ) -> Result<RecordConnection<#struct_name>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    record_query::<super::#struct_name, _>(
                        db, 
                        #filter_pass, 
                        after, 
                        before, 
                        first, 
                        last
                    ).await
                }
            }

            #[async_graphql::Object]
            impl #schema_mutation_root {
                async fn #delete_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    id: <super::#struct_name as DatabaseRecord>::PrimaryKey
                ) -> Result<Option<#struct_name>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    if let Some(record) = super::#struct_name::get(&db, &id).await? {
                        record.delete(&db).await?;
                        Ok(Some(record.try_into()?))
                    } else {
                        Ok(None)
                    }
                }

                async fn #create_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    input: #record_input_ident
                ) -> Result<#struct_name, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    let record = <super::#struct_name as DatabaseRecord>
                        ::create(&db, input.try_into()?)
                        .await?;

                    Ok(record.try_into()?)
                }

                async fn #create_multiple_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    input: Vec<#record_input_ident>
                ) -> Result<Vec<#struct_name>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    let records = input
                        .into_iter()
                        .map(|r| r.try_into())
                        .collect::<Result<Vec<super::#struct_name>, toolkit::anyhow::Error>>()?;

                    <super::#struct_name as DatabaseRecord>::collection(&db)
                        .insert_many(&records)
                        .await?;
        
                    Ok(records
                        .into_iter()
                        .map(|r| r.try_into())
                        .collect::<Result<Vec<_>, toolkit::anyhow::Error>>()?
                    )
                }
            }
        }

        pub use schema::*;
    }.into();

    output
}
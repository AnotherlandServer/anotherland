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

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, ExprLit, ExprPath, ExprTuple, ImplItem, ItemFn, ItemImpl, ItemStruct, Lit, MacroDelimiter, PatPath, Token, Type, TypePath};

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

struct RecordSchemaInput {
    /*db_type: ExprPath,
    schema_type: ExprPath,*/
    base_name: String,
}

impl Parse for RecordSchemaInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = input.parse_terminated(Expr::parse, Token![,])?
            .into_iter();

        Ok(Self {
            /*db_type: fields.next()
                .and_then(|ele| {
                    if let Expr::Path(p) = ele {
                        Some(p)
                    } else {
                        None
                    }
                })
                .expect("Failed to extract record datatype"),
            schema_type: fields.next()
                .and_then(|ele| {
                    if let Expr::Path(p) = ele {
                        Some(p)
                    } else {
                        None
                    }
                })
                .expect("Failed to extract schema datatype"),*/
            base_name: fields.next()
                .and_then(|ele| {
                    if 
                        let Expr::Lit(lit) = ele && 
                        let Lit::Str(s) = lit.lit
                    {
                        Some(s.value())
                    } else {
                        None
                    }
                })
                .expect("Failed to extract schema datatype"),
        })
    }
}

#[proc_macro_attribute]
pub fn graphql_crud(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as RecordSchemaInput);

    //let record_type = args.db_type.clone();
    //let schema_type = args.schema_type.clone();
    let record_ident = ast.ident.clone();
    let schema_query_root = format_ident!("{}QueryRoot", ast.ident);
    let schema_mutation_root = format_ident!("{}MutationRoot", ast.ident);
    
    let record_input_ident = format_ident!("{}Input", ast.ident);
    let read_single_ident = format_ident!("{}", args.base_name);
    let read_multiple_ident = format_ident!("{}s", args.base_name);
    let delete_single_ident = format_ident!("delete_{}", args.base_name);
    let create_single_ident = format_ident!("create_{}", args.base_name);
    let update_single_ident = format_ident!("update_{}", args.base_name);
    let create_multiple_ident = format_ident!("batch_create_{}s", args.base_name);

    let fields = ast.fields.clone();
    let field_assigners: Vec<_> = ast.fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            quote! { #ident: value.#ident }
        })
        .collect();

    let output: TokenStream = quote! {
        #ast

        mod schema {
            extern crate toolkit;

            use super::*;
            use database::DatabaseRecord;
            use toolkit::record_pagination::*;

            #[derive(Default)]
            pub struct #schema_query_root;

            #[derive(Default)]
            pub struct #schema_mutation_root;

            #[derive(async_graphql::SimpleObject)]
            pub struct #record_ident
            #fields

            #[derive(async_graphql::InputObject)]
            pub struct #record_input_ident
            #fields

            impl From<super::#record_ident> for #record_ident {
                fn from(value: super::#record_ident) -> Self {
                    Self {
                        #(#field_assigners),*
                    }
                }
            }

            impl From<#record_input_ident> for super::#record_ident {
                fn from(value: #record_input_ident) -> Self {
                    Self {
                        #(#field_assigners),*
                    }
                }
            }

            #[async_graphql::Object]
            impl #schema_query_root {
                async fn #read_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    id: <super::#record_ident as DatabaseRecord>::PrimaryKey
                ) -> Result<Option<#record_ident>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    Ok(
                        super::#record_ident::get(&db, &id).await?
                        .map(|r| r.into())
                    )
                }

                async fn #read_multiple_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    after: Option<String>,
                    before: Option<String>,
                    first: Option<i32>,
                    last: Option<i32>
                ) -> Result<RecordConnection<#record_ident>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    record_query::<super::#record_ident, _>(db, None, after, before, first, last).await
                }
            }

            #[async_graphql::Object]
            impl #schema_mutation_root {
                async fn #delete_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    id: <super::#record_ident as DatabaseRecord>::PrimaryKey
                ) -> Result<Option<#record_ident>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    if let Some(record) = super::#record_ident::get(&db, &id).await? {
                        record.delete(&db).await?;
                        Ok(Some(record.into()))
                    } else {
                        Ok(None)
                    }
                }

                async fn #create_single_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    input: #record_input_ident
                ) -> Result<#record_ident, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    let record: super::#record_ident = input.into();

                    <super::#record_ident as DatabaseRecord>::collection(&db)
                        .insert_one(record.clone())
                        .await?;
        
                    Ok(record.into())
                }

                async fn #create_multiple_ident(
                    &self, 
                    ctx: &async_graphql::Context<'_>, 
                    input: Vec<#record_input_ident>
                ) -> Result<Vec<#record_ident>, async_graphql::Error> {
                    let db = ctx.data::<mongodb::Database>()?.clone();
                    let records: Vec<super::#record_ident> = input
                        .into_iter()
                        .map(|r| r.into())
                        .collect();

                    <super::#record_ident as DatabaseRecord>::collection(&db)
                        .insert_many(&records)
                        .await?;
        
                    Ok(records.into_iter().map(|r| r.into()).collect())
                }
            }
        }

        pub use schema::*;
    }.into();

    output
}
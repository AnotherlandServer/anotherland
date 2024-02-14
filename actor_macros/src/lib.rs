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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, format_ident};
use syn::{ItemImpl, parse_macro_input, ImplItem, Visibility, FnArg, Pat, Type, Ident, Meta};
use convert_case::{Case, Casing};

fn extract_ident(ty: &Type) -> (Ident, TokenStream) {
    match ty {
        Type::Path(path) => {
            if let Some(segment) = path.path.segments.iter().next() {
                let template = match &segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => args.to_token_stream(),
                    _ => quote!(),
                };

                return (segment.ident.clone(), template);
            }
        },
        _ => panic!("invalid type"),
    }

    todo!()
}

#[proc_macro_attribute]
pub fn actor_actions(_args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ItemImpl);

    let (input_type, template) = extract_ident(input.self_ty.as_ref());

    let generics = input.generics.to_token_stream();

    let input_type_name = input_type.to_string();
    let message_ident = format_ident!("{}ActionMessage", input_type_name);

    let actions: Vec<_> = input.items.iter().filter_map(|f| {
        match f {
            ImplItem::Fn(f) => {
                Some(f.clone())
            },
            _ => None,
        } 
    }).collect();

    let public_actions: Vec<_> = actions.iter().filter_map(|f| {
        match f.vis {
            Visibility::Public(_) => Some(f.clone()),
            _ => None,
        }
      }).collect();

    let ref_action_messages: Vec<_> = actions.iter().map(|f| {
        //let signature = f.sig.to_token_stream();
        let ident = format_ident!("{}", f.sig.ident.to_string().to_case(Case::UpperCamel));
        let arguments: Vec<_> = f.sig.inputs.iter().filter_map(|a| {
            match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(a) => Some(a),
            }
        }).collect();

        let ret_val = match &f.sig.output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, rettype) => rettype.to_token_stream(),
        };

        quote! { 
            #ident{ ret: oneshot::Sender<#ret_val>, #(#arguments),*},
        }
    }).collect();

    let ref_action_message_handlers: Vec<_> = actions.iter().map(|f| {
        //let signature = f.sig.to_token_stream();
        let ident = format_ident!("{}", f.sig.ident.to_string().to_case(Case::UpperCamel));
        let fn_ident = f.sig.ident.clone();
        let arguments: Vec<_> = f.sig.inputs.iter().filter_map(|a| {
            match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(t) => match t.pat.as_ref() {
                    Pat::Ident(i) => Some(i),
                    _ => None,
                },
            }
        }).collect();

        match f.sig.asyncness {
            Some(_) => {
                quote! { 
                    Self::MessageType::#ident{ ret, #(#arguments),* } => {
                        let _ = ret.send(self.#fn_ident(#(#arguments),*).await);
                    }
                }
            },
            None => {
                quote! { 
                    Self::MessageType::#ident{ ret, #(#arguments),* } => {
                        let _ = ret.send(self.#fn_ident(#(#arguments),*));
                    }
                }
            }
        }
    }).collect();

    let ref_actions: Vec<_> = actions.iter().map(|f| {
        let ident = format_ident!("{}", f.sig.ident.to_string().to_case(Case::UpperCamel));
        let signature = f.sig.to_token_stream();
        let visibility = f.vis.to_token_stream();

        let asyncness = if f.sig.asyncness.is_none() {
            quote!(async)
        } else {
            quote!()
        };

        let arguments: Vec<_> = f.sig.inputs.iter().filter_map(|a| {
            match a {
                FnArg::Receiver(_) => None,
                FnArg::Typed(t) => match t.pat.as_ref() {
                    Pat::Ident(i) => Some(i),
                    _ => None,
                },
            }
        }).collect();

        let method_name = f.sig.ident.to_string();
        
        quote! { 
            #visibility #asyncness #signature {
                let (tx, rx) = oneshot::channel();

                match self.send_message(#message_ident::#ident { ret: tx, #(#arguments),*}).await {
                    Ok(_) => rx.await.unwrap(),
                    Err(_) => panic!("failed to send to rpc channel. Method: {}::{}", #input_type_name, #method_name),
                }
            }
        }
    }).collect();

    let remote_ref_actions: Vec<_> = public_actions.iter().filter(|f| {
            for a in f.attrs.iter() {
                if let Meta::Path(path) = &a.meta {
                    for segment in path.segments.iter() {
                        if segment.ident == "rpc" {
                            return true
                        }
                    }
                }
            }

            false
        })
        .map(|f| {
            let ident = format_ident!("{}", f.sig.ident.to_string().to_case(Case::UpperCamel));
            let signature = f.sig.to_token_stream();

            let asyncness = if f.sig.asyncness.is_none() {
                quote!(async)
            } else {
                quote!()
            };

            let arguments: Vec<_> = f.sig.inputs.iter().filter_map(|a| {
                match a {
                    FnArg::Receiver(_) => None,
                    FnArg::Typed(t) => match t.pat.as_ref() {
                        Pat::Ident(i) => Some(i),
                        _ => None,
                    },
                }
            }).collect();
            
            quote! { 
                pub #asyncness #signature {
                    let (tx, rx) = oneshot::channel();

                    match self.send_message(#message_ident::#ident { ret: tx, #(#arguments),*}).await {
                        Ok(_) => rx.await.unwrap(),
                        Err(_) => panic!("failed to send to rpc channel"),
                    }
                }
            }
        }).collect();

    let phantom_data = if template.is_empty() {
        quote!()
    } else {
        quote!{_phantom(PhantomData #template)}
    };

    // remove rpc attributes
    for item in input.items.iter_mut() {
        if let ImplItem::Fn(f) = item {
            f.attrs = f.attrs.clone().into_iter().filter(|a| {
                match &a.meta {
                    Meta::Path(path) => {
                        for segment in path.segments.iter() {
                            if segment.ident == "rpc" {
                                return false
                            }
                        }

                        true
                    },
                    _ => true,
                }
            }).collect();
        }
    }

    let (remote_actor_ref, remote_actor_type, has_remote_actions) = if remote_ref_actions.is_empty() {
        (quote!(), quote!(()), false)
    } else {
        (quote! {
            impl #generics RemoteActorRef<#input_type #template> {
                #(#remote_ref_actions)*
            }
        }, quote!(RemoteActorRef<#input_type #template>), true)
    };

    let expanded = quote! {
        use crate::cluster::actor::common_imports::*;

        #input

        pub enum #message_ident #generics {
            #(#ref_action_messages)*
            #phantom_data
        }

        #[async_trait]
        impl #generics ActorHandler for #input_type #template {
            type MessageType = #message_ident #template;
            type RemoteActorHandler = #remote_actor_type;

            async fn handle_message(&mut self, message: Self::MessageType) {
                match message {
                    #(#ref_action_message_handlers),*,
                    _ => unreachable!(),
                }
            }

            fn has_remote_actions() -> bool {
                #has_remote_actions
            }
        }

        impl #generics ActorRef<#input_type #template> {
            #(#ref_actions)*
        }

        #remote_actor_ref
    };

    proc_macro::TokenStream::from(expanded)
}



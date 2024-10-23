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

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn service_main(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    println!("attr: {}", _attr);

    let output = {
        let attrs = input.attrs;
        let sig = input.sig;
        let block = input.block;
        let config = match _attr.to_string().to_lowercase().as_str() {
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
                toolkit::env_logger::init();

                #config

                #block
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
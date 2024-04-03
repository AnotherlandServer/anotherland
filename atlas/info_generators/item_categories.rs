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

use std::{collections::{HashMap, HashSet}, io, path::Path, str::FromStr};

use convert_case::{Case, Casing};
use csv::ReaderBuilder;
use quote::{format_ident, quote, ToTokens};
use uuid::Uuid;

use crate::write_source;

pub fn generate_item_categories(client_path: &Path) -> Result<(), io::Error> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(client_path.join("Atlas/data/otherlandgame/system/ItemCategories.csv"))?;

    let mut category_names = HashMap::new();
    let mut category_matchers = Vec::new();

    for record in rdr.records() {
        match record {
            Ok(record) => {
                for i in 1..record.len() {
                    if let Some(category) = record.get(i) &&
                        !category.is_empty() {

                        let name = category
                            .split('.')
                            .last().unwrap()
                            .trim()
                            .replace('&', "And")
                            .to_case(Case::UpperCamel);
                        category_names.insert(category.to_string(), format_ident!("{}", name));
                    }
                }
                
                let uuid = record.get(0).unwrap().parse::<Uuid>().unwrap().as_bytes().to_vec();
                let categories: Vec<_> = record.iter()
                    .skip(1)
                    .map(|c| category_names.get(c))
                    .collect();

                let main = categories[0].unwrap().to_token_stream();
                let sub = if let Some(Some(category)) = categories.get(1) {
                    quote!(Some(ItemSubCategory::#category))
                } else {
                    quote!(None)
                };
                let leaf = if let Some(Some(category)) = categories.get(2) {
                    quote!(Some(ItemSubCategory::#category))
                } else {
                    quote!(None)
                };

                category_matchers.push(quote!([#(#uuid),*] => Some(&ItemCategory {
                    main: ItemSubCategory::#main,
                    sub: #sub,
                    leaf: #leaf,
                }),));
            },
            Err(e) => println!("Line error: {:?}", e),
        }
    }

    let categories: Vec<_> = category_names.values().collect();
    

    write_source("item_categories.rs", quote! {
        use bson::Uuid;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum ItemSubCategory {
            #(#categories),*
        }

        pub struct ItemCategory {
            main: ItemSubCategory,
            sub: Option<ItemSubCategory>,
            leaf: Option<ItemSubCategory>,
        }

        impl ItemCategory {
            pub fn main(&self) -> ItemSubCategory {
                self.main
            }

            pub fn sub(&self) -> Option<ItemSubCategory> {
                self.sub
            }

            pub fn leaf(&self) -> Option<ItemSubCategory> {
                self.leaf
            }
        }

        pub fn get_item_category(id: &Uuid) -> Option<&ItemCategory> {
            match id.bytes() {
                #(#category_matchers)*
                _ => None,
            }
        }
    })?;

    Ok(())
}
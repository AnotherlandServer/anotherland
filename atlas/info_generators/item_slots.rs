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

pub fn generate_item_slots(client_path: &Path) -> Result<(), io::Error> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(client_path.join("Atlas/data/otherlandgame/ItemManagement.csv"))?;

    let records = rdr.records();

    let mut slot_types = Vec::new();
    let mut slot_type_total_slots = Vec::new();

    for record in records.take(5) {
        match record {
            Ok(record) => {
                let type_ident = format_ident!("{}", record.get(0).unwrap().to_case(Case::UpperCamel));
                let total_slots: usize = record.get(1).unwrap().parse().unwrap();
                let description = record.get(2).unwrap();

                slot_types.push(quote! {
                    #[doc = #description]
                    #type_ident
                });
                slot_type_total_slots.push(quote!( Self::#type_ident => #total_slots ));
            },
            Err(e) => println!("Line error: {:?}", e),
        }
    }

    let mut slot_idents = Vec::new();
    let mut slot_type_matcher = Vec::new();
    let mut slot_slots_matcher = Vec::new();
    let mut slot_base_appearance_matcher = Vec::new();
    let mut slot_parsers = Vec::new();

    for record in rdr.records().skip(1) {
        match record {
            Ok(record) => {
                let slot_name = record.get(0).unwrap();
                let slot_ident = format_ident!("{}", slot_name.to_case(Case::UpperCamel));
                let type_ident = format_ident!("{}", record.get(1).unwrap().to_case(Case::UpperCamel));
                let base_appearance = record.get(7).unwrap() == "1";

                slot_idents.push(slot_ident.clone());

                let slots: Vec<_> = record.get(2).unwrap().split(';')
                    .filter_map(|slot| {
                        if slot == "-1" {
                            None
                        } else {
                            let slot = &slot_idents[slot.parse::<usize>().unwrap()];
                            Some(quote!{ Self::#slot })
                        }
                    })
                    .collect();

                slot_type_matcher.push(quote!{
                    Self::#slot_ident => &SlotType::#type_ident
                });
                slot_slots_matcher.push(quote!{
                    Self::#slot_ident => &[#(#slots),*]
                });
                slot_base_appearance_matcher.push(quote!{
                    Self::#slot_ident => #base_appearance
                });
                slot_parsers.push(quote! {
                    #slot_name => Ok(Self::#slot_ident),
                });
            },
            Err(e) => println!("Line error: {:?}", e),
        }
    }

    write_source("item_slots.rs", quote! {
        use std::str::FromStr;

        #[derive(Debug)]
        pub enum SlotType {
            #(#slot_types),*
        }

        impl SlotType {
            pub fn total_slots(&self) -> usize {
                match self {
                    #(#slot_type_total_slots),*
                }
            }
        }

        #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
        pub enum Slot {
            #(#slot_idents),*
        }

        impl Slot {
            pub fn slot_type(&self) -> &'static SlotType {
                match self {
                    #(#slot_type_matcher),*
                }
            }

            pub fn slots(&self) -> &'static [Slot] {
                match self {
                    #(#slot_slots_matcher),*
                }
            }

            pub fn is_base_appearance(&self) -> bool {
                match self {
                    #(#slot_base_appearance_matcher),*
                }
            }
        }

        impl FromStr for Slot {
            type Err = std::io::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#slot_parsers)*
                    _ => unimplemented!(),
                }
            }
        }
    })?;

    Ok(())
}
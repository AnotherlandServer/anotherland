// Copyright (C) 2025 AnotherlandServer
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

use std::{borrow::Cow, collections::VecDeque};

use log::warn;
use nom::{error::VerboseError, number::complete::{le_i16, le_i32, le_i8, le_u16, le_u32, le_u8}, sequence::{pair, preceded}};
use quick_xml::{events::{attributes::Attribute, BytesText, Event}, name::QName, Writer};
use nom::{bytes::complete::{take_until, take}, combinator::{map, verify}, multi::many_till, sequence::terminated};

#[derive(Debug)]
enum AttributeType {
    CString,    // 1
    Int32,      // 2
    Int16,      // 3
    Int8,       // 4
    UInt32,     // 5
    UInt16,     // 6
    UInt8,      // 7
}

impl AttributeType {
    fn from_byte(b: u8) -> Option<Self> {
        match b {
            1 => Some(AttributeType::CString),
            2 => Some(AttributeType::Int32),
            3 => Some(AttributeType::Int16),
            4 => Some(AttributeType::Int8),
            5 => Some(AttributeType::UInt32),
            6 => Some(AttributeType::UInt16),
            7 => Some(AttributeType::UInt8),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct AttributeDef {
    name: String,
    attr_type: AttributeType,
}

pub fn parse_tokenized_xml<'a, W: std::io::Write>(input: &'a [u8], writer: &mut Writer<W>) -> Result<&'a [u8], nom::Err<VerboseError<&'a [u8]>>> {
    // Parse all strings until we hit an empty string (just a zero byte)
    let (input, (strings, _)) = many_till(
        // Non-empty zero-terminated string
        map(
            verify(
                // Zero-terminated string
                terminated(take_until(&[0][..]), take(1usize)),
                |s: &[u8]| !s.is_empty()
            ),
            |bytes: &[u8]| std::str::from_utf8(bytes).unwrap_or_default()
        ),
        preceded(
            verify(take(1usize), |b: &[u8]| b[0] == 0),
            map(nom::combinator::success(&b""[..]), |_| ())
        )
    )(input)?;

    // Parse attribute definitions
    let attribute_def = map(
        pair(
            // Type byte
            map(verify(le_u8, |&b| b > 0 && b <= 7),
                |b| AttributeType::from_byte(b).unwrap()),
            // Attribute name (zero-terminated string)
            map(
                terminated(take_until(&[0][..]), take(1usize)),
                |bytes: &[u8]| std::str::from_utf8(bytes).unwrap_or_default().to_string()
            )
        ),
        |(attr_type, name)| AttributeDef { name, attr_type }
    );

    // Parse all attribute definitions until end marker
    let (input, (attribute_defs, _)) = many_till(
        attribute_def, 
        verify(take(1usize), |b: &[u8]| b[0] == 0)
    )(input)?;
 
    // Helper function to parse a string value
    #[allow(clippy::type_complexity)]
    fn parse_string(input: &[u8]) -> Result<(&[u8], String), nom::Err<VerboseError<&[u8]>>> {
        map(
            terminated(take_until(&[0][..]), take(1usize)),
            |bytes: &[u8]| std::str::from_utf8(bytes).unwrap_or_default().to_string()
        )(input)
    }
    
    let mut element_stack = VecDeque::new();
    let mut element_input = input;
    
    loop {
        let mut is_array = false;
        let mut is_byte_array = false;

        // Check if we have enough data to read an element index
        if element_input.is_empty() {
            if element_stack.is_empty() {
                return Ok(element_input);
            } else {
                return Err(nom::Err::Error(VerboseError { errors: vec![(element_input, nom::error::VerboseErrorKind::Context("Unexpected end of input"))] }));
            }
        }
        
        // Parse element index
        let (input, elem_index) = le_u8(element_input)?;

        let (input, elem_index) = if elem_index == 0xff || elem_index == 0xfe {
            is_array = true;
            is_byte_array = elem_index == 0xfe;

            le_u8(input)?
        } else {
            (input, elem_index)
        };
        
        // Zero index means end of current element
        if elem_index == 0 {
            let elem_name = element_stack.pop_back()
                .ok_or(nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("Closing element without matching open element"))] }))?;

            // End element
            writer.write_event(Event::End(
                quick_xml::events::BytesEnd::new(elem_name)
            )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;

            element_input = input;

            if element_stack.is_empty() {
                return Ok(element_input);
            } else {
                continue;
            }
        }
        
        // Get element name from the string table
        let elem_name = if elem_index as usize <= strings.len() {
            strings[elem_index as usize - 1] // Adjust for 1-based indexing
        } else {
            warn!("Invalid element index: {elem_index}");
            return Err(nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("Invalid element index"))] }));
        };

        if !is_array {
            element_stack.push_back(elem_name);
            
            // Start element
            let mut element = quick_xml::events::BytesStart::new(elem_name);

            element_input = input;
            
            // Parse attributes
            element_input = loop {           
                // Get attribute index
                let (input, attr_index) = le_u8(element_input)?;
                
                // Zero index means end of attributes for this element
                if attr_index == 0 {
                    break input;
                }
                
                // Get attribute definition from index
                let attr_def = if attr_index as usize <= attribute_defs.len() {
                    &attribute_defs[attr_index as usize - 1] // Adjust for 1-based indexing
                } else {
                    return Err(nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("Invalid attribute index"))] }));
                };

                // Parse attribute value based on its type
                let (input, value) = match attr_def.attr_type {
                    AttributeType::CString => {
                        // String value
                        let (input, string_val) = parse_string(input)?;
                        (input, string_val)
                    },
                    AttributeType::Int32 => {
                        // 32-bit signed integer
                        let (input, value) = le_i32(input)?;
                        (input, value.to_string())
                    },
                    AttributeType::Int16 => {
                        // 16-bit signed integer
                        let (input, value) = le_i16(input)?;
                        (input, value.to_string())
                    },
                    AttributeType::Int8 => {
                        // 8-bit signed integer
                        let (input, value) = le_i8(input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt32 => {
                        // 32-bit unsigned integer
                        let (input, value) = le_u32(input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt16 => {
                        // 16-bit unsigned integer
                        let (input, value) = le_u16(input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt8 => {
                        // 8-bit unsigned integer
                        let (input, value) = le_u8(input)?;
                        (input, value.to_string())
                    },
                };
                
                element.push_attribute(Attribute {
                    key: QName(attr_def.name.as_bytes()),
                    value: Cow::Owned(value.as_bytes().to_vec()),
                });

                element_input = input;
            };
            
            // Add element to the writer
            writer.write_event(Event::Start(
                element
            )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;
        } else {
            let (input, array_type) = if is_byte_array {
                (input, AttributeType::Int8)
            } else {
                map(
                    verify(le_u8, |&b| b > 0 && b <= 7),
                    |b| AttributeType::from_byte(b).unwrap()
                )(input)?
            };

            let (input, array_size) = le_u32(input)?;

            element_input = input;

            // Start element
            writer.write_event(Event::Start(
                quick_xml::events::BytesStart::new(elem_name)
            )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;

            for _ in 0..array_size {
                let (input, value) = match array_type {
                    AttributeType::CString => {
                        // String value
                        //let (input, string_val) = parse_string(element_input)?;
                        //(input, string_val)
                        todo!()
                    },
                    AttributeType::Int32 => {
                        // 32-bit signed integer
                        let (input, value) = le_i32(element_input)?;
                        (input, value.to_string())
                    },
                    AttributeType::Int16 => {
                        // 16-bit signed integer
                        let (input, value) = le_i16(element_input)?;
                        (input, value.to_string())
                    },
                    AttributeType::Int8 => {
                        // 8-bit signed integer
                        let (input, value) = le_i8(element_input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt32 => {
                        // 32-bit unsigned integer
                        let (input, value) = le_u32(element_input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt16 => {
                        // 16-bit unsigned integer
                        let (input, value) = le_u16(element_input)?;
                        (input, value.to_string())
                    },
                    AttributeType::UInt8 => {
                        // 8-bit unsigned integer
                        let (input, value) = le_u8(element_input)?;
                        (input, value.to_string())
                    },
                };

                writer.write_event(Event::Start(
                    quick_xml::events::BytesStart::new("v")
                )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;
    

                writer.write_event(Event::Text(
                    BytesText::new(&value)
                )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;

                writer.write_event(Event::End(
                    quick_xml::events::BytesEnd::new("v")
                )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;

                element_input = input;
            }

            writer.write_event(Event::End(
                quick_xml::events::BytesEnd::new(elem_name)
            )).map_err(|_| nom::Err::Error(VerboseError { errors: vec![(input, nom::error::VerboseErrorKind::Context("XML write error"))] }))?;
        }
    }
}
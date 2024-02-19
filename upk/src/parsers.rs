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

use nom::{bytes::complete::tag, combinator::map, multi::{count, length_count}, number::complete::{le_i32, le_u16, le_u32, le_u64, u8}, sequence::tuple, IResult, Slice};
use uuid::Uuid;

// raw data structures

#[derive(Debug)]
pub(super) struct Header {
    pub folder_name: String,
    pub flags: u32,
    pub name_table: Vec<NameTableEntry>,
    pub export_table: Vec<ExportTableEntry>,
    pub import_table: Vec<ImportTableEntry>,
    pub id: Uuid,
}

#[derive(Debug)]
pub(super) struct NameTableEntry {
    pub name: String,
    pub flags: u64,
}

#[derive(Debug)]
pub(super) struct ExportTableEntry {
    pub obj_type_ref: i32,
    pub parent_class_ref: i32,
    pub owner_ref: i32,
    pub name_table_idx: u32,
    pub name_count: u32,
    pub flags: u64,
    pub data_size: u32,
    pub data_offset: u32,
    pub additional_fields: Vec<u32>,
}

#[derive(Debug)]
pub(super) struct ImportTableEntry {
    pub package_id_idx: u32,
    pub obj_type_idx: i32,
    pub owner_ref: i32,
    pub name_table_idx: u32,
}

pub(super) fn parse_signature(i: &[u8]) -> IResult<&[u8], (u16, u16, u32)> {
    map(tuple((
        tag(&[0xC1, 0x83, 0x2A, 0x9E]),
        le_u16,
        le_u16,
        le_u32,
    )), |(_, version, licensee, header_size)| (version, licensee, header_size)
    )(i)
}

pub(super) fn parse_file_header(i: &[u8]) -> IResult<&[u8], Header> {
    let whole_header_buffer = i;

    let (i, folder_name) = length_count(le_u32, u8)(i)?;
    let (i, flags) = le_u32(i)?;
    let (i, name_count) = le_u32(i)?;
    let (i, name_offset) = le_u32(i)?;
    let (i, export_count) = le_u32(i)?;
    let (i, export_offset) = le_u32(i)?;
    let (i, import_count) = le_u32(i)?;
    let (i, import_offset) = le_u32(i)?;
    let (i, _) = le_u32(i)?; // depends offset
    let (i, _) = count(le_u32, 4)(i)?; // unknown chunk
    let (i, uuid_bytes) = count(u8, 16)(i)?;

    let name_table_buffer = &whole_header_buffer[(name_offset as usize - 12)..];
    let export_table_buffer = &whole_header_buffer[(export_offset as usize - 12)..];
    let import_table_buffer = &whole_header_buffer[(import_offset as usize - 12)..];

    let name_table = count(parse_name_table_entry, name_count as usize)(name_table_buffer)?.1;
    let export_table = count(parse_export_table_entry, export_count as usize)(export_table_buffer)?.1;
    let import_table = count(parse_import_table_entry, import_count as usize)(import_table_buffer)?.1;

    Ok((i, Header {
        folder_name: String::from_utf8_lossy(&folder_name).to_string(),
        flags,
        name_table,
        export_table,
        import_table,
        id: Uuid::from_bytes_le(uuid_bytes.try_into().unwrap()),
    }))
}

fn parse_name_table_entry(i: &[u8]) -> IResult<&[u8], NameTableEntry> {
    let (i, name) = length_count(le_u32, u8)(i)?;
    let (i, flags) = le_u64(i)?;

    Ok((i, NameTableEntry {
        name: String::from_utf8_lossy(&name).trim_matches('\0').to_string(),
        flags
    }))
}

fn parse_export_table_entry(i: &[u8]) -> IResult<&[u8], ExportTableEntry> {
    let (i, obj_type_ref) = le_i32(i)?;
    let (i, parent_class_ref) = le_i32(i)?;
    let (i, owner_ref) = le_i32(i)?;
    let (i, name_table_idx) = le_u32(i)?;
    let (i, name_count) = le_u32(i)?;
    let (i, _) = le_u32(i)?;
    let (i, flags) = le_u64(i)?;
    let (i, data_size) = le_u32(i)?;
    let (i, data_offset) = le_u32(i)?;
    let (i, _) = le_u32(i)?;
    let (i, num_additional_fields) = le_u32(i)?;
    let (i, _) = count(le_u32, 5)(i)?;
    let (i, additional_fields) = count(le_u32, num_additional_fields as usize)(i)?;

    Ok((i, ExportTableEntry {
        obj_type_ref,
        parent_class_ref,
        owner_ref,
        name_table_idx,
        name_count,
        flags,
        data_size,
        data_offset,
        additional_fields
    }))
}

fn parse_import_table_entry(i: &[u8]) -> IResult<&[u8], ImportTableEntry> {
    let (i, package_id_idx) = le_u32(i)?;
    let (i, _) = le_u32(i)?;
    let (i, obj_type_idx) = le_i32(i)?;
    let (i, _) = le_u32(i)?;
    let (i, owner_ref) = le_i32(i)?;
    let (i, name_table_idx) = le_u32(i)?;
    let (i, _) = le_u32(i)?;

    Ok((i, ImportTableEntry {
        package_id_idx,
        obj_type_idx,
        owner_ref,
        name_table_idx
    }))
}
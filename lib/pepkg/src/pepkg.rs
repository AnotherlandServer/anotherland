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

use std::{fs::File, io::{Cursor, Read, Seek}, path::Path};
use log::trace;
use nom::{combinator::map, error::VerboseError, number::complete::{le_i32, le_u32}, sequence::tuple};
use anyhow::anyhow;
use quick_xml::Writer;

use crate::{parse_tokenized_xml, Error, Federation, Mesh};

#[derive(Debug)]
pub struct PePkg {
    file: File,
    federation_file: FileEntry,
    tile_table: Vec<FileEntry>,
}

#[derive(Debug)]
struct FileEntry {
    size: usize,
    index: i32,
    offset: usize,
}

impl PePkg {
    pub fn open(path:  impl AsRef<Path>) -> Result<Self, Error> {
        let mut f = File::open(path.as_ref())?;

        trace!("Trying to open file: {:?}", path.as_ref());

        let mut header_buffer = [0u8; 36];
        f.read_exact(&mut header_buffer)?;

        let (_, (_, _header_ext_offset, _, file_table_entries, file_table_size, federation_def_offset, file_table_offset, _, _)) = tuple((
            le_i32, // _0
            le_i32, // header_ext_0_offset
            le_i32, // _1
            le_i32, // file_table_entries
            le_i32, // file_table_size
            le_i32, // federation_def_offset
            le_i32, // file_table_offset
            le_i32, // table_2_offset
            le_u32, // header_size
        ))(header_buffer.as_slice())
            .map_err(|e: nom::Err<VerboseError<_>>| anyhow!(e.to_string()))?;

        trace!("Header read. File table entries: {file_table_entries}, file table size: {file_table_size}, federation def offset: {federation_def_offset}, file table offset: {file_table_offset}");

        let mut federation_def_buffer = vec![0; 12];
        f.seek(std::io::SeekFrom::Start(federation_def_offset as u64))?;
        f.read_exact(federation_def_buffer.as_mut_slice())?;

        let (_, federation_file) = map(
            tuple((le_i32, le_i32, le_i32)),
            |(size, index, offset)| FileEntry {
                size: size as usize,
                index,
                offset: offset as usize,
            }
        )(federation_def_buffer.as_slice())
            .map_err(|e: nom::Err<VerboseError<_>>| anyhow!(e.to_string()))?;

        trace!("Federation file read. Size: {}, index: {}, offset: {}", federation_file.size, federation_file.index, federation_file.offset);

        let mut file_table_buffer = vec![0; (file_table_size * 12) as usize];
        f.seek(std::io::SeekFrom::Start(file_table_offset as u64))?;
        f.read_exact(file_table_buffer.as_mut_slice())?;

        // Parse all file entries in a single go
        let (_, file_table) = nom::multi::count(
            map(
            tuple((le_i32, le_i32, le_i32)),
            |(size, index, offset)| FileEntry {
                size: size as usize,
                index,
                offset: offset as usize,
            }
            ),
            file_table_entries as usize
        )(file_table_buffer.as_slice())
            .map_err(|e: nom::Err<VerboseError<_>>| anyhow!(e.to_string()))?;

        trace!("File table read. Entries: {file_table:#?}");

        Ok(Self {
            file: f,
            federation_file,
            tile_table: file_table,
        })
    }

    pub fn tile_count(&self) -> usize {
        self.tile_table.len()
    }

    pub fn read_federation_file_as_xml(&mut self) -> Result<String, Error> {
        let mut buffer = vec![0; self.federation_file.size];
        self.file.seek(std::io::SeekFrom::Start(self.federation_file.offset as u64))?;
        self.file.read_exact(buffer.as_mut_slice())?;

        let mut writer = Writer::new_with_indent(Cursor::new(Vec::<u8>::new()), 32, 4);

        parse_tokenized_xml(&buffer, &mut writer)
            .map_err(|e| anyhow!(e.to_string()))?;

        Ok(String::from_utf8(writer.into_inner().into_inner())?)
    }

    pub fn read_federation_file(&mut self) -> Result<Federation, Error> {
        Ok(serde_xml_rs::from_str::<Federation>(
            &self.read_federation_file_as_xml()?
        )?)
    }

    pub fn read_tile_as_xml(&mut self, index: usize) -> Result<Vec<String>, Error> {
        let file = self.tile_table.get(index).ok_or_else(|| anyhow!("File with index {index} not found"))?;
        let mut buffer = vec![0; file.size];
        self.file.seek(std::io::SeekFrom::Start(file.offset as u64))?;
        self.file.read_exact(buffer.as_mut_slice())?;

        let mut documents = Vec::new();

        let mut input = buffer.as_slice();

        loop {
            let mut writer = Writer::new_with_indent(Cursor::new(Vec::<u8>::new()), 32, 4);

            input = parse_tokenized_xml(input, &mut writer)
                .map_err(|e| anyhow!(e.to_string()))?;

            documents.push(String::from_utf8(writer.into_inner().into_inner())?);

            if input.is_empty() {
                break;
            }
        }

        Ok(documents)
    }

    pub fn read_tile(&mut self, index: usize) -> Result<(Mesh, String, String), Error> {
        let files = self.read_tile_as_xml(index)?;
        if files.len() != 3 {
            return Err(anyhow!("Expected 3 files, got {}", files.len()).into());
        }

        let mesh = serde_xml_rs::from_str::<Mesh>(&files[0])?;

        Ok((mesh, files[1].clone(), files[2].clone()))
    }
}

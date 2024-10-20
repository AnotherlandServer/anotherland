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

use std::{ path::Path, slice::Iter, sync::Arc};

use log::debug;
use tokio::{fs::File, io::{AsyncReadExt, AsyncSeekExt}, sync::RwLock};
use uuid::Uuid;

use crate::{parsers::{parse_file_header, parse_signature}, FName};

use super::upk_result::UPKResult;

pub struct PackageFile {
    file: RwLock<File>,
    version: u16,
    name: String,
    id: Uuid,
    names: Vec<FName>,
    exports: Vec<Arc<Export>>,
    imports: Vec<Arc<Import>>,
}

#[derive(Debug, Clone)]
pub enum LocalObjectIndexRef {
    Null,
    Import(usize),
    Export(usize),
}

pub enum LocalObject {
    Null,
    Export(ExportRef),
    Import(ImportRef),
}

impl LocalObjectIndexRef {
    pub fn from_idx(idx: i32) -> Self {
        if idx == 0 {
            LocalObjectIndexRef::Null
        } else if idx >= 0 {
            LocalObjectIndexRef::Export((idx - 1) as usize)
        } else {
            LocalObjectIndexRef::Import((-idx - 1) as usize)
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, LocalObjectIndexRef::Null)
    }
}

/*pub struct ObjectImportRef((FName, FName, LocalObjectIndexRef));

impl ObjectImportRef {
    pub fn package(&self) -> &str {
        &self.0.0
    }

    pub fn name(&self) -> &str {
        &self.0.1
    }

    pub fn owner(&self) -> &LocalObjectIndexRef {
        &self.0.2
    }
}*/

#[allow(dead_code)]
pub struct Export {
    self_ref: LocalObjectIndexRef,
    obj_type_ref: LocalObjectIndexRef,
    parent_class_ref: LocalObjectIndexRef,
    owner_ref: LocalObjectIndexRef,
    flags: u64,

    name: FName,
    name_count: Option<String>,
    data_offset: usize,
    data_size: usize,
    additional_fields: Vec<u32>,
}

pub struct Import {
    package: FName,
    class_name: FName,
    owner_ref: LocalObjectIndexRef,
    name: FName,
}

pub type ExportRef = Arc<Export>;
pub type ImportRef = Arc<Import>;

impl PackageFile {
    pub async fn open<P: AsRef<Path>>(path: P) -> UPKResult<PackageFile> {
        debug!("Open file: {}", path.as_ref().to_string_lossy());

        let mut file = File::open(path.as_ref()).await?;
        let mut signature_buf = [0; 12];

        file.read_exact(&mut signature_buf).await?;
        let (_, (version, _licensee, header_size)) = parse_signature(&signature_buf)?;

        // read header
        let mut header_buf = vec![0; header_size as usize];
        file.read_exact(&mut header_buf).await?;

        let (_, mut header) = parse_file_header(&header_buf)?;

        // create nametable from header data
        let names: Vec<_> = header.name_table.drain(..).map(|entry| FName::new(entry.name, entry.flags)).collect();

        // build import table
        let mut imports = Vec::new();
        for entry in header.import_table {
            imports.push(Arc::new(Import { 
                package: names[entry.package_id_idx as usize].clone(), 
                class_name: names[entry.obj_type_idx as usize].clone(), 
                owner_ref: LocalObjectIndexRef::from_idx(entry.owner_ref), 
                name:  names[entry.name_table_idx as usize].clone(),
            }));
        }

        // create export table from header data
        let mut exports = Vec::new();
        for (idx, entry) in header.export_table.into_iter().enumerate() {
            exports.push(Arc::new(Export {
                self_ref: LocalObjectIndexRef::Export(idx),
                obj_type_ref: LocalObjectIndexRef::from_idx(entry.obj_type_ref),
                parent_class_ref: LocalObjectIndexRef::from_idx(entry.parent_class_ref),
                owner_ref: LocalObjectIndexRef::from_idx(entry.owner_ref),
                flags: entry.flags,

                name: names[entry.name_table_idx as usize].clone(),
                name_count: if entry.name_count == 0 {
                    None
                } else {
                    Some(format!("{}_{}", names[entry.name_table_idx as usize], entry.name_count - 1))
                },
                data_offset: entry.data_offset as usize,
                data_size: entry.data_size as usize,
                additional_fields: entry.additional_fields,
            }));
        }

        // todo: find a better way of extracting just the filename without extension
        let name = path.as_ref().file_name().unwrap().to_str().unwrap().split('.').next().unwrap().to_string();

        Ok(PackageFile {
            file: RwLock::new(file),
            version,
            name,
            id: header.id,
            names,
            exports,
            imports,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn iter_exports(&self) -> Iter<ExportRef> {
        self.exports.iter()
    }

    pub fn iter_imports(&self) -> Iter<ImportRef> {
        self.imports.iter()
    }
    
    pub fn lookup_export_by_idx(&self, idx: usize) -> Option<ExportRef> {
        self.exports.get(idx).cloned()
    }

    pub fn lookup_name(&self, idx: usize) -> FName {
        self.names.get(idx).cloned()
            .unwrap_or_else(||panic!("Failed to lookup name index {}", idx))
    }

    pub fn lookup_export_by_name(&self, name: &str) -> Option<ExportRef> {
        self.exports.iter()
            .find(|entry| entry.name() == name)
            .cloned()
    }

    pub fn lookup_import(&self, idx: usize) -> Option<ImportRef> {
        self.imports.get(idx).cloned()
        /*.map(|import| { 
            ObjectImportRef((import.package.clone(), import.name.clone(), import.owner_ref.clone())) 
        })*/
    }

    pub fn lookup_local_ref(&self, local_ref: &LocalObjectIndexRef) -> Option<LocalObject> {
        match local_ref {
            LocalObjectIndexRef::Null => Some(LocalObject::Null),
            LocalObjectIndexRef::Export(idx) => {
                self.lookup_export_by_idx(*idx)
                    .map(LocalObject::Export)
            },
            LocalObjectIndexRef::Import(idx) => {
                self.lookup_import(*idx)
                    .map(LocalObject::Import)
            }
        }
    }

    pub async fn read_object_data(&self, export: &Export) -> UPKResult<Vec<u8>> {
        let mut buffer = vec![0; export.data_size];
        
        let mut file_writer = self.file.write().await;
        file_writer.seek(std::io::SeekFrom::Start(export.data_offset as u64)).await?;
        file_writer.read_exact(&mut buffer).await?;
        
        Ok(buffer)
    }
}

impl Export {
    pub fn self_ref(&self) -> LocalObjectIndexRef {
        self.self_ref.clone()
    }

    pub fn name(&self) -> &str {
        if let Some(name) = self.name_count.as_ref() {
            name
        } else {
            &self.name
        }
    }

    pub fn fname(&self) -> &FName {
        &self.name
    }

    pub fn class_ref(&self) -> LocalObjectIndexRef {
        self.obj_type_ref.clone()
    }

    pub fn owner_ref(&self) -> LocalObjectIndexRef {
        self.owner_ref.clone()
    }

    pub fn additional_fields(&self) -> &[u32] {
        &self.additional_fields
    }

    pub fn flags(&self) -> u64 {
        self.flags
    }
}

impl Import {
    pub fn package_name(&self) -> &str {
        &self.package
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn owner(&self) -> &LocalObjectIndexRef {
        &self.owner_ref
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }
}




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

use std::{any::Any, cell::RefCell, ops::Deref, sync::Arc};
use std::fmt::Debug;
use bitflags::bitflags;

use crate::{types::Intrinsic, ExportRef, FName, PackageFile, CLASS};

enum ObjectSource {
    Export(Arc<PackageFile>, ExportRef),
    Intrinsic(String),
}

bitflags! {
    #[derive(Debug)]
    pub struct Flags: u64 {
        // lo
        const TRANSACTIONAL = 0x0000000100000000;
        const IN_SINGULAR_FUNC = 0x0000000200000000;
        const PUBLIC = 0x0000000400000000;
        const PRIVATE = 0x0000008000000000;
        const AUTOMATED = 0x0000010000000000;
        const PROTECTED = 0x0000080000000000;
        const TRANSIENT = 0x0000400000000000;
        const LOAD_FOR_CLIENT = 0x0001000000000000;
        const LOAD_FOR_SERVER = 0x0002000000000000;
        const LOAD_FOR_EDIT = 0x0004000000000000;
        const STANDALONE = 0x0008000000000000;
        const NOT_FOR_CLIENT = 0x0010000000000000;
        const NOT_FOR_SERVER = 0x0020000000000000;
        const NOT_FOR_EDIT = 0x0040000000000000;
        const HAS_STACK = 0x0200000000000000;
        const NATIVE = 0x0400000000000000;
        const MARKED = 0x0800000000000000;

        // hi
        const OBSOLETE = 0x00000020;
        const FINAL = 0x00000080;
        const PER_OBJECT_LOCALIZED = 0x00000100;
        const PROPERTIES_OBJECT =  0x00000200;
        const ARCHETYPE_OBJECT = 0x00000400;
        const REMAPPED_NAME = 0x00000800;
    }
}

pub struct Object {
    source: ObjectSource,
    fqn: String,
    parent: Option<ObjectRef>,
    class: Option<ObjectRef>,
    children: RefCell<Vec<ObjectRef>>,
    object_data: RefCell<Option<Box<dyn Any>>>,
    flags: Flags,
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

impl Object {
    pub fn new(file: Arc<PackageFile>, export: ExportRef, fqn: String, class: ObjectRef, parent: Option<ObjectRef>) -> Self {
        Self {
            source: ObjectSource::Export(file.clone(), export.clone()),
            fqn,
            parent,
            class: Some(class),
            children: RefCell::new(Vec::new()),
            object_data: RefCell::new(None),
            flags: Flags::from_bits_retain(export.flags())
        }
    }

    pub fn new_class(file: Arc<PackageFile>, export: ExportRef) -> Self {
        Self {
            source: ObjectSource::Export(file.clone(), export.clone()),
            fqn: export.name().to_owned(), //format!("Core/{}", export.name()),
            parent: None,
            class: None,
            children: RefCell::new(Vec::new()),
            object_data: RefCell::new(None),
            flags: Flags::empty(),
        }
    }

    pub fn new_intrinsic<T: Send + Sync + Any + 'static>(name: &str, fqn: &str, class: ObjectRef, parent: Option<ObjectRef>, data: T) -> Self {
        Self {
            source: ObjectSource::Intrinsic(name.to_owned()),
            fqn: fqn.to_owned(),
            parent,
            class: Some(class),
            children: RefCell::new(Vec::new()),
            object_data: RefCell::new(Some(Box::new(data))),
            flags: Flags::empty(),
        }
    }

    pub fn new_intrinsic_class(name: &str, intrinsic: Intrinsic) -> Self {
        Self {
            source: ObjectSource::Intrinsic(name.to_owned()),
            fqn: name.to_owned(), //Self::build_fqn(None, None, name, None),
            parent: None,
            class: None,
            children: RefCell::new(Vec::new()),
            object_data: RefCell::new(Some(Box::new(intrinsic))),
            flags: Flags::empty(),
        }
    }

    pub fn package(&self) -> Option<Arc<PackageFile>> {
        if let ObjectSource::Export(file, _) = &self.source {
            Some(file.to_owned())
        } else {
            None
        }
    }

    pub fn parent(&self) -> Option<ObjectRef> {
        self.parent.clone()
    }

    pub fn name(&self) -> &str {
        match &self.source {
            ObjectSource::Export(_, export) => export.name(),
            ObjectSource::Intrinsic(name) => name,
        }
    }

    pub(crate) fn fname(&self) -> &FName {
        match &self.source {
            ObjectSource::Export(_, export) => export.fname(),
            ObjectSource::Intrinsic(_) => panic!("itrinsics don't have fnames!"),
        }
    }

    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }

    pub fn class(&self) -> &ObjectRef {
        self.class.as_ref()
            .unwrap_or(&CLASS)
    }

    pub fn is_class(&self) -> bool {
        self.class.is_none()
    }

    pub fn children(&self) -> &[ObjectRef] {
        unsafe { (*self.children.as_ptr()).as_slice() }
    }

    pub(crate) fn append_child(&self, obj: ObjectRef) {
        self.children.borrow_mut().push(obj)
    }

    pub fn into_ref(self) -> ObjectRef {
        ObjectRef(Arc::new(self))
    }

    pub(crate) fn export(&self) -> Option<&ExportRef> {
        if let ObjectSource::Export(_, export) = &self.source {
            Some(export)
        } else {
            None
        }
    }

    pub fn has_data(&self) -> bool {
        self.object_data.borrow().is_some()
    }

    pub fn is<T: 'static>(&self) -> bool {
        if let Some(data) = self.object_data.borrow().as_ref() {
            data.is::<T>()
        } else {
            false
        }
    }

    pub fn data<T: 'static>(&self) -> Option<&T> {
        unsafe { (*self.object_data.as_ptr()).as_ref() }
            .and_then(|d| d.downcast_ref())
    }

    pub fn set_data<T: Any + 'static>(&self, data: T) {
        if self.has_data() { panic!("object already constains data") }
        self.object_data.replace(Some(Box::new(data)));
    }

    pub fn flags(&self) -> &Flags {
        &self.flags
    }
}

#[derive(Clone)]
pub struct ObjectRef(Arc<Object>);

unsafe impl Send for ObjectRef {}
unsafe impl Sync for ObjectRef {}

impl Deref for ObjectRef {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for ObjectRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ObjectRef({})", self.fqn))
    }
}

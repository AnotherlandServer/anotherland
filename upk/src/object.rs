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

use std::{any::Any, cell::{Cell, RefCell}, ops::Deref, sync::Arc};

use crate::{types::Class, DeserializeUnrealObject, ExportRef, PackageFile, UPKResult};

enum ObjectSource {
    Export(Arc<PackageFile>, ExportRef),
    Intrinsic(String),
}

pub struct Object {
    source: ObjectSource,
    fqn: String,
    parent: Option<ObjectRef>,
    class: Option<ObjectRef>,
    children: RefCell<Vec<ObjectRef>>,
    deserialized: Option<Box<dyn Any>>,
}

unsafe impl Send for Object {}
unsafe impl Sync for Object {}

impl Object {
    pub fn new(file: Arc<PackageFile>, export: ExportRef, class: ObjectRef, parent: Option<ObjectRef>) -> Self {
        Self {
            source: ObjectSource::Export(file.clone(), export.clone()),
            fqn: Self::build_fqn(Some(file.as_ref()), Some(&class), export.name(), parent.as_ref()),
            parent,
            class: Some(class),
            children: RefCell::new(Vec::new()),
            deserialized: None,
        }
    }

    pub fn new_intrinsic(name: &str, class: ObjectRef, parent: Option<ObjectRef>) -> Self {
        Self {
            source: ObjectSource::Intrinsic(name.to_owned()),
            fqn: Self::build_fqn(None, Some(&class), name, parent.as_ref()),
            parent,
            class: Some(class),
            children: RefCell::new(Vec::new()),
            deserialized: None,
        }
    }

    pub fn new_classless_intrinsic(name: &str, parent: Option<ObjectRef>) -> Self {
        Self { 
            source: ObjectSource::Intrinsic(name.to_owned()), 
            fqn: Self::build_fqn(None, None, name, parent.as_ref()),
            parent: None, 
            class: None, 
            children: RefCell::new(Vec::new()), 
            deserialized: None,
        }
    }

    fn build_fqn(package: Option<&PackageFile>, class: Option<&ObjectRef>, object_name: &str, parent: Option<&ObjectRef>) -> String {
        if let Some(parent) = parent {
            format!("{}/{}", parent.fqn, object_name)
        } else if let Some(class) = class && class.name() == "Class" {
            format!("Core/{}", object_name)
        }else if let Some(package) = package {
            format!("{}/{}", package.name(), object_name)
        } else {
            format!("Core/{}", object_name)
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
            ObjectSource::Intrinsic(name) => &name,
        }
    }

    pub fn fully_qualified_name(&self) -> &str {
        &self.fqn
    }

    pub fn class(&self) -> &Class {
        self.class.as_ref().unwrap().deserialized()
            .expect("object does not have a valid class")
    }

    pub fn children(&self) -> &[ObjectRef] {
        unsafe { (*self.children.as_ptr()).as_slice() }
    }

    pub(crate) fn append_child(&mut self, obj: ObjectRef) {
        self.children.borrow_mut().push(obj)
    }

    pub async fn deserialize<T: DeserializeUnrealObject + Send + Sync + Any + 'static>(&mut self) -> UPKResult<()> {
        if let ObjectSource::Export(file, export) = &self.source {
            let deserialized = file.deserialize_export::<T>(self.class(), export).await?;
            self.deserialized = Some(Box::new(deserialized));

            Ok(())
        } else {
            panic!("Can't deserialize an intrinsic object")
        }

    }

    pub fn deserialized<T: 'static>(&self) -> Option<&T> {
        self.deserialized.as_ref().and_then(|d| d.downcast_ref())
    }

    pub fn into_ref(self) -> ObjectRef {
        ObjectRef(Arc::new(self))
    }
}

#[derive(Clone)]
pub struct ObjectRef(Arc<Object>);

impl Deref for ObjectRef {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
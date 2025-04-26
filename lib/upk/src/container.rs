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

use std::{collections::{hash_map::Values, HashMap, VecDeque}, path::{Path, PathBuf}, sync::Arc};

use futures::{future::BoxFuture, Future, FutureExt};
use async_trait::async_trait;
use log::debug;
use once_cell::sync::Lazy;

use crate::{types::{Intrinsic, ScriptClass, StructProperty}, LocalObject, LocalObjectIndexRef, Object, ObjectRef, PackageFile, UPKResult};

#[async_trait]
pub trait DeserializeUnrealObject: Sized + Send + Sync {
    async fn deserialize(object: &ObjectRef, container: &Container, data: &[u8]) -> UPKResult<Self>;
}

pub struct Container {
    base: PathBuf,
    packages: HashMap<String, Arc<PackageFile>>,
    objects: HashMap<String, ObjectRef>,
}

pub(crate) static CLASS: Lazy<ObjectRef> = Lazy::<ObjectRef>::new(|| Object::new_intrinsic_class("Class", Intrinsic::Class).into_ref());

impl Container {
    pub fn new(base: impl AsRef<Path>) -> Self {
        if !base.as_ref().is_dir() { 
            panic!("basepath must be a directory!"); 
        }

        let mut intrinsic_objects = HashMap::new();
        
        intrinsic_objects.insert("Class".to_owned(), CLASS.clone());
        intrinsic_objects.insert("Package".to_string(), Object::new_intrinsic_class("Package", Intrinsic::Package).into_ref());
        intrinsic_objects.insert("ArrayProperty".to_owned(), Object::new_intrinsic_class("ArrayProperty", Intrinsic::ArrayProperty).into_ref());
        intrinsic_objects.insert("BoolProperty".to_owned(), Object::new_intrinsic_class("BoolProperty", Intrinsic::BoolProperty).into_ref());
        intrinsic_objects.insert("ByteProperty".to_owned(), Object::new_intrinsic_class("ByteProperty", Intrinsic::ByteProperty).into_ref());
        intrinsic_objects.insert("ClassProperty".to_owned(), Object::new_intrinsic_class("ClassProperty", Intrinsic::ClassProperty).into_ref());
        intrinsic_objects.insert("ComponentProperty".to_owned(), Object::new_intrinsic_class("ComponentProperty", Intrinsic::ComponentProperty).into_ref());
        intrinsic_objects.insert("Const".to_owned(), Object::new_intrinsic_class("Const", Intrinsic::Const).into_ref());
        intrinsic_objects.insert("DelegateProperty".to_owned(), Object::new_intrinsic_class("DelegateProperty", Intrinsic::DelegateProperty).into_ref());
        intrinsic_objects.insert("Enum".to_owned(), Object::new_intrinsic_class("Enum", Intrinsic::Enum).into_ref());
        intrinsic_objects.insert("FloatProperty".to_owned(), Object::new_intrinsic_class("FloatProperty", Intrinsic::FloatProperty).into_ref());
        intrinsic_objects.insert("Function".to_owned(), Object::new_intrinsic_class("Function", Intrinsic::Function).into_ref());
        intrinsic_objects.insert("InterfaceProperty".to_owned(), Object::new_intrinsic_class("InterfaceProperty", Intrinsic::InterfaceProperty).into_ref());
        intrinsic_objects.insert("IntProperty".to_owned(), Object::new_intrinsic_class("IntProperty", Intrinsic::IntProperty).into_ref());
        intrinsic_objects.insert("MapProperty".to_owned(), Object::new_intrinsic_class("MapProperty", Intrinsic::ArrayProperty).into_ref());
        intrinsic_objects.insert("MetaData".to_owned(), Object::new_intrinsic_class("MetaData", Intrinsic::MapProperty).into_ref());
        intrinsic_objects.insert("Model".to_owned(), Object::new_intrinsic_class("Model", Intrinsic::Model).into_ref());
        intrinsic_objects.insert("NameProperty".to_owned(), Object::new_intrinsic_class("NameProperty", Intrinsic::NameProperty).into_ref());
        intrinsic_objects.insert("ObjectProperty".to_owned(), Object::new_intrinsic_class("ObjectProperty", Intrinsic::ObjectProperty).into_ref());
        intrinsic_objects.insert("ScriptStruct".to_owned(), Object::new_intrinsic_class("ScriptStruct", Intrinsic::ScriptStruct).into_ref());
        intrinsic_objects.insert("State".to_owned(), Object::new_intrinsic_class("State", Intrinsic::State).into_ref());
        intrinsic_objects.insert("StrProperty".to_owned(), Object::new_intrinsic_class("StrProperty", Intrinsic::StrProperty).into_ref());
        intrinsic_objects.insert("StructProperty".to_owned(), Object::new_intrinsic_class("StructProperty", Intrinsic::StructProperty).into_ref());
        intrinsic_objects.insert("ShaderCache".to_owned(), Object::new_intrinsic_class("ShaderCache", Intrinsic::ShaderCache).into_ref());
        intrinsic_objects.insert("StaticMesh".to_owned(), Object::new_intrinsic_class("StaticMesh", Intrinsic::StaticMesh).into_ref());
        intrinsic_objects.insert("FracturedStaticMesh".to_owned(), Object::new_intrinsic_class("FracturedStaticMesh", Intrinsic::FracturedStaticMesh).into_ref());
        intrinsic_objects.insert("Level".to_owned(), Object::new_intrinsic_class("Level", Intrinsic::Level).into_ref());
        intrinsic_objects.insert("LightMapTexture2D".to_owned(), Object::new_intrinsic_class("LightMapTexture2D", Intrinsic::LightMapTexture2D).into_ref());
        intrinsic_objects.insert("Polys".to_owned(), Object::new_intrinsic_class("Polys", Intrinsic::Polys).into_ref());
        intrinsic_objects.insert("World".to_owned(), Object::new_intrinsic_class("World", Intrinsic::World).into_ref());
        intrinsic_objects.insert("ShadowMap1D".to_owned(), Object::new_intrinsic_class("ShadowMap1D", Intrinsic::ShadowMap1D).into_ref());

        intrinsic_objects.insert("Core".to_string(), Object::new_intrinsic("Core", "Core", intrinsic_objects.get("Package").cloned().unwrap(), None, ()).into_ref());

        Self {
            base: base.as_ref().to_path_buf(),
            packages: HashMap::new(),
            objects: intrinsic_objects,
        }
    }

    pub fn mount_package<'a>(&'a mut self, name: &'a str) -> BoxFuture<'a, UPKResult<()>> {
        async move {
            if !self.packages.contains_key(name) {
                debug!("*** Loading package: {name}");

                // open file
                let file = Arc::new(PackageFile::open(
                    self.base.join(format!("{name}.upk"))
                ).await?);

                // register reference to package
                self.packages.insert(name.to_owned(), file.clone());

                // go trough imports and load imported packages
                for import in file.iter_imports() {
                    self.mount_package(import.package_name()).await?;
                }

                // add exports to object map
                debug!("*** Processing package: {name}");

                // fo a first pass to instantiate all the classes
                for export in  file.iter_exports().filter(|export| matches!(export.class_ref(), LocalObjectIndexRef::Null)) {
                    if !matches!(export.owner_ref(), LocalObjectIndexRef::Null) {
                        panic!("Classes can't be owned by objects");
                    }

                    let class = Object::new_class(
                        file.clone(), 
                        export.clone(),
                    ).into_ref();

                    if self.objects.contains_key(class.fully_qualified_name()) {
                        panic!("Duplicated class entry: {}", class.fully_qualified_name());
                    } else {
                        self.objects.insert(class.fully_qualified_name().to_owned(), class);
                    }
                }

                // iterate trough the backlog until all references are resolved
                let mut object_backlog: VecDeque<_> = file
                    .iter_exports()
                    .filter(|export| !matches!(export.class_ref(), LocalObjectIndexRef::Null))
                    .cloned()
                    .collect();

                while let Some(export) = object_backlog.pop_front() {
                    if let Some(class) = self.resolve_class(&file, &export.class_ref()) {
                        let parent_idx = export.owner_ref();
                        let parent;

                        if !parent_idx.is_null() {
                            parent = self.resolve_object(file.clone(), parent_idx.clone());
                            if parent.is_none() {
                                object_backlog.push_back(export);
                                continue;
                            }
                        } else {
                            parent = None;
                        }

                        
                        let object = Object::new(
                            file.clone(), 
                            export.clone(), 
                            self.build_object_fqn(file.clone(), export.self_ref()).unwrap(),
                            class, 
                            parent.clone()
                        ).into_ref();

                        if let Some(parent) = parent.as_ref() {
                            parent.append_child(object.clone());
                        }

                        if self.objects.contains_key(object.fully_qualified_name()) {
                            debug!("Duplicated object entry: {:?}:{}", export.self_ref(), object.fully_qualified_name());
                        } else {
                            self.objects.insert(object.fully_qualified_name().to_owned(), object.clone());
                        }
                    } else {
                        panic!("Failed to resolve class: {:?}", self.build_object_fqn(file.clone(), export.class_ref()))
                    }
                }

                // go trough all classes and deserialize the script class definition
                for class in self.objects.values().filter(|obj| (obj.is_class() || obj.class().name() == "ScriptStruct") && !obj.has_data() ) {                   
                    let script_class = self.deserialize::<ScriptClass>(class).await?;
                    class.set_data(script_class);
                }

                for class in self.objects.values().filter(|obj| (obj.class().name() == "StructProperty") && !obj.has_data() ) {
                    let property = self.deserialize::<StructProperty>(class).await?;
                    class.set_data(property);
                }

                Ok(())
            } else {
                Ok(())
            }
        }.boxed()
    }

    pub fn build_object_fqn(&self, package: Arc<PackageFile>, original_ref: LocalObjectIndexRef) -> Option<String> {
        let mut segments = Vec::new();
        let mut local_ref = original_ref.clone();

        while let Some(object_ref) = package.lookup_local_ref(&local_ref) {
            match &object_ref {
                LocalObject::Null => break,
                LocalObject::Export(obj_export) => {                   
                    segments.push(obj_export.name().to_owned()); 
                    local_ref = obj_export.owner_ref();
                },
                LocalObject::Import(obj_import) => {
                    segments.push(obj_import.name().to_owned());
                    if obj_import.class_name() == "Class" {
                        break;
                    } else {
                        local_ref = obj_import.owner().clone();
                    }
                }
            }
        }
        
        segments.reverse();
        Some(segments.join("/"))
    }

    pub(crate) fn resolve_object(&self, package: Arc<PackageFile>, local_ref: LocalObjectIndexRef) -> Option<ObjectRef> {
        let fqn = self.build_object_fqn(package, local_ref)?;
        self.objects.get(&fqn).cloned()
    }

    pub(crate) fn resolve_class(&self, package: &Arc<PackageFile>, local_ref: &LocalObjectIndexRef) -> Option<ObjectRef> {
        match local_ref {
            LocalObjectIndexRef::Null => Some(CLASS.clone()),
            LocalObjectIndexRef::Export(idx) => {
                let export = package.lookup_export_by_idx(*idx).unwrap();

                self.objects.get(export.name()).cloned()
            },
            LocalObjectIndexRef::Import(idx) => {
                let import = package.lookup_import(*idx).unwrap();
                self.objects.get(import.name()).cloned()
            }
        }
    }

    pub fn umount_package(&mut self, name: &str) {
        self.objects = self.objects.drain().filter(|(_, o)| {
            if let Some(pkg) = o.package() {
                pkg.name() != name
            } else {
                true
            }
        })
        .collect();
    }

    pub fn lookup_object(&self, name: &str) -> Option<&ObjectRef> {
        self.objects.get(name)
    }

    pub async fn read_raw_object(&self, object: &ObjectRef) -> UPKResult<Vec<u8>> {
        let package = object.package().expect("Triet to deserialize intrinsic");
        package.read_object_data(object.export().unwrap()).await
    }

    pub fn deserialize<'a, T: DeserializeUnrealObject + 'a>(&'a self, object: &'a ObjectRef) -> impl Future<Output = UPKResult<T>> + Send + 'a {
        async move {
            let package = object.package().expect("Triet to deserialize intrinsic");
            let data = package.read_object_data(object.export().unwrap()).await?;
            
            T::deserialize(object, self, &data).await
        }.boxed()
    }

    pub fn objects(&self) -> Values<String, ObjectRef> {
        self.objects.values()
    }
}

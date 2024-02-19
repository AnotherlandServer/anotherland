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

use std::{collections::{HashMap, VecDeque}, path::{Path, PathBuf}, pin::Pin, sync::Arc};

use futures::{future::BoxFuture, FutureExt};
use log::debug;

use crate::{object, ExportRef, FName, LocalObjectIndexRef, Object, ObjectImportRef, ObjectRef, PackageFile, UPKResult};

pub struct Container {
    base: PathBuf,
    packages: HashMap<String, Arc<PackageFile>>,
    objects: HashMap<String, ObjectRef>,
}

impl Container {
    pub fn new(base: impl AsRef<Path>) -> Self {
        if !base.as_ref().is_dir() { 
            panic!("basepath must be a directory!"); 
        }

        let mut intrinsic_objects = HashMap::new();
        
        let package_class_intrinsic = Object::new_classless_intrinsic("Package", None).into_ref();
        let core_package = Object::new_intrinsic("Core", package_class_intrinsic.clone(), None).into_ref();
        let class_intrinsic = Object::new_classless_intrinsic("Class", Some(core_package.clone())).into_ref();

        intrinsic_objects.insert("Core".to_string(), core_package.clone());

        intrinsic_objects.insert("Core/Class".to_owned(), class_intrinsic.clone());
        intrinsic_objects.insert("Core/Package".to_string(), package_class_intrinsic);
        intrinsic_objects.insert("Core/ArrayProperty".to_owned(), Object::new_intrinsic("ArrayProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/BoolProperty".to_owned(), Object::new_intrinsic("BoolProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ByteProperty".to_owned(), Object::new_intrinsic("ByteProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ClassProperty".to_owned(), Object::new_intrinsic("ClassProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ComponentProperty".to_owned(), Object::new_intrinsic("ComponentProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Const".to_owned(), Object::new_intrinsic("Const", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/DelegateProperty".to_owned(), Object::new_intrinsic("DelegateProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Enum".to_owned(), Object::new_intrinsic("Enum", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/FloatProperty".to_owned(), Object::new_intrinsic("FloatProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Function".to_owned(), Object::new_intrinsic("Function", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/InterfaceProperty".to_owned(), Object::new_intrinsic("InterfaceProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/IntProperty".to_owned(), Object::new_intrinsic("IntProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/MapProperty".to_owned(), Object::new_intrinsic("MapProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/MetaData".to_owned(), Object::new_intrinsic("MetaData", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Model".to_owned(), Object::new_intrinsic("Model", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/NameProperty".to_owned(), Object::new_intrinsic("NameProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ObjectProperty".to_owned(), Object::new_intrinsic("ObjectProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ScriptStruct".to_owned(), Object::new_intrinsic("ScriptStruct", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/State".to_owned(), Object::new_intrinsic("State", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/StrProperty".to_owned(), Object::new_intrinsic("StrProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/StructProperty".to_owned(), Object::new_intrinsic("StructProperty", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/ShaderCache".to_owned(), Object::new_intrinsic("ShaderCache", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/StaticMesh".to_owned(), Object::new_intrinsic("StaticMesh", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/FracturedStaticMesh".to_owned(), Object::new_intrinsic("FracturedStaticMesh", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Level".to_owned(), Object::new_intrinsic("Level", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/LightMapTexture2D".to_owned(), Object::new_intrinsic("LightMapTexture2D", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/Polys".to_owned(), Object::new_intrinsic("Polys", class_intrinsic.clone(), Some(core_package.clone())).into_ref());
        intrinsic_objects.insert("Core/World".to_owned(), Object::new_intrinsic("World", class_intrinsic.clone(), Some(core_package.clone())).into_ref());

        Self {
            base: base.as_ref().to_path_buf(),
            packages: HashMap::new(),
            objects: intrinsic_objects,
        }
    }

    pub fn mount_package<'a>(&'a mut self, name: &'a str) -> BoxFuture<'a, UPKResult<()>> {
        async move {
            if !self.packages.contains_key(name) {
                println!("Loading package: {}", name);

                // open file
                let file = Arc::new(PackageFile::open(
                    self.base.join(format!("{}.upk", name))
                ).await?);

                // register reference to package
                self.packages.insert(name.to_owned(), file.clone());

                // go trough imports and load imported packages
                for import in file.iter_imports() {
                    self.mount_package(import.package_name()).await?;
                }

                // add exports to object map
                println!("Processing package: {}", name);

                // fo a first pass to instantiate all the classes
                for export in  file.iter_exports().filter(|export| matches!(export.class_ref(), LocalObjectIndexRef::Null)) {
                    if !matches!(export.owner_ref(), LocalObjectIndexRef::Null) {
                        panic!("Classes can't be owned by objects");
                    }

                    let class = Object::new(
                        file.clone(), 
                        export.clone(), 
                        self.objects.get("Core/Class").cloned().unwrap(), 
                        None
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
                    if let Some(class) = self.resolve_object(file.clone(), export.class_ref()) {
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
                            export, 
                            class, 
                            parent
                        ).into_ref();
                        
                        if self.objects.contains_key(object.fully_qualified_name()) {
                            panic!("Duplicated object entry: {}", object.fully_qualified_name());
                        } else {
                            self.objects.insert(object.fully_qualified_name().to_owned(), object);
                        }
                    } else {
                        panic!("Failed to resolve class: {:?}", self.build_object_fqn(file.clone(), export.class_ref()))
                    }
                }

                Ok(())
            } else {
                Ok(())
            }
        }.boxed()
    }

    fn build_object_fqn(&self, mut package: Arc<PackageFile>, mut local_ref: LocalObjectIndexRef) -> Option<String> {
        let mut segments = Vec::new();
        let mut last_class: Option<ObjectRef> = None;

        loop {
            match local_ref {
                LocalObjectIndexRef::Null => {
                    if let Some(class) = last_class {
                        if class.name() == "Class" {
                            segments.push("Core".to_owned());
                        } else {
                            segments.push(package.name().to_owned());
                        }
                    }
                    break;
                },
                LocalObjectIndexRef::Export(idx) => {
                    let export = package.lookup_export_by_idx(idx).unwrap();
                    last_class = self.resolve_class(&package, &export.class_ref());

                    if last_class.as_ref()?.name() == "Class" {
                        segments.push(export.name().to_owned());
                        segments.push("Core".to_owned());
                        break;
                    }
                    
                    segments.push(export.name().to_owned());
                    local_ref = export.owner_ref();
                },
                LocalObjectIndexRef::Import(idx) => {
                    let import = package.lookup_import(idx).unwrap();
                    segments.push(import.name().to_owned());

                    // follow import, but make exception for the Core package, as intrinsicts cannot be
                    // resolved via the export table ot the Core package.
                    if import.package() == "Core" {
                        segments.push(import.package().to_owned());
                        break;
                    }

                    package = self.packages.get(import.package()).cloned().unwrap();
                    let export = package.lookup_export_by_name(import.name()).unwrap();

                    last_class = self.resolve_class(&package, &export.class_ref());
                    local_ref = export.owner_ref();

                    if last_class.as_ref()?.name() == "Class" {
                        segments.push(export.name().to_owned());
                        segments.push("Core".to_owned());
                        break;
                    }
                }
            }
        };
        
        segments.reverse();
        Some(segments.join("/"))
    }

    fn resolve_object(&self, package: Arc<PackageFile>, local_ref: LocalObjectIndexRef) -> Option<ObjectRef> {
        let fqn = self.build_object_fqn(package, local_ref)?;
        self.objects.get(&fqn).cloned()
    }

    fn resolve_class(&self, package: &Arc<PackageFile>, local_ref: &LocalObjectIndexRef) -> Option<ObjectRef> {
        match local_ref {
            LocalObjectIndexRef::Null => self.objects.get("Core/Class").cloned(),
            LocalObjectIndexRef::Export(idx) => {
                let export = package.lookup_export_by_idx(*idx).unwrap();

                self.objects.get(&format!("Core/{}", export.name())).cloned()
            },
            LocalObjectIndexRef::Import(idx) => {
                let import = package.lookup_import(*idx).unwrap();
                self.objects.get(&format!("Core/{}", import.name())).cloned()
            }
        }
    }

    pub fn umount_package(&self, name: &str) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::Container;

    use super::PackageFile;

    #[tokio::test]
    async fn test_package_container() {
        let mut container: Container = Container::new("D:\\Otherland-Next\\Client\\UnrealEngine3\\AmunGame\\CookedPCConsole");
        container.mount_package("LMPlatform_P").await.expect("mounting failed");

        /*let file = PackageFile::open("D:\\Otherland-Next\\Client\\UnrealEngine3\\AmunGame\\CookedPCConsole\\LMPlatform_P.upk").await
            .expect("upk parse error");

        /*for object in file.iter_exports() {
            if object.name().contains("OLFlightTubeActor") {
                let data = file.read_export_raw(object.as_ref()).await.expect("object data");
                println!("{}: {:?}", object.name(), data);
            }
        }*/

        for import in file.iter_imports() {
            println!("{} {} {} {:?}", import.package, import.class_name, import.name, import.owner_ref)
        }*/
    }
}
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

use std::{collections::{hash_map::Entry, HashMap, VecDeque}, path::{Path, PathBuf}, sync::{Arc, Mutex}};

use futures::{future::BoxFuture, Future, FutureExt};
use async_trait::async_trait;
use log::{debug, warn};
use once_cell::sync::Lazy;

use crate::{types::{Intrinsic, ScriptClass, StructProperty}, LocalObject, LocalObjectIndexRef, Object, ObjectBuilder, ObjectRef, PackageFile, UPKResult};

#[async_trait]
pub trait DeserializeUnrealObject: Sized + Send + Sync {
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)>;
}

pub struct Container {
    base: PathBuf,
    packages: HashMap<String, Arc<PackageFile>>,
    objects: Arc<Mutex<HashMap<String, ObjectRef>>>,
}

pub(crate) static CLASS: Lazy<ObjectRef> = Lazy::<ObjectRef>::new(|| Object::new_intrinsic_class("Class", Intrinsic::Class).into_ref());

impl Container {
    pub fn new(base: impl AsRef<Path>) -> Self {
        if !base.as_ref().is_dir() { 
            panic!("basepath must be a directory!"); 
        }

        let container = Self {
            base: base.as_ref().to_path_buf(),
            packages: HashMap::new(),
            objects: Arc::new(Mutex::new(HashMap::new())),
        };

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Class")
                .with_class_ref(CLASS.clone())
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Package")
                .with_data(Intrinsic::Package)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ArrayProperty")
                .with_data(Intrinsic::ArrayProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("BoolProperty")
                .with_data(Intrinsic::BoolProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ByteProperty")
                .with_data(Intrinsic::ByteProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ClassProperty")
                .with_data(Intrinsic::ClassProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ComponentProperty")
                .with_data(Intrinsic::ComponentProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("FloatProperty")
                .with_data(Intrinsic::FloatProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Const")
                .with_data(Intrinsic::Const)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("DelegateProperty")
                .with_data(Intrinsic::DelegateProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Enum")
                .with_data(Intrinsic::Enum)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Function")
                .with_data(Intrinsic::Function)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("InterfaceProperty")
                .with_data(Intrinsic::InterfaceProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("IntProperty")
                .with_data(Intrinsic::IntProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("MapProperty")
                .with_data(Intrinsic::MapProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("MetaData")
                .with_data(Intrinsic::MetaData)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Model")
                .with_data(Intrinsic::Model)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("NameProperty")
                .with_data(Intrinsic::NameProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ObjectProperty")
                .with_data(Intrinsic::ObjectProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ScriptStruct")
                .with_data(Intrinsic::ScriptStruct)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("State")
                .with_data(Intrinsic::State)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StrProperty")
                .with_data(Intrinsic::StrProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StructProperty")
                .with_data(Intrinsic::StructProperty)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ShaderCache")
                .with_data(Intrinsic::ShaderCache)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("FracturedStaticMesh")
                .with_data(Intrinsic::FracturedStaticMesh)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Level")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LightmapTotalSize")
                        .with_class("FloatProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("ShadowmapTotalSize")
                        .with_class("FloatProperty")
                        .build()
                )
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("LightMapTexture2D")
                .with_data(Intrinsic::LightMapTexture2D)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Polys")
                .with_data(Intrinsic::Polys)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("World")
                .with_data(Intrinsic::World)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ShadowMap1D")
                .with_data(Intrinsic::ShadowMap1D)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Plane")
                .with_data(Intrinsic::Plane)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Matrix")
                .with_data(Intrinsic::Matrix)
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("Core")
                .with_class("Package")
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StaticMeshPathEngineInfo")
                .with_class("ScriptStruct")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseConvexHulls")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseSimplifiedMesh")
                        .with_class("BoolProperty")
                        .build()
                )
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StaticMeshLODElement")
                .with_class("ScriptStruct")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("Material")
                        .with_class("ObjectProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bEnableShadowCasting")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bEnableCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StaticMeshLODInfo")
                .with_class("ScriptStruct")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("Elements")
                        .with_class("ArrayProperty")
                        .add_child(
                            ObjectBuilder::new(&container)
                                .with_name("StructProperty1")
                                .with_class("StructProperty")
                                .build()
                        )
                        .build()
                )
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("StaticMesh")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseSimpleLineCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseSimpleBoxCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseSimpleRigidBodyCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseFullPrecisionUVs")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bUsedForInstancing")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bUseMaximumStreamingTexelRatio")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bPartitionForEdgeGeometry")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bCanBecomeDynamic")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("bStripComplexCollisionForConsole")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LightMapResolution")
                        .with_class("IntProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LightMapCoordinateIndex")
                        .with_class("IntProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LODDistanceRatio")
                        .with_class("FloatProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LODMaxRange")
                        .with_class("FloatProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("StreamingDistanceMultiplier")
                        .with_class("FloatProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("PSSMDepthBiasOverride")
                        .with_class("FloatProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("StaticMeshPathEngineInfo")
                        .with_class("ScriptStruct")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseCollision")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseConvexHulls")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("UseSimplifiedMesh")
                        .with_class("BoolProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("PathEngine")
                        .with_class("StructProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("LODInfo")
                        .with_class("StructProperty")
                        .add_child(
                            ObjectBuilder::new(&container)
                                .with_name("StructProperty0")
                                .with_class("StructProperty")
                                .build()
                        )
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("BodySetup")
                        .with_class("ObjectProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("FoliageDefaultSettings")
                        .with_class("ObjectProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("SourceFilePath")
                        .with_class("StrProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("SourceFileTimestamp")
                        .with_class("StrProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("Sockets")
                        .with_class("ArrayProperty")
                        .add_child(
                            ObjectBuilder::new(&container)
                                .with_name("SocketsProperty0")
                                .with_class("ObjectProperty")
                                .build()
                        )
                        .build()
                )
                .build()
        );

        container.add_object(
            ObjectBuilder::new(&container)
                .with_name("ColorMaterialInput")
                .with_class("ScriptStruct")
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("Expression")
                        .with_class("ObjectProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("Mask")
                        .with_class("IntProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("MaskR")
                        .with_class("IntProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("MaskG")
                        .with_class("IntProperty")
                        .build()
                )
                .add_child(
                    ObjectBuilder::new(&container)
                        .with_name("MaskB")
                        .with_class("IntProperty")
                        .build()
                )
                .build()
        );

        container
    }

    pub fn add_object(&self, object: ObjectRef) {
        let fqn = object.fully_qualified_name().to_owned();
        let mut objects = self.objects.lock().unwrap();

        match objects.entry(fqn) {
            Entry::Occupied(mut entry) => {
                debug!("Object already exists: {}", entry.key());
                entry.insert(object);
            },
            Entry::Vacant(entry) => {
                debug!("Adding object: {}", entry.key());
                entry.insert(object);
            },
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

                    self.add_object(class);
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

                        self.add_object(object);
                    } else {
                        panic!("Failed to resolve class: {:?}", self.build_object_fqn(file.clone(), export.class_ref()))
                    }
                }

                let script_structs = {
                    let objects = self.objects.lock().unwrap();

                    objects.values()
                        .filter(|obj| (obj.is_class() || obj.class().name() == "ScriptStruct"))
                        .cloned()
                        .collect::<Vec<_>>()
                };

                // go trough all classes and deserialize the script class definition
                for class in script_structs.iter().filter(|obj| !obj.has_data()) {    
                    let script_class = self.deserialize::<ScriptClass>(class).await?;
                    class.set_data(script_class);
                }

                let struct_properties = {
                    let objects = self.objects.lock().unwrap();
                    objects.values()
                        .filter(|obj| obj.class().name() == "StructProperty")
                        .cloned()
                        .collect::<Vec<_>>()
                };

                for struct_property in struct_properties.iter().filter(|obj| !obj.has_data()) {
                    let property = self.deserialize::<StructProperty>(struct_property).await?;
                    struct_property.set_data(property);
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
        let class = if let Some(object_ref) = package.lookup_local_ref(&local_ref) {
            match &object_ref {
                LocalObject::Null => { return None; },
                LocalObject::Export(obj_export) => {
                    if obj_export.class_ref().is_null() {
                        "Class".to_owned()
                    } else if let Some(class_ref) = self.resolve_class(&package, &obj_export.class_ref()) {
                        class_ref.name().to_owned()
                    } else {
                        warn!("Failed to resolve class for export: {original_ref:?}");
                        return None;
                    }
                },
                LocalObject::Import(obj_import) => {
                    obj_import.class_name().to_owned()
                }
            }
        } else {
            warn!("Failed to resolve local reference: {original_ref:?}");
            return None;
        };

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
        Some(format!("{class}:{}", segments.join("/")))
    }

    pub(crate) fn resolve_object(&self, package: Arc<PackageFile>, local_ref: LocalObjectIndexRef) -> Option<ObjectRef> {
        let fqn = self.build_object_fqn(package, local_ref)?;
        self.objects.lock().unwrap().get(&fqn).cloned()
    }

    pub(crate) fn resolve_class(&self, package: &Arc<PackageFile>, local_ref: &LocalObjectIndexRef) -> Option<ObjectRef> {
        match local_ref {
            LocalObjectIndexRef::Null => Some(CLASS.clone()),
            LocalObjectIndexRef::Export(idx) => {
                let export = package.lookup_export_by_idx(*idx).unwrap();

                self.objects.lock().unwrap().get(&format!("Class:{}", export.name())).cloned()
            },
            LocalObjectIndexRef::Import(idx) => {
                let import = package.lookup_import(*idx).unwrap();
                self.objects.lock().unwrap().get(&format!("Class:{}", import.name())).cloned()
            }
        }
    }

    pub fn umount_package(&mut self, name: &str) {
        let objects = self.objects.clone();

        self.objects = Arc::new(Mutex::new(
            objects.lock().unwrap().drain().filter(|(_, o)| {
                if let Some(pkg) = o.package() {
                    pkg.name() != name
                } else {
                    true
                }
            })
            .collect()
        ));
    }

    pub fn lookup_object(&self, name: &str) -> Option<ObjectRef> {
        self.objects.lock().unwrap().get(name).cloned()
    }

    pub async fn read_raw_object(&self, object: &ObjectRef) -> UPKResult<Vec<u8>> {
        let package = object.package().expect("Triet to deserialize intrinsic");
        package.read_object_data(object.export().unwrap()).await
    }

    pub fn deserialize<'a, T: DeserializeUnrealObject + 'a>(&'a self, object: &'a ObjectRef) -> impl Future<Output = UPKResult<T>> + Send + 'a {
        async move {
            let data = if let Some(package) = object.package() {
                package.read_object_data(object.export().unwrap()).await?
            } else {
                vec![]
            };

            let (_, res) = T::deserialize(object, self, &data).await?;
            Ok(res)
        }.boxed()
    }
}

#[async_trait]
impl DeserializeUnrealObject for ()
{
    async fn deserialize<'a>(_object: &ObjectRef, _container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        Ok((data, ()))
    }
}

#[async_trait]
impl<T1, T2> DeserializeUnrealObject for (T1, T2)
where
    T1: DeserializeUnrealObject + Send + Sync + 'static,
    T2: DeserializeUnrealObject + Send + Sync + 'static,
{
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (data, t1) = T1::deserialize(object, container, data).await?;
        let (data, t2) = T2::deserialize(object, container, data).await?;
        Ok((data, (t1, t2)))
    }
}

#[async_trait]
impl<T1, T2, T3> DeserializeUnrealObject for (T1, T2, T3)
where
    T1: DeserializeUnrealObject + Send + Sync + 'static,
    T2: DeserializeUnrealObject + Send + Sync + 'static,
    T3: DeserializeUnrealObject + Send + Sync + 'static,
{
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (data, t1) = T1::deserialize(object, container, data).await?;
        let (data, t2) = T2::deserialize(object, container, data).await?;
        let (data, t3) = T3::deserialize(object, container, data).await?;
        Ok((data, (t1, t2, t3)))
    }
}

#[async_trait]
impl<T1, T2, T3, T4> DeserializeUnrealObject for (T1, T2, T3, T4)
where
    T1: DeserializeUnrealObject + Send + Sync + 'static,
    T2: DeserializeUnrealObject + Send + Sync + 'static,
    T3: DeserializeUnrealObject + Send + Sync + 'static,
    T4: DeserializeUnrealObject + Send + Sync + 'static,
{
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (data, t1) = T1::deserialize(object, container, data).await?;
        let (data, t2) = T2::deserialize(object, container, data).await?;
        let (data, t3) = T3::deserialize(object, container, data).await?;
        let (data, t4) = T4::deserialize(object, container, data).await?;
        Ok((data, (t1, t2, t3, t4)))
    }
}

#[async_trait]
impl<T1, T2, T3, T4, T5> DeserializeUnrealObject for (T1, T2, T3, T4, T5)
where
    T1: DeserializeUnrealObject + Send + Sync + 'static,
    T2: DeserializeUnrealObject + Send + Sync + 'static,
    T3: DeserializeUnrealObject + Send + Sync + 'static,
    T4: DeserializeUnrealObject + Send + Sync + 'static,
    T5: DeserializeUnrealObject + Send + Sync + 'static,
{
    async fn deserialize<'a>(object: &ObjectRef, container: &Container, data: &'a [u8]) -> UPKResult<(&'a [u8], Self)> {
        let (data, t1) = T1::deserialize(object, container, data).await?;
        let (data, t2) = T2::deserialize(object, container, data).await?;
        let (data, t3) = T3::deserialize(object, container, data).await?;
        let (data, t4) = T4::deserialize(object, container, data).await?;
        let (data, t5) = T5::deserialize(object, container, data).await?;
        Ok((data, (t1, t2, t3, t4, t5)))
    }
}


// Copyright (C) 2023 AnotherlandServer
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

use nom::IResult;
use nom::error::VerboseError;
use nom::error::context;
use nom::multi::count;
use nom::number;
use nom::combinator::fail;
use nom::combinator::success;
use nom::sequence::tuple;
use crate::{Param, ParamError, AnyClass, BoundParamClass, ParamClass, ParamEntity, AvatarId, ParamFlag, generated::cpkt::Uuid};
use bitstream_io::{ByteWriter, LittleEndian, ByteWrite};
use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use serde::de::Deserializer;
use std::convert::{Into, TryInto};
use serde_json::{Value, json};
use glam::f32::Vec3;
use std::collections::{HashSet, HashMap};
use std::ops::Deref;
use std::io;
use log::error;
use log::debug;
use legion::{World, Entity};
use legion::EntityStore;

include!(concat!(env!("OUT_DIR"), "/generated_params.rs"));
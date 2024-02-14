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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Any,
    String,
    Int64,
    Bool,
    AvatarId,
    Uuid,
    LocalizedString,
    ContentRef,
    ContentRefArray,
    Positionable,
    Vector3,
    Vector3Uts,
    Vector4,
    FloatPair,
    IntArray4,
    JsonValue,
    Quarternion,
    Bitset,
    AvatarIdSet,
    UuidSet,
    StringSet,
    Int64Array,
    AvatarIdArray,
    UuidArray,
    StringArray,
    FloatArray,
    IntArray,
    StringMap,
    IntMap,
    Float,
    Int32,
    UuidPair,
    StringPair,
    StringFloatPair,
}
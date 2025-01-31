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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    String,
    StringPair,
    StringFloatPair,
    StringSet,
    Guid,
    GuidPair,
    Bool,
    Int,
    BitField128,
    BitSetFilter,
    Float,
    FloatRange,
    Vector3,
    Vector3Uts,
    Vector4,
    LocalizedString,
    AvatarId,
    UniqueId,
    JsonValue,
    Int64,
    Quarternion,
    Positionable,
    ContentRef,
    ContentRefAndInt,
    ContentRefAndFloat,
    ContentRefList,
    ClassRefPowerRangeList,
    VectorInt,
    VectorInt64,
    VectorFloat,
    VectorString,
    AvatarIdSet,
    VectorAvatarId,
    GuidSet,
    VectorGuid,
    HashmapStringInt,
    HashmapStringString,
    Any,
    VectorLocalizedString,
    InstanceGroup,
}
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

use atlas::{AvatarId, Uuid};
use bevy_ecs::prelude::*;

#[derive(Clone, Debug, Component)]
pub struct AvatarComponent {
    pub id: AvatarId,
    pub instance_id: Option<Uuid>,
    pub record_id: Option<Uuid>,
    pub name: String,
    pub phase_tag: String,
}
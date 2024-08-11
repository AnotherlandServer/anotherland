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

mod portal;
mod npc;

use bevy::app::Plugin;
use npc::subjective_npc;
use portal::subjective_portal;

use super::{plugins::SubjectivityExt, EntityType};

pub struct SubjectiveLensesPlugin;

impl Plugin for SubjectiveLensesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_subjective_params_initializer(EntityType::Portal, subjective_portal);
        app.add_subjective_params_initializer(EntityType::NpcOtherland, subjective_npc);
    }
}
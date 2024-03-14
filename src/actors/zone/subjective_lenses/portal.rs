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

use atlas::{NonClientBaseParams, ParamBox, ParamClass, PortalClass, PortalParams};
use bevy::app::Plugin;
use bevy_ecs::system::{In, Query};

use crate::actors::{zone::plugins::{SubjectivityExt, SubjectivityLensArguments}, EntityType};

pub struct SubjectivePortals;

impl Plugin for SubjectivePortals {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_subjective_lens(EntityType::Portal, portal_lens);
    }
}

fn portal_lens(
    In((_, portal_id)): In<SubjectivityLensArguments>,
    portal: Query<&PortalClass>,
) -> ParamBox {
    let mut portal = portal.get(portal_id).unwrap().clone();
    if portal.tags().map(|tags| tags.contains("PortalHive")).unwrap_or(false) {
        portal.set_current_state(3);
    }

    portal.into_box()
}
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

use atlas::{NonClientBaseParams, ParamBox, ParamClass, PortalClass, PortalComponent, PortalParams};
use bevy::{app::Plugin, prelude::App};
use bevy_ecs::{query::With, system::{In, Query}};

use crate::actors::{zone::plugins::{SubjectivityExt, SubjectivityLensArguments}, EntityType};

pub struct SubjectivePortals;

impl Plugin for SubjectivePortals {
    fn build(&self, app: &mut App) {
        app.add_subjective_lens(EntityType::Portal, portal_lens);
    }
}

fn portal_lens(
    In((_, portal_id)): In<SubjectivityLensArguments>,
    portal: Query<&ParamBox, With<PortalComponent>>,
) -> ParamBox {
    let mut portal = portal
        .get(portal_id)
        .unwrap()
        .clone();

    if let Some(portal) = portal.get_impl_mut::<dyn PortalParams>() {
        if portal.tags().contains("PortalHive") {
            portal.set_current_state(3);
        }
    }

    portal
}
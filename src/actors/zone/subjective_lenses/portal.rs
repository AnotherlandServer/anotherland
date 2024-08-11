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

use atlas::{Param, ParamBox, ParamSet, ParamSetBox, PortalAttribute, PortalComponent, PortalParams};
use bevy_ecs::{query::With, system::{In, Query}};

use crate::actors::zone::plugins::SubjectivityLensArguments;

pub fn subjective_portal(
    In((_, ent)): In<SubjectivityLensArguments>,
    portal: Query<&ParamBox, With<PortalComponent>>,
) -> ParamSetBox {
    let mut set = ParamSet::<PortalAttribute>::new();

    if 
        let Ok(portal) = portal.get(ent) &&
        let Some(portal) = portal.get_impl::<dyn PortalParams>()
    {
        if portal.tags().contains("PortalHive") {
            set.insert(PortalAttribute::CurrentState, Param::Int(3));
        } else {
            set.insert(PortalAttribute::CurrentState, portal.current_state());
        }
    }

    set.into_box()
}
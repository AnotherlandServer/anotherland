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

use atlas::{NpcOtherlandAttribute, NpcOtherlandComponent, NpcOtherlandParams, ParamBox, ParamSet, ParamSetBox};
use bevy_ecs::{query::With, system::{In, Query}};

use crate::actors::zone::plugins::SubjectivityLensArguments;

pub fn subjective_npc(
    In((_, ent)): In<SubjectivityLensArguments>,
    npc: Query<&ParamBox, With<NpcOtherlandComponent>>,
) -> ParamSetBox {
    let mut set = ParamSet::<NpcOtherlandAttribute>::new();

    if 
        let Ok(npc) = npc.get(ent) &&
        let Some(npc) = npc.get_impl::<dyn NpcOtherlandParams>()
    {
        set.insert(NpcOtherlandAttribute::Dialogs, npc.dialogs().to_vec());
        set.insert(NpcOtherlandAttribute::Action0, ("".to_string(), 0.0f32));
        set.insert(NpcOtherlandAttribute::Action0Duration, 0.0);
        set.insert(NpcOtherlandAttribute::Action0Option, 0);
    }

    set.into_box()
}
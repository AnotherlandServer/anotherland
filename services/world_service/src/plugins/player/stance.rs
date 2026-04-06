// Copyright (C) 2026 AnotherlandServer
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

use bevy::ecs::{component::Component, lifecycle::HookContext, query::Changed, system::Query, world::DeferredWorld};
use obj_params::{GameObjectData, Player};
use protocol::oaPlayerClassData;

use crate::{error::WorldResult, plugins::CombatStyle};

#[derive(Component)]
#[component(on_insert = on_stance_insert)]
pub struct Stance {
    stance: Box<dyn ClassStances>,
    rank: u8,
}

impl Stance {
    pub fn new(combat_style: CombatStyle) -> Self {
        let stance: Box<dyn ClassStances> = match combat_style {
            CombatStyle::Assassin => Box::new(Assassin::PrimeKiller),
            CombatStyle::Energizer => Box::new(Energizer::PrimeJuicer),
            CombatStyle::Rage => Box::new(Rage::PrimeChampion),
            CombatStyle::Tech => Box::new(Tech::PrimeShot),
            _ => Box::new(NoneStance),
        };
        Self { stance, rank: 0 }
    }

    pub fn serialize(&self) -> oaPlayerClassData {
        oaPlayerClassData {
            class_hash: 0x9D35021A,
            stance_id: self.stance.get_stance_id(),
            stance_rank: self.rank
        }
    }

    pub fn update_stance(&mut self, stance_id: u8, rank: u8) -> WorldResult<()> {
        self.stance.set_stance_id(stance_id)?;
        self.rank = rank;
        Ok(())
    }
}

fn on_stance_insert(mut world: DeferredWorld, context: HookContext) {
    let stance = world.get::<Stance>(context.entity).unwrap().serialize();
    let mut obj = world.get_mut::<GameObjectData>(context.entity).unwrap();
    obj.set(Player::ClassData, stance.to_bytes());
}

pub fn sync_class_stance(mut query: Query<(&mut GameObjectData, &Stance), Changed<Stance>>) {
    for (mut obj, stance) in query.iter_mut() {
        obj.set(Player::ClassData, stance.serialize().to_bytes());
    }
}

trait ClassStances: Send + Sync {
    fn get_stance_id(&self) -> u8;
    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()>;
}

pub struct NoneStance;

impl ClassStances for NoneStance {
    fn get_stance_id(&self) -> u8 {
        0
    }

    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()> {
        if stance_id != 0 {
            return Err(anyhow::anyhow!("This combat style does not support stances").into());
        }

        Ok(())
    }
}

pub enum Assassin {
    PrimeKiller,
    PrimeDefender
}

impl ClassStances for Assassin {
    fn get_stance_id(&self) -> u8 {
        match self {
            Assassin::PrimeKiller => 0,
            Assassin::PrimeDefender => 1,
        }
    }

    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()> {
        *self = match stance_id {
            0 => Assassin::PrimeKiller,
            1 => Assassin::PrimeDefender,
            _ => return Err(anyhow::anyhow!("Invalid stance id for Assassin: {}", stance_id).into()),
        };
        Ok(())
    }
}

pub enum Energizer {
    PrimeJuicer,
    PrimeSpecialist,
}

impl ClassStances for Energizer {
    fn get_stance_id(&self) -> u8 {
        match self {
            Energizer::PrimeJuicer => 0,
            Energizer::PrimeSpecialist => 1,
        }
    }

    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()> {
        *self = match stance_id {
            0 => Energizer::PrimeJuicer,
            1 => Energizer::PrimeSpecialist,
            _ => return Err(anyhow::anyhow!("Invalid stance id for Energizer: {}", stance_id).into()),
        };
        Ok(())
    }
}

pub enum Rage {
    PrimeChampion,
    PrimeTarget,
}

impl ClassStances for Rage {
    fn get_stance_id(&self) -> u8 {
        match self {
            Rage::PrimeChampion => 0,
            Rage::PrimeTarget => 1,
        }
    }

    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()> {
        *self = match stance_id {
            0 => Rage::PrimeChampion,
            1 => Rage::PrimeTarget,
            _ => return Err(anyhow::anyhow!("Invalid stance id for Rage: {}", stance_id).into()),
        };
        Ok(())
    }
}

pub enum Tech {
    PrimeShot,
    PrimeAdherent,
}

impl ClassStances for Tech {
    fn get_stance_id(&self) -> u8 {
        match self {
            Tech::PrimeShot => 0,
            Tech::PrimeAdherent => 1,
        }
    }

    fn set_stance_id(&mut self, stance_id: u8) -> WorldResult<()> {
        *self = match stance_id {
            0 => Tech::PrimeShot,
            1 => Tech::PrimeAdherent,
            _ => return Err(anyhow::anyhow!("Invalid stance id for Tech: {}", stance_id).into()),
        };
        Ok(())
    }
}
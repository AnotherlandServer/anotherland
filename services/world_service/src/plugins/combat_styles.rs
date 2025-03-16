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

use bevy::{app::{Plugin, PreUpdate}, ecs::{component::Component, entity::Entity, query::Changed, system::Commands}, prelude::{Added, App, Query}};
use obj_params::{tags::PlayerTag, GameObjectData, Player};
use protocol::{HeavyData, HeavyDataCollection, HeavyDataEntry};

pub struct CombatStylesPlugin;

impl Plugin for CombatStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            init_heavy_special_skill_data, 
            init_combat_style,
        ));
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum CombatStyle {
    #[default]
    None,
    Rage,
    Tech,
    Assassin,
    Energizer,
    Hacker,
    Cyber,
}

impl From<CombatStyle> for realm_api::CombatStyle {
    fn from(value: CombatStyle) -> Self {
        match value {
            CombatStyle::None => realm_api::CombatStyle::None,
            CombatStyle::Rage => realm_api::CombatStyle::Rage,
            CombatStyle::Tech => realm_api::CombatStyle::Tech,
            CombatStyle::Assassin => realm_api::CombatStyle::Assassin,
            CombatStyle::Energizer => realm_api::CombatStyle::Energizer,
            CombatStyle::Hacker => realm_api::CombatStyle::Hacker,
            CombatStyle::Cyber => realm_api::CombatStyle::Cyber,
        }
    }
}

impl CombatStyle {
    pub fn id(&self) -> i32 {
        match self {
            CombatStyle::None => 6,
            CombatStyle::Rage => 0,
            CombatStyle::Tech => 1,
            CombatStyle::Assassin => 2,
            CombatStyle::Energizer => 3,
            CombatStyle::Hacker => 4,
            CombatStyle::Cyber => 5,
        }
    }

    pub fn from_id(id: i32) -> Self {
        match id {
            0 => CombatStyle::Rage,
            1 => CombatStyle::Tech,
            2 => CombatStyle::Assassin,
            3 => CombatStyle::Energizer,
            4 => CombatStyle::Hacker,
            5 => CombatStyle::Cyber,
            _ => CombatStyle::None,
        }
    }
}

fn init_heavy_special_skill_data(
    mut query: Query<&mut GameObjectData, Added<PlayerTag>>,
) {
    for mut player in query.iter_mut() {
        player.set(Player::HeavySpecialSkillData, 
            HeavyData {
                class_hash: 0x6206ed68,
                version: 1,
                data: vec![
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Rage_2H_Club_HeavyAbility_DownwardSwing".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Rage_2H_Club_HeavyAbility_UpwardSwing".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Rage_2H_Sword_HeavyAbility_LegSweep".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Rage_2H_Sword_HeavyAbility_Slash".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Rage_2H_PoleArm_HeavyAbility_HalfSpinThrust".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Rage_2H_PoleArm_HeavyAbility_JumpSmash".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_Shotgun_HeavyAbility_BombShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_Shotgun_HeavyAbility_DoubleBarrel".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SniperShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SteelShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_Bow_HeavyAbility_AchillesShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_Bow_HeavyAbility_ShottyShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SniperShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SteelShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SniperShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SteelShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SniperShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Gunner_2H_ShoulderRifle_HeavyAbility_SteelShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Sword_HeavyAbility_JoltSlice".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Sword_HeavyAbility_PiercingFoot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Dagger_HeavyAbility_UpperCutSomerSaultKick".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Dagger_HeavyAbility_WhirlingBlades".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Pole_HeavyAbility_BackSpinThrust".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Assassin_1H_Pole_HeavyAbility_SpinningThrust".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Energizer_2H_Launcher_HeavyAbility_HeadShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Energizer_2H_Launcher_HeavyAbility_HyperShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Energizer_2H_PulseGun_HeavyAbility_LongShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Energizer_2H_PulseGun_HeavyAbility_TrickShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection {
                        entry_count: 2,
                        entries: vec![
                            HeavyDataEntry {
                                field_0: "Energizer_2H_BeamGun_HeavyAbility_DoubleShock".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                            HeavyDataEntry {
                                field_0: "Energizer_2H_BeamGun_HeavyAbility_BurstShot".to_string(),
                                field_1: 1,
                                ..Default::default()
                            },
                        ]
                    },
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                    HeavyDataCollection::default(),
                ],
                ..Default::default()
            }.to_bytes()
        );
    }
}

fn init_combat_style(
    mut query: Query<(Entity, &GameObjectData, Option<&mut CombatStyle>), Changed<PlayerTag>>,
    mut commands: Commands,
) {
    for (ent, player, current_combat_style) in query.iter_mut() {
        let combat_style = CombatStyle::from_id(*player.get::<_, i32>(Player::CombatStyle).unwrap_or(&0));

        if let Some(mut current_combat_style) = current_combat_style {
            *current_combat_style = combat_style;
        } else {
            commands.entity(ent)
                .insert(combat_style);
        }
    }
}

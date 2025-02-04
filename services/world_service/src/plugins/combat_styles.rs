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

use bevy::{app::{Plugin, Update}, prelude::{Added, App, Query, With}};
use log::debug;
use obj_params::{tags::PlayerTag, GameObjectData, Player};
use protocol::{HeavyData, HeavyDataCollection, HeavyDataEntry};

pub struct CombatStylesPlugin;

impl Plugin for CombatStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, init_heavy_special_skill_data);
    }
}

fn init_heavy_special_skill_data(
    mut query: Query<&mut GameObjectData, Added<PlayerTag>>,
) {
    for mut player in query.iter_mut() {
        match player.get::<_, i32>(Player::CombatStyle).unwrap() {
            0 => init_rage_heavy_special_skill_data(player.as_mut()),
            1 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("tech class not implemented yet!")
            },
            2 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("assassin class not implemented yet!")
            },
            3 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("energizer class not implemented yet!")
            },
            4 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("hacker class not implemented yet!")
            },
            5 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("cyber class not implemented yet!")
            },
            6 => {
                init_none_heavy_special_skill_data(player.as_mut());
                debug!("none class not implemented yet!")
            },
            _ => unreachable!()
        }
    }
}

fn init_rage_heavy_special_skill_data(player: &mut GameObjectData) {
    debug!("Init rage heavy special skill data");

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

fn init_none_heavy_special_skill_data(player: &mut GameObjectData) {
    player.set(Player::HeavySpecialSkillData, 
        HeavyData {
            class_hash: 0x6206ed68,
            version: 1,
            data: vec![],
            ..Default::default()
        }.to_bytes()
    );
}
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

use std::str::FromStr;

use bevy::{app::{Plugin, PreUpdate, Update}, ecs::{component::Component, entity::Entity, query::{Changed, Or}, schedule::IntoSystemConfigs, system::Commands}, prelude::{Added, App, Query}};
use obj_params::{tags::PlayerTag, GameObjectData, Player};
use protocol::{ClassSkill, ClassSkills, HeavyData, HeavyDataCollection, HeavyDataEntry};
use toolkit::types::Uuid;

use super::{ConnectionState, CurrentState};

pub struct CombatStylesPlugin;

impl Plugin for CombatStylesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (
            init_heavy_special_skill_data, 
            init_combat_style, 
            init_class_skills.after(init_combat_style)
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

#[allow(clippy::type_complexity)]
fn init_class_skills(
    mut query: Query<(&mut GameObjectData, &CombatStyle, &CurrentState), Or<(Changed<CombatStyle>, Changed<CurrentState>)>>,
) {
    for (mut player, combat_style, state) in query.iter_mut() {
        if state.state < ConnectionState::PlayerReceived {
            continue;
        }

        let mut skills = vec![];
        let level = *player.get::<_, i32>(Player::Lvl).unwrap_or(&1);

        // Class skills are hardcoded for now
        match combat_style {
            CombatStyle::None => {},
            CombatStyle::Rage => {
                if level >= 1 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("fe323341-4292-4830-879a-6e1e3f957493").unwrap(),
                        group: "Recuperate".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("0cb80a4b-4533-4194-a03d-3302b84240c3").unwrap(),
                        group: "PrimeChampionNew".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("ba5fd7b8-4d46-47f9-90d9-18d9bb344e32").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_RawPower".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("ab1f1bb1-a866-4f34-9b04-c952b39ea252").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_BloodStorm".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }

                if level >= 5 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("d1f3e2b1-05cd-4914-af5f-3c362283bbbc").unwrap(),
                        group: "Rage_Class_PrimeGuardian_Rank1".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("015bf264-14fd-494a-b758-be5218ac159b").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Dash".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("598a3005-5a7b-4982-ae69-7ea8bd3b32b8").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Taunt".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }

                if level >= 15 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("b98a8dd5-d50d-42c0-87a9-54813b8656c5").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Rampage".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }
                
                if level >= 25 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("62f1b418-b72a-4bbd-8827-4a4a568e6d57").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Bane".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });

                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("11d52e0b-d053-4fb5-b92d-f2466872fe34").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_VitalitySlash".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }

                if level >= 35 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("f805b67c-4259-4aad-a840-9b3aed9c8f0e").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Banish".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });

                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("d3ea59f7-5101-428a-a25f-988fb7f4482d").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Impulse".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }

                if level >= 45 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("b880c31c-4e07-4913-a36a-ebe99c564341").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_EssenceTouch".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }

                if level >= 55 {
                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("c2a6a790-cc12-4bb0-a2fa-cc18c1b937c3").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_SanguineCurse".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });

                    skills.push(ClassSkill {
                        version: 0,
                        id: Uuid::new(),
                        content_id: Uuid::from_str("65ae9a4f-56e0-42c1-94a8-2cc548119816").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Presage".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    });
                }
            },
            CombatStyle::Tech => {},
            CombatStyle::Assassin => {},
            CombatStyle::Energizer => {},
            CombatStyle::Hacker => {},
            CombatStyle::Cyber => {},
        }

        player.set(Player::CurrentClassSkills,
            ClassSkills {
                class_hash: 0x81E0A735,
                count: skills.len() as u32,
                skills,
            }.to_bytes());
    }
}

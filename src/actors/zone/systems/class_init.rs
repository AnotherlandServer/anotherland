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

use atlas::{ClassSkill, ClassSkills, HeavyData, HeavyDataCollection, HeavyDataEntry, NonClientBaseComponent, ParamBox, PlayerComponent, PlayerParams, Uuid};
use bevy_ecs::{component::Component, entity::Entity, query::{Added, With, Without}, system::{Commands, Query}};

#[derive(Component)]
pub struct CombatStyleNone;

#[derive(Component)]
pub struct CombatStyleRage; // aka warrior

#[derive(Component)]
pub struct CombatStyleTech; // aka marksman

#[derive(Component)]
pub struct CombatStyleAssassin;

#[derive(Component)]
pub struct CombatStyleEnergizer;

#[derive(Component)]
pub struct CombatStyleHacker;

#[derive(Component)]
pub struct CombatStyleCyber;

pub fn setup_combat_style(
    mut players: Query<(Entity, &ParamBox), (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<PlayerComponent>)>,
    mut cmds: Commands,
) {
    for (entity, params) in players.iter_mut() {
        match params.get_impl::<dyn PlayerParams>().unwrap().combat_style() {
            0 => cmds.entity(entity).insert(CombatStyleRage),
            1 => cmds.entity(entity).insert(CombatStyleTech),
            2 => cmds.entity(entity).insert(CombatStyleAssassin),
            3 => cmds.entity(entity).insert(CombatStyleEnergizer),
            4 => cmds.entity(entity).insert(CombatStyleHacker),
            5 => cmds.entity(entity).insert(CombatStyleCyber),
            _ => cmds.entity(entity).insert(CombatStyleNone)
        };
    }
}

pub fn setup_combat_style_none(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleNone>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(ClassSkills {
            class_hash: 0x81E0A735,
            ..Default::default()
        }.to_bytes());
    }
}

pub fn setup_combat_style_rage(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleRage>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(
            ClassSkills {
                class_hash: 0x81E0A735,
                count: 4,
                skills: vec![
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("fe323341-4292-4830-879a-6e1e3f957493").unwrap(),
                        group: "Recuperate".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("0cb80a4b-4533-4194-a03d-3302b84240c3").unwrap(),
                        group: "PrimeChampionNew".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("ba5fd7b8-4d46-47f9-90d9-18d9bb344e32").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_RawPower".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str(   "00000000-1111-2222-3333-000000000003").unwrap(), //Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("ab1f1bb1-a866-4f34-9b04-c952b39ea252").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_BloodStorm".to_string(),
                        //field_4: 1,
                        ..Default::default()
                    },
                    /*ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("b98a8dd5-d50d-42c0-87a9-54813b8656c5").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Rampage".to_string(),
                        //field_4: 15,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("62f1b418-b72a-4bbd-8827-4a4a568e6d57").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Bane".to_string(),
                        //field_4: 25,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("f805b67c-4259-4aad-a840-9b3aed9c8f0e").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_Banish".to_string(),
                        //field_4: 35,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("c2a6a790-cc12-4bb0-a2fa-cc18c1b937c3").unwrap(),
                        group: "ClassAbilities_New_Warrior_Damage_SanguineCurse".to_string(),
                        //field_4: 55,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("d1f3e2b1-05cd-4914-af5f-3c362283bbbc").unwrap(),
                        group: "PrimeGuardianNew".to_string(),
                        //field_4: 5,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("015bf264-14fd-494a-b758-be5218ac159b").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Dash".to_string(),
                        //field_4: 5,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("598a3005-5a7b-4982-ae69-7ea8bd3b32b8").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Taunt".to_string(),
                        //field_4: 5,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("11d52e0b-d053-4fb5-b92d-f2466872fe34").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_VitalitySlash".to_string(),
                        //field_4: 25,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("d3ea59f7-5101-428a-a25f-988fb7f4482d").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Impulse".to_string(),
                        //field_4: 35,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("b880c31c-4e07-4913-a36a-ebe99c564341").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_EssenceTouch".to_string(),
                        //field_4: 45,
                        ..Default::default()
                    },
                    ClassSkill {
                        version: 0,
                        //field_1: Uuid::parse_str("7daff75b-6078-419b-aa75-c06799b21bf8").unwrap(),
                        content_id: Uuid::parse_str("65ae9a4f-56e0-42c1-94a8-2cc548119816").unwrap(),
                        group: "ClassAbilities_New_Warrior_Tank_Presage".to_string(),
                        //field_4: 55,
                        ..Default::default()
                    },*/
                ]
            }.to_bytes()
        );
    }
}

pub fn setup_combat_style_tech(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleTech>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(vec![]);
    }
}

pub fn setup_combat_style_assassin(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleAssassin>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(vec![]);
    }
}

pub fn setup_combat_style_energizer(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleEnergizer>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(vec![]);
    }
}

pub fn setup_combat_style_hacker(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleHacker>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(vec![]);
    }
}

pub fn setup_combat_style_cyber(
    mut players: Query<&mut ParamBox, (With<PlayerComponent>, Without<NonClientBaseComponent>, Added<CombatStyleCyber>)>
) {
    for mut player in players.iter_mut() {
        let player = player.get_impl_mut::<dyn PlayerParams>().unwrap();

        player.set_current_class_skills(vec![]);
    }
}

pub fn set_heavy_skill_data(player: &mut dyn PlayerParams) {
    player.set_heavy_special_skill_data(
        HeavyData {
            class_hash: 0x6206ed68,
            version: 1,
            data: vec![
                HeavyDataCollection {
                    entry_count: 2,
                    entries: vec![
                        HeavyDataEntry {
                            field_0: "Rage_2H_Club_HeavyAbility_DownwardSwing".to_string(),
                            field_1: 4735,
                            ..Default::default()
                        },
                        HeavyDataEntry {
                            field_0: "Rage_2H_Club_HeavyAbility_UpwardSwing".to_string(),
                            field_1: 3962,
                            ..Default::default()
                        },
                    ]
                },
                HeavyDataCollection {
                    entry_count: 2,
                    entries: vec![
                        HeavyDataEntry {
                            field_0: "Rage_2H_Sword_HeavyAbility_LegSweep".to_string(),
                            field_1: 1765,
                            ..Default::default()
                        },
                        HeavyDataEntry {
                            field_0: "Rage_2H_Sword_HeavyAbility_Slash".to_string(),
                            field_1: 4734,
                            ..Default::default()
                        },
                    ]
                },
                HeavyDataCollection {
                    entry_count: 2,
                    entries: vec![
                        HeavyDataEntry {
                            field_0: "Rage_2H_PoleArm_HeavyAbility_HalfSpinThrust".to_string(),
                            field_1: 3965,
                            ..Default::default()
                        },
                        HeavyDataEntry {
                            field_0: "Rage_2H_PoleArm_HeavyAbility_JumpSmash".to_string(),
                            field_1: 4738,
                            ..Default::default()
                        },
                    ]
                },
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
                HeavyDataCollection::default(),
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
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

use async_graphql::{futures_util::TryStreamExt, Context, Error, InputObject, Json, Object};
use database::DatabaseRecord;
use log::error;
use mongodb::{bson::doc, Database};
use obj_params::{ClassItem, EdnaFunction, EdnaModule, GameObjectData, GenericParamSet, ParamSet, Player};
use serde::Deserialize;
use serde_json::Value;
use toolkit::{anyhow::anyhow, types::Uuid};

use crate::{db::{self, Character, CharacterOutput, ItemStorage, ObjectTemplate, Skillbook, State}, item_storage_session::ItemStorageSession};

use super::item_storage_ext::EquipmentResult;

#[derive(Default)]
pub struct CharacterExtRoot;

#[derive(Default)]
pub struct CharacterExtMutationRoot;

#[Object]
impl CharacterExtRoot {
    async fn account_character(&self, ctx: &Context<'_>, account_id: Uuid, index: i32) -> Result<Option<CharacterOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let res = db::Character::collection(&db).find_one(doc! {
            "$and": [
                { "account": account_id },
                { "index": index }
            ]
        }).await?;

        if let Some(character) = res {
            Ok(Some(character.try_into()?))
        } else {
            Ok(None)
        }
    }

    async fn characters_for_account(&self, ctx: &Context<'_>, account_id: Uuid) -> Result<Vec<CharacterOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut res = db::Character::collection(&db)
            .find(doc! {"account": account_id})
            .sort(doc! {"index": 1})
            .await?;
        let mut characters = Vec::new();

        while let Some(character) = res.try_next().await? {
            characters.push(character.try_into()?);
        }

        Ok(characters)
    }
}

#[Object]
impl CharacterExtMutationRoot {
    async fn create_character_in_account(&self, ctx: &Context<'_>, input: CreateCharacterInput) -> Result<CharacterOutput, Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut cursor = db::Character::collection(&db).aggregate(vec![
            doc! {
                "$match": { "account": input.account }
            },
            doc! {
                "$sort": { "index": -1 }
            },
            doc! {
                "$limit": 1
            }
        ]).with_type::<db::Character>().await?;

        let next_index = if cursor.advance().await? {
            let character = cursor.deserialize_current()?;
            character.index + 1
        } else {
            1
        };

        let mut data = GameObjectData::new::<Player>();
        data.set(Player::WorldMapGuid, "f6b8f8b7-a726-4d36-9634-f6d403943fff");
        data.set(Player::ZoneGuid, "4635f288-ec24-4e73-b75c-958f2607a30e".parse::<Uuid>().unwrap());
        data.set(Player::Zone, "ClassSelection_P");
        data.set(Player::TutorialMode, true);
        data.set(Player::CurrentSkin, "Simuloid");
        data.set(Player::VisibleItemInfo, vec![
            20647, // PlayerCharSkinSimuloid0001Default0004
            21190, // SkinColorSimuloid0006Default0002
            21566, // TattooFace0001Default0002
            21550, // Scars0002Default0002
            21633, // CharFaceMale0012Default0002
            21184, // EyeColor0015Default0002
            21571, // LipColor0011Default0002
            21585, // HairSkin0002Default0002
            21638, // HairColor0017Default0002
        ]);
        data.set(Player::DefaultItemsContentGuid, vec![
            20647, // PlayerCharSkinSimuloid0001Default0004
            21190, // SkinColorSimuloid0006Default0002
            21566, // TattooFace0001Default0002
            21550, // Scars0002Default0002
            21633, // CharFaceMale0012Default0002
            21184, // EyeColor0015Default0002
            21571, // LipColor0011Default0002
            21585, // HairSkin0002Default0002
            21638, // HairColor0017Default0002
        ]);
        data.set(Player::CustomizationGender, 1.0);
        data.set(Player::CustomizationHeight, 0.5);
        data.set(Player::CustomizationBustSize, 0.5);
        data.set(Player::CustomizationFat, 0.0);
        data.set(Player::CustomizationSkinny, 0.7);
        data.set(Player::CustomizationMuscular, 0.3);
        data.set(Player::MoveSpeed, 292.0);

        let character = db::Character::create(&db, db::Character {
            id: Uuid::new(),
            account: input.account,
            index: next_index,
            name: input.name,
            data,
        }).await?;

        Ok(character.try_into()?)
    }

    pub async fn update_character_data_diff(&self, ctx: &Context<'_>, id: Uuid, params: serde_json::Value) -> Result<Option<CharacterOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();

        if let Some(mut character) = Character::get(&db, &id).await? {
            let mut params = serde_json::from_value::<Box<dyn GenericParamSet>>(params)?;
            if character.data.class() != params.class() {
                return Err(anyhow!("parameter class mismatch").into());
            }

            character.data.apply(params.as_mut());
            character.save_param_diff_uncommited(&Character::collection(&db))
                .await?;

            Ok(Some(character.try_into()?))
        } else {
            Ok(None)
        }
    }

    pub async fn character_apply_class_item(&self, ctx: &Context<'_>, id: Uuid, class_item: String, clear_inventory: bool) -> Result<EquipmentResult, Error> {
        let db = ctx.data::<Database>()?.clone();

        if 
            let Some(mut character) = Character::get(&db, &id).await? &&
            let Some(template) = ObjectTemplate::collection(&db).find_one(doc! { "name": &class_item, "class": "ClassItem" }).await? &&
            let Ok(default_equipment) = serde_json::from_value::<DefaultEquipment>(template.data.get::<_,Value>(ClassItem::DefaultEquipment).unwrap_or(&Value::Null).clone())
        {
            let mut skillbook = Skillbook::get_or_create(&db, character.id).await?;

            let storage = ItemStorage::get_or_create_for_owner(&db, "inventory", db::StorageOwner::Character(id)).await?;
            let mut session = ItemStorageSession::start(&db, storage.id).await?;
            
            if clear_inventory {
                session.clear_items().await?;
            }

            /*
                Apply weapon and armor templates.
            */
            {
                let mut weapon_idx = 0;

                for weapon_template in &default_equipment.weapon_templates {
                    let mut overrides = ParamSet::<EdnaFunction>::new();

                    if let Some(name_override) = &weapon_template.name_override {
                        overrides.insert(EdnaFunction::Name, name_override.clone());
                    }

                    if let Some(item) = ObjectTemplate::collection(&db).find_one(doc! { "name": &weapon_template.weapon_content_entry }).await? {
                        let id = session.insert_item(item, None, Some(Box::new(overrides))).await?;
                        session.equip_item(id, Some(weapon_idx)).await?;

                        weapon_idx += 1;
                    }
                }

                for armor_template in &default_equipment.armor_templates {
                    let mut overrides = ParamSet::<EdnaModule>::new();

                    if let Some(name_override) = &armor_template.name_override {
                        overrides.insert(EdnaModule::Name, name_override.clone());
                    }

                    if let Some(item) = ObjectTemplate::collection(&db).find_one(doc! { "name": &armor_template.armor_content_entry }).await? {
                        let id = session.insert_item(item, None, Some(Box::new(overrides))).await?;
                        session.equip_item(id, None).await?;
                    }
                }
            }

            /*
                Apply weapon and armor items.
                This is an alternative to the templates above.
            */
            {
                let mut weapon_idx = 0;

                for weapon in &default_equipment.weapons {
                    if let Some(item) = ObjectTemplate::collection(&db).find_one(doc! { "name": weapon }).await? {
                        let id = session.insert_item(item, None, None).await?;
                        session.equip_item(id, Some(weapon_idx)).await?;

                        weapon_idx += 1;
                    }
                }

                for armor in &default_equipment.armors {
                    if let Some(item) = ObjectTemplate::collection(&db).find_one(doc! { "name": armor }).await? {
                        let id = session.insert_item(item, None, None).await?;
                        session.equip_item(id, None).await?;
                    }
                }
            }

            if let Some(combat_style) = default_equipment.combat_style {
                character.data.set(Player::CombatStyle, combat_style);

                if skillbook.combat_style != combat_style.try_into()? {
                    skillbook.change_class(&db, 
                        combat_style.try_into()?, 
                        *character.data.get::<_, i32>(Player::Lvl).unwrap()
                    ).await?;
                }
            }

            if let Some(level) = default_equipment.level {
                character.data.set(Player::Lvl, level);

                skillbook.level_up(level);
            }

            if let Some(true) = default_equipment.level_up_skills {
                for skill in skillbook.skills.iter_mut() {
                    if skill.state == State::Locked {
                        skill.state = State::Unlocked;
                    }
                }
            }

            let (mut session, storage_results) = session.write_uncommitted().await?;

            skillbook
                .save_uncommited(&Skillbook::collection(&db))
                .session(&mut session)
                .await?;

            character
                .save_param_diff_uncommited(&Character::collection(&db))
                .session(&mut session)
                .await?;

            let mut character_update = Character::update_equipment(&db, &mut session, character.id, storage.id).await?;

            session.commit_transaction().await?;

            character.data.changes()
                .for_each(|(attr, val)| {
                    character_update.set_param(attr.name(), val);
                });

            Ok(EquipmentResult {
                error: None,
                storage_result: storage_results
                    .into_iter()
                    .map(|res| res.into())
                    .collect(),
                character_update: if character_update.is_empty() {
                    None
                } else {
                    Some(Json(character_update))
                },
                skillbook: Some(skillbook.try_into()?),
            })
        } else {
            error!("Failed to apply class item: {class_item:?}");

            Ok(EquipmentResult {
                error: None,
                storage_result: vec![],
                character_update: None,
                skillbook: None,
            })
        }
    }
}

#[derive(InputObject)]
pub struct CreateCharacterInput {
    account: Uuid,
    name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "PascalCase")]
pub struct DefaultEquipment {
    combat_style: Option<i32>,
    level: Option<i32>,
    #[serde(rename = "levelUpSkills")]
    level_up_skills: Option<bool>,
    #[serde(rename = "QBoost")]
    qboost: Option<String>,
    #[serde(default)]
    weapon_templates: Vec<WeaponTemplate>,
    #[serde(default)]
    armor_templates: Vec<ArmorTemplate>,
    #[serde(default)]
    weapons: Vec<String>,
    #[serde(default)]
    armors: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponTemplate {
    weapon_content_entry: String,
    name_override: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmorTemplate {
    armor_content_entry: String,
    name_override: Option<String>,
}
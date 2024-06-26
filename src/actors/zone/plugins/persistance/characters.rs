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

use std::str::FromStr;

use atlas::{ParamAttrib, ParamBox, ParamFlag, PlayerAttribute, PlayerComponent, PlayerParams};
use bevy_ecs::{component::Component, entity::Entity, query::{Added, Changed, With}, system::{Commands, Query, Res}};
use bson::{doc, Document};
use log::{debug, error};

use crate::{actors::{zone::resources::Tasks, AvatarComponent, RealmDatabase}, db::{Character, DatabaseRecord}};

#[derive(Component)]
pub struct PreviousParamBox(ParamBox);

pub fn prepare_player_change_detection(
    query: Query<(Entity, &ParamBox), Added<PlayerComponent>>,
    mut cmds: Commands,
) {
    for (entity, params) in query.iter() {
        cmds.entity(entity)
            .insert(PreviousParamBox(params.clone()));
    }
}

pub fn update_player_database(
    mut query: Query<(&AvatarComponent, &ParamBox, &mut PreviousParamBox), (With<PlayerComponent>, Changed<ParamBox>)>,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
) {
    for (avatar, params, mut prev_params) in query.iter_mut() {
        let diff = params.diff(&prev_params.0);
        params.clone_into(&mut prev_params.0);

        let bling = params.get_impl::<dyn PlayerParams>().unwrap().bling();

        if !diff.is_empty() {
            let id = avatar.record_id.unwrap();
            let db = db.0.clone();

            let _guard = tasks.handle.enter();
            tasks.tasks.spawn(async move {
                let collection = Character::collection(db);
                let mut values = Document::new();
                for (key, val) in diff.as_hash_map() {
                    if let Ok(attr) = PlayerAttribute::from_str(&key) {
                        if attr.has_flag(&ParamFlag::Persistent) {
                            values.insert(format!("data.{}", key), bson::to_bson(&val).unwrap());
                        } else {
                            // special case for params that are not stored persistantly
                            // in params.
                            match attr {
                                PlayerAttribute::Bling => {
                                    values.insert("bling", TryInto::<i32>::try_into(&val).unwrap());
                                },
                                _ => (),
                            }
                        }
                    }
                }

                debug!("Store: {:?}", values);

                if let Err(e) = collection.update_one(doc!("guid": id), doc!("$set": values), None).await {
                    error!("Database update failed: {:?}", e);
                }
            });
        }
    }
}
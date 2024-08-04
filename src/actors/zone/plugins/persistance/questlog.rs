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

use atlas::PlayerComponent;
use bevy_ecs::{query::{Changed, With}, system::{Query, Res}};
use bson::doc;
use log::error;

use crate::{actors::{zone::{plugins::QuestLog, resources::Tasks}, AvatarComponent, RealmDatabase}, db::{self, Character, DatabaseRecord}};

pub fn update_player_questlog(
    query: Query<(&AvatarComponent, &QuestLog), (With<PlayerComponent>, Changed<QuestLog>)>,
    tasks: Res<Tasks>,
    db: Res<RealmDatabase>,
) {
    for (avatar, questlog) in query.iter() {
        let id = avatar.record_id.unwrap();
        let db = db.0.clone();
        let mut db_questlog = db::QuestLog::default();
            
        questlog.finished.clone_into(&mut db_questlog.finished);

        db_questlog.in_progress = questlog.in_progress.iter().map(|progress| {
            db::QuestProgress {
                id: progress.info.id,
                condition_progress: progress.condition_progress.iter().map(|(&k, &v)| (k,v)).collect(),
            }
        })
        .collect();

        db_questlog.completed = questlog.completed.iter().map(|progress| {
            db::QuestProgress {
                id: progress.info.id,
                condition_progress: progress.condition_progress.iter().map(|(&k, &v)| (k,v)).collect(),
            }
        })
        .collect();

        if let Ok(doc) = bson::to_document(&db_questlog) {
            let _guard = tasks.handle.enter();
            tasks.tasks.spawn(async move {
                let collection = Character::collection(db);
    
                    if let Err(e) = collection.update_one(doc!("guid": id), doc!("$set": {
                    "questlog": doc
                }), None).await {
                    error!("Database update failed: {:?}", e);
                }
            });
        }
    }
}
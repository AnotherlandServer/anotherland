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

use async_graphql::{Context, InputObject, Object};
use database::DatabaseRecord;
use futures_util::TryStreamExt;
use mongodb::{Database, bson::{doc, from_bson, from_document}};
use toolkit::types::Uuid;

use crate::db::{AvatarSelector, ObjectPlacement, ObjectPlacementOutput, WorldDef, Zone};

#[derive(Default)]
pub struct ObjectPlacementsExtRoot;

#[derive(InputObject)]
pub struct ObjectPlacementSelector {
    world_id: Option<u16>,
    zone_id: Option<Uuid>,
    selector: AvatarSelector,
}

#[Object]
impl ObjectPlacementsExtRoot {
    async fn query_placements_by_selector(&self, ctx: &Context<'_>, query: ObjectPlacementSelector) -> Result<Vec<ObjectPlacementOutput>, async_graphql::Error> {
        let db = ctx.data::<Database>()?.clone();
        let mut matcher = doc! {};

        if let Some(zone_id) = query.zone_id {
            matcher.insert("zone_guid", zone_id);
        } else if 
            let Some(world_id) = query.world_id &&
            let Some(world) = WorldDef::get(&db, &world_id).await?
        {
            let mut zones = Zone::collection(&db)
                .find(doc! { "worlddef_guid": world.guid }).await?;

            let mut guids = Vec::new();
            while let Some(zone) = zones.try_next().await? {
                guids.push(zone.guid);
            }

            matcher.insert("zone_guid", doc! { "$in": guids });
        }

        match query.selector {
            AvatarSelector::InstanceId(id) => {
                matcher.insert("id", id);
            },
            AvatarSelector::ContentId(id) => {
                matcher.insert("content_guid", id);
            },
            AvatarSelector::DialogId(id) => {
                matcher.insert("content.Dialogs", doc! { "$in": [id] });
            },
            AvatarSelector::LootItem(_id) => {},
            AvatarSelector::QuestTag(_id) => {
                todo!()
            },
        }

        let mut result = ObjectPlacement::collection(&db)
            .aggregate(vec![
                doc! {
                    "$lookup": doc! {
                        "from": "object_templates",
                        "localField": "content_guid",
                        "foreignField": "id",
                        "as": "content"
                    }
                },
                doc! {
                    "$set": doc! {
                        "content": doc! {
                            "$arrayElemAt": [
                                "$content",
                                0
                            ]
                        }
                    }
                },
                doc! {
                    "$set": doc! {
                        "content": doc! {
                            "$mergeObjects": [
                                "$data.attributes",
                                "$content.data.attributes"
                            ]
                        }
                    }
                },
                doc! {
                    "$match": matcher
                },
                doc! {
                    "$unset": "content"
                }
            ]).await?;

        let mut res = Vec::new();

        while let Some(placement) = result.try_next().await? {
            let placement = from_document::<ObjectPlacement>(placement)?;
            res.push(placement.try_into()?);
        }

        Ok(res)
    }
}
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

use std::{mem::take, path::Path};

use indicatif::ProgressBar;
use log::info;
use obj_params::{Class, GameObjectData, GenericParamSet, ParamReader};
use realm_api::{ObjectPlacementBuilder, RealmApi};
use tokio::runtime::Handle;

use crate::{error::SeedRealmResult, MP};

pub async fn import_object_placements(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        info!("Importing object placements...");

        let count = db
            .prepare("SELECT COUNT(*) FROM Instance")
            .unwrap()
            .into_iter()
            .next().unwrap().unwrap()
            .read::<i64,_>(0);
    
        let result = db
            .prepare("SELECT * FROM Instance")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let pg = MP.add(ProgressBar::new(count as u64));

        let mut records = Vec::new();
    
        // dump data
        for row in result {   
            let class = Class::from_id(row.read::<i64,_>("ixClass") as u16)
                .unwrap();

            let bin_data = row.read::<&[u8], _>("data");
            let data = if !bin_data.is_empty() {
                GameObjectData::from_generic_set(
                    Box::<dyn GenericParamSet>::from_slice(class, bin_data).unwrap().1
                )
            } else {
                GameObjectData::new_for_class(class)
            };
            
            records.push(
                ObjectPlacementBuilder::default()
                    .id(row.read::<&str,_>("uxInstanceGuid").parse().unwrap())
                    .zone_guid(row.read::<&str,_>("uxZoneGuid").parse().unwrap())
                    .class(class)
                    .content_guid(row.read::<&str,_>("uxContentGuid").parse().unwrap())
                    .editor_name(row.read::<&str,_>("sEditorName").to_owned())
                    .data(data)
                    .phase_tag(row.read::<&str,_>("phaseTag").to_owned())
                    .build().unwrap()
            );

            if records.len() == 100 {
                Handle::current().block_on(async {
                    api.batch_create_object_placements(take(&mut records))
                        .await.unwrap()
                });

                pg.inc(100);
            }
        }

        if !records.is_empty(){
            Handle::current().block_on(async {
                api.batch_create_object_placements(take(&mut records))
                    .await.unwrap()
            });

            pg.finish();
        }
    });

    Ok(())
}
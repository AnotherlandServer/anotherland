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

use std::path::Path;

use log::info;
use realm_api::{RealmApi, ZoneBuilder};

use crate::error::SeedRealmResult;

pub async fn import_zone(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    let records = tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        info!("Importing zones...");
    
        let result = db
            .prepare("SELECT * FROM Zone")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut records = Vec::new();
    
        // dump data
        for row in result {   
            records.push(
                ZoneBuilder::default()
                    .id(row.read::<i64,_>("ixZoneID"))
                    .guid(row.read::<&str,_>("uxZoneGuid").parse().unwrap())
                    .worlddef_guid(row.read::<&str,_>("uxWorldDefGuid").parse().unwrap())
                    .parent_zone_guid(row.read::<&str,_>("uxParentZoneGuid").parse().unwrap())
                    .zone(row.read::<&str,_>("sZone").to_owned())
                    .zone_type(match row.read::<i64,_>("iType") {
                        0 => realm_api::ZoneType::World,
                        1 => realm_api::ZoneType::Ghost,
                        _ => panic!("unknown zone type"),
                    })
                    .is_instance(row.read::<i64,_>("bInstance") != 0)
                    .server(row.read::<&str,_>("sServer").to_owned())
                    .level(row.read::<&str,_>("sLevel").to_owned())
                    .layer(row.read::<&str,_>("sLayer").to_owned())
                    .realu_zone_type(row.read::<&str,_>("sRealUZoneType").to_owned())
                    .game_controller(row.read::<&str,_>("sGameController").to_owned())
                    .build().unwrap()
            );
        }
    
        records
    });

    api.batch_create_zones(records).await?;
    Ok(())
}
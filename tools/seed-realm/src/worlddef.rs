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

use std::path::Path;

use log::info;
use realm_api::{RealmApi, WorldDef};

use crate::error::SeedRealmResult;

pub async fn import_worlddef(game_client_path: &Path, api: &RealmApi) -> SeedRealmResult<()> {
    let records = tokio::task::block_in_place(move || {
        let db = sqlite::open(
            game_client_path
            .join("Atlas/data/otherlandgame/content/dbbba21e-2342-4357-a777-302ed11b978b/instance.db")
        ).unwrap();

        info!("Importing worlddefs...");
    
        let result = db
            .prepare("SELECT * FROM WorldDef")
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());

        let mut records = Vec::new();
    
        // dump data
        for row in result {   
            records.push(WorldDef::new(
                row.read::<i64,_>("ixWorldID") as u16, 
                row.read::<&str,_>("uxWorldDefGuid").parse().unwrap(), 
                row.read::<&str,_>("sWorldDef").to_owned(), 
                row.read::<&str,_>("uxUMapGuid").parse().unwrap()
            ));
        }
    
        records
    });

    api.batch_create_worlddef(records).await?;
    Ok(())
}
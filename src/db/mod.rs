// Copyright (C) 2023 AnotherlandServer
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

mod account;
mod session;
mod content;
mod db;
mod instance;
mod worlddef;
mod zonedef;
mod character;
mod content_placement;
mod cash_shop_bundle;
mod cash_shop_item;
mod cash_shop_vendor;
pub use account::*;
pub use session::*;
pub use content::*;
pub use db::*;
pub use instance::*;
pub use worlddef::*;
pub use zonedef::*;
pub use character::*;
pub use cash_shop_bundle::*;
pub use cash_shop_item::*;
pub use cash_shop_vendor::*;

use crate::{util::AnotherlandResult, ARGS};

use self::content_placement::ContentPlacement;

pub async fn initalize_db() -> AnotherlandResult<()> {
    {
        let db = cluster_database().await;

        Account::init_collection(db.clone()).await?;
        Session::init_collection(db.clone()).await?;
    }

    if let Some(_) = ARGS.mongo_realm_db() {
        let db = realm_database().await;

        Character::init_collection(db.clone()).await?;
        ContentPlacement::init_collection(db.clone()).await?;

    }

    Ok(())
}
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
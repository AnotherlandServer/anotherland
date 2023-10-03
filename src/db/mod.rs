mod account;
mod session;
mod realm;
mod content;
mod db;
mod instance;
mod worlddef;
mod zone;
mod character;
mod worldserverentry;

pub use account::*;
pub use session::*;
pub use realm::*;
pub use content::*;
pub use db::*;
pub use instance::*;
pub use worlddef::*;
pub use zone::*;
pub use character::*;
pub use worldserverentry::*;

use crate::{util::AnotherlandResult, ARGS};

pub async fn initalize_db() -> AnotherlandResult<()> {
    {
        let db = cluster_database().await;

        Account::init_collection(db.clone()).await?;
        Session::init_collection(db.clone()).await?;
    }

    if let Some(_) = ARGS.mongo_realm_db() {
        let db = realm_database().await;

        Character::init_collection(db.clone()).await?;

    }

    Ok(())
}
use std::time::{Instant, SystemTime};

use chrono::{Utc, DateTime};
use log::Record;
use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};
use surrealdb::sql::{Thing, self, Datetime};

use crate::{atlas::Uuid, util::AnotherlandResult, DB};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountRecord {
    pub id: Thing,
    pub numeric_id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: Datetime,
    pub last_login: Option<Datetime>,
    pub banned: bool,
    pub ban_reason: Option<String>
}

impl AccountRecord {
    pub async fn get_by_id(guid: Uuid) -> AnotherlandResult<Option<AccountRecord>> {
        Ok(DB.select(("account", guid)).await?)
    }

    pub async fn get_by_username_or_mail(username_or_email: &str) -> AnotherlandResult<Option<AccountRecord>> {
        Ok(
            DB
            .query("SELECT * FROM account WHERE username = $username_or_email OR email = $username_or_email LIMIT 1")
            .bind(("username_or_email", username_or_email))
            .await?
            .take(0)?
        )
    }

    pub async fn create(username: String, email: String, password: String) -> AnotherlandResult<AccountRecord> {
        let id = Uuid::new_v4();

        // Compute numeric account id by hashing the uuid and trimming it to 32bits.
        // Not ideal, but using a 32bit id for accounts is kinda ludicrous to begin with...
        let mut hasher = Sha1::new();
        hasher.update(id.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());

        let mut created: Vec<AccountRecord> = DB.create("account").content(AccountRecord {
            id: Thing {
                tb: "account".to_owned(),
                id: surrealdb::sql::Id::String(id.to_string())
            },
            numeric_id,
            username,
            email,
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST)?,
            created: Utc::now().into(),
            last_login: None,
            banned: false,
            ban_reason: None,
        }).await?;

        Ok(created.remove(0))
    }

    pub async fn update_last_login(&mut self) -> AnotherlandResult<()> {
        todo!()
    }
}
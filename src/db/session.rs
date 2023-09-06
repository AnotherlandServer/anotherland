use std::time::{Instant, SystemTime};

use chrono::{DateTime, Utc};
use log::Record;
use serde::{Serialize, Deserialize};
use surrealdb::sql::{Thing, Datetime};

use crate::{atlas::Uuid, util::AnotherlandResult, DB};

use super::AccountRecord;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: Thing,
    pub created: Datetime,
    pub last_update: Datetime
}

impl SessionRecord {
    pub async fn get_by_id(guid: Uuid) -> AnotherlandResult<Option<AccountRecord>> {
        Ok(DB.select(("account_session", guid)).await?)
    }

    pub async fn create(account: &AccountRecord) -> AnotherlandResult<SessionRecord> {
        let session: SessionRecord = DB.create("account_session").content(SessionRecord {
            id: Thing {
                tb: "account_session".to_owned(),
                id: surrealdb::sql::Id::String(Uuid::new_v4().to_string())
            },
            created: Utc::now().into(),
            last_update: Utc::now().into(),
        }).await?.remove(0);

        DB
            .query("RELATE $account->started->$account_session;")
            .bind(("account", account.id.clone()))
            .bind(("account_session", session.id.clone()))
            .await?;

        Ok(session)
    }

    pub async fn keepalive(&mut self) -> AnotherlandResult<()> {
        self.last_update = Utc::now().into();
        let _: Option<SessionRecord> = DB.update(self.id.clone()).content(self).await?;
        Ok(())
    }
}
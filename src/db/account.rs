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

use async_trait::async_trait;
use atlas::Uuid;
use bson::doc;
use chrono::{Utc, DateTime};
use log::info;
use mongodb::{Database, options::IndexOptions, IndexModel, Collection};
use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};

use crate::{util::AnotherlandResult, CONF};

use super::DatabaseRecord;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PasswordHash {
    Unset,
    Password(String),
    OneTimePassword(DateTime<Utc>, String),
}

impl PasswordHash {
    pub fn hash_password(password: String) -> Self {
        Self::Password(bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .expect("failed to hash password"))
    }

    pub fn hash_one_time_password(password: String) -> Self {
        Self::OneTimePassword(Utc::now(), bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .expect("failed to hash password"))
    }

    pub fn check_password(&self, password: String) -> bool {
        match self {
            PasswordHash::Unset => false,
            PasswordHash::Password(hash) => bcrypt::verify(password, hash).expect("failed to verify password"),
            PasswordHash::OneTimePassword(date, hash) => {
                if Utc::now().signed_duration_since(date).num_seconds() < CONF.login_server.one_time_password_duration.unwrap_or(900).into() {
                    bcrypt::verify(password, hash).expect("failed to verify password")
                } else {
                    false
                }
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub numeric_id: u32,
    pub username: String,
    pub email: Option<String>,
    pub password: PasswordHash,
    pub created: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub is_gm: bool,
}

impl Account {
    pub async fn get_by_id(db: Database, guid: &Uuid) -> AnotherlandResult<Option<Account>> {
        let collection = db.collection::<Account>("accounts");
        Ok(collection.find_one(doc! {"id": {"$eq": guid}}, None).await?)
    }

    pub async fn get_by_username_or_mail(db: Database, username_or_email: &str) -> AnotherlandResult<Option<Account>> {
        let collection = db.collection::<Account>("accounts");
        Ok(collection.find_one(doc! {
            "$or": [ 
                {"username": {"$eq": username_or_email}}, 
                {"email": {"$eq": username_or_email}}
            ]},
         None).await?)
    }

    pub async fn create(db: Database, username: String, email: Option<String>, password: Option<String>) -> AnotherlandResult<Account> {
        let collection = db.collection::<Account>("accounts");
        let id = Uuid::new();

        // Compute numeric account id by hashing the uuid and trimming it to 32bits.
        // Not ideal, but using a 32bit id for accounts is kinda ludicrous to begin with...
        let mut hasher = Sha1::new();
        hasher.update(id.to_string());
        let result = hasher.finalize();
        
        let numeric_id = u32::from_le_bytes(result[0..4].try_into().unwrap());
        let account = Account {
            id,
            numeric_id,
            username,
            email,
            password: password
                .map(PasswordHash::hash_password)
                .unwrap_or(PasswordHash::Unset),
            created: Utc::now(),
            last_login: None,
            banned: false,
            ban_reason: None,
            is_gm: false,
        };

        let result = collection.insert_one(&account, None).await?;
        info!("Result: {:#?}", result);
        Ok(account)
    }

    pub fn record_login(&mut self) {
        self.last_login = Some(Utc::now());

        // reset password if one-time-password is used
        if matches!(self.password, PasswordHash::OneTimePassword(_, _)) {
            self.password = PasswordHash::Unset;
        }
    }

    pub fn set_password(&mut self, password: String) -> AnotherlandResult<()> {
        self.password = PasswordHash::hash_password(password);
        Ok(())
    }

    pub fn set_one_time_password(&mut self, password: String) -> AnotherlandResult<()> {
        self.password = PasswordHash::hash_one_time_password(password);
        Ok(())
    }

    pub async fn init_collection(db: Database) -> AnotherlandResult<()> {
        let collection = db.collection::<Account>("accounts");
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(),
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("numeric_id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(),
            None).await?;
        
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("username": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build(),
            None).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("email": 1))            
            .options(IndexOptions::builder().unique(true).partial_filter_expression(doc!("email":{"$type":"string"})).build())
            .build(),
            None).await?;

        Ok(())
    }
}

#[async_trait]
impl DatabaseRecord<'_> for Account {
    type Key = Uuid;

    fn collection(db: Database) -> Collection<Self> {
        db.collection::<Self>("accounts")
    }

    fn key(&self) -> &Self::Key {
        &self.id
    }
}
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

use bson::{doc, Uuid};
use chrono::{Utc, DateTime};
use database::{DBResult, DatabaseRecord};
use mongodb::{Database, options::IndexOptions, IndexModel};
use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};
use toolkit::config::CLUSTER_CONF;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Credentials {
    Username {
        name: String,
        email: Option<String>,
        password: PasswordHash
    },
    Steam {
        steam_id: String,
    }
} 

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
                if Utc::now().signed_duration_since(date).num_seconds() < CLUSTER_CONF.login.one_time_password_duration.unwrap_or(900).into() {
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
    pub numeric_id: i32,
    pub credentials: Credentials,
    pub created: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub is_gm: bool,
}

impl Account {
    pub fn unique_name(&self) -> &str {
        match &self.credentials {
            Credentials::Steam { steam_id } => steam_id,
            Credentials::Username { name, .. } => name,
        }
    }

    pub async fn get_by_steam_id(db: &Database, steam_id: &str) -> DBResult<Option<Account>> {
        let collection = Self::collection(db);
        Ok(collection.find_one(doc! {"credentials.Steam.steam_id": {"$eq": steam_id}}).await?)
    }

    pub async fn get_by_username_or_mail(db: &Database, username_or_email: &str) -> DBResult<Option<Account>> {
        let collection = Self::collection(db);
        Ok(collection.find_one(doc! {
            "$or": [ 
                {"credentials.Username.name": {"$eq": username_or_email}}, 
                {"credentials.Username.email": {"$eq": username_or_email}}
            ]}).await?)
    }

    pub async fn create(db: &Database, credentials: Credentials) -> DBResult<Account> {
        let collection = Self::collection(db);
        let id = Uuid::new();

        // Compute numeric account id by hashing the uuid and trimming it to 32bits.
        // Not ideal, but using a 32bit id for accounts is kinda ludicrous to begin with...
        let mut hasher = Sha1::new();
        hasher.update(id.to_string());
        let result = hasher.finalize();
        
        let numeric_id = i32::from_le_bytes(result[0..4].try_into().unwrap()) & 0x7FFFFFFF;
        let account = Account {
            id,
            numeric_id,
            credentials,
            created: Utc::now(),
            last_login: None,
            banned: false,
            ban_reason: None,
            is_gm: false,
        };

        let _ = collection.insert_one(&account).await?;
        Ok(account)
    }

    pub fn record_login(&mut self) {
        self.last_login = Some(Utc::now());

        // reset password if one-time-password is used
        if let Credentials::Username { password, .. } = &mut self.credentials {
            if matches!(password, PasswordHash::OneTimePassword(_, _)) {
                *password = PasswordHash::Unset;
            }
        }
    }

    pub fn set_password(&mut self, password: String) -> DBResult<()> {
        if let Credentials::Username { password: stored_password, .. } = &mut self.credentials {
            *stored_password = PasswordHash::hash_password(password);
        }
        Ok(())
    }

    pub fn set_one_time_password(&mut self, password: String) -> DBResult<()> {
        if let Credentials::Username { password: stored_password, .. } = &mut self.credentials {
            *stored_password = PasswordHash::hash_one_time_password(password);
        }
        Ok(())
    }
}

impl<'de> DatabaseRecord<'de> for Account {
    type PrimaryKey = Uuid;

    fn key(&self) -> &Self::PrimaryKey {
        &self.id
    }
    
    fn collection_name() -> &'static str {
        "accounts"
    }

    async fn build_index(db: &Database) -> DBResult<()> {
        let collection = Self::collection(db);
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("numeric_id": 1))
            .options(IndexOptions::builder().unique(true).build())
            .build()).await?;
        
        collection.create_index(
            IndexModel::builder()
            .keys(doc!("credentials.Username.name": 1))
            .options(IndexOptions::builder().unique(true).partial_filter_expression(doc!("credentials.Username.name":{"$type":"string"})).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("credentials.Username.email": 1))            
            .options(IndexOptions::builder().unique(true).partial_filter_expression(doc!("credentials.Username.email":{"$type":"string"})).build())
            .build()).await?;

        collection.create_index(
            IndexModel::builder()
            .keys(doc!("credentials.Steam.steam_id": 1))            
            .options(IndexOptions::builder().unique(true).partial_filter_expression(doc!("credentials.Steam.steam_id":{"$type":"string"})).build())
            .build()).await?;

        Ok(())
    }
}
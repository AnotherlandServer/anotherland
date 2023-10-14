use async_trait::async_trait;
use bson::{doc, Document};
use chrono::{Utc, DateTime};
use mongodb::{Database, options::IndexOptions, IndexModel, Collection};
use serde::{Serialize, Deserialize};
use sha1::{Sha1, Digest};

use crate::util::AnotherlandResult;
use atlas::Uuid;

use super::DatabaseRecord;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub numeric_id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub banned: bool,
    pub ban_reason: Option<String>,
    pub is_gm: bool,
}

impl Account {
    pub async fn get_by_id(db: Database, guid: &Uuid) -> AnotherlandResult<Option<Account>> {
        let collection = db.collection::<Account>("accounts");
        Ok(collection.find_one(doc! {"id": {"$eq": guid.to_string()}}, None).await?)
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

    pub async fn create(db: Database, username: String, email: String, password: String) -> AnotherlandResult<Account> {
        let collection = db.collection::<Account>("accounts");
        let id = Uuid::new_v4();

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
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST)?,
            created: Utc::now().into(),
            last_login: None,
            banned: false,
            ban_reason: None,
            is_gm: false,
        };

        collection.insert_one(&account, None).await?;
        Ok(account)
    }

    pub async fn update_last_login(&mut self, _db: Database) -> AnotherlandResult<()> {
        todo!()
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
            .options(IndexOptions::builder().unique(true).build())
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
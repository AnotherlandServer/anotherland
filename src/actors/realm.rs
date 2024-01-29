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

use std::net::SocketAddrV4;

use actor_macros::actor_actions;
use async_trait::async_trait;
use atlas::AvatarId;
use bson::doc;
use log::debug;
use mongodb::Database;

use crate::{cluster::actor::Actor, util::AnotherlandResult, db::{Session, realm_database, Character, DatabaseRecord}, CONF};

pub struct Realm {
    realm_db: Database,
    id: u32,
    name: String,
    cluster_frontend: Option<SocketAddrV4>,
}

impl Realm {
    pub async fn initialize() -> AnotherlandResult<Self> {
        let realm_db = realm_database().await;

        Ok(Self {
            realm_db,
            id: CONF.realm.id,
            name: CONF.realm.name.clone(),
            cluster_frontend: None,
        })
    }
}

#[async_trait]
impl Actor for Realm {
    type ActorType = Self;

    fn name(&self) -> Option<&str> { Some("realm") }

    async fn starting(&mut self) -> AnotherlandResult<()> { 
        Ok(()) 
    }
}

#[actor_actions]
impl Realm {
    #[rpc]
    pub async fn get_characters(&self, session: Session) -> AnotherlandResult<Vec<Character>> {
        Character::list(self.realm_db.clone(), &session.account).await
    }

    #[rpc]
    pub async fn get_character(&self, session: Session, id: u32) -> AnotherlandResult<Option<Character>> {
        debug!("Account id: {}", session.account);

        let collection = Character::collection(self.realm_db.clone());
        Ok(collection.find_one(doc!{"$and": [ {"id": {"$eq": id}}, {"account": {"$eq": session.account}}]}, None).await?)
    }


    #[rpc]
    pub async fn get_character_by_name(&self, session: Session, name: String) -> AnotherlandResult<Option<Character>> {
        let collection = Character::collection(self.realm_db.clone());
        Ok(collection.find_one(doc!{"$and": [ {"name": {"$eq": name}}, {"account": {"$eq": session.account}}]}, None).await?)
    }

    #[rpc]
    pub async fn create_character(&mut self, session: Session, name: String) -> AnotherlandResult<Character> {
        Character::create(self.realm_db.clone(), &session.account, &name).await
    }

    #[rpc]
    pub async fn delete_character(&mut self, session: Session, id: u32) -> AnotherlandResult<()> {
        let collection = Character::collection(self.realm_db.clone());
        collection.delete_one(doc!{"$and": [ {"id": {"$eq": id}}, {"account": {"$eq": session.account}}]}, None).await?;
        Ok(())
    }

    #[rpc]
    pub async fn delete_character_by_name(&mut self, session: Session, name: String) -> AnotherlandResult<()> {
        let collection = Character::collection(self.realm_db.clone());
        collection.delete_one(doc!{"$and": [ {"name": {"$eq": name}}, {"account": {"$eq": session.account}}]}, None).await?;
        Ok(())
    }

    #[rpc]
    pub fn update_cluster_frontend_address(&mut self, addr: SocketAddrV4) {
        self.cluster_frontend = Some(addr);
    }

    #[rpc]
    pub fn get_cluster_frontend_address(&self) -> Option<SocketAddrV4> {
        self.cluster_frontend
    }

    #[rpc]
    pub fn claim_avatar_id(&mut self) -> AnotherlandResult<AvatarId> {
        todo!()
    }
}
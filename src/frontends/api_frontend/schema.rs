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

use std::collections::HashSet;
use std::net::{Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_graphql::{ComplexObject, Json};
use async_graphql::{Object, Result, Context, SimpleObject};
use atlas::{ParamBox, Uuid};
use atlas::UUID_NIL;
use chrono::{DateTime, Utc};
use futures::future::{BoxFuture, FutureExt};
use futures::stream::StreamExt;
use futures::stream::FuturesUnordered;
use glam::Vec3;
use log::debug;
use quinn::{ClientConfig, Endpoint};
use serde_derive::{Serialize, Deserialize};
use nom::AsBytes;

use crate::actors::ZoneRegistry;
use crate::cluster::ApiResponse;
use crate::frontends::{ApiCommand, ApiResult, ZoneDownstreamMessage, ZoneServerSkipVerification, ZoneUpstreamMessage};
use crate::{actors::{Authenticator, RealmList, SessionManager}, cluster::RemoteActorRef, util::{AnotherlandError, AnotherlandErrorKind, AnotherlandResult}};

pub struct QueryRoot;

pub struct MutationRoot;

#[Object]
impl QueryRoot {
    async fn account(&self, ctx: &Context<'_>, id: String) -> Result<Account> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .get_account(Uuid::parse_str(&id)?)
            .await?;
        Ok(account.into())
    }

    async fn find_account(&self, ctx: &Context<'_>, username_or_email: String) -> Result<Account> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .find_account(username_or_email).await?;

        Ok(account.into())
    }

    async fn active_sessions(&self, ctx: &Context<'_>) -> Result<Vec<Session>> {
        let sessions = ctx.data::<RemoteActorRef<SessionManager>>()?
            .active_sessions()
            .await?;

        sessions
            .into_iter()
            .map(|session| Session::populate_from_session(ctx.clone(), session))
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect()
    }

    async fn realms(&self, ctx: &Context<'_>) -> Result<Vec<Realm>> {
        Ok(
            ctx.data::<RemoteActorRef<RealmList>>()?
                .get_realms()
                .await
                .into_iter()
                .map(|realm| realm.into())
                .collect()
        )
    }

    async fn zones(&self, ctx: &Context<'_>, realm_id: u32) -> Result<Vec<Zone>> {
        if realm_id == 1 {
            ctx.data::<RemoteActorRef<ZoneRegistry>>()?
                .get_zones()
                .await
                .into_iter()
                .map(|(id, _)| Zone::get_from_id(ctx.clone(), id))
                .collect::<FuturesUnordered<_>>()
                .collect::<Vec<_>>()
                .await
                .into_iter()
                .collect()

        } else {
            Err(async_graphql::Error::new("realm not found"))
        }
    }
}

#[Object]
impl MutationRoot {
    async fn create_account(&self, ctx: &Context<'_>, name: String, email: Option<String>, password: Option<String>) -> Result<Account> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .register(name, email, password).await?;

        Ok(account.into())
    }

    async fn find_or_create_account(&self, ctx: &Context<'_>, name: String, email: Option<String>, password: Option<String>) -> Result<Account> {
        match ctx.data::<RemoteActorRef<Authenticator>>()?
            .find_account(name.clone()).await
        {
            Ok(account) => Ok(account.into()),
            Err(e) => {
                if matches!(e.kind(), AnotherlandErrorKind::Application) {
                    let account = ctx.data::<RemoteActorRef<Authenticator>>()?
                        .register(name, email, password).await?;
        
                    Ok(account.into())
                } else {
                    Err(e.into())
                }
            },
        }
    }

    async fn set_password(&self, ctx: &Context<'_>, id: String, password: String) -> Result<Account> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .set_password(Uuid::parse_str(&id)?, password).await?;

        Ok(account.into())
    }

    async fn set_one_time_password(&self, ctx: &Context<'_>, id: String, password: String) -> Result<Account> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .set_one_time_password(Uuid::parse_str(&id)?, password).await?;

        Ok(account.into())
    }

    async fn lock_server(&self, ctx: &Context<'_>) -> Result<&str> {
        let mut authenticator = ctx.data::<RemoteActorRef<Authenticator>>()?.to_owned();
        authenticator.lock_servers().await?;

        Ok("ok")
    }

    async fn unlock_server(&self, ctx: &Context<'_>) -> Result<&str> {
        let mut authenticator = ctx.data::<RemoteActorRef<Authenticator>>()?.to_owned();
        authenticator.unlock_servers().await?;

        Ok("ok")
    }

    async fn avatar_play_animation(&self, ctx: &Context<'_>, session_id: String, avatar_id: String, animation: String, duration: Option<f32>, option: Option<i32>) -> Result<&str> {
        let session = ctx.data::<RemoteActorRef<SessionManager>>()?.get_session(Uuid::parse_str(session_id)?).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;

        if let Some(zone_id) = session.zone_guid {
            let zone = connect_zone(ctx.clone(), zone_id).await?;

            let avatar_id = avatar_id.parse()
                .map_err(|_| async_graphql::Error::new("invalid avatar id"))?;

            zone.execute_command(ApiCommand::UpdateAvatarParams { 
                session_id: session.id, 
                avatar_id, 
                params: [
                    ("action0".to_string(), (
                        animation,
                        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f32()
                    ).into()),
                    ("action0Duration".to_string(), duration.unwrap_or(1.0).into()),
                    ("action0Option".to_string(), option.unwrap_or(1).into()),
                ].into()
            }).await?;

            Ok("ok")
        } else {
            Err(async_graphql::Error::new("character has not entered a zone"))
        }

    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    id: String,
    username: String,
    email: Option<String>,
    created: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>,
    banned: bool,
    ban_reason: Option<String>,
    is_gm: bool,
}

impl From<crate::db::Account> for Account {
    fn from(value: crate::db::Account) -> Self {
        Self {
            id: value.id.to_string(),
            username: value.username,
            email: value.email,
            created: value.created,
            last_login: value.last_login,
            banned: value.banned,
            ban_reason: value.ban_reason,
            is_gm: value.is_gm,
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Realm {
    pub id: u32,
    pub name: String,
    pub address: String,
}

impl From<crate::actors::RealmEntry> for Realm {
    fn from(value: crate::actors::RealmEntry) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            address: value.address.to_string(),
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct Session {
    id: String,
    account: Account,
    is_gm: bool,
    active_realm: Option<Realm>,
    active_character_id: Option<u32>,
    created: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    
    #[graphql(skip)]
    session_id: Uuid,

    #[graphql(skip)]
    realm_id: Option<u32>,

    #[graphql(skip)]
    character_id: Option<u32>,

    #[graphql(skip)]
    zone_id: Option<Uuid>,
}

impl Session {
    pub async fn populate_from_session(ctx: Context<'_>, session: crate::db::Session) -> Result<Session> {
        let account = ctx.data::<RemoteActorRef<Authenticator>>()?
            .get_account(session.account).await?
            .into();

        let active_realm = match session.realm_id {
            Some(id) => ctx.data::<RemoteActorRef<RealmList>>()?
                                .get_realm(id).await
                                .map(|realm| realm.into()),
            None => None
        };

        Ok(Self {
            id: session.id.to_string(),
            account,
            is_gm: session.is_gm,
            active_realm,
            active_character_id: session.character_id,
            created: session.created,
            last_seen: session.last_seen,
            session_id: session.id,
            realm_id: session.realm_id,
            character_id: session.character_id,
            zone_id: session.zone_guid,

        })
    }
}

#[ComplexObject]
impl Session {
    async fn player_avatar(&self, ctx: &Context<'_>) -> Result<Avatar> {
        if let Some(zone_id) = self.zone_id {
            let zone = connect_zone(ctx.clone(), zone_id).await?;

            let avatar_id = if let ApiResult::PlayerAvatar(id) = zone.execute_command(ApiCommand::GetPlayerAvatarId { session_id: self.session_id }).await? {
                Ok(id)
            } else {
                Err(async_graphql::Error::new("unexpected api response"))
            }?;

            debug!("Got player avatar id: {:?}", avatar_id);

            let avatar = if let ApiResult::Avatar(avatar) = zone.execute_command(ApiCommand::GetAvatar { session_id: self.session_id, avatar_id }).await? {
                Ok(avatar)
            } else {
                Err(async_graphql::Error::new("unexpected api response"))
            }?;

            Ok(Avatar {
                id: avatar_id.to_string(),
                instance_id: avatar.instance_id.map(|id| id.to_string()),
                record_id: avatar.record_id.map(|id| id.to_string()),
                name: avatar.name,
                position: avatar.position.to_array(),
                rotation: avatar.rotation.to_array(),
                params: Json(avatar.params)
            })
        } else {
            Err(async_graphql::Error::new("character has not entered a zone"))
        }
    }

    async fn avatar_interest_list(&self, ctx: &Context<'_>) -> Result<Vec<Avatar>> {
        if let Some(zone_id) = self.zone_id {
            let zone = connect_zone(ctx.clone(), zone_id).await?;

            let interests = if let ApiResult::PlayerInterestList(interests) = zone.execute_command(ApiCommand::GetPlayerInterestList { session_id: self.session_id } ).await? {
                Ok(interests)
            } else {
                Err(async_graphql::Error::new("unexpected api response"))
            }?;

            let avatars = {
                let mut result = Vec::new();

                for avatar_id in interests {
                    let avatar = if let ApiResult::Avatar(avatar) = zone.execute_command(ApiCommand::GetAvatar { session_id: self.session_id, avatar_id }).await? {
                        Ok(avatar)
                    } else {
                        Err(async_graphql::Error::new("unexpected api response"))
                    }?;

                    result.push(Avatar {
                        id: avatar_id.to_string(),
                        instance_id: avatar.instance_id.map(|id| id.to_string()),
                        record_id: avatar.record_id.map(|id| id.to_string()),
                        name: avatar.name,
                        position: avatar.position.to_array(),
                        rotation: avatar.rotation.to_array(),
                        params: Json(avatar.params)
                    });
                }

                Ok::<_, async_graphql::Error>(result)
            }?;

            Ok(avatars)
        } else {
            Err(async_graphql::Error::new("character has not entered a zone"))
        }
    }

    async fn target_avatar(&self, ctx: &Context<'_>) -> Result<Option<Avatar>> {
        if let Some(zone_id) = self.zone_id {
            let zone = connect_zone(ctx.clone(), zone_id).await?;

            if let Some(avatar_id) = 
                if let ApiResult::AvatarId(id) = zone.execute_command(ApiCommand::GetSelectedAvatar { 
                    session_id: self.session_id, 
                    avatar_id: None 
                } ).await? {
                    Ok(id)
                } else {
                    Err(async_graphql::Error::new("unexpected api response"))
                }?
            {
                let avatar = if let ApiResult::Avatar(avatar) = zone.execute_command(ApiCommand::GetAvatar { session_id: self.session_id, avatar_id }).await? {
                    Ok(avatar)
                } else {
                    Err(async_graphql::Error::new("unexpected api response"))
                }?;
    
                Ok(Some(Avatar {
                    id: avatar_id.to_string(),
                    instance_id: avatar.instance_id.map(|id| id.to_string()),
                    record_id: avatar.record_id.map(|id| id.to_string()),
                    name: avatar.name,
                    position: avatar.position.to_array(),
                    rotation: avatar.rotation.to_array(),
                    params: Json(avatar.params)
                }))
            } else {
                Ok(None)
            }
        } else {
            Err(async_graphql::Error::new("character has not entered a zone"))
        }
    }
}

struct ZoneConnection {
    connection: quinn::Connection
}

impl ZoneConnection {
    async fn connect(addr: SocketAddr) -> Result<ZoneConnection> {
        let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(ZoneServerSkipVerification::new())
        .with_no_client_auth();

        let mut endpoint = Endpoint::client(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0))?;
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(config)));

        let connection = endpoint
            .connect(addr, "localhost")?
            .await
            .map_err(async_graphql::Error::new_with_source)?;

        Ok(Self {
            connection
        })
    }

    async fn execute_command(&self, cmd: ApiCommand) -> Result<ApiResult> {
        // write request
        {
            let mut buffer = Vec::new();

            bson::to_bson(&ZoneUpstreamMessage::ApiCommand(cmd)).unwrap()
                .as_document().unwrap()
                .to_writer(&mut buffer).unwrap();

            let mut channel = self.connection.open_uni().await?;
            channel.write_all(&buffer).await?;
            channel.finish().await?;
        }

        // read response
        {
            let mut channel = self.connection.accept_uni().await?;
            let mut buffer = Vec::new();

            while let Some(chunk) = channel.read_chunk(usize::MAX, false).await? {
                let computed_size = chunk.bytes.len() + chunk.offset as usize;
    
                if buffer.len() < computed_size {
                    buffer.resize(computed_size, 0);
                }
    
                buffer[chunk.offset as usize..(chunk.offset as usize + chunk.bytes.len())].copy_from_slice(chunk.bytes.as_bytes());
            }

            if let ZoneDownstreamMessage::ApiResult(result) = bson::from_slice(buffer.as_slice()).map_err(|_| AnotherlandError::from_kind(AnotherlandErrorKind::Parse))? {
                if let ApiResult::Error(e) = result {
                    Err(async_graphql::Error::new(e))
                } else {
                    Ok(result)
                }
            } else {
                Err(async_graphql::Error::new("unexpected message"))
            }
        }
    }
}

async fn connect_zone(ctx: Context<'_>, zone_id: Uuid) -> Result<ZoneConnection> {
    let zone_addr = ctx.data::<RemoteActorRef<ZoneRegistry>>()?
        .resolve_zone_address(zone_id).await
        .ok_or(async_graphql::Error::new("failed to resolve zone"))?;

    ZoneConnection::connect(zone_addr).await
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct WorldDef {
    pub id: u16,
    pub guid: String,
    pub name: String,
    pub umap_guid: String,
}

impl WorldDef {
    pub async fn get_from_id(ctx: Context<'_>, id: Uuid) -> Result<WorldDef> {
        let world_def = ctx.data::<RemoteActorRef<crate::actors::Realm>>()?
            .get_world_def(id).await?;

        match world_def {
            Some(world_def) => {
                Ok(Self {
                    id: world_def.id,
                    guid: world_def.guid.to_string(),
                    name: world_def.name,
                    umap_guid: world_def.umap_guid.to_string(),
                })
            },
            None => Err(async_graphql::Error::new("world def not found"))
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Zone {
    pub id: i64,
    pub guid: String,
    pub worlddef: WorldDef,
    pub parent: Option<Box<Zone>>,
    pub name: String,
    pub r#type: i32,
    pub is_instance: bool,
    pub server: String,
    pub level: String,
    pub layer: String,
    pub realu_zone_type: String,
    pub game_controller: String,
}

impl Zone {
    pub fn get_from_id<'a>(ctx: Context<'a>, id: Uuid) -> BoxFuture<'a, Result<Zone>> {
        async move {
            let zone_def = ctx.data::<RemoteActorRef<crate::actors::Realm>>()?
                .get_zone_def(id).await?;

            match zone_def {
                Some(zone_def) => {
                    let parent = if zone_def.parent_zone_guid != UUID_NIL {
                        Some(Box::new(Self::get_from_id(ctx.clone(), zone_def.parent_zone_guid).await?))
                    } else {
                        None
                    };

                    Ok(Self {
                        id: zone_def.id,
                        guid: zone_def.guid.to_string(),
                        worlddef: WorldDef::get_from_id(ctx.clone(), zone_def.worlddef_guid).await?,
                        parent,
                        name: zone_def.zone,
                        r#type: zone_def.zone_type,
                        is_instance: zone_def.is_instance,
                        server: zone_def.server,
                        level: zone_def.level,
                        layer: zone_def.layer,
                        realu_zone_type: zone_def.realu_zone_type,
                        game_controller: zone_def.game_controller,
                    })
                }
                None => Err(async_graphql::Error::new("zone not found"))
            }
        }.boxed()
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Clone, Debug)]
pub struct Avatar {
    pub id: String,
    pub instance_id: Option<String>,
    pub record_id: Option<String>,
    pub name: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub params: Json<ParamBox>,
}
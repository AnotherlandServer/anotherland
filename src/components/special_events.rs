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

use std::sync::Arc;

use atlas::{CommonConfigClass, OaCommonConfigParams, Uuid};
use bson::doc;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use log::debug;
use log4rs::encode::json;
use mongodb::options::FindOptions;
use serde_derive::Deserialize;
use tokio_stream::StreamExt;

use crate::{db::{realm_database, DatabaseRecord, MiscContent}, util::{AnotherlandError, AnotherlandResult}};

pub struct SpecialEvent {
    name: String,
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
    quests: Vec<u32>,
}

pub struct SpecialMapEvent {
    pub event: Arc<SpecialEvent>,
    pub content: Vec<Uuid>,
    pub event_instances: Vec<Uuid>,
    pub hidden_instances: Vec<Uuid>,
}

impl SpecialMapEvent {
    pub fn name(&self) -> &str { &self.event.name }

    pub fn is_active(&self) -> bool {
        let now = Local::now();
        now >= self.event.start_date && now <= self.event.end_date
    }
}

pub struct SpecialEvents {
    events: Vec<Arc<SpecialEvent>>,
}

impl SpecialEvents {
    pub async fn load() -> AnotherlandResult<SpecialEvents> {
        // load special event config
        let event_config = MiscContent::get(
            realm_database().await, 
            &Uuid::parse_str("704156b1-f0e0-4f3c-a815-50a990953abe").unwrap())
            .await?
            .and_then(|mut v| v.data.take())
            .and_then(|v| v.take::<CommonConfigClass>().ok())
            .ok_or(AnotherlandError::app_err("SpecialEventConfig not found"))?;

        // parse json
        let mut events = Vec::new();
        let json_event_array = event_config.value().get("SpecialEvents")
            .and_then(|v| v.as_array())
            .ok_or(AnotherlandError::app_err("Invalid SpecialEventConfig"))?;
        
        for json_event in json_event_array {
            if let Some(event_name) = json_event.get("EventName").and_then(|v| v.as_str()) {
                let event_start = NaiveDate::parse_from_str(
                    json_event.get("EventStart").unwrap().as_str().unwrap(), 
                    "%Y/%m/%-d").unwrap()
                    .and_hms_opt(0, 0, 0).unwrap()
                    .and_local_timezone(Local).unwrap();
                let event_end = NaiveDate::parse_from_str(
                    json_event.get("EventEnd").unwrap().as_str().unwrap(), 
                    "%Y/%m/%-d").unwrap()
                    .and_hms_opt(23, 59, 59).unwrap()
                    .and_local_timezone(Local).unwrap();
                let quests = json_event.get("Quests")
                    .and_then(|v| v.as_array())
                    .map(|v| v.iter().flat_map(|v| v.as_number().map(|v| v.as_u64().unwrap() as u32)).collect::<Vec<_>>())
                    .unwrap_or(Vec::new());

                events.push(Arc::new(SpecialEvent {
                    name: event_name.to_owned(),
                    start_date: event_start,
                    end_date: event_end,
                    quests
                }));
            }
        }

        Ok(SpecialEvents {
            events
        })
    }

    pub async fn get_events_for_map(&self, map: &str) -> AnotherlandResult<Vec<SpecialMapEvent>> {
        let mut result = MiscContent::collection(realm_database().await).find(doc! {
            "data.CommonConfig.value.v.MapName": {"$eq": map}
        }, FindOptions::default()).await?;

        let mut events = Vec::new();

        while let Some(event) = result.try_next().await?
            .and_then(|mut v| v.data.take())
            .and_then(|v| v.take::<CommonConfigClass>().ok()) {
            
            if let Ok(config) = serde_json::from_value::<JsonMapEventConfig>(event.value().to_owned()) {
                // lookup event
                let event = self.events.iter()
                    .find(|v| v.name == config.event_name)
                    .map(|v| v.to_owned())
                    .ok_or(AnotherlandError::app_err("Event not found"))?;

                events.push(SpecialMapEvent {
                    event,
                    content: config.event_content
                        .map(|v| {
                            v.into_iter()
                                .flat_map(|v| Uuid::parse_str(&v.content_guid))
                                .collect()
                        })
                        .unwrap_or_default(),
                    event_instances: config.event_instances.into_iter()
                        .flat_map(|v| Uuid::parse_str(&v.instance_guid))
                        .collect(),
                    hidden_instances: config.hidden_instances.into_iter()
                        .flat_map(|v| Uuid::parse_str(&v.instance_guid))
                        .collect()
                });
            }
        }

        Ok(events)
    }


}

#[derive(Deserialize)]
struct JsonMapEventConfig {
    #[serde(rename = "MapName")]
    map_name: String,
    #[serde(rename = "EventName")]
    event_name: String,
    #[serde(rename = "KismetLevelName")]
    kismet_level_name: String,
    #[serde(rename = "EventStart")]
    event_start: String,
    #[serde(rename = "EventContent")]
    event_content: Option<Vec<JsonMapEventContent>>,
    #[serde(rename = "EventInstances")]
    event_instances: Vec<JsonMapEventInstance>,
    #[serde(rename = "HiddenInstances")]
    hidden_instances: Vec<JsonMapEventInstance>,
}

#[derive(Deserialize)]
struct JsonMapEventContent {
    #[serde(rename = "ContentGUD")] // this is not a typo. 
    content_guid: String,
}

#[derive(Deserialize)]
struct JsonMapEventInstance {
    #[serde(rename = "InstanceGUD")] // this is not a typo. 
    instance_guid: String,
}
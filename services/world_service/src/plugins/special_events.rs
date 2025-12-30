// Copyright (C) 2025 AnotherlandServer
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

use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use bevy::{app::{Plugin, Update}, ecs::{event::Event, message::Message, resource::Resource, schedule::IntoScheduleConfigs}, platform::collections::{HashMap, HashSet}, prelude::{App, Changed, Commands, Entity, Query, Res, ResMut, Trigger, in_state}, tasks::futures_lite::StreamExt};
use chrono::NaiveDate;
use obj_params::{Class, CommonConfig};
use realm_api::RealmApi;
use serde_json::Value;
use toolkit::types::Uuid;

use crate::{error::WorldResult, instance::InstanceState, object_cache::ObjectCache};

use super::{Active, ConnectionState, ContentInfo, CurrentState, PlayerController, ServerAction};

#[derive(Resource)]
struct SpecialEvents(HashMap<String, Arc<SpecialEventConfig>>);

#[derive(Resource, Default)]
struct ActiveEvent(Option<String>);

#[derive(Event, Message)]
pub struct ActivateEvent {
    event_name: String
}

#[allow(dead_code)]
pub struct SpecialEventConfig {
    id: Uuid,
    name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    kismet_event: String,
    event_content: Vec<Uuid>,
    event_instances: Vec<Uuid>,
    hidden_instances: Vec<Uuid>,
}

pub struct SpecialEventsPlugin {
    events: Mutex<Option<SpecialEvents>>
}

impl SpecialEventsPlugin {
    pub async fn new(object_cache: ObjectCache, realm_api: RealmApi, map: &str) -> WorldResult<Self> {
        let mut events = HashMap::new();

        // Special events config
        let special_events_cfg = object_cache.get_object_by_name("SpecialEventConfig").await?
            .ok_or(anyhow!("SpecialEventConfig not found"))?;

        let special_events = special_events_cfg.data
            .get::<_, Value>(CommonConfig::Value)?["SpecialEvents"]
            .as_array().ok_or(anyhow!("SpecialEventConfig is not an array"))?;

        // Lookup event configs for the current map
        let mut cursor = realm_api.query_object_templates()
            .class(Class::CommonConfig)
            .query().await?;

        while let Some(config) = cursor.try_next().await? {
            if 
                let Ok(value) = config.data.get::<_, Value>(CommonConfig::Value) &&
                let Some(map_name) = value["MapName"].as_str() &&
                let Some(event_name) = value["EventName"].as_str() &&
                map_name == map &&
                let Some(event_config) = special_events.iter().find(|event| {
                    if let Some(cmp_event_name) = event["EventName"].as_str() {
                        cmp_event_name == event_name
                    } else {
                        false
                    }
                })
            {
                events.insert(event_name.to_string(), Arc::new(SpecialEventConfig {
                    id: config.id,
                    name: event_name.to_string(),
                    start_date: event_config["EventStart"].as_str()
                        .and_then(|s|NaiveDate::parse_from_str(s, "%Y/%m/%d").ok()).ok_or(anyhow!("invalid EventStart"))?,
                    end_date: event_config["EventEnd"].as_str()
                        .and_then(|s|NaiveDate::parse_from_str(s, "%Y/%m/%d").ok()).ok_or(anyhow!("invalid EventEnd"))?,
                    kismet_event: value["EventStart"].as_str()
                        .ok_or(anyhow!("EventStart not found"))?.to_string(),
                    event_content: value["EventContent"].as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|v| v["ContentGUD"].as_str().and_then(|s| s.parse().ok()))
                        .collect(),
                    event_instances: value["EventInstances"].as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|v| v["InstanceGUD"].as_str().and_then(|s| s.parse().ok()))
                        .collect(),
                    hidden_instances: value["HiddenInstances"].as_array()
                        .unwrap_or(&vec![])
                        .iter()
                        .filter_map(|v| v["InstanceGUD"].as_str().and_then(|s| s.parse().ok()))
                        .collect()
                }));
            }
        }

        Ok(SpecialEventsPlugin {
            events: Mutex::new(Some(SpecialEvents(events))),
        })
    }
}

impl Plugin for SpecialEventsPlugin {
    fn build(&self, app: &mut App) {
        let events = self.events.lock().unwrap()
            .take().expect("SpecialEventsPlugin not initialized");

        app.insert_resource(events);
        app.init_resource::<ActiveEvent>();

        app.world_mut().add_observer(activate_event);

        app.add_systems(Update, trigger_special_event_on_loading_client);
        app.add_systems(Update, hide_all_event_entities
            .run_if(in_state(InstanceState::Initializing))
        );
    }
}

fn hide_all_event_entities(
    events: Res<SpecialEvents>,
    objects: Query<(Entity, &ContentInfo)>,
    mut commands: Commands,
) {
    for event in events.0.values() {
        for (entity, info) in objects.iter() {
            if event.event_instances.contains(&info.placement_id) || event.event_content.contains(&info.template.id) {
                commands.entity(entity)
                    .remove::<Active>();
            }
        }
    }
}

fn activate_event(
    trigger: Trigger<ActivateEvent>,
    mut active_event: ResMut<ActiveEvent>,
    events: Res<SpecialEvents>,
    objects: Query<(Entity, &ContentInfo)>,
    mut commands: Commands,
) {
    let mut hide_instances = HashSet::new();
    let mut hide_content = HashSet::new();
    let mut show_instances = HashSet::new();
    let mut show_content = HashSet::new();

    // Hide all entities that've been visible during the active event,
    // and show all entities that've been hidden during the active event
    if 
        let Some(active_event) = active_event.0.as_ref() &&
        let Some(event) = events.0.get(active_event)
    {
        for id in event.event_instances.iter() {
            hide_instances.insert(*id);
        }

        for id in event.hidden_instances.iter() {
            show_instances.insert(*id);
        }

        for id in event.event_content.iter() {
            hide_content.insert(*id);
        }
    }

    // Show event entities and hide hidden entities for the next event.
    if let Some(event) = events.0.get(&trigger.event_name) {
        for id in event.event_instances.iter() {
            show_instances.insert(*id);
        }

        for id in event.hidden_instances.iter() {
            hide_instances.insert(*id);
        }

        for id in event.event_content.iter() {
            show_content.insert(*id);
        }

        active_event.0 = Some(trigger.event_name.clone());
    } else {
        active_event.0 = None;
    }

    // Apply changes
    for (entity, info) in objects.iter() {
        if show_instances.contains(&info.placement_id) || show_content.contains(&info.template.id) {
            commands.entity(entity)
                .insert(Active);
        } else if hide_instances.contains(&info.placement_id) || hide_content.contains(&info.template.id) {
            commands.entity(entity)
                .remove::<Active>();
        }
    }
}

fn trigger_special_event_on_loading_client(
    query: Query<(&PlayerController, &CurrentState), Changed<CurrentState>>,
    active_event: Res<ActiveEvent>,
    events: Res<SpecialEvents>,
) {
    for (controller, state) in query.iter() {
        if 
            matches!(state.state, ConnectionState::WaitingForInitialInterests) &&
            let Some(active_event) = active_event.0.as_ref() &&
            let Some(event) = events.0.get(active_event)
        {
            controller.send_packet(
                ServerAction::Event(event.kismet_event.clone()).into_pkt()
            );
        }
    }
}


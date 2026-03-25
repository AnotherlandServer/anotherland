// Copyright (C) 2026 AnotherlandServer
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

use bevy::ecs::{component::Component, entity::Entity, error::Result, query::Changed, system::{EntityCommands, Query}};
use futures::future::join_all;
use log::{debug, warn};
use obj_params::{GameObjectData, Player};
use protocol::{oaAbilityDataPlayer, oaAbilityDataPlayerArray};
use realm_api::{ObjectTemplate, RealmApi, State};
use toolkit::types::Uuid;

use crate::plugins::{AbilityOf, AbilityType, CombatStyle, ContentCache, ContentCacheRef, ContentInfo, LoadContext, LoadableComponent, Scripted, WeakCache};

#[derive(Component)]
pub struct Skillbook(Vec<Skill>);

impl Skillbook {
    pub fn to_bytes(&self) -> Vec<u8> {
        oaAbilityDataPlayerArray {
            class_hash: 0x81E0A735,
            count: self.0.len() as u32,
            skills: self.0.iter()
                .map(|s| oaAbilityDataPlayer {
                    version: 0,
                    id: s.id,
                    content_id: s.template_id,
                    group: s.group.clone(),
                    field_4: s.stance,
                })
                .collect(),
        }.to_bytes()
    }
}

#[allow(unused)]
pub struct Skill {
    pub id: Uuid,
    pub template_id: Uuid,
    pub group: String,
    pub state: State,
    pub stance: i32,
    pub ability: Entity,
}

pub struct SkillbookParams {
    pub character_id: Uuid,
    pub level: i32,
    pub combat_style: CombatStyle,
}

impl LoadableComponent for Skillbook {
    type Parameters = SkillbookParams;
    type ContextData = Vec<(Skill, Arc<ObjectTemplate>)>;

    async fn load(Self::Parameters { character_id, level, combat_style }: Self::Parameters, context: &mut LoadContext<<Self as LoadableComponent>::ContextData>) -> Result<Self> {
        let mut skillbook = RealmApi::get()
            .get_or_create_skillbook(character_id).await?;

        if skillbook.combat_style != combat_style.into() {
            debug!("Player combat style does not match skillbook");

            if let Err(e) = skillbook.change_class(combat_style.into(), Some(level)).await {
                warn!("Failed to change skillbook: {e:?}");
            }
        } else if skillbook.character_level != level {
            let _ = skillbook.level_up(level).await;
        }

        let _ = skillbook.unlock_all().await;

        let skills = 
            join_all(skillbook.skills.iter()
                .map(async |s| {
                    if let Ok(Some(ability)) = ContentCache::get(&ContentCacheRef::Uuid(s.ability_id)).await {
                        Some((
                            Skill {
                                id: s.id,
                                template_id: s.ability_id,
                                group: s.group.clone(),
                                state: s.state,
                                stance: s.stance,
                                ability: Entity::PLACEHOLDER,
                            },
                            ability
                        ))
                    } else {
                        None
                    }
                })
            ).await
            .into_iter()
            .flatten()
            .collect();

        context.set_data(skills);

        Ok(Self(vec![]))
    }

    fn post_load(&mut self, commands: &mut EntityCommands<'_>, mut data: Option<Self::ContextData>) -> Result<()> {
        let ent = commands.id();
        let skills = data.take().unwrap_or_default();

        for (mut skill, template) in skills {
            skill.ability = commands
                .commands()
                .spawn((
                    AbilityOf::new(
                        ent, 
                        AbilityType::ClassSkill { 
                            id: skill.id, 
                            group: skill.group.clone(), 
                            state: skill.state, 
                            stance: skill.stance 
                        }),
                    ContentInfo {
                        placement_id: skill.id,
                        template: template.clone(),
                    },
                    GameObjectData::instantiate(template.clone()),
                    Scripted,
                ))
                .id();

            self.0.push(skill);
        }

        Ok(())
    }
}

pub fn network_sync_skillbook(
    mut query: Query<(&mut GameObjectData, &Skillbook), Changed<Skillbook>>,
) {
    for (mut player, skillbook) in query.iter_mut() {
        debug!("Updating skillbook");

        player.set(Player::CurrentClassSkills, skillbook.to_bytes());
    }
}
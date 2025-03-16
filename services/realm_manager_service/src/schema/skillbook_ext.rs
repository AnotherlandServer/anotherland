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

use async_graphql::{Context, Error, Object};
use database::DatabaseRecord;
use mongodb::Database;
use toolkit::types::Uuid;

use crate::db::{self, CombatStyle, Skillbook, SkillbookOutput, State};

#[derive(Default)]
pub struct SkillbookExtMutationRoot;

#[Object]
impl SkillbookExtMutationRoot {
    pub async fn get_or_create_skillbook(&self, ctx: &Context<'_>, character_id: Uuid) -> Result<SkillbookOutput, Error> {
        let db = ctx.data::<Database>()?.clone();
        Ok(
            db::Skillbook::get_or_create(&db, character_id).await?.try_into()?
        )
    }

    pub async fn skillbook_change_class(&self, ctx: &Context<'_>, character_id: Uuid, combat_style: CombatStyle, level: Option<i32>) -> Result<Option<SkillbookOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut skillbook) = Skillbook::get(&db, &character_id).await? {
            skillbook.change_class(&db, combat_style, level.unwrap_or(skillbook.character_level)).await?;
            skillbook.save(&db).await?;

            Ok(Some(skillbook.try_into()?))
        } else {
            Ok(None)
        }
    }

    pub async fn skillbook_level_up(&self, ctx: &Context<'_>, character_id: Uuid, level: i32) -> Result<Option<SkillbookOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut skillbook) = Skillbook::get(&db, &character_id).await? {
            skillbook.level_up(level);
            skillbook.save(&db).await?;

            Ok(Some(skillbook.try_into()?))
        } else {
            Ok(None)
        }
    }

    pub async fn skillbook_unlock_ability(&self, ctx: &Context<'_>, character_id: Uuid, ability_id: Uuid) -> Result<Option<SkillbookOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut skillbook) = Skillbook::get(&db, &character_id).await? {
            // TODO: Deduct price for ability unlock here

            skillbook.unlock_ability(ability_id);
            skillbook.save(&db).await?;

            Ok(Some(skillbook.try_into()?))
        } else {
            Ok(None)
        }
    }
    
    pub async fn skillbook_unlock_all(&self, ctx: &Context<'_>, character_id: Uuid) -> Result<Option<SkillbookOutput>, Error> {
        let db = ctx.data::<Database>()?.clone();
        if let Some(mut skillbook) = Skillbook::get(&db, &character_id).await? {
            for skill in skillbook.skills.iter_mut() {
                if skill.state == State::Locked {
                    skill.state = State::Unlocked;
                }
            }

            skillbook.save(&db).await?;

            Ok(Some(skillbook.try_into()?))
        } else {
            Ok(None)
        }
    }
}
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

use crate::db::{self, AbilityBarOutput};

#[derive(Default)]
pub struct AbilityBarExtMutationRoot;

#[Object]
impl AbilityBarExtMutationRoot {
    pub async fn get_or_create_ability_bar(&self, ctx: &Context<'_>, character_id: Uuid) -> Result<AbilityBarOutput, Error> {
        let db = ctx.data::<Database>()?.clone();
        Ok(
            db::AbilityBar::get_or_create(&db, character_id).await?.try_into()?
        )
    }
}
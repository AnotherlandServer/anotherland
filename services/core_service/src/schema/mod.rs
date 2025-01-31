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

use account::{AccountMutationRoot, AccountRoot};
use async_graphql::MergedObject;
use realm::{RealmMutationRoot, RealmRoot};
use session::{SessionMutationRoot, SessionRoot};
use state::{StateMutationRoot, StateRoot};

mod account;
mod session;
mod state;
mod realm;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    pub AccountRoot,
    pub SessionRoot,
    pub StateRoot,
    pub RealmRoot,
);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    pub AccountMutationRoot,
    pub SessionMutationRoot,
    pub StateMutationRoot,
    pub RealmMutationRoot,
);

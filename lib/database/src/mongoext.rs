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

use mongodb::Database;

use crate::DatabaseRecord;

#[allow(async_fn_in_trait)]
pub trait DatabaseExt {
    async fn init_collection<T: DatabaseRecord>(&self);
}

impl DatabaseExt for Database {
    async fn init_collection<T: DatabaseRecord>(&self) {
        T::build_index(self).await
            .expect("Failed to build collection index");
    }
}
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

use chrono::Utc;
use cynic::impl_scalar;
use serde_json::Value;

#[cynic::schema("realm_manager_service")]
pub mod schema {}

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "JSON")]
pub struct Json(pub Value);

impl_scalar!(obj_params::Class, schema::Class);
impl_scalar!(toolkit::types::Uuid, schema::UUID);
impl_scalar!(chrono::NaiveDate, schema::NaiveDate);
impl_scalar!(chrono::DateTime<Utc>, schema::DateTime);
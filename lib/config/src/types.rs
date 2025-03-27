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

use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ConfLoginServer {
    pub one_time_password_duration: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ConfClusterConfig {
    #[serde(default)]
    pub login: ConfLoginServer,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ConfRealmMain {
}

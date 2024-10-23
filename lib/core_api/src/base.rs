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

use reqwest::{Client, Url};

pub(crate) struct CoreBase {
    pub base_url: Url,
    pub client: Client,
}

#[derive(Clone)]
pub struct CoreApi(pub(crate) Arc<CoreBase>);

unsafe impl Send for CoreApi {}
unsafe impl Sync for CoreApi {}

impl CoreApi {
    pub fn new(url: Url) -> Self {
        CoreApi(Arc::new(
            CoreBase { 
                base_url: url,
                client: Client::new()
            }
        ))
    }
}
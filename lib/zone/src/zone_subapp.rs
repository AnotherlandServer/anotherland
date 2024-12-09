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

use bevy::app::SubApp;
use toolkit::types::Uuid;

use crate::{ZoneConfig, ZoneInstance, ZoneLabel};

pub trait ZoneSubApp {
    fn zone_instance(&self) -> &ZoneInstance;
    fn zone_id(&self) -> Uuid;
    fn instance_id(&self) -> Uuid;
    fn config(&self) -> Arc<ZoneConfig>;
    fn label(&self) -> ZoneLabel;
}

impl ZoneSubApp for SubApp {
    fn zone_instance(&self) -> &ZoneInstance {
        self.world().get_resource::<ZoneInstance>()
            .expect("not a zone subapp")
    }

    fn zone_id(&self) -> Uuid {
        *self.zone_instance().zone.guid()
    }

    fn instance_id(&self) -> Uuid {
        self.zone_instance().instance_id
    }

    fn config(&self) -> Arc<ZoneConfig> {
        self.zone_instance().config.clone()
    }

    fn label(&self) -> ZoneLabel {
        let instance = self.zone_instance();
        ZoneLabel::new(*instance.zone.guid(), instance.instance_id)
    }
}
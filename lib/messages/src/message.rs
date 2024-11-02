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

use flexbuffers::FlexbufferSerializer;
use serde::{Deserialize, Serialize};
use zeromq::ZmqMessage;

use crate::{Error, MessageResult};

pub trait StructuredMessage<'de>: Serialize + Deserialize<'de> {
    fn topic_name() -> &'static str;

    fn into_message(self) -> MessageResult<ZmqMessage> {
        let mut s = FlexbufferSerializer::new();
        self.serialize(&mut s)?;

        let mut msg = ZmqMessage::from(Self::topic_name());
        msg.push_back(s.take_buffer().into());

        Ok(msg)
    }

    fn from_message(msg: &'de ZmqMessage) -> MessageResult<Self> {
        let topic = 
            msg.get(0)
            .ok_or(Error::EmptyMessage)?
            .as_ascii()
            .ok_or(Error::InvalidTopic)?;

        if topic.as_str() != Self::topic_name() {
            return Err(Error::InvalidTopic);
        }

        Ok(
            flexbuffers::from_slice(
                msg.get(1)
                .ok_or(Error::EmptyMessage)?
            )?
        )
    }
}
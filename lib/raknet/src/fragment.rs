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

use std::collections::HashMap;

use crate::{error::Result, frame::{Message, MessageFrame, Order}};

struct Fragment {
    pub compound_size: u32,
    pub order: Option<Order>,
    pub frames: HashMap<u32, Message>,
}

impl Fragment {
    pub fn new(
        compound_size: u32,
        order: Option<&Order>
    ) -> Self {
        Self {
            compound_size,
            order: order.cloned(),
            frames: HashMap::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.frames.len() >= self.compound_size as usize
    }

    pub fn insert(&mut self, message: Message) {
        if self.is_full() { return; }

        if let Some(split) = message.split() {
            if self.frames.contains_key(&split.index) { return; }

            self.frames.insert(split.index, message);
        }
    }

    pub fn merge(mut self) -> Result<Message> {
        let mut buf = vec![];

        let mut keys: Vec<u32> = self.frames.keys().cloned().collect();
        keys.sort_unstable();

        let message_number = self.frames[keys.last().unwrap()].message_number();
        let reliability = self.frames[keys.last().unwrap()].reliability();

        for i in keys {
            buf.append(self.frames.get_mut(&i).unwrap().data_mut());
        }

        let mut ret = Message::new(message_number, reliability, buf);
        if let Some(order) = self.order { ret.set_order(order); }

        Ok(ret)
    }
}

pub struct FragmentQ {
    fragments: HashMap<u16, Fragment>,
}

impl FragmentQ {
    pub fn new() -> Self {
        Self {
            fragments: HashMap::new(),
        }
    }

    pub fn insert(&mut self, frame: Message) {
        if let Some(split) = frame.split() {
            if let Some(fragments) = self.fragments.get_mut(&split.id) {
                fragments.insert(frame);
            } else {
                let mut v = Fragment::new(split.count, frame.order());
                let k = split.id;

                v.insert(frame);
                self.fragments.insert(k, v);
            }
        } else {
            panic!("Tried to reassemble unsplit packet");
        }
    }

    pub fn flush(&mut self) -> Result<Vec<Message>> {
        let mut ret = vec![];

        let keys: Vec<u16> = self.fragments.keys().cloned().collect();

        for i in keys {
            let a = self.fragments.get_mut(&i).unwrap();
            if a.is_full() {
                let a = self.fragments.remove(&i).unwrap();
                ret.push(a.merge()?);
            }
        }

        Ok(ret)
    }
}
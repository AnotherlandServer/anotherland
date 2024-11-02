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

use std::ops::{Deref, DerefMut};

use log::warn;
use nom::{combinator::fail, error::{context, VerboseError}, number, IResult};

use crate::{Attribute, Value};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Unchanged,
    Changed,
}

pub trait GenericParam {
    fn attribute(&self) -> impl Attribute;
    fn state(&self) -> State;
    fn mark_changed(&mut self);
    fn clear_state(&mut self);
    fn is_changed(&self) -> bool;
}

#[derive(Clone)]
pub struct Param<T: Attribute> {
    attr: T,
    state: State,
    val: Value,
}

impl <T: Attribute> Param <T> {
    pub fn new(attr: T, val: Value) -> Self {
        Self {
            attr,
            state: State::Changed,
            val
        }
    }

    pub fn attribute(&self) -> &T { &self.attr }
    pub fn value(&self) -> &Value { &self.val }
    pub fn set_value(&mut self, val: Value) { self.val = val; }

    pub fn take(self) -> Value { self.val }

    pub(crate) fn from_slice(i: &[u8]) -> IResult<&[u8], Param<T>, VerboseError<&[u8]>> {
        let (i, attribute_id) = context("Attribute Id", number::complete::le_u16)(i)?;
        let attr: T = match attribute_id.try_into() {
            Ok(attr) => attr,
            Err(_) => {
                warn!("failed to parse attribute id {}", attribute_id);
                return fail(i);
            },
        };

        let (i, val) = Value::from_slice(i, attr.flags())?;
        Ok((i, Self {
            attr,
            state: State::Unchanged,
            val
        }))
    }
}

impl <T: Attribute> GenericParam for Param <T> {
    fn attribute(&self) -> impl Attribute {
        self.attr
    }

    fn state(&self) -> State {
        self.state
    }

    fn mark_changed(&mut self) {
        self.state = State::Changed
    }

    fn clear_state(&mut self) {
        self.state = State::Unchanged
    }

    fn is_changed(&self) -> bool {
        self.state == State::Changed
    }
} 

impl <T: Attribute> Deref for Param <T> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl <T: Attribute> DerefMut for Param <T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.state = State::Changed;
        &mut self.val
    }
}
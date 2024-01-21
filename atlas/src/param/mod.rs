// Copyright (C) 2023 AnotherlandServer
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

mod data;
mod flags;
mod error;
mod traits;
mod types;
mod set;
mod parambox;
mod paramsetbox;

pub use data::*;
pub use flags::*;
pub use error::*;
pub use traits::*;
pub use types::*;
pub use set::*;
pub use parambox::*;
pub use paramsetbox::*;

mod test {
    use std::cell::Ref;
    use std::{sync::Arc, cell::RefCell};

    use serde_json::Value;

    use crate::{ParamSet, Param};
    use crate::PlayerAttribute;

    pub struct TestClass(Arc<RefCell<ParamSet<PlayerAttribute>>>);

    impl TestClass {
        fn test<'a>(&'a self) -> Option<Ref<'a, Value>> {
            Ref::filter_map::<Value, _>(self.0.borrow(), 
                |v| 
                    v.get(&PlayerAttribute::Alive)
                    .map(|v| v.try_into().ok())
                    .flatten()
                )
            .ok()
        }

        fn test2<'a>(&'a self) -> Option<bool> {
            self.0.borrow().get(&PlayerAttribute::Alive).map(|v| v.try_into().ok()).flatten()
        }

        fn address_slots<'a, T>(&'a self) -> Option<Ref<'a, T>>
        where
            for<'b> &'b T: TryFrom<&'b Param>,
        {
            Ref::filter_map::<T,_>(
                    self.0.borrow(),
                    |v| {
                        v
                            .get(&PlayerAttribute::AddressSlots)
                            .map(|v| v.try_into().ok())
                            .flatten()
                    },
                )
                .ok()
        }
    }
}
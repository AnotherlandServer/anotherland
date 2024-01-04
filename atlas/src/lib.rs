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

pub mod parsers;
mod param;
mod nativeparam;
mod generated;
mod network_vec3;
mod network_vec4;
mod serialize;
mod avatarid;
mod buffer;
pub mod raknet;

pub use param::*;
pub use nativeparam::*;
pub use generated::*;
pub use avatarid::*;
pub use buffer::*;

#[cfg(test)]
mod tests {
    use crate::{NativeParam, oaPktClusterNodeToClient, CPkt, raknet::Message};
    use uuid::Uuid;

    #[test]
    fn nativeparam_serialization() {
        let original = NativeParam::Struct([
            NativeParam::AvatarId(1),
            NativeParam::Bool(true)
        ].to_vec());

        let bytes = original.to_struct_bytes();
        let serialized = NativeParam::parse_struct(&bytes).unwrap().1;

        assert_eq!(format!("{:#?}", original), format!("{:#?}", serialized));
    }

    #[test]
    fn oa_pkt_cluster_node_to_client_serialization() {
        let mut original = oaPktClusterNodeToClient::default();
        original.field_1 = Uuid::new_v4();
        original.field_3 = NativeParam::Struct([
            NativeParam::AvatarId(1),
            NativeParam::Bool(true)
        ].to_vec());

        let bytes = original.clone().into_message().to_bytes();
        let serialized = Message::from_bytes(&bytes).unwrap().1;

        println!("{:#?}", original.clone().into_message().to_bytes());

        assert_eq!(format!("{:#?}", serialized), format!("{:#?}", Message::AtlasPkt(CPkt::oaPktClusterNodeToClient(Box::new(original)))));
    }
}
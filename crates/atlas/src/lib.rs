pub mod parsers;
mod uuid;
mod cparam;
mod nativeparam;
mod generated;
mod defaults;
pub mod raknet;

pub use cparam::*;
pub use self::uuid::*;
pub use nativeparam::*;
pub use generated::*;
pub use defaults::*;

#[cfg(test)]
mod tests {
    use crate::{NativeParam, oaPktClusterNodeToClient, CPkt, raknet::Message};

    use super::Uuid;

    #[test]
    fn uuid_serialization() {
        let uuid = Uuid::new_v4();
        let bytes = uuid.to_bytes();
        let deserialized = Uuid::from_bytes(&bytes).unwrap().1;

        assert_eq!(deserialized, uuid);
    }

    #[test]
    fn uuid_default() {
        let uuid_1 = Uuid::new_v4();
        let uuid_2 = Uuid::default();

        assert_eq!(uuid_1.to_bytes().len(), uuid_2.to_bytes().len());
    }

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

        let bytes = original.as_message().to_bytes();
        let serialized = Message::from_bytes(&bytes).unwrap().1;

        println!("{:#?}", original.as_message().to_bytes());

        assert_eq!(format!("{:#?}", serialized), format!("{:#?}", Message::AtlasPkt(CPkt::oaPktClusterNodeToClient(Box::new(original)))));
    }
}
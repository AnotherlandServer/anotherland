use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use super::PacketDefintion;

pub struct GeneratedStruct {
    name: String,
    fields: Vec<Rc<RefCell<GeneratedField>>>,
    fields_mapped: HashMap<String, Rc<RefCell<GeneratedField>>>,
}

pub struct GeneratedField {
    name: String,
    r#type: GeneratedFieldType,
    optional: bool,
}

pub enum GeneratedFieldType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    String,
    Array(Box<GeneratedFieldType>),
    Struct(Rc<RefCell<GeneratedStruct>>)
}

impl GeneratedStruct {
    pub fn generate_from_packet_defintion(defintion: &PacketDefintion, struct_registry: &HashMap<String, Rc<RefCell<GeneratedStruct>>>) {
        todo!()
    }
}

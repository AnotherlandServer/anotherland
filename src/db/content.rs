use serde_derive::{Deserialize, Serialize};

use atlas::{Uuid, CParamClass};

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub guid: Uuid,
    pub name: String,
    pub class: u16,
    pub data: Option<CParamClass>,
}

pub enum GameContent {
    NoBinding(Content),
    Buff(Content),
    Drop(Content),
    Enemie(Content),
    Faction(Content),
    Item(Content),
    Metagame(Content),
    Misc(Content),
    Npc(Content),
    Projectile(Content),
    Quest(Content),
    Recipe(Content),
    Skill(Content),
    Spawner(Content),
    Structure(Content),
}
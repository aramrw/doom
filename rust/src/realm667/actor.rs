use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ActorDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub ed_number: Option<i32>,
    pub properties: HashMap<String, String>,
    pub flags: Vec<String>,
    pub states: HashMap<String, Vec<StateFrame>>,
}

#[derive(Debug, Clone)]
pub struct StateFrame {
    pub sprite_prefix: String,
    pub frames: String, // e.g., "ABCD"
    pub duration: i32,  // in tics (1/35th of a second)
    pub action: Option<String>, // e.g., "A_Look", "A_FireBullets"
    pub is_bright: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ActorCategory {
    Weapon,
    Enemy,
    NPC,
    Item,
    Projectile,
    Ammo,
    Unknown,
}

impl ActorDefinition {
    pub fn determine_category(&self) -> ActorCategory {
        if self.flags.contains(&"Monster".to_string()) {
            return ActorCategory::Enemy;
        }
        if self.parent.as_deref() == Some("Weapon") || self.properties.contains_key("Weapon.AmmoType") {
            return ActorCategory::Weapon;
        }
        if self.flags.contains(&"Projectile".to_string()) {
            return ActorCategory::Projectile;
        }
        if self.parent.as_deref() == Some("Ammo") || self.parent.as_deref() == Some("Inventory") {
             // Basic heuristic, can be refined
            return ActorCategory::Item;
        }
        ActorCategory::Unknown
    }
}

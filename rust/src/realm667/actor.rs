use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ActorDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub ed_number: Option<i32>,
    pub properties: HashMap<String, GZValue>,
    pub flags: Vec<String>,
    pub states: HashMap<String, Vec<StateFrame>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GZValue {
    Integer(i32),
    Float(f64),
    String(String),
    Identifier(String),
}

impl GZValue {
    pub fn to_string_lossy(&self) -> String {
        match self {
            GZValue::Integer(i) => i.to_string(),
            GZValue::Float(f) => f.to_string(),
            GZValue::String(s) => s.clone(),
            GZValue::Identifier(id) => id.clone(),
        }
    }
}

impl std::fmt::Display for GZValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GZValue::Integer(i) => write!(f, "{}", i),
            GZValue::Float(fl) => write!(f, "{}", fl),
            GZValue::String(s) => write!(f, "\"{}\"", s),
            GZValue::Identifier(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GZFunctionCall {
    pub name: String,
    pub args: Vec<GZValue>,
}

impl std::fmt::Display for GZFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StateFrame {
    pub sprite_prefix: String,
    pub frames: String,
    pub duration: i32,
    pub action: Option<GZFunctionCall>,
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
        let lower_name = self.name.to_lowercase();
        let parent = self.parent.as_deref().unwrap_or("").to_lowercase();

        if self.flags.contains(&"Monster".to_string()) || parent.contains("enemy") {
            return ActorCategory::Enemy;
        }

        if parent == "weapon"
            || self.properties.contains_key("Weapon.AmmoType")
            || self.properties.contains_key("Weapon.SlotNumber")
        {
            return ActorCategory::Weapon;
        }

        if self.flags.contains(&"Projectile".to_string())
            || self.properties.contains_key("Projectile")
            || parent.contains("projectile")
        {
            return ActorCategory::Projectile;
        }

        if parent == "ammo" || lower_name.contains("ammo") {
            return ActorCategory::Ammo;
        }

        if parent == "inventory" || parent == "custominventory" || parent.contains("item") {
            return ActorCategory::Item;
        }

        ActorCategory::Unknown
    }
}

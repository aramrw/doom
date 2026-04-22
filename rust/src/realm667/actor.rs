use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ActorDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub ed_number: Option<i32>,
    pub properties: HashMap<String, GZValue>,
    pub flags: Vec<String>,
    pub states: HashMap<String, Vec<StateFrame>>,
    pub methods: HashMap<String, Vec<GZFunctionCall>>,
    pub metadata: HashMap<String, String>,
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
    pub actions: Vec<GZFunctionCall>,
    pub is_bright: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActorCategory {
    Weapon,
    Enemy,
    NPC,
    Item,
    Projectile,
    Ammo,
    Prop,
    Unknown,
}

impl ActorDefinition {
    pub fn determine_category(&self) -> ActorCategory {
        let lower_name = self.name.to_lowercase();
        let parent = self.parent.as_deref().unwrap_or("").to_lowercase();
        let l_flags: Vec<String> = self.flags.iter().map(|f| f.to_lowercase().replace('+', "").replace('-', "")).collect();

        // 1. Check metadata hints first
        if let Some(cat) = self.metadata.get("category") {
            let cat_lower = cat.to_lowercase();
            if cat_lower.contains("decoration") || cat_lower.contains("obstacle") || cat_lower.contains("light source") {
                return ActorCategory::Prop;
            }
            if cat_lower.contains("weapon") {
                return ActorCategory::Weapon;
            }
            if cat_lower.contains("enemy") || cat_lower.contains("monster") {
                return ActorCategory::Enemy;
            }
        }

        // 2. Heuristics
        if l_flags.contains(&"projectile".to_string())
            || l_flags.contains(&"missile".to_string())
            || self.properties.contains_key("Projectile")
            || parent.contains("projectile")
            || lower_name.contains("missile")
            || lower_name.contains("projectile")
        {
            return ActorCategory::Projectile;
        }

        if l_flags.contains(&"monster".to_string())
            || parent.contains("enemy")
            || parent.contains("monster")
            || self.states.contains_key("See")
            || self.states.contains_key("Missile")
            || self.states.contains_key("Melee")
        {
            return ActorCategory::Enemy;
        }

        if parent == "ammo" || lower_name.contains("ammo") {
            return ActorCategory::Ammo;
        }

        if parent == "inventory" || parent == "custominventory" || parent.contains("item") || lower_name.contains("pickup") {
            return ActorCategory::Item;
        }

        // Detect Props
        if l_flags.contains(&"solid".to_string()) 
            || self.properties.contains_key("Radius") 
            || self.properties.contains_key("Height")
            || lower_name.contains("statue")
            || lower_name.contains("pillar")
            || lower_name.contains("column")
            || lower_name.contains("brazier")
        {
            return ActorCategory::Prop;
        }

        ActorCategory::Unknown
    }
}

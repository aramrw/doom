# Design Spec: GZDoom Asset Importer (Rust/Godot)

## 1. Objective
Build a robust, GDExtension-based importer in Rust that parses any GZDoom DECORATE or ZScript actor/class and automatically generates the corresponding Godot resources and scenes (Weapons, Enemies, NPCs, Items).

## 2. Parser Architecture (Rust)

### 2.1 Three-Phase Parser
1.  **Pre-processor:** Recursively handles `#include` directives within the PK3/ZIP.
2.  **Lexical Analysis:** Tokenizes the source, handling ZScript-specific block scopes (`Default`, `States`) and optional semi-colons.
3.  **Semantic Parsing:** 
    *   **Class Discovery:** Identifies `class` and `actor` definitions.
    *   **Inheritance Resolver:** Maps the class hierarchy to ensure children inherit properties and states from parents.
    *   **Intent Extraction:** Parses the `States` block to identify action functions (e.g., `A_FireBullets`, `A_FireProjectile`) and their arguments.

### 2.2 Data Representation
```rust
pub struct GZDoomClass {
    pub name: String,
    pub parent: Option<String>,
    pub category: ActorCategory,
    pub properties: HashMap<String, GZValue>,
    pub flags: HashSet<String>,
    pub states: HashMap<String, Vec<StateLine>>,
}

pub enum GZValue {
    Integer(i32),
    Float(f64),
    String(String),
    Identifier(String),
}

pub struct StateLine {
    pub sprite_prefix: String,
    pub frames: String,
    pub duration: i32,
    pub action: Option<GZFunctionCall>,
}
```

## 3. Godot Mapping & Resource Generation

### 3.1 Template Matching
*   **Weapons:** Inherit from `WeaponData` resource; generate `AnimatedSprite3D` based scenes.
*   **Enemies:** Inherit from `DoomEnemyBase`; generate `CharacterBody3D` with navigation and collision.
*   **Items:** Generate `Area3D` or `StaticBody3D` pickup logic.

### 3.2 Function-to-Effect Mapping
The importer will map GZDoom built-ins to project-specific `WeaponEffect` resources:
*   `A_FireBullets` -> `HitscanEffect`
*   `A_FireProjectile` -> `ProjectileEffect`
*   `A_PlaySound` -> `SoundEffect`

## 4. Variable Renaming (Alignment with GZDoom)
To improve clarity, existing Godot variables will be renamed to match GZDoom's naming conventions:

### WeaponData
| Old Name | New Name |
| :--- | :--- |
| `max_bullets` | `ammo_give` |
| `inventory_slot` | `slot_number` |
| `ammo_type` | `ammo_type` (Matches) |

### DoomEnemyBase (Enemy/NPC)
| Old Name | New Name |
| :--- | :--- |
| `attack_damage` | `damage` |
| `attack_range` | `meleerange` (Melee) or `missilerange` (Projectile) |
| `attack_cooldown` | `reactiontime` |
| `speed` | `speed` (Matches) |
| (New Field) | `pain_chance` |

### Player
| Old Name | New Name |
| :--- | :--- |
| `NORMAL_SPEED` | `speed` |
| `SPRINT_SPEED` | `runspeed` |

## 5. Success Criteria
*   Parser successfully ingest any DECORATE/ZScript actor from a PK3.
*   Importer generates functional Godot resources (`.tres`) and scenes (`.tscn`).
*   The generated assets use the original GZDoom property values and state timings.

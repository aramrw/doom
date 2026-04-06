# GZDoom Asset Importer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a robust GDExtension-based importer in Rust that parses GZDoom assets and generates Godot resources, while aligning variable names with GZDoom conventions.

**Architecture:** A three-phase Rust parser (Pre-processor, Lexer, Semantic Parser) that produces a GZDoomClass hierarchy, which is then mapped to Godot .tres and .tscn files.

**Tech Stack:** Rust (godot-rust), GDScript, Godot 4.x.

---

### Phase 1: Godot Variable Refactoring

#### Task 1: Rename Player Variables
**Files:**
- Modify: `player/player.gd`
- Modify: `ui/dialogue_ui.gd` (any references to speed)

- [ ] **Step 1: Rename speed variables in player.gd**
```gdscript
# player/player.gd
@export var speed = 5.0 # was NORMAL_SPEED
@export var runspeed = 7.0 # was SPRINT_SPEED
```

- [ ] **Step 2: Update references in player.gd**
Replace all `NORMAL_SPEED` with `speed` and `SPRINT_SPEED` with `runspeed`.

- [ ] **Step 3: Commit**
```bash
git add player/player.gd
git commit -m "refactor: rename player speed variables to match GZDoom"
```

#### Task 2: Rename WeaponData Variables
**Files:**
- Modify: `weapons/weapon_data.gd`
- Modify: `weapons/weapon_manager.gd`
- Modify: `ui/bullets.gd`

- [ ] **Step 1: Rename variables in weapon_data.gd**
```gdscript
# weapons/weapon_data.gd
@export var slot_number: int = 1 # was inventory_slot
@export var ammo_give: int = 10 # was max_bullets
```

- [ ] **Step 2: Update references in weapon_manager.gd**
Update any code using `inventory_slot` to `slot_number` and `max_bullets` to `ammo_give`.

- [ ] **Step 3: Commit**
```bash
git add weapons/weapon_data.gd weapons/weapon_manager.gd
git commit -m "refactor: rename weapon variables to match GZDoom"
```

#### Task 3: Rename Enemy Variables
**Files:**
- Modify: `enemies/grin/doom_enemy.gd`

- [ ] **Step 1: Rename variables in doom_enemy.gd**
```gdscript
# enemies/grin/doom_enemy.gd
@export var damage: int = 5 # was attack_damage
@export var meleerange: float = 2.0 # was attack_range
@export var reactiontime: float = 1.5 # was attack_cooldown
@export var pain_chance: int = 50 # New field
```

- [ ] **Step 2: Commit**
```bash
git add enemies/grin/doom_enemy.gd
git commit -m "refactor: rename enemy variables to match GZDoom"
```

---

### Phase 2: Rust Core Refactoring

#### Task 4: Define GZValue and GZFunctionCall Enums/Structs
**Files:**
- Modify: `rust/src/realm667/actor.rs`

- [ ] **Step 1: Update actor.rs with new types**
```rust
// rust/src/realm667/actor.rs
#[derive(Debug, Clone, PartialEq)]
pub enum GZValue {
    Integer(i32),
    Float(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GZFunctionCall {
    pub name: String,
    pub args: Vec<GZValue>,
}
```

- [ ] **Step 2: Update StateFrame to use GZFunctionCall**
```rust
pub struct StateFrame {
    pub sprite_prefix: String,
    pub frames: String,
    pub duration: i32,
    pub action: Option<GZFunctionCall>,
    pub is_bright: bool,
}
```

- [ ] **Step 3: Commit**
```bash
git add rust/src/realm667/actor.rs
git commit -m "feat(rust): define GZValue and GZFunctionCall types"
```

---

### Phase 3: Parser Implementation

#### Task 5: Implement Pre-processor for #include
**Files:**
- Modify: `rust/src/realm667/mod.rs`

- [ ] **Step 1: Improve read_zip_file_recursive**
Ensure it correctly joins paths and handles nested includes.

- [ ] **Step 2: Commit**
```bash
git add rust/src/realm667/mod.rs
git commit -m "feat(rust): improve PK3 pre-processor for recursive includes"
```

#### Task 6: Update Lexer for ZScript Support
**Files:**
- Modify: `rust/src/realm667/lexer.rs`

- [ ] **Step 1: Add support for more characters and tokens**
Ensure `.` and `$` are handled in identifiers.

- [ ] **Step 2: Commit**
```bash
git add rust/src/realm667/lexer.rs
git commit -m "feat(rust): update lexer for ZScript compatibility"
```

#### Task 7: Implement Semantic Parser for Classes
**Files:**
- Modify: `rust/src/realm667/parser.rs`

- [ ] **Step 1: Update parse_actor to handle Default and States blocks**
Correctly parse properties into `HashMap<String, GZValue>`.

- [ ] **Step 2: Implement A_ function argument parsing**
Extract arguments for `A_FireBullets`, `A_FireProjectile`, etc.

- [ ] **Step 3: Commit**
```bash
git add rust/src/realm667/parser.rs
git commit -m "feat(rust): implement robust ZScript/DECORATE class parser"
```

---

### Phase 4: Resource Generation

#### Task 8: Implement Enemy Resource Generation
**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Add generate_enemy_resources function**
Generate `.tscn` and `.tres` for enemies using the `DoomEnemyBase` template.

- [ ] **Step 2: Commit**
```bash
git add rust/src/realm667/resource_gen.rs
git commit -m "feat(rust): add enemy resource generation"
```

#### Task 9: Update Importer to use new Generator
**Files:**
- Modify: `rust/src/realm667/mod.rs`

- [ ] **Step 1: Call generate_enemy_resources in import_pk3 loop**

- [ ] **Step 2: Commit**
```bash
git add rust/src/realm667/mod.rs
git commit -m "feat(rust): integrate enemy generation into PK3 importer"
```

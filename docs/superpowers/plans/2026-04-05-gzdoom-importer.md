# Modular Weapon System & GZDoom Importer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a lossless ZScript/DECORATE parser in Rust and a data-driven weapon resource generator in Godot to allow native `.pk3` imports with a CS:GO-style slot structure.

**Architecture:** The Rust GDExtension parses ZScript functions into an AST (`FunctionCall`). This AST is then translated into a hierarchy of Godot Resources (`WeaponData`, `WeaponAction`, `ActionStep`, `WeaponEffect`) based on a mapping schema, preserving exact timings and logic.

**Tech Stack:** Rust (gdext), Godot 4.3 (GDScript, Resources), ZScript/DECORATE.

---

### Task 1: Upgrade Rust Lexer for Function Arguments

**Files:**
- Modify: `rust/src/realm667/lexer.rs`
- Modify: `rust/src/realm667/parser.rs`
- Create: `rust/src/realm667/parser_tests.rs`

- [ ] **Step 1: Write a failing test for Lexer extracting complex function calls**

```rust
// In rust/src/realm667/parser_tests.rs
#[cfg(test)]
mod tests {
    use crate::realm667::lexer::{Lexer, Token};

    #[test]
    fn test_lexer_function_args() {
        let input = "A_FireBullets(5.6, 0, 1, 5, \"BulletPuff\")";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Identifier("A_FireBullets".to_string()));
        assert_eq!(lexer.next_token(), Token::Operator("(".to_string()));
        // Note: The current lexer returns Token::Number(5) then Operator(".") then Number(6)
        // We need to ensure it handles floats properly or the parser can reconstruct them.
    }
}
```

- [ ] **Step 2: Run the test to verify it fails/passes incorrectly**

Run: `cd rust && cargo test test_lexer_function_args`

- [ ] **Step 3: Update the Lexer to handle floats and negative numbers correctly**

Modify `read_number` in `lexer.rs` to capture decimal points and leading minuses.

```rust
// In rust/src/realm667/lexer.rs
    fn read_number_or_float(&mut self) -> Token {
        let start = self.pos;
        let mut has_decimal = false;
        
        // Handle negative numbers
        if self.pos < self.input.len() && self.input[self.pos] == '-' {
            self.pos += 1;
        }

        while self.pos < self.input.len() && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.') {
            if self.input[self.pos] == '.' {
                if has_decimal { break; } // Second decimal point means end of number
                has_decimal = true;
            }
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        Token::NumberStr(s) // Use a string to preserve precision/negatives for now
    }
```
*(Note: You will need to update the `Token` enum to use `NumberStr(String)` instead of `Number(i32)` and fix compiler errors in the parser).*

- [ ] **Step 4: Commit**

```bash
git add rust/src/realm667/lexer.rs rust/src/realm667/parser_tests.rs
git commit -m "test: upgrade lexer to handle floats and negative numbers"
```

### Task 2: Build the AST for Function Calls in Rust

**Files:**
- Modify: `rust/src/realm667/actor.rs`
- Modify: `rust/src/realm667/parser.rs`

- [ ] **Step 1: Define the AST structs**

```rust
// In rust/src/realm667/actor.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Value>,
}

// Update StateFrame to use FunctionCall instead of Option<String>
#[derive(Debug, Clone)]
pub struct StateFrame {
    pub sprite_prefix: String,
    pub frames: String,
    pub duration: i32,
    pub action: Option<FunctionCall>,
    pub is_bright: bool,
}
```

- [ ] **Step 2: Write a failing parser test for `FunctionCall`**

```rust
// In rust/src/realm667/parser_tests.rs
    use crate::realm667::actor::{FunctionCall, Value};
    use crate::realm667::parser::Parser;

    #[test]
    fn test_parse_function_call() {
        let input = "SHTG A 3 A_FireBullets(5.6, 0, 1, 5, \"BulletPuff\")";
        let mut parser = Parser::new(input);
        let frames = parser.parse_state_line(); // Assuming we expose this for testing
        
        assert_eq!(frames.len(), 1);
        let action = frames[0].action.clone().unwrap();
        assert_eq!(action.name, "A_FireBullets");
        assert_eq!(action.args.len(), 5);
        assert_eq!(action.args[0], Value::Number(5.6));
        assert_eq!(action.args[4], Value::String("BulletPuff".to_string()));
    }
```

- [ ] **Step 3: Update `parse_state_line` to extract arguments**

```rust
// In rust/src/realm667/parser.rs
    // ... inside parse_state_line ...
        let mut action = None;
        if let Token::Identifier(a) = self.cur_token.clone() {
            if a.starts_with("A_") {
                let mut func = FunctionCall { name: a, args: Vec::new() };
                self.next_token();
                
                if self.cur_token == Token::Operator("(".to_string()) {
                    self.next_token(); // skip (
                    while self.cur_token != Token::Operator(")".to_string()) && self.cur_token != Token::Eof {
                        match &self.cur_token {
                            Token::NumberStr(s) => {
                                if let Ok(n) = s.parse::<f64>() {
                                    func.args.push(Value::Number(n));
                                }
                            },
                            Token::StringLiteral(s) => func.args.push(Value::String(s.clone())),
                            Token::Identifier(s) => func.args.push(Value::Identifier(s.clone())),
                            Token::Comma => {}, // skip
                            _ => {}
                        }
                        self.next_token();
                    }
                    self.next_token(); // skip )
                }
                action = Some(func);
            }
        }
```

- [ ] **Step 4: Run tests and commit**

Run: `cd rust && cargo test`

```bash
git add rust/src/realm667/actor.rs rust/src/realm667/parser.rs
git commit -m "feat: parse ZScript function arguments into AST"
```

### Task 3: Create Godot Resource Classes

**Files:**
- Create: `weapons/effects/weapon_effect.gd`
- Create: `weapons/effects/raw_zscript_effect.gd`
- Modify: `weapons/weapon_action.gd`
- Modify: `weapons/action_step.gd`
- Modify: `weapons/weapon_data.gd`

- [ ] **Step 1: Create base and specific Effect classes**

```gdscript
# weapons/effects/weapon_effect.gd
extends Resource
class_name WeaponEffect

func execute(_weapon_manager: Node3D, _target_data: Dictionary = {}):
	pass
```

```gdscript
# weapons/effects/raw_zscript_effect.gd
extends WeaponEffect
class_name RawZScriptEffect

@export var function_name: String = ""
@export var arguments: Array = []

func execute(_weapon_manager: Node3D, _target_data: Dictionary = {}):
	print("Executing raw ZScript: ", function_name, " with args: ", arguments)
```

- [ ] **Step 2: Update Data, Action, and Step**

```gdscript
# weapons/weapon_data.gd (Update)
@export var inventory_slot: int = 0 # 0=Primary, 1=Secondary, 2=Melee, 3=Utility
@export var ammo_type: String = "Bullets"
```

```gdscript
# weapons/action_step.gd (Update)
extends Resource
class_name ActionStep

@export var frame_index: int = 0
@export var duration: float = 0.085 # Doom tics * 0.0285
@export var effects: Array[WeaponEffect] = []
```

```gdscript
# weapons/weapon_action.gd (Update)
extends Resource
class_name WeaponAction

@export var animation_name: String = "shoot"
@export var steps: Array[ActionStep] = []
```

- [ ] **Step 3: Commit**

```bash
git add weapons/
git commit -m "feat: setup godot resource structure for weapon effects"
```

### Task 4: Upgrade Rust Resource Generator

**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Map `FunctionCall` to Godot Effect Strings**

```rust
// In rust/src/realm667/resource_gen.rs
use crate::realm667::actor::{ActorDefinition, Value};

impl ResourceGenerator {
    fn generate_effect_resource(func: &crate::realm667::actor::FunctionCall) -> String {
        match func.name.as_str() {
            "A_FireBullets" => {
                let damage = if func.args.len() > 3 {
                    if let Value::Number(d) = func.args[3] { d } else { 5.0 }
                } else { 5.0 };
                format!(
r#"[gd_resource type="Resource" script_class="HitscanEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/hitscan_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
damage = {}
"#, damage)
            },
            "A_PlaySound" => {
                // ... map sound ...
                format!(
r#"[gd_resource type="Resource" script_class="SoundEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/sound_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
"#)
            },
            _ => {
                // Fallback to RawZScriptEffect
                let args_str = format!("{:?}", func.args); // Needs proper formatting for Godot Array
                format!(
r#"[gd_resource type="Resource" script_class="RawZScriptEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/raw_zscript_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
function_name = "{}"
arguments = []
"#, func.name)
            }
        }
    }
}
```

- [ ] **Step 2: Generate ActionSteps based on Tics**

*Modify `generate_weapon_resources` to loop over `actor.states` correctly, create an `ActionStep.tres` for each frame, and calculate `duration = tics * (1.0 / 35.0)`.*

- [ ] **Step 3: Compile and Test via Godot**
Run: `cd rust && cargo build`

- [ ] **Step 4: Commit**

```bash
git add rust/src/realm667/resource_gen.rs
git commit -m "feat: generate mapped weapon effects and precise action steps"
```

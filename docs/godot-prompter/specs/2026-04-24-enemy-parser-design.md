# Enemy Parser and Resource Generator Design

## Overview
The goal is to achieve "one-click" automated importing of GZDoom/DECORATE/ZSCRIPT enemies into Godot 4.x. The current system is functional but requires manual cleanup for many enemy types. This design focuses on hardening the `ResourceGenerator` and `nom_parser` to handle mapping automatically, ensuring robust TSCN and SpriteFrames generation similar to the successful `Cultist` implementation.

## Architecture

### 1. Robust and Strict Parsing
- **Strict Validation:** The parser will be strictly deterministic. If the parser encounters a ZScript/DECORATE construct, animation state, or sprite reference that it cannot definitively map to a Godot equivalent, it MUST return an `Err` or `panic`, halting the import process for that specific actor.
- **Fail-Fast & Cleanup:** Upon encountering an unrecoverable error during actor parsing or resource generation, the importer will immediately halt, and the `ResourceGenerator` must clean up any partially generated files (TSCN, `.tres` resources, etc.) to prevent corrupt imports.
- **Intrinsically Driven Mapping:** Animation mappings are derived 1:1 from the actor's state definitions. The system will map every defined state label found in the definition. If a label or sprite reference is not understood according to GZDoom's sprite naming conventions (e.g., `XXXXA1`), the parser will refuse to proceed.
- **Keymap-Driven:** The parser will resolve every state, animation frame, and sprite reference against the PK3's manifest, ensuring the sprite exists and the frame timing/actions are valid. If a referenced asset is missing, the importer must fail and log the missing resource clearly.

### 2. TSCN Standardization
- **Component-Based:** All enemy TSCNs will be generated using a standard scene tree:
  - `CharacterBody3D` (root, with `doom_enemy.gd`)
  - `CollisionShape3D` (CapsuleShape3D, sized based on `Radius`/`Height`)
  - `AnimatedSprite3D` (for visual rendering)
  - `NavigationAgent3D` (for AI movement)
  - `RayCast3D` (for line-of-sight/attack checks)
  - `AudioStreamPlayer3D` (linked to `EnemySounds`)
- **Properties Injection:** All actor properties defined in DECORATE (Health, Speed, etc.) will be injected via `export` variables on the `doom_enemy.gd` script or directly into the generated TSCN structure.

### 3. Automated Projectile/Sound Handling
- **Sounds:** The system will automatically map `SNDINFO` aliases to sound files in the PK3, extracting them into a standard `sounds/mod_name/category/` structure (e.g., `Death`, `Pain`, `Taunt`).
- **Projectiles:** If an enemy's ZScript references a known projectile (or if the importer can detect a projectile actor in the same script), the importer will automatically generate the corresponding TSCN for the projectile and link it.

## Testing Strategy
- **Unit Tests (Rust):** Continue expanding `parser_tests.rs` with complex actor definitions that include all standard GZDoom property and state permutations.
- **Integration Tests:** Add a suite of test PK3 files representing "easy" (rat), "medium" (cultist-like), and "hard" (complex multi-state enemy) cases. Success is defined as the generated TSCN being draggable into a scene and functioning as expected without modification.

## Design Approval
Please review this approach for automating the enemy import process. If this architecture looks correct, I will proceed to write the implementation plan.

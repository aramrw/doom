# Modular Weapon System & GZDoom Importer Design

## Overview
A high-fidelity, 100% automated ZScript/DECORATE importer and weapon system for Godot. The goal is to allow native loading of `.pk3` files into the game, replicating GZDoom's weapon logic exactly while providing a human-readable, data-driven structure in the Godot Inspector. 

This enables the game to act as a modern GZDoom engine runner, supporting complex CS:GO-style slot management (Primary, Secondary, Melee, Utility) while retaining the wild variety of Realm667 assets.

## Core Principles
1. **Lossless Parsing**: The Rust parser will extract all arguments from ZScript functions (e.g., `A_FireBullets(5, 4, -1, 5, "Puff")`) into structured data, ensuring no GZDoom logic is lost.
2. **Data-Driven Logic**: Weapon behavior is entirely defined by Godot Resources (`WeaponData`, `WeaponAction`, `ActionStep`, `WeaponEffect`), making it easy to tweak or extend without touching code.
3. **1:1 Time Translation**: Doom Tics (35 frames per second) are perfectly translated to Godot seconds (`tics * 0.0285`).
4. **Schema-Driven Extensibility**: The mapping of ZScript functions to Godot effects is driven by a schema, allowing new GZDoom features to be supported without recompiling the Rust extension.

## Architecture

### 1. Rust Deserialization Layer (The "Compiler")
The Rust GDExtension (`rsdoom`) acts as the parser and compiler.

* **Lexer/Parser Upgrade**: Enhances the existing parser to fully tokenize ZScript function calls, capturing arguments of mixed types (numbers, strings, identifiers, flags).
* **AST Structures**:
  ```rust
  struct FunctionCall {
      name: String, // e.g., "A_FireBullets"
      args: Vec<Value>, // e.g., [Number(5.6), Number(0), Number(1), Number(5), String("BulletPuff")]
  }

  struct StateFrame {
      sprite_prefix: String,
      frames: String,
      duration: i32, // In Doom Tics
      action: Option<FunctionCall>,
  }
  ```
* **Test-Driven Automation**: The parsing logic will be verified via a comprehensive Rust test suite to ensure complex ZScript strings are accurately mapped to the `FunctionCall` structs before any Godot resources are generated.

### 2. Godot Resource Structure (The "Runtime")
The parsed AST is translated into a hierarchy of Godot Resources.

* **`WeaponData` (Root Resource)**
  * Represents the item itself (e.g., CS:GO slot integration).
  * `inventory_slot`: Integer (0=Primary, 1=Secondary, 2=Melee, 3=Utility).
  * `ammo_type`: String.
  * `actions`: Dictionary mapping ZScript labels (e.g., "Fire", "AltFire") to `WeaponAction` resources.
  * `sprite_frames`: Links to the generated animation resource.

* **`WeaponAction` (The State Machine Entry)**
  * Represents a sequence of events triggered by player input.
  * `animation_name`: String (e.g., "shoot").
  * `steps`: Array of `ActionStep` resources.

* **`ActionStep` (A Single Frame)**
  * Represents a specific point in time during the animation.
  * `duration`: Float (seconds).
  * `effects`: Array of `WeaponEffect` resources to execute simultaneously.

* **`WeaponEffect` (The Behavior)**
  * Polymorphic resources executing specific game logic.
  * `HitscanEffect`: Maps to `A_FireBullets`, `A_CustomPunch`. Contains damage, spread, and puff scene references.
  * `ProjectileEffect`: Maps to `A_FireProjectile`. Contains speed, damage, and projectile scene references.
  * `SoundEffect`: Maps to `A_PlaySound`.
  * `RawZScriptEffect`: A fallback/catch-all resource storing the unmapped `FunctionCall` data to ensure lossless import.

### 3. Implementation Strategy (The Schema)
To avoid hardcoding every possible ZScript function in Rust, the importer will use a mapping schema.
* When Rust encounters `A_FireBullets`, it consults the schema to know that `Arg[0]` maps to the `spread` property of a `HitscanEffect` resource, and `Arg[3]` maps to `damage`.
* Unrecognized functions fall back to generating a `RawZScriptEffect` so the data remains visible in the Godot Inspector for manual wiring or future extension updates.

## Testing Strategy
1. **Rust Unit Tests**:
   * Parse simple functions: `A_PlaySound("weapons/shotgf")` -> verifies string extraction.
   * Parse complex functions: `A_FireBullets (5.6, 0, 1, 5, "BulletPuff")` -> verifies float, int, and string extraction in order.
   * Parse state blocks: Verify duration math (`3 tics = ~0.085s`) and frame grouping.
2. **Integration Tests (Godot)**:
   * Load a generated `WeaponData.tres`.
   * Trigger the "Fire" action.
   * Verify the correct sequence of `HitscanEffect` and `SoundEffect` fires at the correct timestamps.

## Conclusion
This design bridges the gap between GZDoom's flexible scripting and Godot's visual, node-based workflow. By treating ZScript as structured data and mapping it losslessly to Godot resources, the game achieves native `.pk3` import capability while establishing a robust foundation for a CS:GO-style tactical weapon system.
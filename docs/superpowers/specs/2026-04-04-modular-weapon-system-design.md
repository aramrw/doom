# Design Spec: Modular 2.5D Weapon System (Next-Gen Doom)

## 1. Overview
A decoupled, resource-driven weapon system for a 2.5D Godot game. The system allows for "infinite" weapon types (Hitscan, Projectile, Melee, Magic, Chained Combos) by separating weapon data from the logic that executes attacks. It is designed to be compatible with Realm667/ZDoom standards (DECORATE) and leverages modern Godot features (3D lighting, particles) while maintaining a retro 2D aesthetic.

## 2. Goals
- **Modularity:** The `WeaponManager` and `Player` should be "dumb" vessels that don't know the specifics of a weapon.
- **Extensibility:** Adding a new weapon type should only require creating new `Resource` files, not modifying core code.
- **Frame-Precision:** Trigger damage, sounds, and visuals on specific animation frames (Option A).
- **Modern 2.5D:** Support billboarded 2D sprites in 3D with point lights, fog, and particle effects.
- **Importer Ready:** Data structures should map closely to Doom's `DECORATE` states and actions.

## 3. Architecture

### 3.1 Core Components
- **`WeaponManager` (Node3D):**
    - The "Socket" in the Player scene.
    - Handles weapon switching and input propagation.
    - Manages the `AnimatedSprite2D` and `AudioStreamPlayer`.
    - Listens for frame changes to trigger events.
- **`WeaponData` (Resource):**
    - The "Container" for a specific weapon (e.g., TOZ-34, Plasma Rifle).
    - Contains: `weapon_name`, `sprite_frames`, `scale`, `offset`, and a Dictionary of `WeaponAction` resources (mapped to "Fire", "AltFire", "Reload").
- **`WeaponAction` (Resource):**
    - A "State Machine" or "Sequence" defining a single move (e.g., Shooting).
    - Contains an array of `ActionStep` resources.
    - Logic for looping, branching (combos), or charging.
- **`ActionStep` (Resource):**
    - Represents a single "Frame" of the action (e.g., `TOZF A 1`).
    - Contains: `frame_index`, `duration`, and an array of `WeaponEffect` resources.

### 3.2 Modular Effects (`WeaponEffect` Resource)
Effects are independent plugins that do one thing.
- **`HitscanEffect`**: Performs RayCast3D checks for bullets/melee. (Stats: damage, pellets, spread, range).
- **`ProjectileEffect`**: Spawns a `BaseProjectile` scene. (Stats: scene_path, speed, gravity).
- **`SoundEffect`**: Plays an `AudioStream`.
- **`VisualEffect`**: Triggers screen shake, toggles a `PointLight3D` (muzzle flash), or emits particles.
- **`AmmoEffect`**: Modifies the weapon's internal ammo count or the player's global inventory.

### 3.3 The 2.5D Projectile Scene (`BaseProjectile.tscn`)
- **`Area3D`**: For collision detection.
- **`Sprite3D`**: Billboarded sprite for the 2D look.
- **`PointLight3D`**: For glowing projectiles (Fireballs, Plasma).
- **`GPUParticles3D`**: For trails (Smoke, Magic).
- **`Collision Logic`**: Calls `take_damage()` on targets and triggers an "Explosion" state/sprite upon impact.

## 4. Data Flow
1. **Input:** `Player` detects `shoot` -> calls `WeaponManager.fire()`.
2. **Action Start:** `WeaponManager` tells `WeaponData.primary_action` to `execute()`.
3. **Execution:** The `Action` tells the `WeaponManager` which animation to play.
4. **Events:** As the `AnimatedSprite2D` plays, `WeaponManager` checks the current `ActionStep` for the active frame.
5. **Impact:** Any `WeaponEffect` in that step is triggered (e.g., a `HitscanEffect` performs a raycast and applies damage).

## 5. UI Integration
The HUD becomes generic:
- `WeaponData` includes a `hud_hint` (e.g. `AMMO_SHELLS`, `ENERGY_BAR`).
- When equipped, the HUD matches the hint and displays the appropriate sub-module (e.g., showing the shell count or a charging energy bar).

## 6. Future Expansion: The Realm667 Importer
The design directly supports an `EditorScript` that can parse `DECORATE` text:
- `TOZF A 1` -> Creates an `ActionStep` with `frame_index: 0` and `duration: 1/35s`.
- `A_FireBullets` -> Adds a `HitscanEffect` to that step.
- `A_PlayWeaponSound` -> Adds a `SoundEffect` to that step.

## 7. Implementation Plan
1. **Phase 1: Refactor Resources.** Create `WeaponEffect`, `ActionStep`, `WeaponAction`, and the new `WeaponData`.
2. **Phase 2: The Generic Manager.** Update `WeaponManager` to handle action execution and frame-event listening.
3. **Phase 3: The Projectile System.** Create the `BaseProjectile` scene.
4. **Phase 4: Sample Weapons.** Rebuild the Pistol (Hitscan) and create a Fireball Staff (Projectile) to verify.
5. **Phase 5: Melee.** Implement the Combat Knife using the `HitscanEffect` with short range.
6. **Phase 6: Generic HUD.** Update `hud.gd` to respond to `WeaponData` hints.

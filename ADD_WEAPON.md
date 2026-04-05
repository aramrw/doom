# Modular Weapon Integration Guide

This document specifies the architecture and workflow for adding new weapons to the game. It serves as a blueprint for manual integration and a specification for future automation tools (e.g., Rust-based Realm667 importers).

## Weapon Architecture Overview

Weapons are defined as a hierarchy of Godot resources (`.tres` files), making them modular and data-driven.

### 1. Resource Hierarchy

1.  **WeaponData (`weapon_data.gd`)**: The root resource.
    *   Links to `SpriteFrames`.
    *   Contains a dictionary of `WeaponAction` resources (e.g., "primary", "secondary").
    *   Defines `max_bullets`, `weapon_name`, and visual offsets.
2.  **SpriteFrames (`.tres`)**: Defines the animations (`idle`, `shoot`, `reload`).
3.  **WeaponAction (`weapon_action.gd`)**: Defines a high-level behavior.
    *   Links to an array of `ActionStep` resources.
    *   Properties: `animation_name`, `loop` (for auto-fire), `consumes_ammo`.
4.  **ActionStep (`action_step.gd`)**: Defines what happens during a specific frame of an animation.
    *   Links to an array of `WeaponEffect` resources.
    *   Properties: `frame_index`.
5.  **WeaponEffect (`weapon_effect.gd`)**: The base class for logic triggered during a step.
    *   **HitscanEffect**: Damage, spread, impact logic.
    *   **SoundEffect**: Audio stream, pitch randomness.
    *   **ProjectileEffect**: Spawns a physical projectile.

---

## Step-by-Step Integration Workflow

### Phase 1: Asset Preparation
1.  Identify the source assets (e.g., Realm667 PK3/Zip).
2.  Organize sprites into sub-folders for clarity: `idle/`, `shoot/`, `reload/`.
3.  Ensure sounds are in `.ogg` or `.wav` format.

### Phase 2: SpriteFrames Definition
Create `[weapon_name]_spriteframes.tres`.
*   **Idle**: Usually a single frame (Looping: True).
*   **Shoot**: The firing sequence (Looping: False).
*   **Reload**: The full reload sequence (Looping: False).

### Phase 3: Action & Effect Logic
1.  **Create Effects**:
    *   `[weapon_name]_fire_effect.tres` (HitscanEffect): Set damage and spread.
    *   `[weapon_name]_fire_sound.tres` (SoundEffect): Link the firing sound.
2.  **Create Step**:
    *   `[weapon_name]_fire_step.tres` (ActionStep): Assign the effects and set `frame_index` (usually frame 0 or 1 of the shoot animation).
3.  **Create Action**:
    *   `[weapon_name]_fire_action.tres` (WeaponAction): Link the step, set `animation_name = "shoot"`, and toggle `loop` for automatic weapons.

### Phase 4: Weapon Data Assembly
Create `[weapon_name]_wpdata.tres`.
*   Link the `SpriteFrames`.
*   Populate the `actions` dictionary: `{"primary": ExtResource("...fire_action.tres")}`.
*   Set `max_bullets` and `hud_hint`.

### Phase 5: Player Registration
1.  Open `player.tscn`.
2.  Locate the `WeaponManager` node.
3.  Assign the `wpdata.tres` to one of the weapon slots (`primary_weapon`, `secondary_weapon`, or `third_weapon`).

---

## Automation Considerations for Realm667 Imports

A Rust/Python parser can automate Phase 2, 3, and 4 by:
1.  **Reading DECORATE**: Mapping `States` to `WeaponAction` and `ActionStep`.
2.  **Reading SNDINFO**: Resolving sound aliases to paths for `SoundEffect`.
3.  **Regex Sprite Matching**: Using Doom's 4-character sprite prefix + frame letter (e.g., `MACIA0`) to automatically build `SpriteFrames`.
4.  **Template Generation**: Using a boilerplate `.tres` template and replacing UIDs and paths.

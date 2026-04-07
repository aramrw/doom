# Unified Pickup System & Robust Importer Progress

## Completed Features
1.  **Unified Pickup Component (`items/pickup.gd`)**: 
    *   Handles `PickupResource` (base class for `WeaponData` and `ItemData`).
    *   Supports `AUTO`, `MANUAL`, or `BOTH` modes.
    *   Automatically shows "G TO PICKUP [ITEM]" in the HUD via a new label.
2.  **Weapon Swapping & Dropping**:
    *   Updated `WeaponManager.gd` to swap with the current slot when inventory is full.
    *   Spawns a dropped weapon pickup in front of the player.
3.  **Rust Importer Upgrades (`rsdoom`)**:
    *   **Animation Fidelity**: Now uses DECORATE frame durations (tics) to set Godot SpriteFrames durations at 35 FPS.
    *   **Auto-Pickup Generation**: Generates `[name]_pickup.tscn` with explicit `SpriteFrames` linkage and `autoplay="ground"`.
    *   **Resource Scanning**: Automatically triggers a Godot filesystem scan after import to ensure new textures are registered.

## Current State
*   **"G" Key**: Added to the `interact` input map in `project.godot`.
*   **Holy Snow**: Successfully imported with fixed projectile linkage and animation timing.
*   **Interaction**: Refined with proximity fallbacks and explicit "G to pickup" HUD hints.

## Pending Tasks (Next Session)
1.  **WAD Robustness**: Improve the Rust parser's robustness when reading raw WAD structures inside zip files to prevent asset deletion/skipping.
2.  **Sound Logic**: Verify SoundEffect triggers in `ActionStep` are correctly playing for all imported weapons.

# Design Spec: Literal State Mapping for Realm667 Importer

## Goal
Improve the accuracy of the Realm667 importer by mapping Doom actor states directly to Godot animations instead of compressing them into a single `idle` state.

## Architecture

### 1. Asset Extractor Updates
- The `extract_actor_assets_clean` function in the Rust importer will no longer use a fixed `label_to_folder` mapping for Props.
- For `ActorCategory::Prop`, it will use the lowercase version of the state label (e.g., `Spawn`, `Active`, `Inactive`) as both the folder name and the animation name.
- Resulting directory structure: `godot_data/sprites/<mod_name>/<actor_name>/<state_name>/<frame>.png`.

### 2. Resource Generator Updates
- `generate_prop_resources` will iterate through all captured states in the `ActorDefinition`.
- Each state will become a distinct animation in the `SpriteFrames` resource.
- **Autoplay Logic**: 
    1. Prefer `spawn` if available.
    2. Fallback to `active` if available.
    3. Fallback to the first animation alphabetically.
- All 4 volumetric `AnimatedSprite3D` nodes in the `.tscn` will be configured to use this `autoplay` animation.

## Data Flow
1. **Parser**: Captures all states and their frames from DECORATE.
2. **Inheritance Resolver**: Ensures child actors have the full set of parent states.
3. **Asset Extractor**: Creates individual folders per state and extracts frames into them.
4. **Generator**: Builds `SpriteFrames` with multiple named animations and creates the `.tscn`.

## Validation
- Verify `BRZ1G0` (inactive frame) is in an `inactive` animation, not at the end of `idle`.
- Verify `FSTAA0-C` are in `active` and `FSTAD0` is in `inactive`.
- Verify standard props like `ImpStatue` still work using their `spawn` state.

Here is a comprehensive spec detailing the GZDoom file formats for your Rust lexer/parser, followed by the exact mapping of those parsed constructs into your Godot architecture. 

### Part 1: GZDoom File & Syntax Spec (The Input)

GZDoom relies on two primary archive formats and several C-like scripting languages for internal logic. 

#### 1. The Containers: PK3 vs. WAD
Your parser needs to handle extracting data from these two distinct packaging methods before lexing the text.

* **PK3 (Modern):** This is literally a standard ZIP archive. Your current `zip` crate implementation handles this perfectly. The structure acts as a namespace (e.g., `sprites/`, `sounds/`, `mapinfo/`). The root directory contains the primary definition files like `DECORATE`, `ZSCRIPT`, and `SNDINFO`.
* **WAD (Legacy):**  If you plan to support classic Doom files, a WAD is not a zipped hierarchy. It has a strict binary layout:
    * **Header (12 bytes):** * `4 bytes`: Magic string (`IWAD` or `PWAD`).
        * `4 bytes`: Integer (Little Endian) defining the number of lumps (files).
        * `4 bytes`: Integer (Little Endian) defining the pointer/offset to the directory.
    * **Directory (Array of 16-byte structs):**
        * `4 bytes`: Offset to the lump's data.
        * `4 bytes`: Size of the lump's data.
        * `8 bytes`: ASCII name of the lump (padded with null bytes, e.g., `POSSA1\0\0`).
        * *Note:* WADs use "Marker Lumps" (0-byte files like `S_START` and `S_END`) to define virtual directories for sprites and sounds.

#### 2. DECORATE / ZSCRIPT Lexical Spec
For `src/realm667/lexer.rs`, GZDoom scripts are heavily heavily reliant on standard C-family tokenization. 

* **Comments:** `//` for line, `/* ... */` for block. ZScript also supports nested block comments.
* **Identifiers:** Alphanumeric + underscores. Often case-insensitive in GZDoom. 
* **Strings:** Enclosed in `"`. Can contain escape characters (`\n`, `\"`).
* **Numbers:** Integers (`10`) and Floats (`10.5`). 
* **Punctuation:** `{ }` for scoping, `;` for line termination (enforced in ZScript, sometimes optional in DECORATE), `:` for inheritance and state labels.

#### 3. Actor / Class Syntax Parsing
For `src/realm667/parser.rs`, the structure of an entity follows this hierarchy:

```c
// DECORATE
ACTOR <Name> : <Parent> <DoomEdNumber> replaces <ReplacedActor>
{
    <Property> <Value>
    +<FLAG>
    -<FLAG>
    States
    {
        <Label>:
            <Sprite> <Frames> <Duration> <Keyword/Action>
            Loop
    }
}
```

**State Frame Breakdown:**
This is the hardest part to parse correctly because it mixes strings and integers without clear delimiters.
* `Sprite`: Exactly 4 characters (e.g., `POSS`, `Q2BL`).
* `Frames`: 1 or more characters defining the animation sequence (e.g., `A`, `AB`, `ABCD`). Each character maps to a separate frame.
* `Duration`: Integer defining how many engine tics (1/35th of a second) the frame lasts. Negative numbers usually mean "infinite".
* `Keywords`: `Bright` (disables lighting/shading), `Offset(x,y)`.
* `Action`: Functions like `A_Look()`, `A_FireProjectile("Rocket")`. In ZScript, this can be an entire anonymous C++ style block `{ A_CustomPunch(); A_PlaySound(); }`.
* `Flow Control`: `Loop`, `Stop`, `Wait`, `Fail`, `Goto <Label>`.

---

### Part 2: GZDoom to Godot Mapping Spec (The Output)

Based on your current Rust codebase and GDScript files, here is how the GZDoom AST maps directly to your Godot Engine constructs. 

#### 1. Global Definitions
* **`SNDINFO` -> Godot File System:** GZDoom maps logical sound names (`weapons/quakeaxe`) to physical files (`QAXEFIR.ogg`). Your `AssetHandler` uses this to extract the correct `.ogg`/`.wav` files into the `godot_data/sounds/` directory.

#### 2. The Entity (Actor/Class) Hierarchy
* **DECORATE `ACTOR` / ZSCRIPT `Class` -> Godot `PackedScene` (`.tscn`):**
    * **Enemy:** Maps to `CharacterBody3D` using `res://enemies/grin/doom_enemy.gd`.
    * **Weapon:** Maps to a logical `WeaponData` resource (no physical scene for the weapon logic, just UI/Manager data) and an `Area3D` for the floor pickup (`pickup.tscn`).
    * **Projectile:** Maps to `Area3D` using `res://weapons/projectiles/base_projectile.gd`.
    * **Prop:** Maps to `StaticBody3D` (if solid) using `res://assets/base_prop.gd`.

#### 3. Properties & Flags
* **`Health`, `Speed`, `Radius`, `Height` -> Exported Node Variables:** Mapped directly to the `.tscn` root node script variables. GZDoom sizes must be divided by ~40 to convert Doom Units to Godot Meters. Speeds must be converted from Doom units per tic to meters per second (`val / 40.0 * 35.0`).
* **`Weapon.SlotNumber` -> `WeaponData.slot_number`**
* **`+SOLID` -> `CollisionShape3D`:** Determines if a prop generates a collision node.

#### 4. The States Block -> State Machine & Visuals
* **State Labels (`Spawn:`, `Ready:`, `Fire:`) -> Godot Animations:**
    * The `ResourceGenerator` collapses the GZDoom frames into `SpriteFrames` (`.tres`).
    * `Spawn` generally maps to `"idle"` or `"walk"`.
    * `Fire` maps to `"shoot"`.
    * Each frame char (e.g., `Q2BL A`, `Q2BL B`) is converted to an `[ext_resource type="Texture2D"]` pointing to the extracted PNG. The Doom `Duration` determines the Godot frame duration.

#### 5. State Actions -> WeaponData / ActionSteps
Because Godot doesn't execute DECORATE code natively, your system abstracts actions into custom Resources.
* **State Block `Fire:` -> `WeaponAction` (`.tres`):** Represents an entire attack sequence.
* **Individual Frame -> `ActionStep` (`.tres`):** Links a specific animation frame index to a list of effects.
* **GZDoom Functions -> `WeaponEffect` Subclasses:**
    * `A_FireBullets()` -> `HitscanEffect` (Sets `damage`, `pellets`, `spread_angle`).
    * `A_FireProjectile()` -> `ProjectileEffect` (Sets `projectile_scene` to the target `.tscn`).
    * `A_StartSound()` / `A_PlaySound()` -> `SoundEffect`.
    * `A_Quake()` / `A_Recoil()` -> `CameraShakeEffect`.

---

### Architectural Review: Alternative Approaches

While the current spec you've built is functional, it's worth exploring alternative architectural points of view, as your current trajectory introduces a few technical bottlenecks.

**1. Lexer/Parser Fragility**
Building a manual, character-by-character lexer (`src/realm667/lexer.rs`) for ZScript is incredibly difficult to maintain. ZScript is practically a full object-oriented programming language.
* *Alternative:* Instead of hand-rolling the parser, using a Rust parser combinator framework like `nom` or a lexer generator like `logos` would allow you to define the GZDoom grammar much more rigidly and safely, saving you from endless edge-case bugs when parsing complex ZScript anonymous function blocks.

**2. Texture Memory and Draw Call Inefficiency**
Currently, your `ResourceGenerator` creates individual `Texture2D` `[ext_resource]` entries for *every single frame of animation* and injects them into a `SpriteFrames` resource. For a mod with dozens of monsters (each with 8-directional sprites), this will cause Godot to load thousands of individual image files, resulting in massive I/O bottlenecks and high RAM usage.
* *Alternative:* Rather than mapping directly to `AnimatedSprite3D` and `SpriteFrames`, build a packing step into your Rust importer that combines all the extracted PNGs for an actor into a single `SpriteSheet` (Texture Atlas). You can then map GZDoom frames to Godot `AtlasTexture` resources or use a `Sprite3D` combined with an `AnimationPlayer` that simply manipulates the `region_rect`.

**3. Hardcoded Action Mapping**
Right now, `ResourceGenerator` heavily relies on hardcoded string checks (`if name == "A_FireProjectile"`). If a PK3 uses a custom ZScript function that inherits from `A_FireProjectile`, your parser will miss it entirely and fail to generate the `ProjectileEffect`.
* *Alternative:* Consider compiling ZScript actions into an intermediate bytecode or mapping them directly to auto-generated GDScript files attached to the nodes, rather than strictly trying to force them into a predefined `WeaponEffect` Resource struct. This allows modders to bring over custom ZScript logic without breaking your importer

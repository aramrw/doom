# Implementation Plan for Slitherfist Enemy

**Goal:** Implement the Slitherfist enemy with its defined movement, attack patterns, health, and animations.

**Envisioned Functionality:**
The Slitherfist will be a medium-tier enemy, posing a dynamic threat with both close-range and area-denial capabilities.
*   **Appearance:** A serpentine creature with a humanoid torso ending in powerful, bladed "fists." Its skin will be dark, perhaps with glowing green or purple veins.
*   **Movement Patterns:**
    *   **Slithering Chase:** The primary movement will be slithering along the ground using `NavigationAgent3D` to pursue the player. It will adapt to terrain and obstacles.
    *   **Charge Attack:** When at mid-range and not attacking, it might occasionally initiate a rapid charge towards the player, leading into a melee attack.
    *   **Evade/Reposition:** If the player is too close and it's attempting a ranged attack, or if its health is low, it might quickly slither sideways or backward to create distance.
*   **Attack Types:**
    *   **Melee - "Blade Swipe":** When within close proximity to the player, the Slitherfist will perform a quick, wide swipe with its bladed fists, dealing moderate damage. This attack will have a short wind-up and cooldown.
    *   **Ranged - "Corrosive Spit":** From a medium distance, the Slitherfist will rear back and spit a slow-moving, glowing projectile. Upon impact with the environment or the player, this projectile will explode into a small, temporary puddle of corrosive goo that deals continuous damage to anything standing in it. This encourages player movement and creates environmental hazards.
*   **Health:** Moderate health (e.g., 80-120 HP), making it a manageable but persistent threat.
*   **Animations:** Idle, Slither, Charge (Start/Loop/End), Melee (Windup/Attack/Recover), Ranged (Windup/Spit/Recover), Hit, Death.
*   **Sound Effects:** Ambient hisses, slithering, melee attack sounds, ranged attack sounds (gurgle, thwip, sizzle), hit, death shriek.
*   **AI Logic (State Machine):** Idle, Pursue, MeleeAttack, RangedAttack, Evade, Death.
*   **Visual Effects:** Hit flash shader, corrosive puddle particle effect and shader.

**Approach:** Component-based system with a Node-based State Machine for AI logic.

---

**Scene Tree Design for Slitherfist:**

```
Slitherfist (CharacterBody3D) # Root: handles overall enemy state, AI logic, input from components
├── CollisionShape3D          # Defines the enemy's physical body for collisions
├── MeshInstance3D            # (Optional) If we use a 3D model instead of AnimatedSprite3D
│   └── AnimationPlayer       # (If MeshInstance3D is used) For 3D model animations
├── AnimatedSprite3D          # Visual representation (if using 2D sprites in 3D world)
├── NavigationAgent3D         # For pathfinding and movement
├── RayCast3D                 # For obstacle avoidance or line-of-sight checks
├── HealthComponent (Node)    # Manages health, damage, and death state
├── AttackComponent (Node)    # Handles melee and ranged attack logic (timing, damage)
├── CorrosivePuddleSpawner (Node) # Spawns corrosive puddle scenes
├── CorrosivePuddleScene (PackedScene) # Resource for the corrosive puddle
├── EnemySounds (AudioStreamPlayer3D) # Plays enemy specific sounds
├── VisionArea (Area3D)       # Detects player presence
│   └── CollisionShape3D      # Defines the vision cone/range
├── MeleeAttackArea (Area3D)  # Detects player for melee attack
│   └── CollisionShape3D      # Defines melee attack range
├── States (Node)             # Parent node for a node-based state machine (Idle, Pursue, Attack, etc.)
│   ├── IdleState (Node)
│   ├── PursueState (Node)
│   ├── MeleeAttackState (Node)
│   ├── RangedAttackState (Node)
│   └── EvadeState (Node)
```

---

**Tasks:**

1.  **Create Base Enemy Scene and Script:**
    *   **Description:** Start by creating a base `Slitherfist.tscn` from the `doom_enemy_template.tscn` and a corresponding `Slitherfist.gd` script. This script will coordinate components and manage the state machine.
    *   **Skill Reference:** `scene-organization`, `component-system`
    *   **Files:**
        *   `enemies/slitherfist/Slitherfist.tscn`
        *   `enemies/slitherfist/Slitherfist.gd`

2.  **Implement Health Component:**
    *   **Description:** Create a reusable `HealthComponent.gd` that can be attached to any character (player, enemy). It will manage health, taking damage, and emitting `damaged` and `died` signals.
    *   **Skill Reference:** `component-system`, `resource-pattern` (if health data needs to be a Resource).
    *   **Files:**
        *   `components/HealthComponent.gd` (or `enemies/HealthComponent.gd`)
    *   **Action:** Add `HealthComponent` as a child node to `Slitherfist.tscn`.

3.  **Implement Attack Component:**
    *   **Description:** Create a reusable `AttackComponent.gd` to handle various attack types (melee, ranged). It will manage cooldowns, damage calculation, and trigger visual/audio feedback. It will emit `attack_ready` and `ranged_projectile_fired` signals.
    *   **Skill Reference:** `component-system`, `resource-pattern` (for attack data like damage, cooldowns).
    *   **Files:**
        *   `components/AttackComponent.gd` (or `enemies/AttackComponent.gd`)
    *   **Action:** Add `AttackComponent` as a child node to `Slitherfist.tscn`.

4.  **Create Corrosive Puddle Scene (Ranged Attack Effect):**
    *   **Description:** Design a new `CorrosivePuddle.tscn` (an `Area3D` with a `Timer`, particle effect, and shader) to represent the area-of-effect damage from the Slitherfist's ranged attack.
    *   **Skill Reference:** `scene-organization`, `shader-basics`, `resource-pattern` (for puddle properties).
    *   **Files:**
        *   `enemies/slitherfist/CorrosivePuddle.tscn`
        *   `enemies/slitherfist/CorrosivePuddle.gd` (script for damage over time and despawn logic)

5.  **Set up AI State Machine (Node-based):**
    *   **Description:** Implement the individual AI states (Idle, Pursue, MeleeAttack, RangedAttack, Evade) as separate GDScript files, inheriting from a base `EnemyState.gd`. The `Slitherfist.gd` will manage state transitions.
    *   **Skill Reference:** `state-machine`, `component-system` (if states are seen as components).
    *   **Files:**
        *   `enemies/slitherfist/states/EnemyState.gd` (base class)
        *   `enemies/slitherfist/states/IdleState.gd`
        *   `enemies/slitherfist/states/PursueState.gd`
        *   `enemies/slitherfist/states/MeleeAttackState.gd`
        *   `enemies/slitherfist/states/RangedAttackState.gd`
        *   `enemies/slitherfist/states/EvadeState.gd`
    *   **Action:** Add these state nodes as children of a parent `States` node in `Slitherfist.tscn`.

6.  **Integrate Navigation and Movement:**
    *   **Description:** Utilize the `NavigationAgent3D` for pathfinding. Implement movement in `_physics_process` based on the current AI state and `NavigationAgent3D` output.
    *   **Skill Reference:** `ai-navigation`, `player-controller` (movement principles apply).
    *   **Files:** `enemies/slitherfist/Slitherfist.gd`

7.  **Add Visuals and Animations:**
    *   **Description:** Import or create a 3D model for Slitherfist (or use `AnimatedSprite3D` if sticking to 2D sprites in 3D). Set up `AnimationPlayer` or `AnimatedSprite3D` for all defined animations. Implement hit flash shader.
    *   **Skill Reference:** `animation-system`, `shader-basics`.
    *   **Files:** (Depends on assets, but generally `Slitherfist.tscn` and potentially new shader files).

8.  **Integrate Audio:**
    *   **Description:** Populate `EnemySounds` node with specific audio clips for slithering, attacks, hit, and death.
    *   **Skill Reference:** `audio-system`.
    *   **Files:** `enemies/slitherfist/Slitherfist.tscn`, audio files in `enemies/slitherfist/sounds/`.

9.  **Set up Detection Areas:**
    *   **Description:** Configure `VisionArea` and `MeleeAttackArea` (`Area3D` nodes) with appropriate `CollisionShape3D`s and connect their signals to the `Slitherfist.gd` script for AI decision-making.
    *   **Skill Reference:** `scene-organization`.
    *   **Files:** `enemies/slitherfist/Slitherfist.tscn`

10. **Refine and Test:**
    *   **Description:** Thoroughly test each component and the overall AI behavior. Adjust parameters for movement speed, attack damage, cooldowns, and detection ranges.
    *   **Skill Reference:** `godot-debugging`, `godot-testing`.

# Reusable Projectile System Design

A flexible, inheritance-based projectile system for Godot 4.x that supports both simple billboarded sprites and 8-way directional sprites (Doom-style). It automatically handles team detection and collision logic for both players and enemies.

## 1. Architecture

### 1.1 Script Hierarchy
*   **`BaseProjectile.gd`**: Core logic for movement, damage, and collision.
    *   **`DirectionalProjectile.gd`**: Extends `BaseProjectile` to add 8-way directional sprite logic.
    *   **`SimpleProjectile.gd`**: Extends `BaseProjectile` for simple billboarded sprite logic.

### 1.2 Automatic Team Detection
The `setup()` function determines the projectile's behavior based on the `firer`:
*   If `firer` is in the `Player` group:
    *   Sets collision mask to hit the `Enemies` layer.
    *   Adds itself to the `player_projectiles` group.
*   If `firer` is in the `Enemy` group:
    *   Sets collision mask to hit the `Player` layer.
    *   Adds itself to the `enemy_projectiles` group.

## 2. Components

### 2.1 `BaseProjectile.gd`
*   **Properties**:
    *   `speed`: Float (default: 20.0)
    *   `damage`: Int (default: 10)
    *   `direction`: Vector3
    *   `lifetime`: Float (default: 5.0s)
*   **Methods**:
    *   `setup(firer: Node3D, target_dir: Vector3, custom_damage: int, custom_speed: float)`
    *   `_on_body_entered(body: Node3D)`: Handles damage and impact.
    *   `_on_impact()`: Virtual function for particle/sound effects.

### 2.2 `DirectionalProjectile.gd`
*   **Logic**: Calculates the angle between its `direction` and the `Camera3D` position every frame.
*   **Animations**: Expects `[anim_name]_1` through `[anim_name]_5` with automatic horizontal flipping for angles 6-8.

### 2.3 `SimpleProjectile.gd`
*   **Logic**: Always faces the camera (billboard) and plays a single looping animation.

## 3. Node Structure (`Projectile.tscn`)

*   **`Area3D` (Root)**
    *   **`CollisionShape3D`**: Sphere shape for collision.
    *   **`AnimatedSprite3D`**: Visuals (Billboard: Y-Only for Directional, Enabled for Simple).
    *   **`GPUParticles3D` (Optional)**: For trails or impact bursts.
    *   **`OmniLight3D` (Optional)**: For dynamic lighting effects.
    *   **`AudioStreamPlayer3D`**: For "whoosh" and "impact" sounds.

## 4. Lifecycle
1.  **Instantiation**: Spawned by Weapon or Enemy.
2.  **Configuration**: `setup()` is called immediately after `add_child()`.
3.  **Movement**: Moves along `direction * speed` in `_physics_process`.
4.  **Impact**: On collision, deals damage, plays effects, and calls `queue_free()`.
5.  **Timeout**: Automatically `queue_free()` after `lifetime` expires to prevent orphaned nodes.

## 5. Testing Strategy
*   **Verification**: Create a test scene where the Player and a Cultist both shoot projectiles.
*   **Validation**: Confirm that Player projectiles do not hurt the Player and Enemy projectiles do not hurt Enemies.
*   **Visual Check**: Confirm that `DirectionalProjectile` correctly switches sprite frames as the player moves around it.

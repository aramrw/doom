# Camera Shake & Trauma System Design

A robust, multi-layered camera shake system for Godot 4.x using a "Trauma" model. This system separates movement (head-bob), rotational recoil (gimbal), and high-frequency jitter (offsets) to ensure smooth, non-conflicting camera behavior.

## 1. Architecture

### 1.1 The Trauma Model
*   **Trauma (0.0 to 1.0)**: Represents the "stress" on the camera. 
*   **Shake = Trauma^2**: Squaring the trauma creates a more organic feel (small trauma = very little shake, high trauma = heavy shake).
*   **Decay**: Trauma decreases linearly over time (e.g., `trauma = max(trauma - decay * delta, 0)`).

### 1.2 The Camera Hierarchy (Gimbal Pattern)
To prevent different systems from fighting for the same properties:
1.  **Head (Node3D)**: Handles translational head-bobbing and movement offsets.
2.  **ShakeGimbal (Node3D)**: Child of Head. Handles rotational "kick" and recoil (Pitch/Roll).
3.  **Camera3D**: Child of ShakeGimbal. Handles high-frequency jitter via `h_offset` and `v_offset`.

## 2. Components

### 2.1 `CameraShakeEffect.gd` (Resource)
*   **Properties**:
    *   `trauma_amount`: Float (0.0 to 1.0). How much "stress" to add to the camera.
*   **Method**:
    *   `execute(source_node, weapon_manager)`: Finds the Player and calls `add_trauma(trauma_amount)`.

### 2.2 `Player.gd` Integration
*   **Properties**:
    *   `trauma`: Float (Current trauma level).
    *   `trauma_decay`: Float (How fast trauma drops, default: 0.8).
    *   `max_roll`: Float (Max degrees for rotational shake).
    *   `max_offset`: Vector2 (Max pixels for `h_offset`/`v_offset`).
*   **Methods**:
    *   `add_trauma(amount)`: Increases trauma (capped at 1.0).
    *   `_process_camera_shake(delta)`: Calculates and applies shake based on current trauma using `FastNoiseLite`.

## 3. Integration Plan

### 3.1 Setup
1.  Update `player.tscn` to include the `ShakeGimbal` node.
2.  Implement trauma logic in `player.gd`.
3.  Create `CameraShakeEffect.gd` script.

### 3.2 Shotgun Configuration
1.  Create `toz_34_shake_effect.tres` with `trauma_amount = 0.5`.
2.  Add this effect to the `ActionStep` of the TOZ-34 shotgun.

## 4. Testing Strategy
*   **Verification**: Fire the shotgun and observe the camera "kick" and "jitter."
*   **Validation**: Confirm that the shake decays smoothly and doesn't interfere with the player's ability to aim or the existing head-bob.

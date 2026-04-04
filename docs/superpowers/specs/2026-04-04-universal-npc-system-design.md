# Universal NPC System Design

A flexible actor system for Godot 4.x that supports friendly, neutral, and hostile behaviors within a single base class. NPCs can follow the player, engage in combat with specific groups, and participate in interactive dialogue.

## 1. Architecture

### 1.1 `BaseNPC.gd` (Extends `DoomEnemyBase`)
Inherits 8-way directional visuals and basic movement but adds a high-level state machine.
*   **States**: `IDLE`, `FOLLOW`, `COMBAT`, `TALKING`.
*   **Behavior Toggles**:
    *   `attacks_player`: Boolean (Initial hostility).
    *   `attacks_enemies`: Boolean (Assists player by attacking "Enemies" group).
    *   `become_hostile_on_damage`: Boolean (Aggro on player hit).
    *   `follows_player`: Boolean (Bodyguard behavior).

### 1.2 Target Selection
The NPC scans for targets in the `Enemies` group (if `attacks_enemies` is true) and the `Player` group (if `attacks_player` is true). It prioritizes the closest valid target.

## 2. Dialogue & Interaction

### 2.1 Interaction Logic
*   **Trigger**: An `Area3D` around the NPC detected by the player.
*   **Input**: A new `interact` action (Key: E) in the Input Map.
*   **UI**: A `DialogueUI` singleton or HUD component that displays a text box at the bottom of the screen.

### 2.2 Data Structure
*   `dialogue_lines`: Array[String] - The sequence of text to display.
*   `npc_name`: String - Displayed in the dialogue header.

## 3. Implementation: ShotgunMonk
A specific NPC instance configured as a friendly bodyguard.
*   **Assets**: Uses `ShotgunMonk.pk3`.
*   **Default Behavior**: `follows_player = true`, `attacks_enemies = true`, `attacks_player = false`.
*   **Combat**: Reuses the `Reusable Projectile System` to fire shotgun blasts at enemies.

## 4. Testing Strategy
*   **Friendly Test**: Stand near ShotgunMonk and verify he follows.
*   **Combat Test**: Spawn a Cultist and verify ShotgunMonk attacks it.
*   **Aggro Test**: Shoot ShotgunMonk and verify he becomes hostile if toggled.
*   **Dialogue Test**: Press 'E' near him and verify the text box appears and advances.

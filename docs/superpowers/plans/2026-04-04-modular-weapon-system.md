# Modular Weapon System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the weapon system into a decoupled, resource-driven architecture that supports hitscan, projectiles, and melee via modular "Action" and "Effect" plugins.

**Architecture:** A "Socket and Plugin" pattern where `WeaponManager` executes `WeaponAction` resources. Each action is a sequence of `ActionStep` resources that trigger `WeaponEffect` plugins (Hitscan, Projectile, Sound, etc.) on specific animation frames.

**Tech Stack:** Godot 4 (GDScript), 3D environment with 2D Billboarded Sprites.

---

### Task 1: Core Base Resources

**Files:**
- Create: `weapons/weapon_effect.gd`
- Create: `weapons/action_step.gd`
- Create: `weapons/weapon_action.gd`

- [ ] **Step 1: Create `weapon_effect.gd` base class**
```gdscript
# weapons/weapon_effect.gd
extends Resource
class_name WeaponEffect

func execute(_source_node: Node, _weapon_manager: Node) -> void:
	pass
```

- [ ] **Step 2: Create `action_step.gd` resource**
```gdscript
# weapons/action_step.gd
extends Resource
class_name ActionStep

@export var frame_index: int = 0
@export var duration_seconds: float = 0.1
@export var effects: Array[WeaponEffect] = []
```

- [ ] **Step 3: Create `weapon_action.gd` resource**
```gdscript
# weapons/weapon_action.gd
extends Resource
class_name WeaponAction

@export var animation_name: String = "shoot"
@export var steps: Array[ActionStep] = []
@export var loop: bool = false
```

- [ ] **Step 4: Commit**
```bash
git add weapons/weapon_effect.gd weapons/action_step.gd weapons/weapon_action.gd
git commit -m "feat: add base weapon action and effect resources"
```

---

### Task 2: Concrete Effects (Hitscan & Sound)

**Files:**
- Create: `weapons/effects/hitscan_effect.gd`
- Create: `weapons/effects/sound_effect.gd`

- [ ] **Step 1: Implement `HitscanEffect`**
```gdscript
# weapons/effects/hitscan_effect.gd
extends WeaponEffect
class_name HitscanEffect

@export var damage: int = 10
@export var pellets: int = 1
@export var spread_angle: float = 0.0
@export var range_distance: float = 100.0

func execute(source_node: Node, weapon_manager: Node) -> void:
	var raycast = weapon_manager.aim_raycast
	var original_rotation = raycast.rotation_degrees
	
	for i in range(pellets):
		var spread_x = randf_range(-spread_angle, spread_angle)
		var spread_y = randf_range(-spread_angle, spread_angle)
		raycast.rotation_degrees = original_rotation + Vector3(spread_x, spread_y, 0)
		raycast.target_position = Vector3(0, 0, -range_distance)
		raycast.force_raycast_update()
		
		if raycast.is_colliding():
			var target = raycast.get_collider()
			if target.has_method("take_damage"):
				target.take_damage(damage)
	
	raycast.rotation_degrees = original_rotation
```

- [ ] **Step 2: Implement `SoundEffect`**
```gdscript
# weapons/effects/sound_effect.gd
extends WeaponEffect
class_name SoundEffect

@export var sound: AudioStream
@export var pitch_randomness: float = 0.1

func execute(_source_node: Node, weapon_manager: Node) -> void:
	if weapon_manager.shoot_sound:
		weapon_manager.shoot_sound.stream = sound
		weapon_manager.shoot_sound.pitch_scale = 1.0 + randf_range(-pitch_randomness, pitch_randomness)
		weapon_manager.shoot_sound.play()
```

- [ ] **Step 3: Commit**
```bash
git add weapons/effects/hitscan_effect.gd weapons/effects/sound_effect.gd
git commit -m "feat: add hitscan and sound weapon effects"
```

---

### Task 3: Refactor WeaponData and WeaponManager

**Files:**
- Modify: `weapons/weapon_data.gd`
- Modify: `weapons/weapon_manager.gd`

- [ ] **Step 1: Update `WeaponData` to hold actions**
```gdscript
# weapons/weapon_data.gd (partial update)
# Replace existing properties with modular ones
@export var actions: Dictionary = {} # String (e.g. "primary") to WeaponAction
@export var hud_hint: String = "BULLETS" 
# Keep sprite_frames, sprite_offset as they are used by the manager
```

- [ ] **Step 2: Update `WeaponManager` core loop**
```gdscript
# weapons/weapon_manager.gd (refactor fire() and signals)
var current_action: WeaponAction = null
var current_step_index: int = 0

func fire():
	if is_reloading or current_action: return
	start_action("primary")

func start_action(action_name: String):
	if not current_weapon.actions.has(action_name): return
	current_action = current_weapon.actions[action_name]
	current_step_index = 0
	gun_sprite.play(current_action.animation_name)
	process_step()

func _on_gun_sprite_frame_changed():
	if not current_action: return
	process_step()

func process_step():
	var current_frame = gun_sprite.frame
	for step in current_action.steps:
		if step.frame_index == current_frame:
			for effect in step.effects:
				effect.execute(self, self)
```

- [ ] **Step 3: Commit**
```bash
git add weapons/weapon_data.gd weapons/weapon_manager.gd
git commit -m "refactor: update weapon manager to use action-based execution"
```

---

### Task 4: Projectile System

**Files:**
- Create: `weapons/projectile.tscn`
- Create: `weapons/projectile.gd`
- Create: `weapons/effects/projectile_effect.gd`

- [ ] **Step 1: Create `projectile.gd`**
```gdscript
# weapons/projectile.gd
extends Area3D
class_name Projectile

var speed: float = 20.0
var damage: int = 10
var direction: Vector3 = Vector3.FORWARD

func _physics_process(delta):
	position += direction * speed * delta

func _on_body_entered(body):
	if body.has_method("take_damage"):
		body.take_damage(damage)
	queue_free() # Add explosion logic later
```

- [ ] **Step 2: Implement `ProjectileEffect`**
```gdscript
# weapons/effects/projectile_effect.gd
extends WeaponEffect
class_name ProjectileEffect

@export var projectile_scene: PackedScene
@export var speed: float = 30.0
@export var damage: int = 25

func execute(source_node: Node, weapon_manager: Node) -> void:
	var proj = projectile_scene.instantiate()
	weapon_manager.get_tree().root.add_child(proj)
	proj.global_transform = weapon_manager.aim_raycast.global_transform
	proj.speed = speed
	proj.damage = damage
	proj.direction = -weapon_manager.aim_raycast.global_transform.basis.z
```

- [ ] **Step 3: Commit**
```bash
git add weapons/projectile.gd weapons/effects/projectile_effect.gd
git commit -m "feat: add projectile system and effect"
```

---

### Task 5: Melee Integration (Combat Knife)

**Files:**
- Create: `weapons/melee/knife_wpdata.tres` (Conceptual, use Editor)

- [ ] **Step 1: Plan Melee Logic**
Melee is just a `HitscanEffect` with:
- `range_distance: 2.0`
- `damage: 30`
- `pellets: 3` (Wide spread to simulate a sweep)

---

### Task 6: Generic HUD Update

**Files:**
- Modify: `ui/Pulsing Heart/hud.gd`

- [ ] **Step 1: Update HUD to respond to Weapon Hints**
```gdscript
# ui/Pulsing Heart/hud.gd
func update_weapon_ui(weapon: WeaponData):
	match weapon.hud_hint:
		"BULLETS":
			bullets.show()
			magazines.show()
		"MELEE":
			bullets.hide()
			magazines.hide()
		"ENERGY":
			# Future energy bar logic
			pass
```

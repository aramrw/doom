# Camera Shake & Trauma System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a multi-layered camera shake system using a trauma model and a gimbal node hierarchy.

**Architecture:** A "ShakeGimbal" node is added between the head and the camera. Trauma decays over time and is used to calculate rotational noise and translational jitter.

**Tech Stack:** Godot 4.x (GDScript)

---

### Task 1: Player Scene & Script Updates

**Files:**
- Modify: `player/player.tscn` (Manually or via script)
- Modify: `player/player.gd`

- [ ] **Step 1: Add ShakeGimbal node to `player.tscn`**
Insert a Node3D between the `Camera` parent and the `Camera` node itself.

- [ ] **Step 2: Update `player.gd` with Trauma logic**

```gdscript
# Add to player.gd properties
@onready var shake_gimbal: Node3D = $CharacterBody3D/Head/ShakeGimbal # Adjust path as needed

var trauma: float = 0.0
var trauma_decay: float = 0.8
var trauma_power: float = 2.0

var noise = FastNoiseLite.new()
var noise_y = 0

@export var max_roll: float = 5.0 # Max degrees
@export var max_offset: Vector2 = Vector2(0.1, 0.1) # Max meters

func _ready():
	# ... (existing ready logic)
	add_to_group("Player") # Ensure player is in group
	noise.seed = randi()
	noise.frequency = 0.5

func add_trauma(amount: float):
	trauma = clamp(trauma + amount, 0.0, 1.0)

func _process(delta):
	# ... (existing process logic)
	_process_camera_shake(delta)

func _process_camera_shake(delta):
	if trauma > 0:
		trauma = max(trauma - trauma_decay * delta, 0.0)
		
		var shake = pow(trauma, trauma_power)
		noise_y += 1
		
		# High frequency jitter
		camera.h_offset = max_offset.x * shake * noise.get_noise_2d(noise.seed, noise_y)
		camera.v_offset = max_offset.y * shake * noise.get_noise_2d(noise.seed + 1, noise_y)
		
		# Rotational kick
		shake_gimbal.rotation.z = deg_to_rad(max_roll * shake * noise.get_noise_2d(noise.seed + 2, noise_y))
		shake_gimbal.rotation.x = deg_to_rad(max_roll * shake * noise.get_noise_2d(noise.seed + 3, noise_y))
	else:
		camera.h_offset = 0
		camera.v_offset = 0
		shake_gimbal.rotation = Vector3.ZERO
```

- [ ] **Step 3: Commit**

```bash
git add player/player.gd player/player.tscn
git commit -m "feat: add trauma-based camera shake to player"
```

---

### Task 2: Camera Shake Effect

**Files:**
- Create: `weapons/effects/camera_shake_effect.gd`

- [ ] **Step 1: Create `CameraShakeEffect.gd`**

```gdscript
extends WeaponEffect
class_name CameraShakeEffect

@export var trauma_amount: float = 0.3

func execute(_source_node: Node, weapon_manager: Node) -> void:
	var player = weapon_manager.get_parent()
	if player.has_method("add_trauma"):
		player.add_trauma(trauma_amount)
```

- [ ] **Step 2: Commit**

```bash
git add weapons/effects/camera_shake_effect.gd
git commit -m "feat: add CameraShakeEffect resource"
```

---

### Task 3: Shotgun Integration

**Files:**
- Create: `weapons/shotguns/TOZ-34/toz_34_shake_effect.tres`
- Modify: `weapons/shotguns/TOZ-34/toz_34_fire_step.tres`

- [ ] **Step 1: Create `toz_34_shake_effect.tres`**
Use the `CameraShakeEffect` script and set `trauma_amount = 0.5`.

- [ ] **Step 2: Add shake effect to `toz_34_fire_step.tres`**
Append the shake effect to the `effects` array in the fire step.

- [ ] **Step 3: Commit**

```bash
git add weapons/shotguns/TOZ-34/
git commit -m "feat: integrate camera shake with TOZ-34 shotgun"
```

---

### Task 4: Verification

- [ ] **Step 1: Run game and fire shotgun**
- [ ] **Step 2: Verify camera "kicks" and "jitters"**
- [ ] **Step 3: Verify shake decays properly**

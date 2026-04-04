# Reusable Projectile System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a reusable, inheritance-based projectile system for players and enemies, supporting both simple billboards and 8-way directional sprites.

**Architecture:** A base `BaseProjectile` class handles movement, team-based collision, and lifetime. `DirectionalProjectile` and `SimpleProjectile` extend it to handle visuals. The cultist will be updated to shoot a `DirectionalProjectile` magic ball.

**Tech Stack:** Godot 4.x (GDScript)

---

### Task 1: Core Projectile Logic

**Files:**
- Create: `weapons/projectiles/base_projectile.gd`

- [ ] **Step 1: Create `BaseProjectile.gd`**

```gdscript
extends Area3D
class_name BaseProjectile

@export var speed: float = 20.0
@export var damage: int = 10
@export var lifetime: float = 5.0

var direction: Vector3 = Vector3.ZERO
var firer: Node3D = null

func _ready():
	# Auto-destroy after lifetime
	get_tree().create_timer(lifetime).timeout.connect(queue_free)
	body_entered.connect(_on_body_entered)

func setup(p_firer: Node3D, p_direction: Vector3, p_damage: int = -1, p_speed: float = -1.0):
	firer = p_firer
	direction = p_direction.normalized()
	
	if p_damage > 0: damage = p_damage
	if p_speed > 0: speed = p_speed
	
	# Team detection logic
	if firer.is_in_group("Player"):
		collision_mask = 4 # Layer 3: Enemies (adjust based on your project's layers)
		add_to_group("player_projectiles")
	else:
		collision_mask = 2 # Layer 2: Player
		add_to_group("enemy_projectiles")

func _physics_process(delta):
	global_position += direction * speed * delta

func _on_body_entered(body: Node3D):
	if body.has_method("take_damage"):
		body.take_damage(damage)
	
	_on_impact()

func _on_impact():
	# Virtual function for children to override
	queue_free()
```

- [ ] **Step 2: Commit**

```bash
git add weapons/projectiles/base_projectile.gd
git commit -m "feat: add BaseProjectile core logic"
```

---

### Task 2: Visual Implementations

**Files:**
- Create: `weapons/projectiles/directional_projectile.gd`
- Create: `weapons/projectiles/simple_projectile.gd`

- [ ] **Step 1: Create `DirectionalProjectile.gd`**

```gdscript
extends BaseProjectile
class_name DirectionalProjectile

@onready var sprite = $AnimatedSprite3D

func _process(_delta):
	update_sprite_angle()

func update_sprite_angle():
	var camera = get_viewport().get_camera_3d()
	if not camera or not sprite: return
	
	# Forward direction is the projectile's movement direction
	var forward_dir = direction
	var to_camera_dir = global_position.direction_to(camera.global_position)
	
	# Project both onto XZ plane for 2D angle calculation
	var forward_2d = Vector2(forward_dir.x, forward_dir.z).normalized()
	var to_cam_2d = Vector2(to_camera_dir.x, to_camera_dir.z).normalized()
	
	var angle = to_cam_2d.angle_to(forward_2d)
	var angle_index = int(round(angle / (PI / 4.0)))
	if angle_index < 0: angle_index += 8
	angle_index = angle_index % 8
	
	var anim_suffix = ""
	var flip = false
	match angle_index:
		0: anim_suffix = "1"; flip = false
		1: anim_suffix = "2"; flip = false
		2: anim_suffix = "3"; flip = false
		3: anim_suffix = "4"; flip = false
		4: anim_suffix = "5"; flip = false
		5: anim_suffix = "4"; flip = true
		6: anim_suffix = "3"; flip = true
		7: anim_suffix = "2"; flip = true
			
	sprite.flip_h = flip
	# Expects animation names to be like "fly_1", "fly_2", etc.
	var target_anim = "fly_" + anim_suffix
	
	if sprite.animation != target_anim:
		var current_frame = sprite.frame
		var current_progress = sprite.frame_progress
		sprite.play(target_anim)
		sprite.set_frame_and_progress(current_frame, current_progress)
```

- [ ] **Step 2: Create `SimpleProjectile.gd`**

```gdscript
extends BaseProjectile
class_name SimpleProjectile

@onready var sprite = $AnimatedSprite3D

func _ready():
	super._ready()
	if sprite:
		sprite.play("fly")
```

- [ ] **Step 3: Commit**

```bash
git add weapons/projectiles/directional_projectile.gd weapons/projectiles/simple_projectile.gd
git commit -m "feat: add Directional and Simple projectile variants"
```

---

### Task 3: Update `ProjectileEffect`

**Files:**
- Modify: `weapons/effects/projectile_effect.gd`

- [ ] **Step 1: Update `execute` method**

```gdscript
# weapons/effects/projectile_effect.gd
extends WeaponEffect
class_name ProjectileEffect

@export var projectile_scene: PackedScene
@export var speed: float = 30.0
@export var damage: int = 25

func execute(source_node: Node, weapon_manager: Node) -> void:
	if not projectile_scene: return
	
	var proj = projectile_scene.instantiate()
	# Add to tree
	weapon_manager.get_tree().root.add_child(proj)
	
	# Position at the aim raycast
	var ray = weapon_manager.aim_raycast
	proj.global_transform = ray.global_transform
	
	# Use the new setup method
	if proj.has_method("setup"):
		var dir = -ray.global_transform.basis.z
		# firer is the player's body usually, or the weapon_manager's parent
		var firer = weapon_manager.get_parent() 
		proj.setup(firer, dir, damage, speed)
```

- [ ] **Step 2: Commit**

```bash
git add weapons/effects/projectile_effect.gd
git commit -m "feat: update ProjectileEffect to use new setup method"
```

---

### Task 4: Cultist Projectile Setup

**Files:**
- Create: `enemies/cultist/cultist_projectile.tscn` (via manual creation or placeholder check)
- Modify: `enemies/grin/doom_enemy.gd`

- [ ] **Step 1: Modify `doom_enemy.gd` to support projectiles**

```gdscript
# Add export for projectile scene
@export var projectile_scene: PackedScene = null

# In attack() function, replace the direct damage with projectile spawn if scene exists
func attack():
	# ... (keep existing setup logic)
	
	await get_tree().create_timer(0.4).timeout
	
	if not is_dead and not is_hit:
		if projectile_scene:
			var proj = projectile_scene.instantiate()
			get_tree().root.add_child(proj)
			proj.global_position = global_position + Vector3(0, 1.2, 0) # Fire from chest height
			
			var dir = global_position.direction_to(player.body.global_position)
			proj.setup(self, dir, attack_damage, 15.0) # Speed 15.0 for magic ball
			
			sfx.taunt()
		else:
			# Fallback to raycast/melee
			los_raycast.force_raycast_update()
			if los_raycast.get_collider() == player.body:
				player.take_damage(attack_damage)
				sfx.taunt()
	
	# ... (keep cooldown logic)
```

- [ ] **Step 2: Commit**

```bash
git add enemies/grin/doom_enemy.gd
git commit -m "feat: add projectile support to DoomEnemyBase"
```

---

### Task 5: Verification

- [ ] **Step 1: Test with Cultist**
- [ ] **Step 2: Verify player health decreases on hit**
- [ ] **Step 3: Verify projectile frames change when orbiting it**

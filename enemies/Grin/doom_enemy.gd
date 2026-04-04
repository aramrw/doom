extends CharacterBody3D
class_name DoomEnemyBase

@export var speed: float = 4.0
@export var attack_range: float = 2.0 
@export var attack_damage: int = 5
@export var attack_cooldown: float = 1.5 
@export var detection_range: float = 20.0 # Only wake up if player is within 20 meters

@onready var sprite = $AnimatedSprite3D
@onready var nav_agent = $NavigationAgent3D
@onready var los_raycast = $RayCast3D
@onready var collision_shape = $CollisionShape3D # Make sure your collision node is named this!

@onready var sfx = $EnemySounds

var player: Node3D
var current_anim_state: String = "walk"

# --- PROJECTILE ---
@export var projectile_scene: PackedScene = null

# --- STATE VARIABLES ---
var is_attacking: bool = false
var on_cooldown: bool = false 
var is_hit: bool = false  # For the flinch/pain state
var is_dead: bool = false # For the corpse state

@export var health: int = 100
var gravity: float = ProjectSettings.get_setting("physics/3d/default_gravity")

func _ready():
	player = get_tree().get_first_node_in_group("Player")
	los_raycast.add_exception(self)
	await get_tree().physics_frame

func _physics_process(delta):
	# Always apply gravity, even to corpses, so they don't float
	if not is_on_floor():
		velocity.y -= gravity * delta

	# If he's dead, he just falls to the floor and does nothing else
	if is_dead:
		move_and_slide()
		return

	update_sprite_angle() 
	
	# Freeze the enemy if they are attacking OR flinching from pain
	if not player or is_attacking or is_hit:
		velocity.x = 0
		velocity.z = 0
		move_and_slide() 
		return

	var distance_to_player = global_position.distance_to(player.body.global_position)
	
	los_raycast.target_position = to_local(player.body.global_position) + Vector3(0, 1, 0)
	los_raycast.force_raycast_update()
	var ray_hit = los_raycast.get_collider()
	
	if distance_to_player <= detection_range: 
		if distance_to_player <= attack_range and ray_hit == player.body: 
			if not on_cooldown: 
				attack() 
			else: 
				stand_and_stare() 
		else: 
			chase_player() 
	else: 
		# Optional: Play an idle animation if the player is too far away 
		current_anim_state = "walk" # Or "idle" if you have it 
		velocity = Vector3.ZERO
		
	move_and_slide()

# --- PROPER DAMAGE & DEATH ---
func take_damage(amount: int):
	if is_dead: 
		return 
		
	health -= amount
	
	if health <= 0:
		die()
	else:
		is_hit = true
		is_attacking = false 
		current_anim_state = "pain_1" 
		sfx.hurt()
		
		# --- THE CLEAN HDR HIT FLASH ---
		# We boost the modulation to 10.0 (Pure White HDR Glow)
		sprite.modulate = Color(10, 10, 10, 1.0) 
		var tween = create_tween()
		tween.tween_property(sprite, "modulate", Color(1, 1, 1, 1), 0.15)
		
		update_sprite_angle()
		
		# Wait for the flinch to finish
		await get_tree().create_timer(0.3).timeout
			
		if not is_dead: 
			is_hit = false

func die():
	is_dead = true
	is_hit = false
	is_attacking = false
	sprite.modulate = Color(1, 1, 1)
	
	# freeze the enemy
	velocity.x = 0
	velocity.z = 0
	
	# Classic Doom extreme-death check!
	# If health drops to -20 or lower, play the gory death
	if health <= -20:
		current_anim_state = "death_2"
	else:
		current_anim_state = "death_1"
	sfx.death()
	
	update_sprite_angle()
	
	# 1. Wait for the death animation to completely finish
	await sprite.animation_finished
	
	# 2. Wait an extra 0.3 seconds while resting on the final frame
	await get_tree().create_timer(1.5).timeout
	
	# 3. Finally, delete the enemy
	queue_free()

# --- MOVEMENT/ATTACK LOGIC ---
func chase_player():
	current_anim_state = "walk"
	nav_agent.target_position = player.body.global_position
	var next_path_pos = nav_agent.get_next_path_position()
	
	var current_pos = global_position
	next_path_pos.y = current_pos.y 
	
	var direction = current_pos.direction_to(next_path_pos)
	
	velocity.x = direction.x * speed 
	velocity.z = direction.z * speed 
	
	var look_target = global_position - direction
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)

func stand_and_stare():
	current_anim_state = "walk" 
	velocity.x = 0
	velocity.z = 0
	
	var dir_to_player = global_position.direction_to(player.body.global_position)
	var look_target = global_position - dir_to_player
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)

func attack():
	is_attacking = true
	on_cooldown = true 
	current_anim_state = "attack" 
	
	velocity.x = 0
	velocity.z = 0 
	
	var dir_to_player = global_position.direction_to(player.body.global_position)
	var look_target = global_position - dir_to_player
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)
	
	await get_tree().create_timer(0.4).timeout
	
	# Only do damage if the enemy wasn't killed or stunned during the 0.4s wind-up!
	if not is_dead and not is_hit:
		if projectile_scene:
			var proj = projectile_scene.instantiate()
			get_tree().root.add_child(proj)
			# Fire from chest height (around 1.2 meters)
			proj.global_position = global_position + Vector3(0, 1.2, 0) 
			
			# Target the player's body
			var dir = global_position.direction_to(player.body.global_position)
			proj.setup(self, dir, attack_damage, 15.0) # Speed 15.0 for magic ball
			
			sfx.taunt()
		else:
			# Fallback to raycast/melee
			los_raycast.force_raycast_update()
			if los_raycast.get_collider() == player.body:
				player.take_damage(attack_damage)
				
				# trigger taunt
				sfx.taunt()
		
	is_attacking = false
	
	await get_tree().create_timer(attack_cooldown).timeout
	on_cooldown = false

# --- VISUALS ---
func update_sprite_angle():
	var camera = get_viewport().get_camera_3d()
	if not camera: return
	
	# Define which states actually have 8 angles
	var directional_states = ["walk", "attack"]
	
	if current_anim_state in directional_states:
		# --- 8-WAY DIRECTIONAL LOGIC ---
		var forward_dir = global_transform.basis.z
		var to_camera_dir = global_position.direction_to(camera.global_position)
		var angle = to_camera_dir.signed_angle_to(forward_dir, Vector3.UP)
		var angle_index = int(round(angle / (PI / 4.0)))
		if angle_index < 0: angle_index += 8
		
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
		var target_anim = current_anim_state + "_" + anim_suffix
		
		if sprite.animation != target_anim:
			var current_frame = sprite.frame
			var current_progress = sprite.frame_progress
			sprite.play(target_anim)
			if sprite.animation.begins_with(current_anim_state):
				sprite.set_frame_and_progress(current_frame, current_progress)
				
	else:
		# --- NORMAL BILLBOARD LOGIC (Pain, Death) ---
		sprite.flip_h = false # Never flip the death/pain animations
		
		if sprite.animation != current_anim_state:
			sprite.play(current_anim_state)

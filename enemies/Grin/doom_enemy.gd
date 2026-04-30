extends CharacterBody3D
class_name DoomEnemyBase

@export var speed: float = 4.0
@export var meleerange: float = 2.0 
@export var damage: int = 5
@export var reactiontime: float = 1.5 
@export var detection_range: float = 20.0 # Only wake up if player is within 20 meters
@export var pain_chance: int = 50

@onready var sprite = $AnimatedSprite3D
@onready var nav_agent = $NavigationAgent3D
@onready var los_raycast = $RayCast3D
@onready var collision_shape = $CollisionShape3D # Make sure your collision node is named this!

@onready var sfx = $EnemySounds

var player: Node3D
var target_node: Node3D = null
var current_anim_state: String = "walk"

# --- PROJECTILE ---
@export var projectile_scene: PackedScene = null
@export var missile_action: Resource = null # WeaponAction
@export var melee_action: Resource = null # WeaponAction

# --- STATE VARIABLES ---
var is_attacking: bool = false
var on_cooldown: bool = false 
var is_hit: bool = false  # For the flinch/pain state
var is_dead: bool = false # For the corpse state

@export var health: int = 100
@export var use_gravity: bool = true
var gravity: float = ProjectSettings.get_setting("physics/3d/default_gravity")

# --- VISUAL VARIABLES ---
var visual_mat: ShaderMaterial = null
var outline_active: bool = false

func _ready():
	player = get_tree().get_first_node_in_group("Player")
	
	if los_raycast:
		los_raycast.add_exception(self)
		los_raycast.collision_mask = 3 # Layer 1: World + Layer 2: Player
		los_raycast.enabled = true
	
	# Initialize our combined visual material
	visual_mat = ShaderMaterial.new()
	visual_mat.shader = load("res://shaders/enemy_visuals.gdshader")
	sprite.material_override = visual_mat
	
	await get_tree().physics_frame

func _process(_delta):
	# If dead, definitely no outline
	if is_dead and outline_active:
		set_outline(false)
		
	if sprite and visual_mat:
		var tex = sprite.sprite_frames.get_frame_texture(sprite.animation, sprite.frame)
		visual_mat.set_shader_parameter("tex", tex)

func set_outline(active: bool, color: Color = Color.RED):
	if is_dead: 
		outline_active = false
		if visual_mat: visual_mat.set_shader_parameter("outline_active", false)
		return
		
	outline_active = active
	if visual_mat:
		visual_mat.set_shader_parameter("outline_active", active)
		visual_mat.set_shader_parameter("outline_color", color)

func _physics_process(delta):
	# Always apply gravity, even to corpses, so they don't float
	if use_gravity or is_dead:
		if not is_on_floor():
			velocity.y -= gravity * delta

	# If he's dead, he just falls to the floor and does nothing else
	if is_dead:
		move_and_slide()
		return

	update_sprite_angle() 
	
	# Freeze the enemy if they are attacking OR flinching from pain
	if is_attacking or is_hit:
		velocity.x = 0
		velocity.z = 0
		move_and_slide() 
		return

	if not target_node:
		# If no target, try to find player by default if we are an enemy
		if not is_in_group("NPCs"):
			target_node = player.body if player else null
		
	if not target_node:
		current_anim_state = "see"
		velocity = Vector3.ZERO
		move_and_slide()
		return

	var target_body = target_node
	if target_node.has_node("CharacterBody3D"):
		target_body = target_node.get_node("CharacterBody3D")

	var distance_to_target = global_position.distance_to(target_body.global_position)
	
	los_raycast.target_position = to_local(target_body.global_position) + Vector3(0, 1, 0)
	los_raycast.force_raycast_update()
	var ray_hit = los_raycast.get_collider()
	
	if distance_to_target <= detection_range: 
		var can_melee = distance_to_target <= meleerange and melee_action != null
		var can_missile = missile_action != null
		
		if (can_melee or can_missile) and ray_hit == target_body: 
			if not on_cooldown and not is_attacking: 
				attack() 
			else: 
				stand_and_stare() 
		else: 
			chase_target() 
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
	print("Enemy ", name, " took ", amount, " damage. Health: ", health)
	
	if health <= 0:
		die()
	else:
		is_hit = true
		is_attacking = false 
		current_anim_state = "pain" 
		sfx.hurt()
		
		# --- AGGRESSIVE HIT FLASH (Shader + Modulate) ---
		if visual_mat:
			# Shader Mix to White
			var tween = create_tween()
			tween.tween_method(func(v): visual_mat.set_shader_parameter("hit_flash", v), 1.0, 0.0, 0.2)
		
		# Extreme modulation boost to ensure visibility 
		sprite.modulate = Color(15, 15, 15, 1.0) 
		var mod_tween = create_tween()
		mod_tween.tween_property(sprite, "modulate", Color(1, 1, 1, 1), 0.2)
		
		update_sprite_angle()
		
		# Wait for the flinch to finish
		await get_tree().create_timer(0.3).timeout
			
		if not is_dead: 
			is_hit = false

func die():
	is_dead = true
	is_hit = false
	is_attacking = false
	
	# --- RAPID FINAL FLASH ---
	# Fast rapid flashes right before the death animation kicks in
	if visual_mat:
		var ftween = create_tween()
		for i in range(1): # 3 very rapid flashes
			ftween.tween_method(func(v): visual_mat.set_shader_parameter("hit_flash", v), 1.0, 0.0, 0.05)
			ftween.tween_interval(0.02)
		sprite.modulate = Color(20.001, 20.001, 20.001, 1.0) # One last bright white pop
		ftween.tween_property(sprite, "modulate", Color(1, 1, 1, 1), 0.1)
		await ftween.finished
	
	# Reset visuals to normal for the death sprites
	sprite.modulate = Color(1, 1, 1)
	if visual_mat:
		visual_mat.set_shader_parameter("hit_flash", 0.0)
	
	# freeze the enemy
	velocity.x = 0
	velocity.z = 0
	
	# Classic Doom extreme-death check!
	# If health drops to -20 or lower, play the gory death
	if health <= -20:
		current_anim_state = "xdeath"
	else:
		current_anim_state = "death"
	sfx.death()
	
	update_sprite_angle()
	
	# 1. Wait for the death animation to completely finish
	await sprite.animation_finished
	
	# 2. Wait an extra 0.3 seconds while resting on the final frame
	await get_tree().create_timer(1.5).timeout
	
	# 3. Finally, delete the enemy
	queue_free()

# --- MOVEMENT/ATTACK LOGIC ---
func chase_target():
	if not target_node or not is_instance_valid(target_node): 
		target_node = null
		return
		
	var target_body = target_node
	if target_node.has_node("CharacterBody3D"):
		target_body = target_node.get_node("CharacterBody3D")
		
	current_anim_state = "see"
	nav_agent.target_position = target_body.global_position
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
	if not target_node or not is_instance_valid(target_node):
		target_node = null
		return
		
	var target_body = target_node
	if target_node.has_node("CharacterBody3D"):
		target_body = target_node.get_node("CharacterBody3D")
		
	current_anim_state = "see" 
	velocity.x = 0
	velocity.z = 0
	
	var dir_to_target = global_position.direction_to(target_body.global_position)
	var look_target = global_position - dir_to_target
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)

func attack():
	print("DEBUG: Attack initiated. on_cooldown: ", on_cooldown, " is_attacking: ", is_attacking)
	if is_attacking or on_cooldown: return
	if not target_node or not is_instance_valid(target_node):
		target_node = null
		return
		
	var target_body = target_node
	if target_node.has_node("CharacterBody3D"):
		target_body = target_node.get_node("CharacterBody3D")
		
	is_attacking = true
	on_cooldown = true 
	
	velocity.x = 0
	velocity.z = 0 
	
	var dir_to_target = global_position.direction_to(target_body.global_position)
	var look_target = global_position - dir_to_target
	look_target.y = global_position.y
	if global_position.distance_to(look_target) > 0.1:
		look_at(look_target, Vector3.UP)
	
	var distance_to_target = global_position.distance_to(target_body.global_position)
	
	# Determine which action to use
	var action_to_use = null
	if distance_to_target <= meleerange and melee_action:
		action_to_use = melee_action
		current_anim_state = "melee"
	elif missile_action:
		action_to_use = missile_action
		current_anim_state = "missile"
	else:
		current_anim_state = "missile"
		
	if action_to_use:
		await perform_action(action_to_use)
	else:
		# Original Fallback logic
		await get_tree().create_timer(0.4).timeout
		
		# Only do damage if the enemy wasn't killed or stunned during the 0.4s wind-up!
		if not is_dead and not is_hit:
			if projectile_scene:
				var proj = projectile_scene.instantiate()
				get_tree().root.add_child(proj)
				# Fire from chest height (around 1.2 meters)
				proj.global_position = global_position + Vector3(0, 1.2, 0) 
				
				# Target the target's body
				var dir = global_position.direction_to(target_body.global_position)
				proj.setup(self, dir, damage, 15.0) # Speed 15.0 for magic ball
				
				sfx.taunt()
			else:
				# Fallback to raycast/melee
				los_raycast.force_raycast_update()
				if los_raycast.get_collider() == target_body:
					if target_body.has_method("take_damage"):
						target_body.take_damage(damage)
					elif target_body.get_parent() and target_body.get_parent().has_method("take_damage"):
						target_body.get_parent().take_damage(damage)
					
					# trigger taunt
					sfx.taunt()
		
	is_attacking = false
	
	print("DEBUG: Waiting for reactiontime: ", reactiontime)
	await get_tree().create_timer(reactiontime).timeout
	print("DEBUG: Reactiontime finished.")
	on_cooldown = false

func perform_action(action: Resource):
	if not action: return
	
	for step in action.steps:
		if is_dead or is_hit: break
		
		# Wait until the sprite reaches the target frame
		while sprite.frame < step.frame_index and not is_dead and not is_hit:
			await get_tree().process_frame
			
		if is_dead or is_hit: break
		
		# Execute effects
		for effect in step.effects:
			execute_effect(effect)

func execute_effect(effect: Resource):
	if not effect: return
	
	if effect.has_method("execute"):
		effect.execute(self, null)
	elif effect.get_script() and effect.get_script().get_global_name() == "ProjectileEffect":
		# Hardcoded fallback for known effect types if needed
		spawn_projectile(effect.projectile_scene)
	elif effect.get_script() and effect.get_script().get_global_name() == "SoundEffect":
		if effect.sound:
			sfx.play_custom(effect.sound)

func spawn_projectile(p_scene: PackedScene):
	if not p_scene: return
	var proj = p_scene.instantiate()
	get_tree().root.add_child(proj)
	proj.global_position = global_position + Vector3(0, 1.2, 0)
	
	var target_body = target_node
	if target_node.has_node("CharacterBody3D"):
		target_body = target_node.get_node("CharacterBody3D")
	
	var dir = global_position.direction_to(target_body.global_position)
	if proj.has_method("setup"):
		proj.setup(self, dir, damage, 15.0)

# --- VISUALS ---
func update_sprite_angle():
	var camera = get_viewport().get_camera_3d()
	if not camera: return
	
	# Any state can potentially have 8 angles now
	# We check if the suffixed version exists in SpriteFrames
	
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
		
	var target_anim = current_anim_state.to_lower() + "_" + anim_suffix
	var fallback_anim = current_anim_state.to_lower()
	
	if sprite.sprite_frames.has_animation(target_anim):
		sprite.flip_h = flip
		if sprite.animation != target_anim:
			var current_frame = sprite.frame
			var current_progress = sprite.frame_progress
			sprite.play(target_anim)
			if sprite.animation.begins_with(current_anim_state.to_lower()):
				sprite.set_frame_and_progress(current_frame, current_progress)
	elif sprite.sprite_frames.has_animation(fallback_anim):
		sprite.flip_h = false
		if sprite.animation != fallback_anim:
			sprite.play(fallback_anim)
	elif sprite.sprite_frames.has_animation(fallback_anim.replace("_1", "")):
		sprite.flip_h = false
		sprite.play(fallback_anim.replace("_1", ""))

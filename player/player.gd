extends Node3D

@onready var camera: Camera3D = $CharacterBody3D/ShakeGimbal/Camera
@onready var shake_gimbal: Node3D = $CharacterBody3D/ShakeGimbal
@onready var body: CharacterBody3D = $CharacterBody3D
@onready var weapon_manager = $WeaponManager
@onready var inventory_manager = $InventoryManager
@onready var item_manager = $ItemManager
@onready var default_height = camera.position.y
@onready var gun_sprite = $WeaponManager/WeaponLayer/GunSprite
@onready var hud = $Hud

var trauma: float = 0.0
var trauma_decay: float = 3.5 # Fast decay
var trauma_power: float = 2.0

var noise = FastNoiseLite.new()
var noise_y = 0.0

@export var max_roll: float = 1.5 # Minimal rotation
@export var max_offset: Vector2 = Vector2(0.03, 0.03) # Minimal displacement

@export_category("Sensitivity")
@export var mouse_sensitivity = 0.03;

@export var runspeed = 7.0
@export var speed = 5.0
@export var SPEED = speed
@export var LOOK_SPEED = 2.5 
@export var SPRINT_FOV_MOD = 1.1 # 10% increase
@export var FOV_CHANGE_SPEED = 5.0

var is_sprinting = false
@onready var default_fov = camera.fov

const BOB_FREQ = 2.7
const BOB_AMP = 0.03

# Add these for the 2D gun bob (these are in pixels, adjust to taste!)
const GUN_BOB_AMP_X = 5.0 
const GUN_BOB_AMP_Y = 5.0 

var tbob = 0.0
var gun_default_pos = Vector2.ZERO 
var item_default_pos = Vector2.ZERO
var last_targeted_node: Node3D = null

# --- NEW HEALTH VARIABLES ---
var max_health: int = 100
var health: int = max_health

func _ready():
	add_to_group("Player")
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	gun_default_pos = gun_sprite.position
	item_default_pos = item_manager.item_sprite.position
	
	noise.seed = randi()
	noise.frequency = 0.5
	
	# hookup hud to weapon_manager
	weapon_manager.ammo_updated.connect(hud.update_bullets)
	hud.update_bullets(weapon_manager.bullets)
	weapon_manager.magazine_count_updated.connect(hud.update_magazines)
	hud.update_magazines(weapon_manager.magazine_count)
	
	inventory_manager.inventory_changed.connect(_on_inventory_changed)
	_on_inventory_changed()
	
	if weapon_manager.current_weapon:
		hud.update_weapon_ui(weapon_manager.current_weapon)
	
	if hud:
		hud.update_health(health)

func _on_inventory_changed():
	if hud:
		var active = inventory_manager.get_active_item()
		var count = 0
		if active:
			count = inventory_manager.counts.get(active.item_name, 0)
		
		item_manager.update_item(active)
		hud.update_inventory_ui(active, count)

func _process(delta):
	# Don't allow shooting/reloading while dialogue is active
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	var in_dialogue = dialogue_ui and dialogue_ui.is_active
	
	if not in_dialogue:
		if not item_manager.is_active:
			if Input.is_action_just_pressed("shoot"): 
				weapon_manager.fire()
			if Input.is_action_just_pressed("reload"):
				weapon_manager.reload()
		
		if Input.is_action_just_pressed("inv_next"):
			inventory_manager.cycle_next()
		if Input.is_action_just_pressed("inv_prev"):
			inventory_manager.cycle_prev()
		if Input.is_action_just_pressed("inv_use"):
			inventory_manager.use_active_item(self)
			
		if Input.is_key_pressed(KEY_5):
			var active = inventory_manager.get_active_item()
			if active:
				item_manager.activate(active)
		
	if Input.is_action_just_pressed("interact"):
		handle_interaction()
		
	if Input.is_key_pressed(KEY_1):
		item_manager.deactivate()
		weapon_manager.switch_to_slot("primary")
		if weapon_manager.current_weapon:
			hud.update_weapon_ui(weapon_manager.current_weapon)
	
	if Input.is_key_pressed(KEY_2):
		item_manager.deactivate()
		weapon_manager.switch_to_slot("secondary")
		if weapon_manager.current_weapon:
			hud.update_weapon_ui(weapon_manager.current_weapon)
	
	if Input.is_key_pressed(KEY_3):
		item_manager.deactivate()
		weapon_manager.switch_to_slot("third")
		if weapon_manager.current_weapon:
			hud.update_weapon_ui(weapon_manager.current_weapon)
		
	_process_camera_shake(delta)
	if Input.is_action_just_pressed("ui_cancel"):
		if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		else:
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
			
	update_target_outline()

func update_target_outline():
	var interact_ray = $CharacterBody3D/ShakeGimbal/Camera/InteractRay
	var aim_ray = $CharacterBody3D/ShakeGimbal/Camera/AimRayCast
	
	# Priority to interaction range, then combat range
	var target = null
	if interact_ray.is_colliding():
		target = interact_ray.get_collider()
	elif aim_ray.is_colliding():
		target = aim_ray.get_collider()
		
	# Traverse up to find the base node if needed
	var target_node = null
	if target:
		if target.has_method("set_outline"):
			target_node = target
		elif target.get_parent() and target.get_parent().has_method("set_outline"):
			target_node = target.get_parent()
			
	# Clear previous target if changed
	if last_targeted_node and last_targeted_node != target_node:
		if is_instance_valid(last_targeted_node):
			last_targeted_node.set_outline(false)
		last_targeted_node = null
		
	# Set current target
	if target_node and is_instance_valid(target_node):
		var outline_color = Color(1, 1, 1, 1) # Neutral White
		
		# Check if hostile
		var is_hostile = false
		if target_node.is_in_group("Enemies"):
			if target_node.is_in_group("NPCs"):
				if target_node.get("attacks_player"):
					is_hostile = true
			else:
				is_hostile = true
				
		if is_hostile:
			outline_color = Color(1, 0, 0, 1) # Hostile Red
			
		target_node.set_outline(true, outline_color)
		last_targeted_node = target_node

func handle_interaction():
	print("Player: handle_interaction called")
	# 1. If dialogue is already open, advance it and return
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui and dialogue_ui.visible:
		dialogue_ui.advance()
		return
		
	# 2. Otherwise, look for something to interact with
	var interact_ray = $CharacterBody3D/ShakeGimbal/Camera/InteractRay
	interact_ray.collision_mask = 5 # Layer 1 (Environment) + Layer 3 (Interaction)
	interact_ray.force_raycast_update()
	
	if interact_ray.is_colliding():
		var collider = interact_ray.get_collider()
		print("Player: interaction ray hit: ", collider.name, " on layer: ", collider.collision_layer)
		
		# Robust check: search up the hierarchy for an 'interact' method
		var current = collider
		while current:
			if current.has_method("interact"):
				print("Player: Calling interact() on ", current.name)
				current.interact(self)
				return
			current = current.get_parent()
		
		print("Player: No interact() method found in hierarchy of ", collider.name)
		return

	# 3. Proximity fallback
	print("Player: raycast missed, checking proximity")
	var space_state = body.get_world_3d().direct_space_state
	var query = PhysicsShapeQueryParameters3D.new()
	query.collision_mask = 5 # Layer 1 + Layer 3
	var shape = SphereShape3D.new()
	shape.radius = 2.0
	query.shape = shape
	query.transform = body.global_transform
	
	var results = space_state.intersect_shape(query)
	for result in results:
		var collider = result.collider
		print("Player: proximity found: ", collider.name, " on layer: ", collider.collision_layer)
		if collider.has_method("interact"):
			collider.interact(self)
			return
		elif collider.get_parent() and collider.get_parent().has_method("interact"):
			collider.get_parent().interact(self)
			return

func get_inventory_manager() -> Node:
	return inventory_manager

func _physics_process(delta: float) -> void:
	if not body.is_on_floor():
		body.velocity.y -= 9.8 * delta

	# Check if we should lock movement due to dialogue
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	var in_dialogue = dialogue_ui and dialogue_ui.is_active

	if in_dialogue:
		body.velocity.x = 0
		body.velocity.z = 0
		body.move_and_slide()
		return

	# 1. Update sprinting state (only if moving forward and not in dialogue)
	var is_moving_forward = Input.is_action_pressed("move_forward")
	is_sprinting = Input.is_action_pressed("sprint") and is_moving_forward and not in_dialogue

	# 2. Set current speed
	SPEED = runspeed if is_sprinting else speed

	# 3. Handle FOV Change
	var target_fov = default_fov * (SPRINT_FOV_MOD if is_sprinting else 1.0)
	camera.fov = lerp(camera.fov, target_fov, delta * FOV_CHANGE_SPEED)

	# 1. Handle Keyboard Look
	var look_dir = Input.get_vector("look_left", "look_right", "look_up", "look_down")
	
	body.rotate_y(-look_dir.x * LOOK_SPEED * delta)
	camera.rotate_x(look_dir.y * LOOK_SPEED * delta)
	camera.rotation.x = clamp(camera.rotation.x, deg_to_rad(-80), deg_to_rad(80))

	# 2. Get Movement Input
	var input_dir = Input.get_vector("move_left", "move_right", "move_forward", "move_backward")
	var direction = (body.transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	
	# 3. Handle Movement Velocity
	if direction:
		body.velocity.x = direction.x * SPEED
		body.velocity.z = direction.z * SPEED
	else:
		body.velocity.x = move_toward(body.velocity.x, 0, SPEED)
		body.velocity.z = move_toward(body.velocity.z, 0, SPEED)

	# 4. Handle Head Bob (3D Camera)
	var bob_multiplier = 1.5 if is_sprinting else 1.0
	tbob += delta * body.velocity.length() * float(body.is_on_floor()) * bob_multiplier
	camera.transform.origin = _headbob(tbob)
	
	# 5. Handle Weapon Bob (2D Sprite)
	var gun_bob_pos = Vector2.ZERO
	gun_bob_pos.y = sin(tbob * BOB_FREQ) * GUN_BOB_AMP_Y
	gun_bob_pos.x = cos(tbob * BOB_FREQ / 2) * GUN_BOB_AMP_X
	gun_sprite.position = gun_default_pos + gun_bob_pos
	
	# 6. Handle Item Bob
	if item_manager.is_active:
		var item_bob_pos = Vector2.ZERO
		item_bob_pos.y = sin(tbob * BOB_FREQ * 0.8) * GUN_BOB_AMP_Y * 0.5
		item_bob_pos.x = cos(tbob * BOB_FREQ / 2.5) * GUN_BOB_AMP_X * 0.5
		item_manager.item_sprite.position = item_default_pos + item_bob_pos

	# 7. Actually move the body
	body.move_and_slide()
	
func handle_pickup(resource: PickupResource) -> bool:
	if resource == null:
		return false
		
	if resource is InventoryItemData:
		return inventory_manager.add_item(resource as InventoryItemData)
		
	if resource is ItemData:
		var item = resource as ItemData
		match item.type:
			ItemData.ItemType.HLTH_MEDKIT:
				if health >= max_health: return false
				health = clamp(health + item.amount, 0, max_health)
				hud.update_health(health)
				return true
				
			ItemData.ItemType.AMMO_MAGAZINE:
				weapon_manager.handle_item_pickup(item)
				return true
				
			ItemData.ItemType.AMMO_SHELL:
				print("Picked up ", item.amount, " shells.")
				return true
				
			ItemData.ItemType.AMMO_ROCKET:
				print("Picked up ", item.amount, " rockets.")
				return true
		return false
		
	elif resource is WeaponData:
		return weapon_manager.handle_weapon_pickup(resource as WeaponData)
		
	return false

func _headbob(time) -> Vector3:
	var pos = Vector3.ZERO
	pos.y = sin(time * BOB_FREQ) * BOB_AMP + default_height
	pos.x = cos(time * BOB_FREQ / 2) * BOB_AMP
	return pos
	
func _unhandled_input(event: InputEvent) -> void:
	# Don't allow looking around while in dialogue
	var dialogue_ui = get_tree().get_first_node_in_group("DialogueUI")
	if dialogue_ui and dialogue_ui.is_active:
		return
		
	# Only look around if the mouse is currently captured
	if event is InputEventMouseMotion and Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
		
		# Rotate the body left/right (Horizontal)
		body.rotate_y(-event.relative.x * mouse_sensitivity)
		
		# Rotate the camera up/down (Vertical)
		camera.rotate_x(-event.relative.y * mouse_sensitivity)
		
		# Clamp the camera so the player can't do backflips
		camera.rotation.x = clamp(camera.rotation.x, deg_to_rad(-80), deg_to_rad(80))

# --- NEW DAMAGE FUNCTION ---
func take_damage(amount: int):
	health -= amount
	print("Player Health: ", health)
	
	if hud:
		hud.update_health(health)
		hud.flash_damage()
	
	if health <= 0:
		die()

func die():
	print("Player Died!")
	# For now, just restart the level when you die
	get_tree().reload_current_scene()

func add_trauma(amount: float):
	trauma = clamp(trauma + amount, 0.0, 1.0)

func has_inventory_item(item_name: String) -> bool:
	return inventory_manager.has_item(item_name)

func _process_camera_shake(delta):
	if trauma > 0:
		pass

extends Node3D

@onready var camera: Camera3D = $CharacterBody3D/Camera
@onready var body: CharacterBody3D = $CharacterBody3D
@onready var weapon_manager = $WeaponManager
@onready var default_height = camera.position.y
@onready var gun_sprite = $WeaponManager/WeaponLayer/GunSprite
@onready var hud = $Hud

@export_category("Sensitivity")
@export var mouse_sensitivity = 0.03;

const SPEED = 5.0
const LOOK_SPEED = 2.5 
const BOB_FREQ = 2.7
const BOB_AMP = 0.03

# Add these for the 2D gun bob (these are in pixels, adjust to taste!)
const GUN_BOB_AMP_X = 5.0 
const GUN_BOB_AMP_Y = 5.0 

var tbob = 0.0
var gun_default_pos = Vector2.ZERO 

# --- NEW HEALTH VARIABLES ---
var max_health: int = 100
var health: int = max_health

func _ready():
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
	gun_default_pos = gun_sprite.position
	
	# hookup hud to weapon_manager
	weapon_manager.ammo_updated.connect(hud.update_bullets)
	hud.update_bullets(weapon_manager.bullets)
	weapon_manager.magazine_count_updated.connect(hud.update_magazines)
	hud.update_magazines(weapon_manager.magazine_count)
	
	if weapon_manager.current_weapon:
		hud.update_weapon_ui(weapon_manager.current_weapon)
	
	if hud:
		hud.update_health(health)

func _process(_delta):
	if Input.is_action_just_pressed("shoot"): 
		weapon_manager.fire()
	if Input.is_action_just_pressed("reload"):
		weapon_manager.reload()
		
	if Input.is_key_pressed(KEY_1):
		weapon_manager.switch_to_slot("primary")
		if weapon_manager.current_weapon:
			hud.update_weapon_ui(weapon_manager.current_weapon)
	
	if Input.is_key_pressed(KEY_2):
		weapon_manager.switch_to_slot("secondary")
		if weapon_manager.current_weapon:
			hud.update_weapon_ui(weapon_manager.current_weapon)
		
	if Input.is_action_just_pressed("ui_cancel"):
		if Input.mouse_mode == Input.MOUSE_MODE_CAPTURED:
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		else:
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func _physics_process(delta: float) -> void:
	if not body.is_on_floor():
		body.velocity.y -= 9.8 * delta

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
	tbob += delta * body.velocity.length() * float(body.is_on_floor())
	camera.transform.origin = _headbob(tbob)
	
	# 5. Handle Weapon Bob (2D Sprite)
	var gun_bob_pos = Vector2.ZERO
	gun_bob_pos.y = sin(tbob * BOB_FREQ) * GUN_BOB_AMP_Y
	gun_bob_pos.x = cos(tbob * BOB_FREQ / 2) * GUN_BOB_AMP_X
	gun_sprite.position = gun_default_pos + gun_bob_pos

	# 6. Actually move the body
	body.move_and_slide()
	
func handle_item_pickup(item: ItemData):
	match item.type:
		ItemData.ItemType.HLTH_MEDKIT:
			health = clamp(health + item.amount, 0, max_health)
			hud.update_health(health)
			
		ItemData.ItemType.AMMO_MAGAZINE:
			weapon_manager.handle_item_pickup(item);
			
		ItemData.ItemType.AMMO_SHELL:
			# You can add logic for other guns here later!
			print("Picked up ", item.amount, " shells.")
			
		ItemData.ItemType.AMMO_ROCKET:
			print("Picked up ", item.amount, " rockets.")

func _headbob(time) -> Vector3:
	var pos = Vector3.ZERO
	pos.y = sin(time * BOB_FREQ) * BOB_AMP + default_height
	pos.x = cos(time * BOB_FREQ / 2) * BOB_AMP
	return pos
	
func _unhandled_input(event: InputEvent) -> void:
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
	
	if health <= 0:
		die()

func die():
	print("Player Died!")
	# For now, just restart the level when you die
	get_tree().reload_current_scene()

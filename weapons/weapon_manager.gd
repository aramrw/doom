extends Node

@onready var gun_sprite = $WeaponLayer/GunSprite
@onready var shoot_sound = $ShootSound 

signal ammo_updated(bullet_count: int)
signal magazine_count_updated(magazine_count: int)

@export var aim_raycast: RayCast3D

# --- NEW: Modular Weapon Data ---
@export var current_weapon: WeaponData

# We keep these here because they change constantly during gameplay
var bullets: int = 0
@export var max_magazines = 10
@export var magazine_count: int = 1
var is_reloading = false

# --- NEW: Action State ---
var current_action: WeaponAction = null
var current_step_index: int = 0
var current_shot_count: int = 0 # Tracked for spray patterns/recoil
var last_shot_time: float = 0.0

func _ready():
	# Connect to the frame changed signal to trigger effects on specific frames
	gun_sprite.frame_changed.connect(_on_gun_sprite_frame_changed)
	
	# When the game starts, load whatever gun is in the slot
	if current_weapon:
		equip_weapon(current_weapon)

func equip_weapon(new_weapon: WeaponData):
	current_weapon = new_weapon
	# Initialize bullets to the weapon's max capacity
	bullets = current_weapon.max_bullets 
	
	if current_weapon.sprite_frames:
		gun_sprite.sprite_frames = current_weapon.sprite_frames
		gun_sprite.play("idle")
		gun_sprite.offset = current_weapon.sprite_offset
		
	ammo_updated.emit(bullets)

func fire():
	if is_reloading or current_action:
		return
		
	if bullets <= 0:
		# Maybe play a click sound here later
		return
		
	start_action("primary")

func start_action(action_name: String):
	if not current_weapon or not current_weapon.actions.has(action_name):
		return
		
	current_action = current_weapon.actions[action_name]
	current_step_index = 0
	
	# Handle spray timing (CS:GO style)
	var now = Time.get_ticks_msec() / 1000.0
	if now - last_shot_time > 0.5: # Reset spray after 0.5s of idle
		current_shot_count = 0
	
	gun_sprite.play(current_action.animation_name)
	# Process the first frame immediately
	process_step()

func _on_gun_sprite_frame_changed():
	if not current_action:
		return
	process_step()

func process_step():
	var current_frame = gun_sprite.frame
	
	# Find any steps that match this frame
	for step in current_action.steps:
		if step.frame_index == current_frame:
			for effect in step.effects:
				effect.execute(self, self)
				
				# If we fired a bullet, track it
				if effect is HitscanEffect or effect is ProjectileEffect:
					bullets -= 1
					current_shot_count += 1
					last_shot_time = Time.get_ticks_msec() / 1000.0
					ammo_updated.emit(bullets)

func reload():
	if is_reloading or current_action or magazine_count <= 0:
		return

	is_reloading = true
	# Look for a reload action, otherwise fallback to animation name "reload"
	if current_weapon.actions.has("reload"):
		start_action("reload")
	else:
		gun_sprite.play("reload")

func _on_gun_sprite_animation_finished() -> void:
	if current_action:
		# Check if it should loop (like a chainsaw or auto-fire)
		if current_action.loop and Input.is_action_pressed("shoot"):
			start_action("primary")
		else:
			current_action = null
			gun_sprite.play("idle")
			
	elif gun_sprite.animation == "reload":
		is_reloading = false
		bullets = current_weapon.max_bullets
		magazine_count -= 1
		
		ammo_updated.emit(bullets)
		magazine_count_updated.emit(magazine_count)
		
		gun_sprite.play("idle")

func handle_item_pickup(data: ItemData):
	magazine_count += data.amount
	magazine_count_updated.emit(magazine_count) 
	print("Picked up item! New total: ", magazine_count)

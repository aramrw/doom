extends Node

@onready var gun_sprite = $WeaponLayer/GunSprite
var offhand_sprite: AnimatedSprite2D
@onready var shoot_sound = $ShootSound 

signal ammo_updated(bullet_count: int)
signal magazine_count_updated(magazine_count: int)
signal enemy_hit

@export var aim_raycast: RayCast3D

# --- Weapon Slots ---
@export var primary_weapon: WeaponData
@export var secondary_weapon: WeaponData
@export var third_weapon: WeaponData

var current_weapon: WeaponData
var current_slot: String = "none"

# Track ammo per slot
var slot_ammo = {
	"primary": {"bullets": 0, "magazines": 1},
	"secondary": {"bullets": 0, "magazines": 1},
	"third": {"bullets": 0, "magazines": 1}
}

# We keep these here because they are the "active" counts used by the manager
var bullets: int = 0
@export var magazine_count: int = 1
var is_reloading = false

# --- Action State ---
var current_action: WeaponAction = null
var current_step_index: int = 0
var current_shot_count: int = 0 # Tracked for spray patterns/recoil
var last_shot_time: float = 0.0
var last_processed_frame: int = -1

func _ready():
	print("WeaponManager: _ready starting")
	# Setup offhand sprite
	offhand_sprite = gun_sprite.duplicate()
	offhand_sprite.name = "OffhandSprite"
	$WeaponLayer.add_child(offhand_sprite)
	
	# Connect to the frame changed signal to trigger effects on specific frames
	gun_sprite.frame_changed.connect(_on_gun_sprite_frame_changed)
	
	# Default to primary slot if available
	if primary_weapon:
		print("WeaponManager: Found primary_weapon, switching...")
		switch_to_slot("primary")
	else:
		print("WeaponManager: NO primary_weapon found")

func switch_to_slot(slot_name: String):
	if slot_name == current_slot: return
	if is_reloading or current_action: return
	
	var next_weapon = null
	match slot_name:
		"primary": next_weapon = primary_weapon
		"secondary": next_weapon = secondary_weapon
		"third": next_weapon = third_weapon
	
	if not next_weapon: return
	
	# Save current ammo if we are switching from a valid slot
	if current_slot != "none":
		slot_ammo[current_slot]["bullets"] = bullets
		slot_ammo[current_slot]["magazines"] = magazine_count
	
	# Switch weapon
	current_slot = slot_name
	current_weapon = next_weapon
	
	# Load next ammo
	bullets = slot_ammo[current_slot]["bullets"]
	magazine_count = slot_ammo[current_slot]["magazines"]
	
	# If this is the first time equipping, fill it up
	if bullets == 0 and magazine_count > 0:
		bullets = current_weapon.ammo_give
	
	equip_weapon(current_weapon)

func equip_weapon(new_weapon: WeaponData):
	if not new_weapon:
		print("WeaponManager: equip_weapon called with NULL")
		return
		
	print("WeaponManager: equipping ", new_weapon.item_name)
	if new_weapon.sprite_frames:
		gun_sprite.sprite_frames = new_weapon.sprite_frames
		gun_sprite.play("idle")
		gun_sprite.offset = new_weapon.sprite_offset
		gun_sprite.flip_h = new_weapon.flip_h
	else:
		print("WeaponManager: weapon has NO sprite_frames")
		
	# Handle offhand
	if new_weapon.offhand_frames:
		offhand_sprite.show()
		offhand_sprite.sprite_frames = new_weapon.offhand_frames
		offhand_sprite.play("idle")
		offhand_sprite.offset = new_weapon.offhand_offset
		offhand_sprite.flip_h = new_weapon.offhand_flip_h
	else:
		offhand_sprite.hide()
		
	ammo_updated.emit(bullets)
	magazine_count_updated.emit(magazine_count)

func fire():
	if is_reloading or current_action:
		return
		
	start_action("primary")

func start_action(action_name: String):
	if not current_weapon or not current_weapon.actions.has(action_name):
		return
		
	var action = current_weapon.actions[action_name]
	if action.consumes_ammo and bullets <= 0:
		return
		
	current_action = action
	current_step_index = 0
	last_processed_frame = -1 # Reset for new action
	
	# Handle spray timing (CS:GO style)
	var now = Time.get_ticks_msec() / 1000.0
	if now - last_shot_time > 0.5: # Reset spray after 0.5s of idle
		current_shot_count = 0
	
	gun_sprite.play(current_action.animation_name)
	if current_weapon.offhand_frames:
		offhand_sprite.play(current_action.animation_name)
	
	# Process the first frame immediately
	process_step()

func _on_gun_sprite_frame_changed():
	if not current_action:
		return
	process_step()

func process_step():
	var current_frame = gun_sprite.frame
	
	if current_frame == last_processed_frame:
		return
	
	last_processed_frame = current_frame
	
	# Find any steps that match this frame
	for step in current_action.steps:
		if step.frame_index == current_frame:
			for effect in step.effects:
				if not effect: continue
				
				# Use a more robust check for firing effects
				var is_firing = effect.has_method("execute") and (
					effect is HitscanEffect or 
					effect is ProjectileEffect or 
					effect.get_class() == "HitscanEffect" or 
					effect.get_class() == "ProjectileEffect"
				)
				
				if effect.has_method("execute"):
					effect.execute(self, self)
				else:
					push_error("WeaponManager: Effect resource at " + effect.resource_path + " has no execute method!")
				
				# If we fired a bullet, track it
				if is_firing:
					if current_action.consumes_ammo:
						bullets -= 1
						print("WeaponManager: Consumed ammo (", effect.get_class(), "). Remaining: ", bullets)
						ammo_updated.emit(bullets)
					else:
						print("WeaponManager: Effect triggered but action does not consume ammo.")
					
					current_shot_count += 1
					last_shot_time = Time.get_ticks_msec() / 1000.0
				else:
					# This is likely a SoundEffect or other non-projectile/hitscan logic
					pass

func reload():
	if is_reloading or current_action or magazine_count <= 0:
		return

	# If the weapon has no reload action AND no reload animation, do a fast refill
	if not current_weapon.actions.has("reload") and not gun_sprite.sprite_frames.has_animation("reload"):
		is_reloading = false
		bullets = current_weapon.ammo_give
		magazine_count -= 1
		ammo_updated.emit(bullets)
		magazine_count_updated.emit(magazine_count)
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
			if current_weapon.offhand_frames:
				offhand_sprite.play("idle")
			
	elif gun_sprite.animation == "reload":
		is_reloading = false
		bullets = current_weapon.ammo_give
		magazine_count -= 1
		
		ammo_updated.emit(bullets)
		magazine_count_updated.emit(magazine_count)
		
		gun_sprite.play("idle")

func handle_weapon_pickup(data: WeaponData) -> bool:
	# 1. Check if we already have this weapon (compare names for resource robustness)
	var existing = [primary_weapon, secondary_weapon, third_weapon]
	for wp in existing:
		if wp and wp.item_name == data.item_name:
			magazine_count += 2
			magazine_count_updated.emit(magazine_count)
			return true
		
	# 2. Try to find an empty slot
	if not primary_weapon:
		primary_weapon = data
		switch_to_slot("primary")
		return true
	elif not secondary_weapon:
		secondary_weapon = data
		switch_to_slot("secondary")
		return true
	elif not third_weapon:
		third_weapon = data
		switch_to_slot("third")
		return true
		
	# 3. All slots full - SWAP with current weapon
	if current_slot != "none":
		# Drop current weapon before replacing
		_drop_weapon(current_weapon)
		
		# Replace in the active slot
		match current_slot:
			"primary": primary_weapon = data
			"secondary": secondary_weapon = data
			"third": third_weapon = data
		
		# Re-equip the new weapon
		current_weapon = data
		equip_weapon(current_weapon)
		return true
		
	return false

func _drop_weapon(wp_data: WeaponData):
	if not wp_data: return
	
	# Find a pickup scene to spawn
	var scene_to_spawn = wp_data.pickup_scene
	if not scene_to_spawn:
		# Fallback: look for the auto-generated pickup scene in the same folder
		var path = wp_data.resource_path.replace("_wpdata.tres", "_pickup.tscn")
		if FileAccess.file_exists(path):
			scene_to_spawn = load(path)
			
	if scene_to_spawn:
		var pickup = scene_to_spawn.instantiate()
		get_tree().root.add_child(pickup)
		
		# Position it in front of the player
		var player = get_tree().get_first_node_in_group("Player")
		if player:
			var body = player.get_node("CharacterBody3D")
			pickup.global_position = body.global_position + (-body.global_transform.basis.z * 1.5)
			pickup.global_position.y += 0.5

func handle_item_pickup(data: ItemData):
	magazine_count += data.amount
	magazine_count_updated.emit(magazine_count) 
	print("Picked up item! New total: ", magazine_count)

extends Node

@onready var gun_sprite = $WeaponLayer/GunSprite
@onready var shoot_sound = $ShootSound 

signal ammo_updated(bullet_count: int)
signal magazine_count_updated(magazine_count: int)

@export var aim_raycast: RayCast3D

# --- NEW: This replaces all the hardcoded damage/bullet stats! ---
@export var current_weapon: WeaponData

# We keep these here because they change constantly during gameplay
var bullets: int = 0
@export var max_magazines = 10
@export var magazine_count: int = 1
var is_reloading = false

func _ready():
	# When the game starts, load whatever gun is in the slot
	if current_weapon:
		equip_weapon(current_weapon)

func equip_weapon(new_weapon: WeaponData):
	current_weapon = new_weapon
	bullets = current_weapon.max_bullets
	
	if current_weapon.sprite_frames:
		gun_sprite.sprite_frames = current_weapon.sprite_frames
		gun_sprite.play("idle")
		
		# --- NEW: Apply the custom offset for this specific gun ---
		gun_sprite.offset = current_weapon.sprite_offset
		
	if current_weapon.shoot_sound:
		shoot_sound.stream = current_weapon.shoot_sound
		
	ammo_updated.emit(bullets)

func reload():
	# Updated to check current_weapon.max_bullets
	if is_reloading or bullets == current_weapon.max_bullets or magazine_count <= 0:
		return

	is_reloading = true
	gun_sprite.play("reload")
	# reload_sound.play() 
	
func handle_item_pickup(data: ItemData):
	magazine_count += data.amount
	magazine_count_updated.emit(magazine_count) 
	print("Picked up item! New total: ", magazine_count)

func _on_gun_sprite_animation_finished() -> void:
	if gun_sprite.animation == "shoot":
		gun_sprite.play("idle")
		
	elif gun_sprite.animation == "reload":
		is_reloading = false
		# Updated to use current_weapon.max_bullets
		bullets = current_weapon.max_bullets 
		magazine_count -= 1
		
		ammo_updated.emit(bullets)
		magazine_count_updated.emit(magazine_count)
		
		gun_sprite.play("idle")

func fire():
	# Safety check: make sure we actually have a weapon equipped
	if current_weapon == null:
		return
		
	if gun_sprite.animation == "shoot" and gun_sprite.is_playing():
		return
	
	if bullets <= 0:
		return
		
	bullets -= 1
	ammo_updated.emit(bullets)
		
	gun_sprite.play("shoot")
	shoot_sound.play()
	
	# --- NEW: The Shotgun/Spread Logic ---
	var original_rotation = aim_raycast.rotation_degrees
	
	# Loop through however many pellets the weapon has (1 for pistol, 8 for TOZ-34)
	for i in range(current_weapon.pellet_count):
		
		# Apply random spread
		var spread_x = randf_range(-current_weapon.spread_angle, current_weapon.spread_angle)
		var spread_y = randf_range(-current_weapon.spread_angle, current_weapon.spread_angle)
		
		aim_raycast.rotation_degrees = original_rotation + Vector3(spread_x, spread_y, 0)
		aim_raycast.force_raycast_update()
		
		if aim_raycast.is_colliding():
			var target = aim_raycast.get_collider()
			if target.has_method("take_damage"):
				# Updated to use current_weapon.damage
				target.take_damage(current_weapon.damage) 
				
	# Reset the raycast back to dead center for the next shot
	aim_raycast.rotation_degrees = original_rotation

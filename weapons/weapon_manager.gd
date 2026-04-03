extends Node

@onready var gun_sprite = $WeaponLayer/GunSprite
@onready var shoot_sound = $ShootSound 

signal ammo_updated(bullet_count: int);
signal magazine_count_updated(magazine_count: int);

# 1. Grab the RayCast we just made. 
# (Since this script is on WeaponManager, we go up one folder with "..", 
# then down into CharacterBody3D/Camera to find it).
@export var aim_raycast: RayCast3D;
@export_category("Gun Info")
@export var damage: int = 35 
@export var bullets: int = 10
@export_subgroup("Magazines")
@export var magazine_max = 10;
@export var magazine_count: int = 1;

func reload():
	if magazine_count <= 0:
		return
	magazine_count -= 1;
	magazine_count_updated.emit(magazine_count);
	

func fire():
	if gun_sprite.animation == "shoot" and gun_sprite.is_playing():
		return
	
	if bullets <= 0:
		return
	bullets -= 1;
	ammo_updated.emit(bullets);
		
	gun_sprite.play("shoot")
	shoot_sound.play()
	
	# 3. Force the raycast to update its position instantly
	aim_raycast.force_raycast_update()
	
	# 4. Check if the laser is touching anything
	if aim_raycast.is_colliding():
		var target = aim_raycast.get_collider()
		
		# Check if the thing we hit has our take_damage function
		if target.has_method("take_damage"):
			target.take_damage(damage)

func _on_gun_sprite_animation_finished() -> void:
	if gun_sprite.animation == "shoot":
		gun_sprite.play("idle")

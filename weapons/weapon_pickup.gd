extends Area3D
class_name WeaponPickup

@export var weapon_data: WeaponData

@onready var anim_sprite = $AnimatedSprite3D

func _ready():
	if weapon_data and weapon_data.sprite_frames:
		anim_sprite.sprite_frames = weapon_data.sprite_frames
		# Use 'ground' or 'idle' animation for the floor item
		if anim_sprite.sprite_frames.has_animation("ground"):
			anim_sprite.play("ground")
		elif anim_sprite.sprite_frames.has_animation("idle"):
			anim_sprite.play("idle")
	
	# We'll use a manual check in WeaponManager for "G" pickup, 
	# but we can also allow auto-pickup if desired.
	body_entered.connect(_on_body_entered)

func _on_body_entered(body):
	# Optional: auto-pickup if slot is empty? 
	# For now, let's stick to manual "G" pickup.
	pass

func interact(weapon_manager: Node):
	if weapon_manager.has_method("handle_weapon_pickup"):
		if weapon_manager.handle_weapon_pickup(weapon_data):
			queue_free()

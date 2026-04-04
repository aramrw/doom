# weapons/effects/projectile_effect.gd
extends WeaponEffect
class_name ProjectileEffect

@export var projectile_scene: PackedScene
@export var speed: float = 30.0
@export var damage: int = 25

func execute(_source_node: Node, weapon_manager: Node) -> void:
	if not projectile_scene: return
	
	var proj = projectile_scene.instantiate()
	# Add to the scene tree root or a dedicated projectiles node
	weapon_manager.get_tree().root.add_child(proj)
	
	# Position at the aim raycast
	var ray = weapon_manager.aim_raycast
	proj.global_transform = ray.global_transform
	
	# Set properties if the projectile script supports them
	if proj.has_method("setup"):
		proj.setup(damage, speed, -ray.global_transform.basis.z)
	elif "speed" in proj:
		proj.speed = speed
		proj.damage = damage
		proj.direction = -ray.global_transform.basis.z

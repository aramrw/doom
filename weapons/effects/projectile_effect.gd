# weapons/effects/projectile_effect.gd
extends WeaponEffect
class_name ProjectileEffect

@export var projectile_scene: PackedScene
@export var speed: float = 30.0
@export var damage: int = 25

func execute(source_node: Node, weapon_manager: Node) -> void:
	if not projectile_scene: return
	
	var proj = projectile_scene.instantiate()
	# Add to tree
	weapon_manager.get_tree().root.add_child(proj)
	
	# Position at the aim raycast
	var ray = weapon_manager.aim_raycast
	proj.global_transform = ray.global_transform
	
	# Use the new setup method
	if proj.has_method("setup"):
		var dir = -ray.global_transform.basis.z
		# firer is the player's body usually, or the weapon_manager's parent
		var firer = weapon_manager.get_parent() 
		proj.setup(firer, dir, damage, speed)

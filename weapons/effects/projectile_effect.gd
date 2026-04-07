# weapons/effects/projectile_effect.gd
extends WeaponEffect
class_name ProjectileEffect

@export var projectile_scene: PackedScene
@export var speed: float = 30.0
@export var damage: int = 25
@export var lifetime: float = 5.0
@export var projectile_scale: float = 1.0

func execute(source_node: Node, weapon_manager: Node) -> void:
	if not projectile_scene: return
	
	var proj = projectile_scene.instantiate()
	
	# Set properties BEFORE adding to tree so _ready() picks them up
	proj.scale = Vector3.ONE * projectile_scale
	if "lifetime" in proj:
		proj.lifetime = lifetime
	
	# Position at the aim raycast
	var ray = weapon_manager.aim_raycast
	proj.global_transform = ray.global_transform
	
	# Use the setup method
	if proj.has_method("setup"):
		var dir = -ray.global_transform.basis.z
		var firer = weapon_manager.get_parent() 
		proj.setup(firer, dir, damage, speed, lifetime)
		
	# Add to tree LAST so _ready() runs with all data set
	weapon_manager.get_tree().root.add_child(proj)

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
	
	# Add to tree EARLY so we can access global_transform
	source_node.get_tree().root.add_child(proj)
	
	# Initial position
	var spawn_pos = source_node.global_position + Vector3(0, 1.2, 0)
	if weapon_manager and weapon_manager.aim_raycast:
		spawn_pos = weapon_manager.aim_raycast.global_position
	proj.global_position = spawn_pos

	# Determine direction and firer
	var dir = Vector3.ZERO
	var firer = source_node
	
	if weapon_manager and weapon_manager.aim_raycast:
		dir = -weapon_manager.aim_raycast.global_transform.basis.z
		firer = weapon_manager.get_parent()
	else:
		# Enemy logic
		var target_node = source_node.get("target_node")
		var target_pos = Vector3.ZERO
		var has_target = false
		
		if target_node and is_instance_valid(target_node):
			var target_body = target_node
			if target_node.has_node("CharacterBody3D"):
				target_body = target_node.get_node("CharacterBody3D")
			target_pos = target_body.global_position
			has_target = true
		elif source_node.get("player") and source_node.player:
			target_pos = source_node.player.global_position
			has_target = true
			
		if has_target:
			dir = proj.global_position.direction_to(target_pos + Vector3(0, 1.0, 0))
		else:
			dir = -source_node.global_transform.basis.z

	# Apply to projectile
	if proj.has_method("setup"):
		proj.setup(firer, dir, damage, speed, lifetime)
	
	if dir.length() > 0.001:
		proj.look_at(proj.global_position + dir, Vector3.UP)

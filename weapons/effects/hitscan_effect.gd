extends WeaponEffect
class_name HitscanEffect

@export var damage: int = 10
@export var pellets: int = 1
@export var spread_angle: float = 0.0
@export var range_distance: float = 100.0

func execute(source_node: Node, weapon_manager: Node) -> void:
	var raycast = null
	if weapon_manager and weapon_manager.aim_raycast:
		raycast = weapon_manager.aim_raycast
	elif source_node.has_node("RayCast3D"):
		raycast = source_node.get_node("RayCast3D")
	
	if not raycast: return
	
	var original_rotation = raycast.rotation_degrees
	
	for i in range(pellets):
		var spread_x = randf_range(-spread_angle, spread_angle)
		var spread_y = randf_range(-spread_angle, spread_angle)
		raycast.rotation_degrees = original_rotation + Vector3(spread_x, spread_y, 0)
		raycast.target_position = Vector3(0, 0, -range_distance)
		raycast.force_raycast_update()
		
		if raycast.is_colliding():
			var target = raycast.get_collider()
			if target.has_method("take_damage"):
				target.take_damage(damage)
				if weapon_manager and weapon_manager.has_signal("enemy_hit"):
					weapon_manager.enemy_hit.emit()
	
	raycast.rotation_degrees = original_rotation

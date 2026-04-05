extends WeaponEffect
class_name CameraShakeEffect

@export var trauma_amount: float = 0.01

func execute(_source_node: Node, weapon_manager: Node) -> void:
	var player = weapon_manager.get_parent()
	if player.has_method("add_trauma"):
		player.add_trauma(trauma_amount)

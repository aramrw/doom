# weapons/effects/visual_effect.gd
extends WeaponEffect
class_name VisualEffect

@export var screen_shake: float = 0.0
@export var muzzle_flash: bool = false
# Future: Add particle systems here

func execute(_source_node: Node, _weapon_manager: Node) -> void:
	# Implementation for screen shake or light flash
	pass

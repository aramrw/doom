extends WeaponEffect
class_name SoundEffect

@export var sound: AudioStream
@export var pitch_randomness: float = 0.1
@export var volume_db: float = 0.0

func execute(source_node: Node, weapon_manager: Node) -> void:
	var player = null
	if weapon_manager and weapon_manager.shoot_sound:
		player = weapon_manager.shoot_sound
	elif source_node.has_node("EnemySounds"):
		player = source_node.get_node("EnemySounds")
	
	if player:
		player.stream = sound
		player.pitch_scale = 1.0 + randf_range(-pitch_randomness, pitch_randomness)
		player.volume_db = volume_db
		player.play();

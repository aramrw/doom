extends WeaponEffect
class_name SoundEffect

@export var sound: AudioStream
@export var pitch_randomness: float = 0.1
@export var volume_db: float = 0.0

func execute(_source_node: Node, weapon_manager: Node) -> void:
	if weapon_manager.shoot_sound:
		var shoot_sound: AudioStreamPlayer = weapon_manager.shoot_sound
		shoot_sound.stream = sound
		shoot_sound.pitch_scale = 1.0 + randf_range(-pitch_randomness, pitch_randomness)
		shoot_sound.volume_db = volume_db
		shoot_sound.play();

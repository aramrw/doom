extends WeaponEffect
class_name SoundEffect

@export var sound: AudioStream
@export var pitch_randomness: float = 0.1

func execute(_source_node: Node, weapon_manager: Node) -> void:
    if weapon_manager.shoot_sound:
        weapon_manager.shoot_sound.stream = sound
        weapon_manager.shoot_sound.pitch_scale = 1.0 + randf_range(-pitch_randomness, pitch_randomness)
        weapon_manager.shoot_sound.play()

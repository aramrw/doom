extends StaticBody3D

## GZDoom-style Prop State Machine
## Maps DECORATE state labels (Spawn, Active, Inactive, etc.) to Godot animations.

@export var initial_state: String = "spawn"

func _ready():
	# GZDoom actors always start in the 'Spawn' state.
	# Fallback to the first available animation if 'spawn' doesn't exist.
	if not _has_state(initial_state):
		var available = _get_available_states()
		if available.size() > 0:
			initial_state = available[0]
			# If 'spawn' is missing but 'idle' exists, prefer that as fallback
			if "idle" in available:
				initial_state = "idle"
	
	goto_state(initial_state)

## Transitions all internal sprites to the requested state label.
func goto_state(state_label: String):
	var anim_name = state_label.to_lower()
	for child in get_children():
		if child is AnimatedSprite3D:
			if child.sprite_frames.has_animation(anim_name):
				child.play(anim_name)

## Thing_Activate equivalent
func activate():
	goto_state("active")

## Thing_Deactivate equivalent
func deactivate():
	goto_state("inactive")

func _has_state(state_label: String) -> bool:
	var frames = _get_sprite_frames()
	return frames != null and frames.has_animation(state_label.to_lower())

func _get_available_states() -> Array:
	var frames = _get_sprite_frames()
	if frames:
		return Array(frames.get_animation_names())
	return []

func _get_sprite_frames() -> SpriteFrames:
	for child in get_children():
		if child is AnimatedSprite3D:
			return child.sprite_frames
	return null

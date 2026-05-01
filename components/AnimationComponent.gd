extends Node
class_name AnimationComponent

@export var sprite: AnimatedSprite3D
@export var actor: Node3D

var current_state: String = "idle"

func play_idle():
	_play_anim("idle")

func play_chase():
	_play_anim("chase")

func on_attack_fired(_damage):
	_play_anim("attack")

func on_damaged(_amount, _new_health, _source):
	_play_anim("pain")

func on_died(_source):
	_play_anim("death")

func _play_anim(anim_name: String):
	current_state = anim_name
	
	# Fallback if animation doesn't exist (e.g., chase might just use idle or walk)
	var target_anim = anim_name
	if not sprite.sprite_frames.has_animation(target_anim):
		if target_anim == "chase" and sprite.sprite_frames.has_animation("walk"):
			target_anim = "walk"
		elif not sprite.sprite_frames.has_animation(target_anim):
			return # Animation not found

	sprite.play(target_anim)
	
	# If it's a one-shot animation like attack or pain, return to idle after
	if anim_name in ["attack", "pain"]:
		sprite.set_frame_and_progress(0, 0.0)
		
		# Connect to animation_finished if not already connected
		if not sprite.animation_finished.is_connected(_on_animation_finished):
			sprite.animation_finished.connect(_on_animation_finished)

func _on_animation_finished():
	if current_state in ["attack", "pain"]:
		if sprite.animation_finished.is_connected(_on_animation_finished):
			sprite.animation_finished.disconnect(_on_animation_finished)
		play_idle()

func _process(_delta):
	if not actor or not sprite:
		return
	# flipping logic could go here, but it's currently in the main script
	pass

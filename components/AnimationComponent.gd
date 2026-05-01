extends Node
class_name AnimationComponent

@export var use_multi_directional: bool = false
@export var sprite: AnimatedSprite3D
@export var actor: Node3D
@export var flash_duration: float = 0.2
@export var death_offset: Vector3 = Vector3.ZERO
@export var death_linger_time: float = 2.0

var current_state: String = "idle"
var visual_mat: ShaderMaterial
var _flash_tween: Tween
var is_dead: bool = false
var original_sprite_pos: Vector3

func _ready():
	if not sprite and get_parent().has_node("AnimatedSprite3D"):
		sprite = get_parent().get_node("AnimatedSprite3D")
	
	if sprite:
		original_sprite_pos = sprite.position
		visual_mat = ShaderMaterial.new()
		var shader = load("res://shaders/enemy_visuals.gdshader")
		if shader:
			visual_mat.shader = shader
		
		sprite.material_override = visual_mat
		sprite.visible = true
		sprite.modulate = Color(1, 1, 1, 1)
		
		if actor:
			play_idle()

func _process(_delta):
	if sprite and visual_mat:
		var frames = sprite.sprite_frames
		if frames and frames.has_animation(sprite.animation):
			var tex = frames.get_frame_texture(sprite.animation, sprite.frame)
			visual_mat.set_shader_parameter("tex", tex)

func play_idle():
	if is_dead: return
	_play_anim("idle")

func play_chase():
	if is_dead: return
	_play_anim("chase")

func on_attack_fired(_damage):
	if is_dead: return
	_play_anim("attack")

func on_damaged(_amount, new_health, _source):
	if is_dead: return
	flash()
	if new_health > 0:
		_play_anim("pain")

func on_died(_source):
	if is_dead: return
	is_dead = true
	
	if sprite and death_offset != Vector3.ZERO:
		sprite.position = original_sprite_pos + death_offset
		
	_play_anim("death")
	_handle_death_cleanup()

func _handle_death_cleanup():
	# Wait for the death animation to finish
	if sprite and sprite.sprite_frames.has_animation("death"):
		await sprite.animation_finished
	else:
		await get_tree().create_timer(1.0).timeout
	
	# Linger
	if death_linger_time > 0:
		await get_tree().create_timer(death_linger_time).timeout
	
	# Finally, remove the parent
	get_parent().queue_free()

func flash():
	if not visual_mat: return
	if _flash_tween:
		_flash_tween.kill()
	_flash_tween = create_tween()
	_flash_tween.tween_method(func(v): visual_mat.set_shader_parameter("hit_flash", v), 1.0, 0.0, flash_duration)

func _play_anim(anim_name: String):
	if not sprite: return
	
	current_state = anim_name
	
	if use_multi_directional:
		update_facing_animation()
	else:
		var target_anim = anim_name
		if not sprite.sprite_frames.has_animation(target_anim):
			if target_anim == "chase":
				if sprite.sprite_frames.has_animation("move"):
					target_anim = "move"
				elif sprite.sprite_frames.has_animation("walk"):
					target_anim = "walk"
				else:
					target_anim = "idle"
			elif target_anim in ["pain", "death"]:
				return
			else:
				return

		sprite.play(target_anim)
	
	if anim_name in ["attack", "pain"]:
		sprite.set_frame_and_progress(0, 0.0)
		if not sprite.animation_finished.is_connected(_on_animation_finished):
			sprite.animation_finished.connect(_on_animation_finished)

func _on_animation_finished():
	if is_dead: return
	if current_state in ["attack", "pain"]:
		if sprite.animation_finished.is_connected(_on_animation_finished):
			sprite.animation_finished.disconnect(_on_animation_finished)
		current_state = "idle"

func update_facing_animation():
	var camera = get_viewport().get_camera_3d()
	if not camera or not actor or not sprite:
		return
		
	var to_cam = (camera.global_position - actor.global_position)
	var to_cam2 = Vector2(to_cam.x, to_cam.z).normalized()
	
	var fwd = -actor.global_transform.basis.z
	var fwd2 = Vector2(fwd.x, fwd.z).normalized()
	
	var angle = rad_to_deg(fwd2.angle_to(to_cam2))
	if angle < 0:
		angle += 360.0
		
	var sector = int(round(angle / 45.0)) % 8
	var frame_index = 1
	var flip = false
	
	match sector:
		0:
			frame_index = 1
			flip = false
		1:
			frame_index = 2
			flip = true
		2:
			frame_index = 3
			flip = true
		3:
			frame_index = 4
			flip = true
		4:
			frame_index = 5
			flip = false
		5:
			frame_index = 4
			flip = false
		6:
			frame_index = 3
			flip = false
		7:
			frame_index = 2
			flip = false
			
	var target_anim = current_state + "_" + str(frame_index)
	var valid_anim = ""
	
	var frames = sprite.sprite_frames
	if not frames: return
	
	if frames.has_animation(target_anim):
		valid_anim = target_anim
	elif frames.has_animation(current_state + "_1"):
		valid_anim = current_state + "_1"
	elif frames.has_animation(current_state):
		valid_anim = current_state
	elif current_state == "chase":
		if frames.has_animation("move_" + str(frame_index)):
			valid_anim = "move_" + str(frame_index)
		elif frames.has_animation("walk_" + str(frame_index)):
			valid_anim = "walk_" + str(frame_index)
		elif frames.has_animation("move_1"):
			valid_anim = "move_1"
		elif frames.has_animation("walk_1"):
			valid_anim = "walk_1"
		elif frames.has_animation("move"):
			valid_anim = "move"
		elif frames.has_animation("walk"):
			valid_anim = "walk"
			
	if valid_anim == "":
		return
		
	sprite.flip_h = flip
	
	if sprite.animation != valid_anim:
		var current_frame = sprite.frame
		var current_progress = sprite.frame_progress
		var was_playing = sprite.is_playing()
		
		var base_old = str(sprite.animation).get_slice("_", 0)
		var base_new = valid_anim.get_slice("_", 0)
		var should_preserve_frame = was_playing and (base_old == base_new)
		
		sprite.play(valid_anim)
		
		if should_preserve_frame:
			sprite.set_frame_and_progress(current_frame, current_progress)

func _physics_process(_delta):
	if not actor or not sprite:
		return
		
	if use_multi_directional:
		update_facing_animation()
		
	if is_dead: return
	
	if current_state in ["idle", "walk", "chase", "move"]:
		if actor.velocity.length() > 0.1:
			if current_state not in ["chase", "move", "walk"]:
				play_chase()
		else:
			if current_state != "idle":
				play_idle()

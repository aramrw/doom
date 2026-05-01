extends Node
class_name AnimationComponent

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

	current_state = anim_name
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

func _physics_process(_delta):
	if is_dead: return
	if not actor or not sprite:
		return
	
	if current_state in ["idle", "walk", "chase", "move"]:
		if actor.velocity.length() > 0.1:
			if current_state not in ["chase", "move", "walk"]:
				play_chase()
		else:
			if current_state != "idle":
				play_idle()

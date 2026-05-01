extends Node
class_name AnimationComponent

@export var sprite: AnimatedSprite3D
@export var actor: Node3D
@export var flash_duration: float = 0.2

var current_state: String = "idle"
var visual_mat: ShaderMaterial
var _flash_tween: Tween
var is_dead: bool = false

func _ready():
	print("[AnimationComponent] Initializing for ", get_parent().name)
	
	if not sprite and get_parent().has_node("AnimatedSprite3D"):
		sprite = get_parent().get_node("AnimatedSprite3D")
	
	if sprite:
		# Setup the shader material
		visual_mat = ShaderMaterial.new()
		var shader = load("res://shaders/enemy_visuals.gdshader")
		if shader:
			visual_mat.shader = shader
		else:
			push_error("[AnimationComponent] FAILED to load shader res://shaders/enemy_visuals.gdshader")
		
		sprite.material_override = visual_mat
		
		# Ensure sprite is visible
		sprite.visible = true
		sprite.modulate = Color(1, 1, 1, 1)
		
		if actor:
			play_idle()

func _process(_delta):
	# Sync the texture to the shader every frame
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
	
	print("[AnimationComponent] ", get_parent().name, " FLASHING.")
	flash()
	
	# Only play pain if we are still alive
	if new_health > 0:
		_play_anim("pain")

func on_died(_source):
	if is_dead: return
	is_dead = true
	print("[AnimationComponent] Playing death for ", get_parent().name)
	_play_anim("death")

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
	
	# Only auto-switch if we are in a movement-capable state
	if current_state in ["idle", "walk", "chase", "move"]:
		if actor.velocity.length() > 0.1:
			if current_state not in ["chase", "move", "walk"]:
				play_chase()
		else:
			if current_state != "idle":
				play_idle()

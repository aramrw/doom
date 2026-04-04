extends BaseProjectile
class_name DirectionalProjectile

@onready var sprite = $AnimatedSprite3D

func _process(_delta):
	update_sprite_angle()

func update_sprite_angle():
	var camera = get_viewport().get_camera_3d()
	if not camera or not sprite: return
	
	# Forward direction is the projectile's movement direction
	var forward_dir = direction
	var to_camera_dir = global_position.direction_to(camera.global_position)
	
	# Project both onto XZ plane for 2D angle calculation
	var forward_2d = Vector2(forward_dir.x, forward_dir.z).normalized()
	var to_cam_2d = Vector2(to_camera_dir.x, to_camera_dir.z).normalized()
	
	var angle = to_cam_2d.angle_to(forward_2d)
	var angle_index = int(round(angle / (PI / 4.0)))
	if angle_index < 0: angle_index += 8
	angle_index = angle_index % 8
	
	var anim_suffix = ""
	var flip = false
	match angle_index:
		0: anim_suffix = "1"; flip = false
		1: anim_suffix = "2"; flip = false
		2: anim_suffix = "3"; flip = false
		3: anim_suffix = "4"; flip = false
		4: anim_suffix = "5"; flip = false
		5: anim_suffix = "4"; flip = true
		6: anim_suffix = "3"; flip = true
		7: anim_suffix = "2"; flip = true
			
	sprite.flip_h = flip
	var target_anim = "fly_" + anim_suffix
	
	if sprite.animation != target_anim:
		var current_frame = sprite.frame
		var current_progress = sprite.frame_progress
		sprite.play(target_anim)
		sprite.set_frame_and_progress(current_frame, current_progress)

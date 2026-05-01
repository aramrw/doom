extends CharacterBody3D
class_name BaseEnemy

@export var death_linger_time: float = 2.0

@onready var sprite = $AnimatedSprite3D
@onready var health_component = $HealthComponent
@onready var attack_component = $AttackComponent
@onready var chase_component = $ChaseComponent
@onready var animation_component = $AnimationComponent

func _ready():
	print("[%s] _ready() called" % name)
	# Make sure the animation component has its references
	if animation_component:
		animation_component.sprite = sprite
		animation_component.actor = self
		animation_component.play_idle()
	
	# Wire up signals
	if health_component and animation_component:
		health_component.damaged.connect(animation_component.on_damaged)
		health_component.died.connect(animation_component.on_died)
		health_component.died.connect(_on_died)
	
	if attack_component and animation_component:
		attack_component.is_aggressive = true
		attack_component.attack_fired.connect(animation_component.on_attack_fired)
		
	if chase_component:
		chase_component.is_aggressive = true


func _process(_delta):
	# Simple sprite flipping based on target position
	var target = null
	if chase_component and is_instance_valid(chase_component.target):
		target = chase_component.target
	elif attack_component and is_instance_valid(attack_component.target):
		target = attack_component.target
		
	if target and sprite:
		# Very simple flip logic based on X position relative to the camera
		var camera = get_viewport().get_camera_3d()
		if camera:
			var to_target = target.global_position - global_position
			var right_dir = camera.global_transform.basis.x
			var dot = to_target.dot(right_dir)
			
			if dot > 0.1:
				sprite.flip_h = true
			elif dot < -0.1:
				sprite.flip_h = false


func take_damage(amount: int, source = null):
	if health_component:
		health_component.take_damage(amount, source)

func _on_died(_source):
	# Stop logic components
	if chase_component:
		chase_component.set_physics_process(false)
	if attack_component:
		attack_component.set_process(false)
		
	# Disable collisions so player doesn't bump into the "corpse"
	collision_layer = 0
	collision_mask = 0

	# Wait for the death animation to finish
	if sprite.sprite_frames.has_animation("death"):
		await sprite.animation_finished
	else:
		await get_tree().create_timer(1.0).timeout
	
	# Linger for a bit before disappearing
	if death_linger_time > 0:
		await get_tree().create_timer(death_linger_time).timeout
	
	queue_free()

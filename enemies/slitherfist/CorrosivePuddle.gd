extends Area3D
class_name CorrosivePuddle

@export var damage_per_second: int = 5
@export var duration: float = 3.0
@export var tick_rate: float = 0.5 # How often to apply damage

var _damage_timer: Timer
var _despawn_timer: Timer

func _ready():
	_damage_timer = Timer.new()
	_damage_timer.wait_time = tick_rate
	_damage_timer.autostart = true
	_damage_timer.timeout.connect(_on_damage_timer_timeout)
	add_child(_damage_timer)

	_despawn_timer = Timer.new()
	_despawn_timer.wait_time = duration
	_despawn_timer.one_shot = true
	_despawn_timer.timeout.connect(queue_free)
	add_child(_despawn_timer)
	
	_despawn_timer.start()
	
	# Connect to body_entered and body_exited signals
	body_entered.connect(_on_body_entered)
	body_exited.connect(_on_body_exited)

func _on_damage_timer_timeout():
	for body in get_overlapping_bodies():
		if body.has_method("take_damage"):
			body.take_damage(damage_per_second * tick_rate) # Scale damage by tick rate

func _on_body_entered(body: Node3D):
	# Optional: Apply initial damage or visual effect
	pass

func _on_body_exited(body: Node3D):
	# Optional: Remove any persistent effects from the body
	pass

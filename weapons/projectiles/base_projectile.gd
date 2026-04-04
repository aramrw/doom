extends Area3D
class_name BaseProjectile

@export var speed: float = 20.0
@export var damage: int = 10
@export var lifetime: float = 5.0

var direction: Vector3 = Vector3.ZERO
var firer: Node3D = null

func _ready():
	# Auto-destroy after lifetime
	get_tree().create_timer(lifetime).timeout.connect(queue_free)
	body_entered.connect(_on_body_entered)

func setup(p_firer: Node3D, p_direction: Vector3, p_damage: int = -1, p_speed: float = -1.0):
	firer = p_firer
	direction = p_direction.normalized()
	
	if p_damage > 0: damage = p_damage
	if p_speed > 0: speed = p_speed
	
	# Team detection logic
	if firer.is_in_group("Player"):
		collision_mask = 4 # Layer 3: Enemies
		add_to_group("player_projectiles")
	else:
		collision_mask = 2 # Layer 2: Player
		add_to_group("enemy_projectiles")

func _physics_process(delta):
	global_position += direction * speed * delta

func _on_body_entered(body: Node3D):
	if body.has_method("take_damage"):
		body.take_damage(damage)
	
	_on_impact()

func _on_impact():
	# Virtual function for children to override
	queue_free()

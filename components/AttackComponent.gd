extends Node
class_name AttackComponent

signal attack_fired(damage_amount)

@export var attack_damage: int = 10
@export var attack_range: float = 2.5
@export var time_between_attacks: float = 1.0
@export var is_aggressive: bool = true
@export var target_group: String = "Player"
@export var debug_mode: bool = true
@export var check_los: bool = true

@export var actor: Node3D

var target: Node3D = null
var cooldown_timer: float = 0.0
var los_ray: RayCast3D

func _ready():
	if not actor:
		actor = get_parent()
	
	if check_los:
		los_ray = RayCast3D.new()
		los_ray.enabled = false # We'll manual update it
		los_ray.collision_mask = 1 | 2 | 4 # Env + Player + Enemies
		add_child(los_ray)
		
	if debug_mode:
		print("[AttackComponent] ", actor.name, " initialized. Range: ", attack_range)

func _process(delta: float):
	if not actor or not is_aggressive:
		return
		
	if cooldown_timer > 0:
		cooldown_timer -= delta
		
	_find_nearest_target()
	
	if not is_instance_valid(target):
		return
		
	var dist = actor.global_position.distance_to(target.global_position)
	
	if dist <= attack_range and cooldown_timer <= 0:
		if not check_los or _has_los():
			_perform_attack(dist)

func _has_los() -> bool:
	if not is_instance_valid(target) or not los_ray:
		return true
		
	los_ray.global_position = actor.global_position + Vector3(0, 1, 0) # Ray from eye height
	los_ray.target_position = los_ray.to_local(target.global_position + Vector3(0, 1, 0))
	los_ray.force_raycast_update()
	
	if los_ray.is_colliding():
		var collider = los_ray.get_collider()
		# If we hit the target or its parent/child, we have LOS
		return collider == target or collider.get_parent() == target or target.get_parent() == collider
	
	return true

func _perform_attack(current_dist: float):
	cooldown_timer = time_between_attacks
	attack_fired.emit(attack_damage)
	
	if debug_mode:
		print_rich("[color=red][Attack][/color] ", actor.name, " hitting ", target.name, " at dist: ", "%.2f" % current_dist)
	
	if target.has_method("take_damage"):
		target.take_damage(attack_damage, actor)
	elif target.get_parent() and target.get_parent().has_method("take_damage"):
		target.get_parent().take_damage(attack_damage, actor)
	else:
		var health = target.get_node_or_null("HealthComponent")
		if not health and target.get_parent():
			health = target.get_parent().get_node_or_null("HealthComponent")
			
		if health and health.has_method("take_damage"):
			health.take_damage(attack_damage, actor)

func _find_nearest_target():
	var targets = get_tree().get_nodes_in_group(target_group)
	var nearest_dist = INF
	var nearest_node = null
	
	for t in targets:
		# EXCLUSION: Don't hit yourself or your family
		if t == actor or t.get_parent() == actor or t == actor.get_parent():
			continue
			
		if not is_instance_valid(t):
			continue
		
		var d = actor.global_position.distance_to(t.global_position)
		if d < nearest_dist:
			nearest_dist = d
			nearest_node = t
	
	target = nearest_node
